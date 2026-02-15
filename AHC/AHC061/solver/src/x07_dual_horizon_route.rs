use crate::{x06_expert_switch_hybrid, AiModel, Game, State};

fn predicted_ai_moves(game: &Game, state: &State, models: &[AiModel], step: usize) -> Vec<(usize, usize)> {
    let scores = crate::calc_scores(game, state);
    let top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
    let primary: Vec<(usize, usize)> = top2.iter().map(|x| x.0).collect();
    let uncertainty = crate::uncertainty_risk(&top2);
    let secondary_cap = if game.m >= 7 && uncertainty >= 0.40 {
        3
    } else if game.m >= 6 && uncertainty >= 0.28 {
        2
    } else {
        1
    };
    let secondary = crate::build_secondary_ai_moves(&scores, &top2, secondary_cap);
    if uncertainty >= 0.24 && (step % 2 == 1) {
        secondary
    } else {
        primary
    }
}

fn leader_flags(scores: &[i64], m: usize) -> Vec<bool> {
    let mut is_leader = vec![false; m];
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    for p in 1..m {
        if scores[p] == max_ai_i64 {
            is_leader[p] = true;
        }
    }
    is_leader
}

fn best_local_move(game: &Game, state: &State, models: &[AiModel]) -> ((usize, usize), f64) {
    let cands = crate::get_candidates(game, state, 0);
    if cands.is_empty() {
        return (state.pos[0], f64::NEG_INFINITY);
    }
    let scores = crate::calc_scores(game, state);
    let s0 = scores[0] as f64;
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let phase = state.turn as f64 / game.t as f64;
    let conflict_map = crate::estimate_conflict_map(game, state, models);
    let cur = state.pos[0];
    let is_leader = leader_flags(&scores, game.m);

    let mut best_mv = cands[0];
    let mut best_local = f64::NEG_INFINITY;
    for mv in cands {
        let local = crate::evaluate_local_move(
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
        if local > best_local {
            best_local = local;
            best_mv = mv;
        }
    }
    (best_mv, best_local)
}

fn rollout_value(
    game: &Game,
    state: &State,
    models: &[AiModel],
    first_mv: (usize, usize),
    steps: usize,
) -> f64 {
    let mut cur = state.clone();
    let mut total = 0.0;
    for step in 0..steps {
        let my_mv = if step == 0 {
            first_mv
        } else {
            best_local_move(game, &cur, models).0
        };
        let mut moves = Vec::with_capacity(game.m);
        moves.push(my_mv);
        moves.extend_from_slice(&predicted_ai_moves(game, &cur, models, step));
        let next = crate::simulate_turn(game, &cur, &moves);
        let gain = crate::strategic_score(game, &next) - crate::strategic_score(game, &cur);
        total += (0.94_f64).powi(step as i32) * gain;
        cur = next;
    }
    total
}

pub(super) fn choose_move_x07_dual_horizon_route(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
    if game.m != 4 {
        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
    }
    let phase_now = state.turn as f64 / game.t as f64;
    if phase_now > 0.72 {
        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
    }

    let candidates = crate::get_candidates(game, state, 0);
    if candidates.len() <= 1 {
        return candidates.first().copied().unwrap_or(state.pos[0]);
    }

    let scores = crate::calc_scores(game, state);
    let s0 = scores[0] as f64;
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let phase = state.turn as f64 / game.t as f64;
    let conflict_map = crate::estimate_conflict_map(game, state, models);
    let cur = state.pos[0];
    let is_leader = leader_flags(&scores, game.m);

    let mut ranked = Vec::<((usize, usize), f64)>::new();
    for &mv in &candidates {
        let local = crate::evaluate_local_move(
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
        ranked.push((mv, local));
    }
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let candidate_cap = ranked.len().min(6);

    let top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
    let uncertainty = crate::uncertainty_risk(&top2);
    let blend = (0.35 + 1.40 * uncertainty + 0.30 * phase_now).clamp(0.35, 0.88);
    let long_steps = if phase_now > 0.48 { 6 } else { 7 };

    let mut best_mv = ranked[0].0;
    let mut best_score = f64::NEG_INFINITY;
    for &(mv, local) in ranked.iter().take(candidate_cap) {
        let short_v = rollout_value(game, state, models, mv, 3);
        let long_v = rollout_value(game, state, models, mv, long_steps);
        let total = (1.0 - blend) * short_v + blend * long_v + 0.10 * local;
        if total > best_score {
            best_score = total;
            best_mv = mv;
        }
    }
    best_mv
}
