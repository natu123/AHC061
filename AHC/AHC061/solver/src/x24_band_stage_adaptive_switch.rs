use std::collections::HashMap;

use crate::{
    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route, x06_expert_switch_hybrid,
    x11_contest_frontier_recovery, x18_robust_minmax_guard, x19_frontier_recovery_sweep, AiModel, Game,
    State,
};

fn score_weights(
    game: &Game,
    uncertainty: f64,
    phase: f64,
    conf: f64,
    leader_gap: f64,
) -> (f64, f64, f64, f64, f64) {
    let w01 = 0.18 + 0.08 * conf.min(1.5);
    let mut w04 = 0.42 + 0.25 * (1.0 - phase).max(0.0).min(1.0);
    let mut w06 = 0.30 + 0.10 * phase + 0.15 * conf;
    let mut w18 = 0.08 + 0.18 * uncertainty;
    let mut w11 = 0.02 + 0.05 * (leader_gap / 1200.0).min(1.0);

    if game.m == 4 && uncertainty > 0.18 {
        w04 += 0.04;
        w06 += 0.06;
    }
    if phase > 0.72 {
        w18 += 0.09;
        w04 *= 0.88;
    }
    if conf > 1.3 {
        w18 += 0.12;
        w11 += 0.02;
        w06 -= 0.06;
        w04 *= 0.94;
    }
    if game.m >= 6 && phase < 0.35 {
        w11 += 0.02;
    }

    let total = (w01 + w04 + w06 + w18 + w11).max(1e-12);
    (w01 / total, w04 / total, w06 / total, w18 / total, w11 / total)
}

fn weighted_votes(
    game: &Game,
    state: &State,
    models: &[AiModel],
    uncertainty: f64,
    phase: f64,
    conf: f64,
    leader_gap: f64,
) -> Vec<((usize, usize), f64)> {
    let (w01, w04, w06, w18, w11) = score_weights(game, uncertainty, phase, conf, leader_gap);
    let mut votes = vec![
        (x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models), w01),
        (x04_macro_route::choose_move_x04_macro_route(game, state, models), w04),
        (x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models), w06),
    ];
    if uncertainty >= 0.18 || conf > 1.0 {
        votes.push((x18_robust_minmax_guard::choose_move_x18_robust_minmax_guard(game, state, models), w18));
    }
    if phase > 0.52 && game.m >= 5 {
        votes.push((
            x19_frontier_recovery_sweep::choose_move_x19_frontier_recovery_sweep(game, state, models),
            w11,
        ));
    }
    if phase < 0.45 && uncertainty < 0.26 {
        votes.push((
            x11_contest_frontier_recovery::choose_move_x11_contest_frontier_recovery(
                game, state, models,
            ),
            0.03,
        ));
    }
    if uncertainty > 0.32 && game.m <= 5 {
        votes.push((x02_monte_carlo::choose_move_monte_carlo(game, state, models), 0.04));
    }
    votes
}

fn conflict_signal(conflict_map: &[Vec<f64>], mv: (usize, usize), game: &Game) -> f64 {
    let (x, y) = mv;
    let p = conflict_map[x][y];
    let lv = game.u.max(1) as f64;
    p + 0.000_000_5 * lv * game.v[x][y] as f64
}

fn two_step_probe(
    game: &Game,
    state: &State,
    models: &[AiModel],
    first_mv: (usize, usize),
    top2: &[((usize, usize), (usize, usize), f64)],
    keep_conflict: bool,
) -> f64 {
    let mut cur = state.clone();
    let scores = crate::calc_scores(game, state);
    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let _s0 = scores[0] as f64;
    let mut leaders = vec![false; game.m];
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
            let mut best_v = f64::NEG_INFINITY;
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
                if keep_conflict {
                    v -= 0.35 * conflict[mv.0][mv.1];
                }
                if v > best_v {
                    best_v = v;
                    next_mv = mv;
                }
            }
            moves.push(next_mv);
            if crate::uncertainty_risk(top2) >= 0.20 {
                moves.extend(crate::build_secondary_ai_moves(&scores_now, top2, 1));
            } else {
                moves.extend(top2.iter().map(|x| x.0));
            }
        }
        cur = crate::simulate_turn(game, &cur, &moves);
        total += (0.84_f64).powi(step as i32) * crate::strategic_score(game, &cur);
    }
    total
}

pub(super) fn choose_move_x24_band_stage_adaptive_switch(
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
    let conflict_map = crate::estimate_conflict_map(game, state, models);
    let conf = (conflict_map.iter().flatten().sum::<f64>() / (game.n as f64 * game.n as f64)).min(5.0);

    let cands = crate::get_candidates(game, state, 0);
    if cands.is_empty() {
        return state.pos[0];
    }
    if cands.len() <= 3 {
        return cands[0];
    }

    let mut vote_map: HashMap<(usize, usize), f64> = HashMap::new();
    for (mv, w) in weighted_votes(game, state, models, uncertainty, phase, conf, leader_gap) {
        *vote_map.entry(mv).or_insert(0.0) += w;
    }

    let mut leaders = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai {
            leaders[p] = true;
        }
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
        let c = conflict_signal(&conflict_map, mv, game);
        let entry = vote_map.entry(mv).or_insert(0.0);
        *entry += 0.020 * local + 0.006 * game.v[mv.0][mv.1] as f64 / 10000.0 - 0.09 * c;
    }

    let mut pool: Vec<_> = vote_map.into_iter().collect();
    if pool.is_empty() {
        return x04_macro_route::choose_move_x04_macro_route(game, state, models);
    }
    pool.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    pool.truncate(pool.len().max(7).min(16));

    let mut best_mv = pool[0].0;
    let mut best_score = f64::NEG_INFINITY;
    for &(mv, w) in pool.iter().take(10) {
        let mut p1 = Vec::with_capacity(game.m);
        p1.push(mv);
        p1.extend(top2.iter().map(|x| x.0).take(game.m - 1));
        let mut p2 = Vec::with_capacity(game.m);
        p2.push(mv);
        p2.extend(crate::build_secondary_ai_moves(&scores, &top2, 1).into_iter().take(game.m - 1));

        let s1 = crate::simulate_turn(game, state, &p1);
        let s2 = crate::simulate_turn(game, state, &p2);
        let rollout = two_step_probe(game, state, models, mv, &top2, uncertainty > 0.30);
        let score = 0.58 * crate::strategic_score(game, &s1)
            + 0.26 * crate::strategic_score(game, &s2)
            + 0.13 * rollout
            + 0.12 * crate::frontier_potential(game, &s1)
            + 2.0 * w
            - 1.2 * conflict_map[mv.0][mv.1];
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
