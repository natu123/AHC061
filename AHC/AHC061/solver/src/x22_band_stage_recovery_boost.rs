use std::collections::HashMap;

use crate::{
    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route, x06_expert_switch_hybrid,
    x11_contest_frontier_recovery, x18_robust_minmax_guard, x19_frontier_recovery_sweep, AiModel, Game,
    State,
};

fn phase_weights(
    phase: f64,
    uncertainty: f64,
    gap: f64,
    m: usize,
) -> (f64, f64, f64, f64, f64, f64) {
    let base = 0.45 + 0.2 * (1.0 - phase);
    let mut w04 = base * (1.0 - 0.25 * uncertainty);
    let mut w01 = 0.20 + 0.12 * uncertainty;
    let mut w06 = 0.28 + 0.18 * phase;
    let mut w18 = 0.08 + (0.15 * uncertainty);
    let mut w19 = 0.05 + (0.20 * gap / 100_000.0).min(0.2);
    let w02 = if m <= 5 && phase < 0.60 { 0.12 } else { 0.02 };
    if phase > 0.70 {
        w18 += 0.10;
        w04 *= 0.85;
        w19 += 0.02;
    }
    if uncertainty > 0.30 && phase > 0.45 {
        w18 += 0.10;
        w06 += 0.06;
    }
    if gap < 2500.0 {
        w01 += 0.08;
        w04 += 0.04;
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
    gap: f64,
    conf: f64,
) -> Vec<((usize, usize), f64)> {
    let (w01, w04, w06, w18, w19, w02) = phase_weights(phase, uncertainty, gap, game.m);
    let mut votes = vec![
        (
            x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models),
            w01,
        ),
        (x04_macro_route::choose_move_x04_macro_route(game, state, models), w04),
        (
            x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models),
            w06,
        ),
    ];
    if uncertainty > 0.16 || conf > 1.0 {
        votes.push((
            x18_robust_minmax_guard::choose_move_x18_robust_minmax_guard(game, state, models),
            w18,
        ));
    }
    if phase > 0.40 && gap > 2_000.0 {
        votes.push((
            x19_frontier_recovery_sweep::choose_move_x19_frontier_recovery_sweep(game, state, models),
            w19,
        ));
    }
    if phase >= 0.40 && phase <= 0.82 && uncertainty < 0.36 {
        votes.push((
            x11_contest_frontier_recovery::choose_move_x11_contest_frontier_recovery(game, state, models),
            0.07,
        ));
    }
    if gap < 1200.0 && game.m <= 5 {
        votes.push((x02_monte_carlo::choose_move_monte_carlo(game, state, models), w02));
    }
    votes
}

fn recovery_probe(
    game: &Game,
    state: &State,
    models: &[AiModel],
    first_mv: (usize, usize),
    phase: f64,
) -> f64 {
    let top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
    let scores = crate::calc_scores(game, state);
    let mut leaders = vec![false; game.m];
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    for p in 1..game.m {
        if scores[p] == max_ai_i64 {
            leaders[p] = true;
        }
    }
    let mut cur = state.clone();
    let mut total = 0.0;
    for step in 0..2 {
        let mut moves = vec![first_mv];
        let s0 = scores[0] as f64;
        if step > 0 {
            moves.pop();
            let cands = crate::get_candidates(game, &cur, 0);
            if cands.is_empty() {
                break;
            }
            let conf = crate::estimate_conflict_map(game, &cur, models);
            let cm = crate::calc_scores(game, &cur);
            let phase_now = cur.turn as f64 / game.t as f64;
            let mut best = cands[0];
            let mut best_v = f64::NEG_INFINITY;
            for &mv in &cands {
                let v = crate::evaluate_local_move(
                    game,
                    &cur,
                    mv,
                    &cm,
                    s0,
                    max_ai_i64,
                    phase_now,
                    &conf,
                    cur.pos[0],
                    &leaders,
                );
                if v > best_v {
                    best_v = v;
                    best = mv;
                }
            }
            moves.push(best);
            if crate::uncertainty_risk(&top2) >= 0.20 {
                moves.extend(crate::build_secondary_ai_moves(&cm, &top2, 1));
            } else {
                moves.extend(top2.iter().map(|x| x.0));
            }
        }
        cur = crate::simulate_turn(game, &cur, &moves);
        let gain = crate::strategic_score(game, &cur);
        if phase > 0.6 {
            total += (0.82_f64).powi(step as i32) * gain;
        } else {
            total += (0.90_f64).powi(step as i32) * gain;
        }
    }
    total
}

fn local_gap_bias(conflict_map: &[Vec<f64>], mv: (usize, usize), game: &Game) -> f64 {
    let x = mv.0 as f64;
    let y = mv.1 as f64;
    let center = ((4.0 - x).abs() + (4.0 - y).abs()) / 8.0;
    let conflict = conflict_map[mv.0][mv.1];
    conflict * 12.0 + center * 5.0 + 0.000_01 * game.v[mv.0][mv.1] as f64
}

pub(super) fn choose_move_x22_band_stage_recovery_boost(
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
    let gap = ((max_ai_i64 - scores[0]).abs() as f64).min(100_000.0);
    let conflict_map = crate::estimate_conflict_map(game, state, models);
    let conf = conflict_map
        .iter()
        .flatten()
        .sum::<f64>()
        / ((game.n * game.n) as f64);

    let cands = crate::get_candidates(game, state, 0);
    if cands.is_empty() {
        return state.pos[0];
    }
    if cands.len() <= 3 {
        return cands[0];
    }

    let mut vote_map: HashMap<(usize, usize), f64> = HashMap::new();
    for (mv, w) in weighted_votes(game, state, models, uncertainty, phase, gap, conf) {
        *vote_map.entry(mv).or_insert(0.0) += w;
    }

    let mut leaders = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai_i64 {
            leaders[p] = true;
        }
    }
    let s0 = scores[0] as f64;
    for &mv in cands.iter().take(16) {
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
            &leaders,
        );
        let base = vote_map.entry(mv).or_insert(0.0);
        *base += 0.018 * local + 0.06 / (1.0 + local_gap_bias(&conflict_map, mv, game));
    }

    let mut pool: Vec<_> = vote_map.into_iter().collect();
    if pool.is_empty() {
        return x04_macro_route::choose_move_x04_macro_route(game, state, models);
    }
    pool.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    pool.truncate(pool.len().max(6).min(14));

    let mut best_mv = pool[0].0;
    let mut best = f64::NEG_INFINITY;
    for &(mv, base_w) in pool.iter().take(10) {
        let mut p1 = vec![mv];
        p1.extend(top2.iter().map(|x| x.0).take(game.m - 1));
        let mut p2 = vec![mv];
        p2.extend(crate::build_secondary_ai_moves(&scores, &top2, 1).into_iter().take(game.m - 1));
        let s1 = crate::simulate_turn(game, state, &p1);
        let s2 = crate::simulate_turn(game, state, &p2);
        let score1 = 0.50 * crate::strategic_score(game, &s1);
        let score2 = 0.30 * crate::strategic_score(game, &s2);
        let rollout = 0.20 * recovery_probe(game, state, models, mv, phase);
        let front = 0.10 * crate::frontier_potential(game, &s1);
        let risk = 0.08 * conflict_map[mv.0][mv.1];
        let score = score1 + score2 + rollout + front + base_w * 2.0 - risk;
        if score > best {
            best = score;
            best_mv = mv;
        }
    }

    if best.is_finite() {
        best_mv
    } else {
        x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models)
    }
}
