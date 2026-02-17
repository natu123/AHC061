use std::collections::HashMap;

use crate::{
    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route, x06_expert_switch_hybrid,
    x11_contest_frontier_recovery, x18_robust_minmax_guard, x19_frontier_recovery_sweep, AiModel, Game,
    State,
};

fn propose_weights(
    game: &Game,
    uncertainty: f64,
    phase: f64,
    conflict_pressure: f64,
    gap: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    let w01 = 0.12 + 0.22 * (1.0 - phase).max(0.0).min(1.0);
    let mut w04 = 0.39 + 0.36 * (1.0 - uncertainty).max(0.0).min(1.0);
    let mut w06 = 0.30 + 0.10 * phase;
    let mut w18 = 0.07 + 0.28 * uncertainty * conflict_pressure;
    let w19 = 0.04 + 0.20 * (gap / 1_00_000.0).min(1.0);
    let mut w02 = 0.01;

    if game.m <= 5 && phase < 0.55 {
        w02 += 0.10;
        w04 += 0.05;
    }
    if conflict_pressure > 1.3 {
        w18 += 0.14;
        w06 -= 0.05;
    }
    if phase > 0.72 {
        w18 += 0.12;
        w04 *= 0.90;
    }
    if uncertainty < 0.18 {
        w04 += 0.07;
        w02 *= 1.4;
    }

    let total = (w01 + w04 + w06 + w18 + w19 + w02).max(1e-12);
    (w01 / total, w04 / total, w06 / total, w18 / total, w19 / total, w02 / total)
}

fn weighted_votes(
    game: &Game,
    state: &State,
    models: &[AiModel],
    uncertainty: f64,
    phase: f64,
    conflict_pressure: f64,
    gap: f64,
) -> Vec<((usize, usize), f64)> {
    let (w01, w04, w06, w18, w19, w02) = propose_weights(game, uncertainty, phase, conflict_pressure, gap);
    let mv1 = x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models);
    let mv4 = x04_macro_route::choose_move_x04_macro_route(game, state, models);
    let mv6 = x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
    let mut votes = vec![(mv1, w01), (mv4, w04), (mv6, w06)];

    if uncertainty > 0.18 || conflict_pressure > 1.2 {
        let mv18 = x18_robust_minmax_guard::choose_move_x18_robust_minmax_guard(game, state, models);
        votes.push((mv18, w18));
        if phase > 0.55 {
            let mv19 = x19_frontier_recovery_sweep::choose_move_x19_frontier_recovery_sweep(
                game,
                state,
                models,
            );
            votes.push((mv19, w19));
        }
    }
    if game.m <= 5 && phase < 0.58 && uncertainty < 0.30 {
        let mv2 = x02_monte_carlo::choose_move_monte_carlo(game, state, models);
        votes.push((mv2, w02));
    }
    if phase < 0.35 && uncertainty > 0.22 {
        let mv11 = x11_contest_frontier_recovery::choose_move_x11_contest_frontier_recovery(
            game, state, models,
        );
        votes.push((mv11, 0.04));
    }
    votes
}

fn conflict_pressure_value(conflict_map: &[Vec<f64>], mv: (usize, usize), game: &Game, state: &State) -> f64 {
    let p = conflict_map[mv.0][mv.1];
    let v = game.v[mv.0][mv.1] as f64;
    let lvl = state.level[mv.0][mv.1] as f64 + 1.0;
    p * v / lvl
}

fn evaluate_rollout(game: &Game, state: &State, models: &[AiModel], first_mv: (usize, usize), guard: bool) -> f64 {
    let mut cur = state.clone();
    let top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
    let scores = crate::calc_scores(game, state);
    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let conflict = crate::estimate_conflict_map(game, state, models);
    let s0 = scores[0] as f64;
    let mut leaders = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai {
            leaders[p] = true;
        }
    }
    let mut total = 0.0;
    for step in 0..2 {
        let mut moves = Vec::with_capacity(game.m);
        if step == 0 {
            moves.push(first_mv);
        } else {
            let cands = crate::get_candidates(game, &cur, 0);
            if cands.is_empty() {
                break;
            }
            let phase = cur.turn as f64 / game.t as f64;
            let mut best_mv = cands[0];
            let mut best_v = f64::NEG_INFINITY;
            for &mv in &cands {
                let mut v = crate::evaluate_local_move(
                    game,
                    &cur,
                    mv,
                    &scores,
                    s0,
                    max_ai,
                    phase,
                    &conflict,
                    cur.pos[0],
                    &leaders,
                );
                if guard {
                    v -= 0.22 * conflict[mv.0][mv.1] * game.v[mv.0][mv.1] as f64 / 100_000.0;
                }
                if v > best_v {
                    best_v = v;
                    best_mv = mv;
                }
            }
            moves.push(best_mv);
            if crate::uncertainty_risk(&top2) >= 0.22 {
                moves.extend(crate::build_secondary_ai_moves(&scores, &top2, 1));
            } else {
                moves.extend(top2.iter().map(|x| x.0));
            }
        }
        cur = crate::simulate_turn(game, &cur, &moves);
        total += (0.88_f64).powi(step as i32) * crate::strategic_score(game, &cur);
    }
    total
}

pub(super) fn choose_move_x21_band_stage_adaptive_guard(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
    if !(4..=6).contains(&game.m) {
        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
    }

    let top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
    let uncertainty = crate::uncertainty_risk(&top2);
    let scores = crate::calc_scores(game, state);
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let phase = state.turn as f64 / game.t as f64;
    let gap = ((max_ai_i64 as f64 - scores[0] as f64).abs()).min(100_000.0);
    let conflict_map = crate::estimate_conflict_map(game, state, models);
    let conf = (conflict_map
        .iter()
        .flat_map(|r| r.iter())
        .sum::<f64>()
        / (game.n as f64 * game.n as f64))
        .min(10.0);

    let cands = crate::get_candidates(game, state, 0);
    if cands.is_empty() {
        return state.pos[0];
    }
    if cands.len() <= 3 {
        return cands[0];
    }

    let mut vote: HashMap<(usize, usize), f64> = HashMap::new();
    for (mv, w) in weighted_votes(game, state, models, uncertainty, phase, conf, gap) {
        *vote.entry(mv).or_insert(0.0) += w;
    }

    let mut is_leader = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai_i64 {
            is_leader[p] = true;
        }
    }
    let s0 = scores[0] as f64;
    for &mv in cands.iter().take(18) {
        let local = crate::evaluate_local_move(
            game,
            state,
            mv,
            &scores,
            s0,
            max_ai_i64,
            phase,
            &conflict_map,
            state.pos[0],
            &is_leader,
        );
        let guard = if conf > 1.2 { -0.06 } else { 0.00 };
        let conflict_pen = conflict_pressure_value(&conflict_map, mv, game, state);
        let bonus = if conflict_pen > 0.35 { -0.14 } else { 0.0 };
        let entry = vote.entry(mv).or_insert(0.0);
        *entry += 0.02 * local + bonus + conf * 0.0 + guard;
    }

    let mut pool: Vec<_> = vote.into_iter().collect();
    if pool.is_empty() {
        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
    }
    pool.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    pool.truncate(pool.len().max(8).min(16));

    let mut best_mv = pool[0].0;
    let mut best_score = f64::NEG_INFINITY;
    for &(mv, wv) in pool.iter().take(12) {
        let mut primary = Vec::with_capacity(game.m);
        primary.push(mv);
        primary.extend(top2.iter().map(|x| x.0).take(game.m - 1));
        let mut secondary = Vec::with_capacity(game.m);
        secondary.push(mv);
        secondary.extend(crate::build_secondary_ai_moves(&scores, &top2, 1));

        let s_primary = crate::simulate_turn(game, state, &primary);
        let s_secondary = crate::simulate_turn(game, state, &secondary);
        let core = 0.56 * crate::strategic_score(game, &s_primary)
            + 0.26 * crate::strategic_score(game, &s_secondary);
        let local = crate::evaluate_local_move(
            game,
            state,
            mv,
            &scores,
            s0,
            max_ai_i64,
            phase,
            &conflict_map,
            state.pos[0],
            &is_leader,
        );
        let rollout = 0.20 * evaluate_rollout(
            game,
            state,
            models,
            mv,
            uncertainty > 0.30 && conf > 1.1,
        );
        let frontier = 0.10 * crate::frontier_potential(game, &s_primary);
        let risk = 0.12 * conflict_map[mv.0][mv.1];
        let score = core + rollout + frontier + 0.02 * local + 2.0 * wv - risk;
        if score > best_score {
            best_score = score;
            best_mv = mv;
        }
    }

    if best_score.is_finite() {
        best_mv
    } else {
        x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models)
    }
}
