use crate::{
    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route, x06_expert_switch_hybrid,
    x07_dual_horizon_route, x08_pressure_frontier, x09_regret_mix, AiModel, Game, State,
};

fn build_candidates(
    game: &Game,
    state: &State,
    models: &[AiModel],
    phase: f64,
    uncertainty: f64,
) -> Vec<(usize, usize)> {
    let mut cand = Vec::<(usize, usize)>::new();

    let base = crate::get_candidates(game, state, 0);
    for mv in base {
        if !cand.contains(&mv) {
            cand.push(mv);
        }
    }

    if phase <= 0.70 {
        let m6 = x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
        let m1 = x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models);
        if !cand.contains(&m6) {
            cand.push(m6);
        }
        if !cand.contains(&m1) {
            cand.push(m1);
        }
    } else if !cand.contains(&state.pos[0]) {
        cand.push(state.pos[0]);
    }

    if (3..=5).contains(&game.m) {
        let mv2 = x02_monte_carlo::choose_move_monte_carlo(game, state, models);
        if !cand.contains(&mv2) {
            cand.push(mv2);
        }
    }

    if game.m == 4 && phase <= 0.80 {
        let mv4 = x04_macro_route::choose_move_x04_macro_route(game, state, models);
        if !cand.contains(&mv4) {
            cand.push(mv4);
        }
    }

    if phase <= 0.55 {
        let mv7 = x07_dual_horizon_route::choose_move_x07_dual_horizon_route(game, state, models);
        if !cand.contains(&mv7) {
            cand.push(mv7);
        }
    }

    if phase >= 0.45 || uncertainty >= 0.22 {
        let mv8 = x08_pressure_frontier::choose_move_x08_pressure_frontier(game, state, models);
        if !cand.contains(&mv8) {
            cand.push(mv8);
        }
    }

    if uncertainty >= 0.16 {
        let mv9 = x09_regret_mix::choose_move_x09_regret_mix(game, state, models);
        if !cand.contains(&mv9) {
            cand.push(mv9);
        }
    }

    if cand.len() > 16 {
        cand.truncate(16);
    }
    cand
}

pub(super) fn choose_move_x14_adaptive_risk_lane(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
    let phase = state.turn as f64 / game.t as f64;
    let top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
    let uncertainty = crate::uncertainty_risk(&top2);
    let scores = crate::calc_scores(game, state);
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let conflict = crate::estimate_conflict_map(game, state, models);
    let s0 = scores[0] as f64;

    let mut leaders = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai_i64 {
            leaders[p] = true;
        }
    }

    let candidates = build_candidates(game, state, models, phase, uncertainty);
    if candidates.is_empty() {
        return state.pos[0];
    }

    let primary: Vec<(usize, usize)> = top2.iter().map(|x| x.0).collect();
    let secondary_cap = if game.m >= 8 {
        3
    } else if game.m >= 6 {
        2
    } else {
        1
    };
    let secondary = crate::build_secondary_ai_moves(&scores, &top2, secondary_cap);

    let mut best_mv = candidates[0];
    let mut best_val = f64::NEG_INFINITY;

    for &mv in &candidates {
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
        let s1 = crate::strategic_score(game, &ns1);
        let s2 = crate::strategic_score(game, &ns2);

        let risk = conflict[mv.0][mv.1];
        let value = (0.68 - 0.12 * phase) * s1
            + (0.32 + 0.12 * phase) * s2
            + (0.05 + 0.20 * uncertainty) * local
            - (0.90 + 0.40 * uncertainty) * risk * game.v[mv.0][mv.1] as f64;

        if value > best_val {
            best_val = value;
            best_mv = mv;
        }
    }

    if best_val.is_finite() {
        best_mv
    } else {
        x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models)
    }
}
