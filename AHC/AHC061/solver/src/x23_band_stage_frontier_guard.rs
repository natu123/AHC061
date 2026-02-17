use std::collections::HashMap;

use crate::{
    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route, x06_expert_switch_hybrid,
    x11_contest_frontier_recovery, x18_robust_minmax_guard, AiModel, Game, State,
};

fn frontier_signal(game: &Game, state: &State, mv: (usize, usize)) -> f64 {
    let mut sig = 0.0;
    let x = mv.0 as isize;
    let y = mv.1 as isize;
    const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for (dx, dy) in DIRS {
        let nx = x + dx;
        let ny = y + dy;
        if crate::in_bounds(game.n, nx, ny) {
            let ux = nx as usize;
            let uy = ny as usize;
            let owner = state.owner[ux][uy];
            let v = game.v[ux][uy] as f64;
            if owner == -1 {
                sig += v * 0.95;
            } else if owner == 0 {
                sig += 0.40 * v;
            } else if owner == 0 && state.level[ux][uy] == 1 {
                sig += 0.25 * v;
            }
            if state.level[ux][uy] == 1 && owner != 0 {
                sig += 0.10 * v;
            }
        }
    }
    sig
}

fn weighted_votes(
    game: &Game,
    state: &State,
    models: &[AiModel],
    uncertainty: f64,
    phase: f64,
    conflict_peak: f64,
) -> Vec<((usize, usize), f64)> {
    let w01 = 0.18 + 0.10 * (1.0 - uncertainty);
    let mut w04 = 0.40 + 0.35 * (1.0 - phase);
    let mut w06 = 0.28 + 0.15 * phase;
    let mut w18 = 0.08 + 0.18 * uncertainty * conflict_peak;
    let w11 = 0.04 + 0.20 * (phase * (1.0 - uncertainty)).min(1.0);
    let w02 = if game.m <= 5 { 0.05 } else { 0.01 };
    if phase > 0.65 {
        w18 += 0.10;
        w04 *= 0.90;
    }
    if uncertainty > 0.30 && conflict_peak > 0.6 {
        w18 += 0.12;
        w06 -= 0.05;
    }
    let total = (w01 + w04 + w06 + w18 + w11 + w02).max(1e-12);
    let mut votes = vec![
        (
            x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models),
            w01 / total,
        ),
        (
            x04_macro_route::choose_move_x04_macro_route(game, state, models),
            w04 / total,
        ),
        (
            x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models),
            w06 / total,
        ),
        (
            x18_robust_minmax_guard::choose_move_x18_robust_minmax_guard(game, state, models),
            w18 / total,
        ),
        (
            x11_contest_frontier_recovery::choose_move_x11_contest_frontier_recovery(game, state, models),
            w11 / total,
        ),
    ];
    if game.m <= 5 {
        votes.push((
            x02_monte_carlo::choose_move_monte_carlo(game, state, models),
            w02 / total,
        ));
    }
    votes
}

fn score_with_guard(
    game: &Game,
    state: &State,
    mv: (usize, usize),
    scores: &[i64],
    phase: f64,
    phase_gap: f64,
    conflict_map: &[Vec<f64>],
    local: f64,
    leaders: &[bool],
    phase_gap_risk: f64,
) -> f64 {
    let s0 = scores[0] as f64;
    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let frontier = frontier_signal(game, state, mv);
    let conflict = conflict_map[mv.0][mv.1];
    let risk = conflict * (2.0 + phase_gap * 1e-5) * (1.0 + phase);
    let safety = -1.2 * (state.level[mv.0][mv.1] as f64).powf(1.1) * risk.min(5.0);
    let move_pressure = if phase_gap_risk > 1_000.0 { 0.0012 } else { 0.0 };
    let mut score = local + 0.03 * frontier + 0.02 * phase_gap + move_pressure + safety;
    if mv == state.pos[0] && phase > 0.6 {
        score -= 0.30;
    }
    score += crate::evaluate_local_move(
        game,
        state,
        mv,
        scores,
        s0,
        max_ai,
        phase,
        conflict_map,
        state.pos[0],
        leaders,
    ) * 0.01;
    score
}

fn rollout_mix(game: &Game, state: &State, models: &[AiModel], first_mv: (usize, usize), top2: &[((usize, usize), (usize, usize), f64)], phase: f64) -> f64 {
    let mut cur = state.clone();
    let mut gain = 0.0;
    for step in 0..2 {
        let mut moves = Vec::with_capacity(game.m);
        if step == 0 {
            moves.push(first_mv);
        } else {
            let cands = crate::get_candidates(game, &cur, 0);
            if cands.is_empty() {
                break;
            }
            let conflict = crate::estimate_conflict_map(game, &cur, models);
            let scores = crate::calc_scores(game, &cur);
            let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
            let s0 = scores[0] as f64;
            let mut leaders = vec![false; game.m];
            for p in 1..game.m {
                if scores[p] == max_ai {
                    leaders[p] = true;
                }
            }
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
                    cur.turn as f64 / game.t as f64,
                    &conflict,
                    cur.pos[0],
                    &leaders,
                );
                v -= 0.0015 * frontier_signal(game, &cur, mv);
                if v > best_v {
                    best_v = v;
                    best_mv = mv;
                }
            }
            moves.push(best_mv);
            if phase < 0.70 {
                moves.extend(crate::build_secondary_ai_moves(&scores, top2, 1));
            } else {
                moves.extend(top2.iter().map(|x| x.0));
            }
        }
        cur = crate::simulate_turn(game, &cur, &moves);
        gain += (0.9_f64).powi(step as i32) * crate::strategic_score(game, &cur);
    }
    gain
}

pub(super) fn choose_move_x23_band_stage_frontier_guard(
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
    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let phase = state.turn as f64 / game.t as f64;
    let phase_gap = ((max_ai as f64 - scores[0] as f64).abs()).min(100_000.0);
    let conflict_map = crate::estimate_conflict_map(game, state, models);
    let conflict_peak = conflict_map
        .iter()
        .flatten()
        .fold(0.0_f64, |a, b| if *b > a { *b } else { a });

    let mut leaders = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai {
            leaders[p] = true;
        }
    }

    let cands = crate::get_candidates(game, state, 0);
    if cands.is_empty() {
        return state.pos[0];
    }
    if cands.len() <= 4 {
        return cands[0];
    }

    let mut vote_map: HashMap<(usize, usize), f64> = HashMap::new();
    for (mv, w) in weighted_votes(game, state, models, uncertainty, phase, conflict_peak) {
        *vote_map.entry(mv).or_insert(0.0) += w;
    }

    let s0 = scores[0] as f64;
    for &mv in cands.iter().take(20) {
        let local = crate::evaluate_local_move(
            game,
            state,
            mv,
            &scores,
            s0,
            max_ai,
            phase,
            &conflict_map,
            state.pos[0],
            &leaders,
        );
        let risk_guard = score_with_guard(game, state, mv, &scores, phase, phase_gap, &conflict_map, local, &leaders, phase_gap);
        let entry = vote_map.entry(mv).or_insert(0.0);
        *entry += 0.02 * local + 0.001 * risk_guard + 0.000_02 * frontier_signal(game, state, mv);
    }

    let mut pool: Vec<_> = vote_map.into_iter().collect();
    if pool.is_empty() {
        return x04_macro_route::choose_move_x04_macro_route(game, state, models);
    }
    pool.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    pool.truncate(pool.len().max(8).min(16));

    let mut best_mv = pool[0].0;
    let mut best_score = f64::NEG_INFINITY;
    for &(mv, w) in pool.iter().take(10) {
        let mut primary = Vec::with_capacity(game.m);
        primary.push(mv);
        primary.extend(top2.iter().map(|x| x.0).take(game.m - 1));
        let mut secondary = Vec::with_capacity(game.m);
        secondary.push(mv);
        secondary.extend(crate::build_secondary_ai_moves(&scores, &top2, 1).into_iter().take(game.m - 1));
        let s1 = crate::simulate_turn(game, state, &primary);
        let s2 = crate::simulate_turn(game, state, &secondary);
        let core = 0.54 * crate::strategic_score(game, &s1) + 0.28 * crate::strategic_score(game, &s2);
        let rollout = 0.18 * rollout_mix(game, state, models, mv, &top2, phase);
        let frontier = 0.12 * frontier_signal(game, state, mv);
        let risk = 0.15 * conflict_map[mv.0][mv.1] + 0.02 * (state.level[mv.0][mv.1] as f64);
        let score = core + rollout + frontier + w - risk;
        if score > best_score {
            best_score = score;
            best_mv = mv;
        }
    }

    if best_score.is_finite() {
        best_mv
    } else {
        x18_robust_minmax_guard::choose_move_x18_robust_minmax_guard(game, state, models)
    }
}
