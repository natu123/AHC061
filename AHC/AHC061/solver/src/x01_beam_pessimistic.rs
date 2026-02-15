use crate::{AiModel, Game, State};

fn best_one_step_score(game: &Game, state: &State, models: &[AiModel]) -> f64 {
    let candidates = crate::get_candidates(game, state, 0);
    if candidates.is_empty() {
        return 0.0;
    }
    let scores = crate::calc_scores(game, state);
    let ai_top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
    let predicted_primary: Vec<(usize, usize)> = ai_top2.iter().map(|x| x.0).collect();
    let uncertainty = crate::uncertainty_risk(&ai_top2);
    let secondary_cap = if game.m >= 7 && uncertainty >= 0.40 {
        3
    } else if game.m >= 6 && uncertainty >= 0.28 {
        2
    } else {
        1
    };
    let predicted_secondary = crate::build_secondary_ai_moves(&scores, &ai_top2, secondary_cap);
    let risk_w = crate::pessimism_weight(game, uncertainty);

    if candidates.len() == 1 {
        let mut primary = Vec::with_capacity(game.m);
        primary.push(candidates[0]);
        primary.extend_from_slice(&predicted_primary);
        let score_primary = crate::absolute_score(game, &crate::simulate_turn(game, state, &primary));

        let mut secondary = Vec::with_capacity(game.m);
        secondary.push(candidates[0]);
        secondary.extend_from_slice(&predicted_secondary);
        let score_secondary =
            crate::absolute_score(game, &crate::simulate_turn(game, state, &secondary));
        return (1.0 - risk_w) * score_primary + risk_w * score_secondary;
    }

    let s0 = scores[0] as f64;
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let phase = state.turn as f64 / game.t as f64;
    let conflict_map = crate::estimate_conflict_map(game, state, models);
    let cur = state.pos[0];

    let mut is_leader = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai_i64 {
            is_leader[p] = true;
        }
    }

    let mut best_val = f64::NEG_INFINITY;
    for &mv in &candidates {
        let local_score = crate::evaluate_local_move(
            game,
            state,
            mv,
            &scores,
            s0,
            max_ai_i64,
            phase,
            &conflict_map,
            cur,
            &is_leader,
        );

        let mut primary = Vec::with_capacity(game.m);
        primary.push(mv);
        primary.extend_from_slice(&predicted_primary);
        let score_primary = crate::absolute_score(game, &crate::simulate_turn(game, state, &primary));

        let mut secondary = Vec::with_capacity(game.m);
        secondary.push(mv);
        secondary.extend_from_slice(&predicted_secondary);
        let score_secondary =
            crate::absolute_score(game, &crate::simulate_turn(game, state, &secondary));

        let rollout = (1.0 - risk_w) * score_primary + risk_w * score_secondary;
        let total = rollout + 0.12 * local_score;
        if total > best_val {
            best_val = total;
        }
    }
    best_val
}

pub(super) fn choose_move_x01_beam_pessimistic(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
    let candidates = crate::get_candidates(game, state, 0);
    if candidates.len() == 1 {
        return candidates[0];
    }

    let scores = crate::calc_scores(game, state);
    let s0 = scores[0] as f64;
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let phase = state.turn as f64 / game.t as f64;
    let conflict_map = crate::estimate_conflict_map(game, state, models);
    let cur = state.pos[0];
    let ai_top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
    let predicted_primary: Vec<(usize, usize)> = ai_top2.iter().map(|x| x.0).collect();
    let uncertainty = crate::uncertainty_risk(&ai_top2);
    let secondary_cap = if game.m >= 7 && uncertainty >= 0.40 {
        3
    } else if game.m >= 6 && uncertainty >= 0.28 {
        2
    } else {
        1
    };
    let predicted_secondary = crate::build_secondary_ai_moves(&scores, &ai_top2, secondary_cap);
    let risk_w = crate::pessimism_weight(game, uncertainty);

    let mut is_leader = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai_i64 {
            is_leader[p] = true;
        }
    }

    let mut scored: Vec<((usize, usize), f64, State)> = Vec::with_capacity(candidates.len());

    for &(x, y) in &candidates {
        let local_score = crate::evaluate_local_move(
            game,
            state,
            (x, y),
            &scores,
            s0,
            max_ai_i64,
            phase,
            &conflict_map,
            cur,
            &is_leader,
        );

        let mut primary = Vec::with_capacity(game.m);
        primary.push((x, y));
        primary.extend_from_slice(&predicted_primary);
        let next_state = crate::simulate_turn(game, state, &primary);
        let score_primary = crate::absolute_score(game, &next_state);

        let mut secondary = Vec::with_capacity(game.m);
        secondary.push((x, y));
        secondary.extend_from_slice(&predicted_secondary);
        let score_secondary = crate::absolute_score(game, &crate::simulate_turn(game, state, &secondary));

        let rollout_score = (1.0 - risk_w) * score_primary + risk_w * score_secondary;
        let base_total = rollout_score + 0.12 * local_score;
        scored.push(((x, y), base_total, next_state));
    }

    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let mut beam_width = if scored.len() >= 32 {
        8
    } else if scored.len() >= 16 {
        6
    } else {
        4
    };
    if game.m == 5 && phase <= 0.80 && uncertainty >= 0.18 {
        beam_width = scored.len();
    } else if game.m == 6 && phase <= 0.72 && uncertainty >= 0.22 {
        beam_width = (beam_width + 3).min(scored.len());
    }

    let mut best = scored[0].0;
    let mut best_total = f64::NEG_INFINITY;
    for (idx, (mv, base_total, next_state)) in scored.iter().enumerate() {
        if idx >= beam_width {
            break;
        }
        let future = if state.turn + 1 < game.t {
            best_one_step_score(game, next_state, models)
        } else {
            0.0
        };
        let total = *base_total + 0.18 * future;
        if total > best_total {
            best_total = total;
            best = *mv;
        }
    }
    best
}
