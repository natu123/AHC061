use std::collections::{HashMap, HashSet};
use std::env;

use crate::{
    x04_macro_route, x06_expert_switch_hybrid, x18_robust_minmax_guard, x19_frontier_recovery_sweep,
    x20_band_stage_ensemble, x21_band_stage_adaptive_guard, x22_band_stage_recovery_boost,
    x25_race_adaptive_recovery, x26_reactive_frontier_pressure, AiModel, Game, State,
};

fn env_f64(name: &str, default: f64, min: f64, max: f64) -> f64 {
    env::var(name)
        .ok()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(default)
        .clamp(min, max)
}

fn env_usize(name: &str, default: usize, min: usize, max: usize) -> usize {
    env::var(name)
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(default)
        .clamp(min, max)
}

fn leader_flags(game: &Game, scores: &[i64]) -> Vec<bool> {
    let mut flags = vec![false; game.m];
    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    for p in 1..game.m {
        if scores[p] == max_ai {
            flags[p] = true;
        }
    }
    flags
}

fn role_signal(
    game: &Game,
    state: &State,
    mv: (usize, usize),
    leaders: &[bool],
    phase: f64,
) -> f64 {
    let (x, y) = mv;
    let owner = state.owner[x][y];
    let level = state.level[x][y];
    let v = game.v[x][y] as f64;

    if owner == -1 {
        (1.04 - 0.28 * phase).clamp(0.50, 1.10) * v
    } else if owner == 0 {
        if level < game.u {
            (0.68 + 0.20 * (1.0 - phase)) * v * (game.u - level) as f64 / game.u as f64
        } else {
            -0.10 * v
        }
    } else {
        let opp = owner as usize;
        let lead_bonus = if leaders.get(opp).copied().unwrap_or(false) {
            1.0
        } else {
            0.0
        };
        if level == 1 {
            (1.15 + 0.55 * phase + 0.45 * lead_bonus) * v
        } else {
            (0.28 + 0.35 * phase + 0.20 * lead_bonus) * v / level as f64
        }
    }
}

fn neighborhood_signal(game: &Game, state: &State, mv: (usize, usize)) -> f64 {
    const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut gain = 0.0_f64;
    for (dx, dy) in DIRS {
        let nx = mv.0 as isize + dx;
        let ny = mv.1 as isize + dy;
        if !crate::in_bounds(game.n, nx, ny) {
            continue;
        }
        let ux = nx as usize;
        let uy = ny as usize;
        let v = game.v[ux][uy] as f64;
        match state.owner[ux][uy] {
            -1 => gain += 0.07 * v,
            0 => {
                if state.level[ux][uy] < game.u {
                    gain += 0.04 * v;
                }
            }
            _ => {
                if state.level[ux][uy] == 1 {
                    gain += 0.09 * v;
                } else {
                    gain += 0.03 * v / state.level[ux][uy] as f64;
                }
            }
        }
    }
    gain
}

fn rollout_move_value(
    game: &Game,
    state: &State,
    mv: (usize, usize),
    scores: &[i64],
    leaders: &[bool],
    phase: f64,
) -> f64 {
    let role = role_signal(game, state, mv, leaders, phase);
    let around = neighborhood_signal(game, state, mv);
    let dist = state.pos[0].0.abs_diff(mv.0) as f64 + state.pos[0].1.abs_diff(mv.1) as f64;

    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1) as f64;
    let s0 = scores[0] as f64;
    let gap = ((max_ai - s0) / s0.max(1.0)).clamp(-1.0, 2.0);
    let chase = if gap > 0.0 { 1.0 + 0.40 * gap } else { 0.92 };

    let owner = state.owner[mv.0][mv.1];
    let level = state.level[mv.0][mv.1];
    let mut score = chase * role + 0.10 * around - (0.03 + 0.02 * phase) * dist;
    if owner == 0 && level == game.u && mv == state.pos[0] {
        score -= 0.12 * game.v[mv.0][mv.1] as f64;
    }
    if owner > 0 && level >= 2 {
        score -= 0.06 * game.v[mv.0][mv.1] as f64;
    }
    score
}

fn pick_rollout_move(game: &Game, state: &State, phase: f64) -> (usize, usize) {
    let cands = crate::get_candidates(game, state, 0);
    if cands.is_empty() {
        return state.pos[0];
    }
    if cands.len() == 1 {
        return cands[0];
    }
    let scores = crate::calc_scores(game, state);
    let leaders = leader_flags(game, &scores);
    let mut best_mv = cands[0];
    let mut best = f64::NEG_INFINITY;
    for &mv in &cands {
        let v = rollout_move_value(game, state, mv, &scores, &leaders, phase);
        if v > best {
            best = v;
            best_mv = mv;
        }
    }
    best_mv
}

fn scenario_ai_moves(
    top2: &[((usize, usize), (usize, usize), f64)],
    secondary: &[(usize, usize)],
    leaders: &[bool],
    scenario_id: usize,
    step: usize,
) -> Vec<(usize, usize)> {
    let mut out = Vec::with_capacity(top2.len());
    for i in 0..top2.len() {
        let player = i + 1;
        let p1 = top2[i].0;
        let p2 = top2[i].1;
        let conf = top2[i].2;
        let sec = secondary.get(i).copied().unwrap_or(p1);
        let leader = leaders.get(player).copied().unwrap_or(false);

        let mv = match scenario_id {
            0 => p1,
            1 => sec,
            2 => {
                if conf < 0.70 || ((step + player) % 2 == 1) {
                    p2
                } else {
                    p1
                }
            }
            _ => {
                if leader {
                    if conf < 0.90 { p2 } else { p1 }
                } else if conf < 0.62 {
                    sec
                } else {
                    p1
                }
            }
        };
        out.push(mv);
    }
    out
}

fn advisor_suggestions(
    game: &Game,
    state: &State,
    models: &[AiModel],
    gap_ratio: f64,
    phase: f64,
) -> Vec<((usize, usize), f64)> {
    let mut w_x04 = env_f64("AHC_X67_W_X04", 0.22, 0.0, 2.0);
    let mut w_x06 = env_f64("AHC_X67_W_X06", 0.12, 0.0, 2.0);
    let mut w_x18 = env_f64("AHC_X67_W_X18", 0.10, 0.0, 2.0);
    let mut w_x19 = env_f64("AHC_X67_W_X19", 0.18, 0.0, 2.0);
    let mut w_x20 = env_f64("AHC_X67_W_X20", 0.08, 0.0, 2.0);
    let mut w_x21 = env_f64("AHC_X67_W_X21", 0.08, 0.0, 2.0);
    let w_x22 = env_f64("AHC_X67_W_X22", 0.05, 0.0, 2.0);
    let w_x25 = env_f64("AHC_X67_W_X25", 0.05, 0.0, 2.0);
    let mut w_x26 = env_f64("AHC_X67_W_X26", 0.12, 0.0, 2.0);

    if gap_ratio >= 0.20 {
        w_x04 += 0.10;
        w_x19 += 0.08;
        w_x26 += 0.08;
        w_x06 -= 0.03;
    } else if phase >= 0.70 {
        w_x18 += 0.06;
        w_x20 += 0.05;
        w_x21 += 0.05;
    } else {
        w_x06 += 0.03;
    }

    let total = (w_x04 + w_x06 + w_x18 + w_x19 + w_x20 + w_x21 + w_x22 + w_x25 + w_x26).max(1e-9);
    vec![
        (
            x04_macro_route::choose_move_x04_macro_route(game, state, models),
            w_x04 / total,
        ),
        (
            x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models),
            w_x06 / total,
        ),
        (
            x18_robust_minmax_guard::choose_move_x18_robust_minmax_guard(game, state, models),
            w_x18 / total,
        ),
        (
            x19_frontier_recovery_sweep::choose_move_x19_frontier_recovery_sweep(game, state, models),
            w_x19 / total,
        ),
        (
            x20_band_stage_ensemble::choose_move_x20_band_stage_ensemble(game, state, models),
            w_x20 / total,
        ),
        (
            x21_band_stage_adaptive_guard::choose_move_x21_band_stage_adaptive_guard(game, state, models),
            w_x21 / total,
        ),
        (
            x22_band_stage_recovery_boost::choose_move_x22_band_stage_recovery_boost(game, state, models),
            w_x22 / total,
        ),
        (
            x25_race_adaptive_recovery::choose_move_x25_race_adaptive_recovery(game, state, models),
            w_x25 / total,
        ),
        (
            x26_reactive_frontier_pressure::choose_move_x26_reactive_frontier_pressure(game, state, models),
            w_x26 / total,
        ),
    ]
}

fn evaluate_candidate_rollout(
    game: &Game,
    state: &State,
    models: &[AiModel],
    first_mv: (usize, usize),
    phase0: f64,
    gap_ratio: f64,
) -> f64 {
    let horizon_base = env_usize("AHC_X67_HORIZON_BASE", 3, 2, 5);
    let horizon_late = env_usize("AHC_X67_HORIZON_LATE", 2, 1, 4);
    let horizon = if phase0 < 0.76 { horizon_base } else { horizon_late };
    let scenario_env = env_usize("AHC_X67_SCENARIO_COUNT", 0, 0, 6);
    let scenario_count = if scenario_env > 0 {
        scenario_env
    } else if game.m >= 6 || gap_ratio >= 0.20 {
        4
    } else {
        3
    };
    let mut scenario_values = Vec::with_capacity(scenario_count);

    for scenario_id in 0..scenario_count {
        let mut cur = state.clone();
        let mut total = 0.0_f64;
        for step in 0..horizon {
            let phase = (cur.turn as f64 / game.t as f64).clamp(0.0, 1.0);
            let prev_scores = crate::calc_scores(game, &cur);
            let leaders = leader_flags(game, &prev_scores);
            let my_mv = if step == 0 {
                first_mv
            } else {
                pick_rollout_move(game, &cur, phase)
            };
            let top2 = crate::choose_predicted_ai_top2_moves(game, &cur, models);
            let sec_cap = if game.m >= 7 {
                3
            } else if game.m >= 5 {
                2
            } else {
                1
            };
            let secondary = crate::build_secondary_ai_moves(&prev_scores, &top2, sec_cap);
            let ai_moves = scenario_ai_moves(&top2, &secondary, &leaders, scenario_id, step);

            let mut all_moves = Vec::with_capacity(game.m);
            all_moves.push(my_mv);
            all_moves.extend(ai_moves);

            let mut next = crate::simulate_turn(game, &cur, &all_moves);
            next.turn = (cur.turn + 1).min(game.t);

            let next_scores = crate::calc_scores(game, &next);
            let my_prev = prev_scores[0] as f64;
            let my_next = next_scores[0] as f64;
            let lead_prev = prev_scores.iter().skip(1).copied().max().unwrap_or(1).max(1) as f64;
            let lead_next = next_scores.iter().skip(1).copied().max().unwrap_or(1).max(1) as f64;

            let ratio_prev = my_prev / lead_prev.max(1.0);
            let ratio_next = my_next / lead_next.max(1.0);
            let ratio_gain = (ratio_next - ratio_prev).clamp(-2.0, 2.0);
            let sabotage = ((lead_prev - lead_next) / lead_prev.max(1.0)).clamp(-1.0, 1.0);
            let growth = ((my_next - my_prev) / my_prev.max(1.0)).clamp(-1.0, 1.0);

            let strategic = crate::strategic_score(game, &next);
            let role = role_signal(game, &cur, my_mv, &leaders, phase);
            let step_value = 0.72 * strategic
                + 20_000.0 * ratio_gain
                + 9_000.0 * sabotage
                + 5_500.0 * growth
                + 0.05 * role;
            total += (0.91_f64).powi(step as i32) * step_value;

            cur = next;
        }
        scenario_values.push(total);
    }

    if scenario_values.is_empty() {
        return f64::NEG_INFINITY;
    }

    let mean = scenario_values.iter().sum::<f64>() / scenario_values.len() as f64;
    scenario_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let min = scenario_values[0];
    let q25 = scenario_values[(scenario_values.len() - 1) / 4];
    let risk = if gap_ratio >= 0.20 {
        env_f64("AHC_X67_RISK_DEFICIT", 0.14, 0.0, 0.8)
    } else if gap_ratio >= 0.05 {
        env_f64("AHC_X67_RISK_NEUTRAL", 0.24, 0.0, 0.8)
    } else {
        env_f64("AHC_X67_RISK_LEAD", 0.36, 0.0, 0.8)
    };
    (1.0 - risk) * mean + risk * min + 0.08 * q25
}

pub(super) fn choose_move_x67_gear_shift_hybrid(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
    let candidates = crate::get_candidates(game, state, 0);
    if candidates.is_empty() {
        return state.pos[0];
    }
    if candidates.len() == 1 {
        return candidates[0];
    }

    let scores = crate::calc_scores(game, state);
    let s0 = scores[0] as f64;
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let phase = (state.turn as f64 / game.t as f64).clamp(0.0, 1.0);
    let gap_ratio = ((max_ai_i64 as f64 - s0) / s0.max(1.0)).clamp(-1.0, 2.0);
    let leaders = leader_flags(game, &scores);
    let conflict = crate::estimate_conflict_map(game, state, models);

    let advisor_votes = advisor_suggestions(game, state, models, gap_ratio, phase);
    let mut vote_map = HashMap::<(usize, usize), f64>::new();
    for (mv, w) in advisor_votes {
        *vote_map.entry(mv).or_insert(0.0) += w;
    }

    let mut local_map = HashMap::<(usize, usize), f64>::new();
    let mut local_rank = Vec::<((usize, usize), f64)>::new();
    let mut sabotage_rank = Vec::<((usize, usize), f64)>::new();
    let mut stable_rank = Vec::<((usize, usize), f64)>::new();

    for &mv in &candidates {
        let local = crate::evaluate_local_move(
            game,
            state,
            mv,
            &scores,
            s0,
            max_ai_i64,
            phase,
            &conflict,
            state.pos[0],
            &leaders,
        );
        local_map.insert(mv, local);

        let v = game.v[mv.0][mv.1] as f64;
        let risk = conflict[mv.0][mv.1] * v;
        let role = role_signal(game, state, mv, &leaders, phase);
        let around = neighborhood_signal(game, state, mv);

        local_rank.push((mv, local));
        sabotage_rank.push((mv, role + 0.08 * around - (0.18 + 0.25 * phase) * risk));
        stable_rank.push((mv, local + 0.05 * role - (0.58 + 0.15 * phase) * risk));
    }

    local_rank.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    sabotage_rank.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    stable_rank.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let mut pool = Vec::<(usize, usize)>::new();
    let mut seen = HashSet::<(usize, usize)>::new();
    for list in [
        local_rank.iter().take(6),
        sabotage_rank.iter().take(5),
        stable_rank.iter().take(5),
    ] {
        for (mv, _) in list {
            if seen.insert(*mv) {
                pool.push(*mv);
            }
        }
    }

    let mut voted: Vec<((usize, usize), f64)> = vote_map.iter().map(|(k, v)| (*k, *v)).collect();
    voted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    for (mv, _) in voted.into_iter().take(8) {
        if seen.insert(mv) {
            pool.push(mv);
        }
    }

    if pool.is_empty() {
        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
    }

    let cap = pool.len().min(14);
    let local_weight = env_f64("AHC_X67_LOCAL_WEIGHT", 0.16, 0.0, 0.8);
    let vote_weight = env_f64("AHC_X67_VOTE_WEIGHT", 15000.0, 0.0, 100000.0);
    let mut best_mv = pool[0];
    let mut best_score = f64::NEG_INFINITY;
    for &mv in pool.iter().take(cap) {
        let rollout = evaluate_candidate_rollout(game, state, models, mv, phase, gap_ratio);
        let local = *local_map.get(&mv).unwrap_or(&0.0);
        let vote = *vote_map.get(&mv).unwrap_or(&0.0);
        let total = rollout + local_weight * local + vote_weight * vote;
        if total > best_score {
            best_score = total;
            best_mv = mv;
        }
    }

    if best_score.is_finite() {
        best_mv
    } else {
        local_rank[0].0
    }
}
