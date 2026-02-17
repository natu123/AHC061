use std::collections::HashMap;

use crate::{
    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route, x06_expert_switch_hybrid,
    x11_contest_frontier_recovery, x13_frontier_consensus, x18_robust_minmax_guard, AiModel, Game, State,
};

fn recovery_weights(
    phase: f64,
    uncertainty: f64,
    leader_gap: f64,
    conf: f64,
    m: usize,
) -> (f64, f64, f64, f64, f64) {
    let w01 = 0.15 + 0.10 * (1.0 - phase);
    let mut w04 = 0.33 + 0.22 * (1.0 - uncertainty);
    let mut w06 = 0.28 + 0.16 * phase;
    let mut w18 = 0.10 + 0.18 * uncertainty * conf;
    let mut w13 = 0.14 + 0.06 * (1.0 - conf);
    let mut w2 = if m <= 5 && phase < 0.45 {
        (0.02 + 0.20 * (1.0 - uncertainty)).min(0.12)
    } else {
        0.02
    };

    if conf > 1.1 {
        w18 += 0.14;
        w04 -= 0.08;
    }
    if uncertainty > 0.30 {
        w18 += 0.06;
        w06 += 0.05;
    }
    if leader_gap > 2200.0 {
        w13 += 0.03;
        w06 -= 0.03;
    }
    if m <= 5 && phase < 0.45 {
        w04 += 0.05;
    }

    if uncertainty < 0.30 && phase >= 0.40 {
        w2 += 0.04;
    }

    let total = (w01 + w04 + w06 + w18 + w13 + w2).max(1e-12);
    (w01 / total, w04 / total, w06 / total, w18 / total, (w13 + w2) / total)
}

fn propose_votes(
    game: &Game,
    state: &State,
    models: &[AiModel],
    uncertainty: f64,
    phase: f64,
    leader_gap: f64,
    conf: f64,
) -> Vec<((usize, usize), f64)> {
    let (w01, w04, w06, w18, w13plus) = recovery_weights(
        phase,
        uncertainty,
        leader_gap,
        conf,
        game.m,
    );
    let mut votes = vec![
        (x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models), w01),
        (x04_macro_route::choose_move_x04_macro_route(game, state, models), w04),
        (
            x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models),
            w06,
        ),
    ];
    votes.push((
        x18_robust_minmax_guard::choose_move_x18_robust_minmax_guard(game, state, models),
        w18,
    ));
    votes.push((x13_frontier_consensus::choose_move_x13_frontier_consensus(game, state, models), w13plus));
    if uncertainty > 0.2 || conf > 1.0 {
        votes.push((
            x11_contest_frontier_recovery::choose_move_x11_contest_frontier_recovery(
                game, state, models,
            ),
            0.03,
        ));
    }
    if uncertainty < 0.28 && phase < 0.60 && game.m <= 5 {
        votes.push((x02_monte_carlo::choose_move_monte_carlo(game, state, models), 0.04));
    }
    votes
}

fn pressure_score(game: &Game, state: &State, mv: (usize, usize)) -> f64 {
    let (x, y) = mv;
    let mut penalty = 0.0;
    let dirs: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for (dx, dy) in &dirs {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if crate::in_bounds(game.n, nx, ny) {
            let ux = nx as usize;
            let uy = ny as usize;
            let owner = state.owner[ux][uy];
            let level = state.level[ux][uy] as f64;
            let v = game.v[ux][uy] as f64;
            penalty += match owner {
                -1 => -0.12 * v,
                0 => {
                    if level < game.u as f64 {
                        -0.10 * v / 100.0 * level
                    } else {
                        0.06 * v
                    }
                }
                _ => 0.03 * v / level.max(1.0),
            };
        }
    }
    penalty
}

fn build_rollout(
    game: &Game,
    state: &State,
    models: &[AiModel],
    first_mv: (usize, usize),
    top2: &[((usize, usize), (usize, usize), f64)],
    risk_mode: bool,
) -> f64 {
    let mut cur = state.clone();
    let scores = crate::calc_scores(game, state);
    let mut leaders = vec![false; game.m];
    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    for p in 1..game.m {
        if scores[p] == max_ai {
            leaders[p] = true;
        }
    }

    let mut total = 0.0_f64;
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
            let scores_now = crate::calc_scores(game, &cur);
            let max_now = scores_now.iter().skip(1).copied().max().unwrap_or(1).max(1);
            let phase_now = cur.turn as f64 / game.t as f64;
            let mut next_mv = cands[0];
            let mut best = f64::NEG_INFINITY;
            for &mv in &cands {
                let mut v = crate::evaluate_local_move(
                    game,
                    &cur,
                    mv,
                    &scores_now,
                    scores_now[0] as f64,
                    max_now,
                    phase_now,
                    &conflict,
                    cur.pos[0],
                    &leaders,
                );
                v -= 0.12 * pressure_score(game, state, mv);
                if risk_mode {
                    v -= 0.25 * conflict[mv.0][mv.1];
                }
                if v > best {
                    best = v;
                    next_mv = mv;
                }
            }
            moves.push(next_mv);
            moves.extend(crate::build_secondary_ai_moves(&scores_now, top2, 1));
        }
        cur = crate::simulate_turn(game, &cur, &moves);
        total += (0.88_f64).powi(step as i32) * crate::strategic_score(game, &cur);
    }
    total
}

pub(super) fn choose_move_x25_race_adaptive_recovery(
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
    let leader_gap = ((max_ai as f64 - scores[0] as f64).abs()).min(100_000.0);
    let conflict = crate::estimate_conflict_map(game, state, models);
    let conf = conflict.iter().flatten().sum::<f64>() / (game.n as f64 * game.n as f64);

    let cands = crate::get_candidates(game, state, 0);
    if cands.is_empty() {
        return state.pos[0];
    }
    if cands.len() <= 3 {
        return cands[0];
    }

    let mut votes: HashMap<(usize, usize), f64> = HashMap::new();
    for (mv, w) in propose_votes(game, state, models, uncertainty, phase, leader_gap, conf) {
        *votes.entry(mv).or_insert(0.0) += w;
    }

    let mut leaders = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai {
            leaders[p] = true;
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
            max_ai,
            phase,
            &conflict,
            state.pos[0],
            &leaders,
        );
        let base = votes.entry(mv).or_insert(0.0);
        *base += 0.015 * local - 0.07 * pressure_score(game, state, mv);
    }

    let mut pool: Vec<_> = votes.into_iter().collect();
    if pool.is_empty() {
        return x04_macro_route::choose_move_x04_macro_route(game, state, models);
    }
    pool.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    pool.truncate(pool.len().max(8).min(16));

    let mut best_mv = pool[0].0;
    let mut best = f64::NEG_INFINITY;
    let risk_mode = uncertainty > 0.24 || conf > 1.05;
    for &(mv, w) in pool.iter().take(12) {
        let mut p1 = Vec::with_capacity(game.m);
        p1.push(mv);
        p1.extend(top2.iter().map(|x| x.0).take(game.m - 1));
        let mut p2 = Vec::with_capacity(game.m);
        p2.push(mv);
        p2.extend(crate::build_secondary_ai_moves(&scores, &top2, 1).into_iter().take(game.m - 1));
        let s1 = crate::simulate_turn(game, state, &p1);
        let s2 = crate::simulate_turn(game, state, &p2);
        let rollout = build_rollout(game, state, models, mv, &top2, risk_mode);
        let front = 0.10 * crate::frontier_potential(game, &s1);
        let score = 0.50 * crate::strategic_score(game, &s1)
            + 0.25 * crate::strategic_score(game, &s2)
            + 0.18 * rollout
            + 0.12 * w
            + front
            - 0.02 * conflict[mv.0][mv.1];
        if score > best {
            best = score;
            best_mv = mv;
        }
    }

    if best.is_finite() {
        best_mv
    } else {
        x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models)
    }
}
