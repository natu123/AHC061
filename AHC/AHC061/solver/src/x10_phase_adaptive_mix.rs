use crate::{
    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route, x06_expert_switch_hybrid,
    x07_dual_horizon_route, AiModel, Game, State,
};

fn top_move_candidates(game: &Game, state: &State, models: &[AiModel], phase: f64) -> Vec<(usize, usize)> {
    let mut ordered = Vec::<(usize, usize)>::new();
    let top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
    let uncertainty = crate::uncertainty_risk(&top2);
    let cands = crate::get_candidates(game, state, 0);
    if cands.is_empty() {
        return ordered;
    }

    let mut local_rank = Vec::<((usize, usize), f64)>::new();
    let scores = crate::calc_scores(game, state);
    let s0 = scores[0] as f64;
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let conflict = crate::estimate_conflict_map(game, state, models);
    let is_leader = {
        let mut v = vec![false; game.m];
        for p in 1..game.m {
            if scores[p] == max_ai_i64 {
                v[p] = true;
            }
        }
        v
    };

    for &mv in &cands {
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
            &is_leader,
        );
        local_rank.push((mv, local));
    }
    local_rank.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    for &(mv, _) in local_rank.iter().take(2) {
        ordered.push(mv);
    }

    if game.m == 4 {
        if phase <= 0.72 {
            ordered.push(x04_macro_route::choose_move_x04_macro_route(game, state, models));
        }
        if uncertainty >= 0.14 {
            ordered.push(x07_dual_horizon_route::choose_move_x07_dual_horizon_route(
                game,
                state,
                models,
            ));
        }
    } else if game.m <= 5 {
        ordered.push(x02_monte_carlo::choose_move_monte_carlo(game, state, models));
    } else {
        ordered.push(x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
            game,
            state,
            models,
        ));
    }

    if phase >= 0.40 || uncertainty >= 0.18 {
        ordered.push(x06_expert_switch_hybrid::choose_move_x06_expert_switch(
            game,
            state,
            models,
        ));
    }
    if phase >= 0.65 || uncertainty >= 0.33 {
        ordered.push(x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
            game,
            state,
            models,
        ));
    }

    let mut uniq = Vec::with_capacity(ordered.len());
    for mv in ordered {
        if !uniq.contains(&mv) {
            uniq.push(mv);
        }
    }
    uniq
}

fn eval_candidate(
    game: &Game,
    state: &State,
    _models: &[AiModel],
    mv: (usize, usize),
    top2: &[((usize, usize), (usize, usize), f64)],
    scores: &[i64],
    conflict_map: &[Vec<f64>],
    phase: f64,
    is_leader: &[bool],
) -> f64 {
    let s0 = scores[0] as f64;
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let uncertainty = crate::uncertainty_risk(top2);

    let local = crate::evaluate_local_move(
        game,
        state,
        mv,
        scores,
        s0,
        max_ai_i64,
        phase,
        conflict_map,
        state.pos[0],
        is_leader,
    );

    let primary: Vec<(usize, usize)> = top2.iter().map(|x| x.0).collect();
    let secondary_cap = if game.m >= 7 && uncertainty >= 0.40 {
        3
    } else if game.m >= 6 && uncertainty >= 0.28 {
        2
    } else {
        1
    };
    let secondary = crate::build_secondary_ai_moves(scores, top2, secondary_cap);

    let mut moves1 = Vec::with_capacity(game.m);
    moves1.push(mv);
    moves1.extend_from_slice(&primary);
    let mut moves2 = Vec::with_capacity(game.m);
    moves2.push(mv);
    moves2.extend_from_slice(&secondary);

    let ns1 = crate::simulate_turn(game, state, &moves1);
    let ns2 = crate::simulate_turn(game, state, &moves2);
    let s1 = crate::strategic_score(game, &ns1);
    let s2 = crate::strategic_score(game, &ns2);
    let base_owner = state.owner[mv.0][mv.1];
    let lv = state.level[mv.0][mv.1] as f64;
    let v = game.v[mv.0][mv.1] as f64;
    let recovery_boost = if base_owner == 0 && lv >= 1.0 {
        0.12 * v * (game.u as f64 - lv + 1.0) / game.u as f64
    } else {
        0.0
    };
    let risk = (1.0 - (-conflict_map[mv.0][mv.1]).exp()).clamp(0.0, 1.0);
    let phase_w = 0.60 + 0.20 * phase;

    (1.0 - phase_w) * s1
        + phase_w * s2
        + 0.10 * local
        + recovery_boost
        - 0.80 * risk * v
}

pub(super) fn choose_move_x10_phase_adaptive_mix(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
    let phase = state.turn as f64 / game.t as f64;
    let top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
    let candidates = top_move_candidates(game, state, models, phase);
    if candidates.is_empty() {
        return state.pos[0];
    }

    let scores = crate::calc_scores(game, state);
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let conflict_map = crate::estimate_conflict_map(game, state, models);
    let mut is_leader = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai_i64 {
            is_leader[p] = true;
        }
    }

    let mut best_mv = candidates[0];
    let mut best_val = f64::NEG_INFINITY;
    for mv in &candidates {
        let v = eval_candidate(
            game,
            state,
            models,
            *mv,
            &top2,
            &scores,
            &conflict_map,
            phase,
            &is_leader,
        );
        if v > best_val {
            best_val = v;
            best_mv = *mv;
        }
    }
    best_mv
}




