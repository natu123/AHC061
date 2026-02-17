use crate::{
    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route, x06_expert_switch_hybrid, AiModel, Game,
    State,
};

fn recovery_pressure(game: &Game, state: &State, cand: (usize, usize)) -> f64 {
    let (x, y) = cand;
    let owner = state.owner[x][y];
    let level = state.level[x][y];
    let value = game.v[x][y] as f64;
    let mut score = 0.0;

    if owner == -1 {
        score += 0.9 * value;
    } else if owner == 0 {
        if level < game.u {
            score += (0.60 + 0.20 * (game.u - level) as f64 / game.u as f64) * value;
        } else {
            score -= 0.04 * value;
        }
    } else if level == 1 {
        score += 1.05 * value;
    } else {
        score += 0.20 * value;
    }
    score
}

fn candidate_pool(
    game: &Game,
    state: &State,
    models: &[AiModel],
    phase: f64,
    uncertainty: f64,
) -> Vec<(usize, usize)> {
    let mut out = Vec::<(usize, usize)>::new();
    let base = crate::get_candidates(game, state, 0);
    let mut base_local = Vec::<((usize, usize), f64)>::new();
    let scores = crate::calc_scores(game, state);
    let s0 = scores[0] as f64;
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let conflict = crate::estimate_conflict_map(game, state, models);
    let mut leaders = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai_i64 {
            leaders[p] = true;
        }
    }
    for &mv in base.iter() {
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
        base_local.push((mv, local));
    }
    base_local.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    for (mv, _) in base_local.into_iter().take(7) {
        if !out.contains(&mv) {
            out.push(mv);
        }
    }

    let x04_mv = x04_macro_route::choose_move_x04_macro_route(game, state, models);
    if phase <= 0.72 {
        if !out.contains(&x04_mv) {
            out.push(x04_mv);
        }
    }

    if game.m == 5 || uncertainty >= 0.20 {
        let x02_mv = x02_monte_carlo::choose_move_monte_carlo(game, state, models);
        if !out.contains(&x02_mv) {
            out.push(x02_mv);
        }
    }

    if uncertainty >= 0.30 {
        let x06_mv = x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
        if !out.contains(&x06_mv) {
            out.push(x06_mv);
        }
    } else {
        let x01_mv = x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models);
        if !out.contains(&x01_mv) {
            out.push(x01_mv);
        }
    }

    out
}

pub(super) fn choose_move_x16_safe_recovery_route(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
    if !(4..=6).contains(&game.m) {
        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
    }
    let phase = state.turn as f64 / game.t as f64;
    let top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
    let uncertainty = crate::uncertainty_risk(&top2);
    let scores = crate::calc_scores(game, state);
    let s0 = scores[0] as f64;
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let current_score = crate::strategic_score(game, state);
    let conflict = crate::estimate_conflict_map(game, state, models);
    let mut leaders = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai_i64 {
            leaders[p] = true;
        }
    }

    let primary: Vec<(usize, usize)> = top2.iter().map(|x| x.0).collect();
    let secondary = crate::build_secondary_ai_moves(&scores, &top2, 2);

    let mut best_mv = state.pos[0];
    let mut best_val = f64::NEG_INFINITY;
    for mv in candidate_pool(game, state, models, phase, uncertainty) {
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

        let mut m1 = Vec::with_capacity(game.m);
        m1.push(mv);
        m1.extend_from_slice(&primary);
        let mut m2 = Vec::with_capacity(game.m);
        m2.push(mv);
        m2.extend_from_slice(&secondary);
        let ns1 = crate::simulate_turn(game, state, &m1);
        let ns2 = crate::simulate_turn(game, state, &m2);
        let gain1 = crate::strategic_score(game, &ns1) - current_score;
        let gain2 = crate::strategic_score(game, &ns2) - current_score;

        let recovery = recovery_pressure(game, state, mv);
        let conflict_penalty = (1.0 - (-conflict[mv.0][mv.1]).exp())
            * game.v[mv.0][mv.1] as f64
            * if game.m >= 6 { 1.10 } else { 0.78 };
        let future_risk =
            -0.18 * ((game.v[mv.0][mv.1] as f64).min(500.0) / 500.0) * (0.30 + phase);

        let val = if phase < 0.45 {
            0.68 * gain1 + 0.22 * gain2 + 0.14 * local + 0.24 * recovery - conflict_penalty + future_risk
        } else {
            0.56 * gain1
                + 0.28 * gain2
                + 0.10 * local
                + 0.16 * recovery
                - 0.95 * conflict_penalty
        };

        if val > best_val {
            best_val = val;
            best_mv = mv;
        }
    }

    if best_val.is_finite() {
        best_mv
    } else {
        x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models)
    }
}
