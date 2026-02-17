use std::collections::HashMap;

use crate::{
    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route, x06_expert_switch_hybrid,
    x11_contest_frontier_recovery, AiModel, Game, State,
};

fn propose_weights(
    game: &Game,
    _state: &State,
    uncertainty: f64,
    phase: f64,
    phase_gap: f64,
) -> (f64, f64, f64, f64, f64) {
    let late_penalty = if phase >= 0.70 { 0.25 } else { 0.0 };
    let uncertainty_penalty = (0.40 - uncertainty).max(0.0);
    let mut w01 = 0.08 + 0.12 * uncertainty + 0.05 * (phase_gap / 1e5).min(1.0);
    let mut w04 = 0.40 + 0.25 * uncertainty_penalty + 0.20 * (1.0 - phase);
    let mut w06 = 0.38 + 0.20 * uncertainty + 0.10 * late_penalty;
    let mut w02 = 0.08;
    let mut w11 = 0.0;

    if game.m == 4 {
        w04 += 0.05;
        w01 *= 0.95;
        w06 *= 1.02;
        if phase < 0.35 {
            w02 += 0.12;
            w04 -= 0.08;
        }
    } else if game.m == 5 {
        w04 *= 1.05;
        w06 += 0.06;
        w11 += 0.01;
    } else if game.m == 6 {
        w01 += 0.04;
        w04 *= 0.95;
        w06 *= 1.06;
        if phase > 0.60 {
            w11 += 0.10;
        }
    }

    let total = (w01 + w04 + w06 + w02 + w11).max(1e-12);
    (w01 / total, w04 / total, w06 / total, w02 / total, w11 / total)
}

fn weighted_votes(
    game: &Game,
    state: &State,
    models: &[AiModel],
    uncertainty: f64,
    phase: f64,
    phase_gap: f64,
) -> Vec<((usize, usize), f64)> {
    let (w01, w04, w06, w02, w11) = propose_weights(game, state, uncertainty, phase, phase_gap);
    let mv1 = x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models);
    let mv2 = x04_macro_route::choose_move_x04_macro_route(game, state, models);
    let mv6 = x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
    let mut mvs: Vec<((usize, usize), f64)> = vec![(mv1, w01), (mv2, w04), (mv6, w06)];
    let mv0 = x02_monte_carlo::choose_move_monte_carlo(game, state, models);
    if game.m <= 5 && phase < 0.55 {
        mvs.push((mv0, w02));
    }
    if game.m == 6 && phase > 0.55 {
        let mv11 = x11_contest_frontier_recovery::choose_move_x11_contest_frontier_recovery(
            game, state, models,
        );
        mvs.push((mv11, w11));
    }
    mvs
}

fn evaluate_rollout(
    game: &Game,
    state: &State,
    models: &[AiModel],
    first_mv: (usize, usize),
    conflict_penalty: f64,
) -> f64 {
    let mut cur = state.clone();
    for step in 0..2 {
        let mut moves = Vec::with_capacity(game.m);
        if step == 0 {
            moves.push(first_mv);
        } else {
            let cands = crate::get_candidates(game, &cur, 0);
            if cands.is_empty() {
                break;
            }
            let top2 = crate::choose_predicted_ai_top2_moves(game, &cur, models);
            let scores = crate::calc_scores(game, &cur);
            let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
            let s0 = scores[0] as f64;
            let phase = cur.turn as f64 / game.t as f64;
            let conflict = crate::estimate_conflict_map(game, &cur, models);
            let mut leaders = vec![false; game.m];
            for p in 1..game.m {
                if scores[p] == max_ai {
                    leaders[p] = true;
                }
            }
            let mut best_mv = cands[0];
            let mut best_v = f64::NEG_INFINITY;
            for &mv in &cands {
                let v = crate::evaluate_local_move(
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
    }
    let growth = crate::strategic_score(game, &cur) - crate::strategic_score(game, state);
    growth - 0.45 * conflict_penalty
}

pub(super) fn choose_move_x20_band_stage_ensemble(
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
    let mut max_enemy = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let phase = state.turn as f64 / game.t as f64;
    let phase_score_gap = (max_enemy as f64 - scores[0] as f64).abs();
    let conflict_map = crate::estimate_conflict_map(game, state, models);

    let cands = crate::get_candidates(game, state, 0);
    if cands.is_empty() {
        return state.pos[0];
    }
    if cands.len() <= 3 {
        return cands[0];
    }
    max_enemy = max_enemy.max(1);
    let mut vote_map: HashMap<(usize, usize), f64> = HashMap::new();
    for (mv, w) in weighted_votes(game, state, models, uncertainty, phase, phase_score_gap) {
        *vote_map.entry(mv).or_insert(0.0) += w;
    }

    let mut leaders = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_enemy {
            leaders[p] = true;
        }
    }

    let s0 = scores[0] as f64;
    for &mv in cands.iter().take(12) {
        let local = crate::evaluate_local_move(
            game,
            state,
            mv,
            &scores,
            s0,
            max_enemy,
            phase,
            &conflict_map,
            state.pos[0],
            &leaders,
        );
        let base = *vote_map.get(&mv).unwrap_or(&0.0);
        let entry = vote_map.entry(mv).or_insert(0.0);
        *entry = base + local * 0.02;
    }

    let mut pool = vote_map.into_iter().collect::<Vec<_>>();
    if pool.is_empty() {
        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
    }
    pool.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    pool.truncate(pool.len().min(12).max(4));

    let top_candidates = cands.iter().take(14).collect::<Vec<_>>();
    let mut append_pool = Vec::<((usize, usize), f64)>::new();
    for (mv, w) in pool.iter() {
        if !top_candidates.contains(&mv) && *w > 0.0 {
            append_pool.push((*mv, *w * 0.5));
        }
        if pool.len() > 14 {
            break;
        }
    }
    pool.extend_from_slice(&append_pool);

    let mut best_mv = pool[0].0;
    let mut best_score = f64::NEG_INFINITY;
    for (mv, vote_w) in pool.iter().take(12) {
        let mut moves_primary = Vec::with_capacity(game.m);
        moves_primary.push(*mv);
        moves_primary.extend(top2.iter().map(|x| x.0).take(game.m - 1));
        let mut moves_secondary = Vec::with_capacity(game.m);
        moves_secondary.push(*mv);
        moves_secondary.extend(
            crate::build_secondary_ai_moves(&scores, &top2, 1).into_iter().take(game.m - 1),
        );
        let s_primary = crate::simulate_turn(game, state, &moves_primary);
        let s_secondary = crate::simulate_turn(game, state, &moves_secondary);
        let core = 0.50 * crate::strategic_score(game, &s_primary)
            + 0.28 * crate::strategic_score(game, &s_secondary);
        let two_step = 0.16 * evaluate_rollout(
            game,
            state,
            models,
            *mv,
            conflict_map[mv.0][mv.1],
        );
        let frontier = 0.18 * crate::frontier_potential(game, &s_primary);
        let risk = 0.09 * phase_score_gap;
        let stability = if uncertainty >= 0.30 { 0.7 } else { 1.0 };
        let score = (core + two_step + frontier + risk)
            + vote_w * 2.0
            + 0.12 * (state.level[mv.0][mv.1] as f64)
            - (1.0 - stability) * 0.10 * conflict_map[mv.0][mv.1] * game.v[mv.0][mv.1] as f64;
        if score > best_score {
            best_score = score;
            best_mv = *mv;
        }
    }

    if best_score.is_finite() {
        best_mv
    } else if uncertainty >= 0.32 {
        x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models)
    } else {
        x04_macro_route::choose_move_x04_macro_route(game, state, models)
    }
}
