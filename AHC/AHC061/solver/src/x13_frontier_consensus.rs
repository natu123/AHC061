use crate::{
    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route, x06_expert_switch_hybrid,
    x07_dual_horizon_route, x08_pressure_frontier, x09_regret_mix, AiModel, Game, State,
};

fn frontier_pressure(game: &Game, state: &State, mv: (usize, usize)) -> f64 {
    let (x, y) = mv;
    let owner = state.owner[x][y];
    let level = state.level[x][y];
    let value = game.v[x][y] as f64;

    let mut score = 0.0_f64;
    if owner == -1 {
        score += 1.1 * value;
    } else if owner > 0 && level == 1 {
        score += 0.7 * value;
    } else if owner == 0 && level < game.u {
        score += 0.5 * value * (game.u - level) as f64 / game.u as f64;
    }

    const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for (dx, dy) in DIRS {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if !crate::in_bounds(game.n, nx, ny) {
            continue;
        }
        let ux = nx as usize;
        let uy = ny as usize;
        let ov = game.v[ux][uy] as f64;
        let oowner = state.owner[ux][uy];
        if oowner == -1 {
            score += 0.08 * ov;
        } else if oowner == 0 {
            if state.level[ux][uy] < game.u {
                score += 0.05 * ov;
            }
        } else if state.level[ux][uy] == 1 {
            score += 0.18 * ov;
        } else {
            score += 0.03 * ov / state.level[ux][uy] as f64;
        }
    }

    score
}

fn advisor_pool(
    game: &Game,
    state: &State,
    models: &[AiModel],
    phase: f64,
    uncertainty: f64,
) -> Vec<(usize, usize)> {
    let mut ordered = Vec::<(usize, usize)>::new();
    ordered.push(x06_expert_switch_hybrid::choose_move_x06_expert_switch(
        game, state, models,
    ));
    ordered.push(x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
        game, state, models,
    ));

    if (3..=5).contains(&game.m) {
        ordered.push(x02_monte_carlo::choose_move_monte_carlo(game, state, models));
    }

    if game.m == 4 {
        ordered.push(x04_macro_route::choose_move_x04_macro_route(game, state, models));
    }

    if phase <= 0.7 {
        ordered.push(x07_dual_horizon_route::choose_move_x07_dual_horizon_route(
            game,
            state,
            models,
        ));
    }

    if uncertainty >= 0.2 {
        ordered.push(x08_pressure_frontier::choose_move_x08_pressure_frontier(
            game, state, models,
        ));
    }

    if game.m >= 6 {
        ordered.push(x09_regret_mix::choose_move_x09_regret_mix(game, state, models));
    }

    let mut uniq = Vec::with_capacity(ordered.len());
    for mv in ordered {
        if !uniq.contains(&mv) {
            uniq.push(mv);
        }
    }
    uniq
}

pub(super) fn choose_move_x13_frontier_consensus(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
    let phase = state.turn as f64 / game.t as f64;
    let candidates = crate::get_candidates(game, state, 0);
    if candidates.is_empty() {
        return state.pos[0];
    }

    let top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
    let uncertainty = crate::uncertainty_risk(&top2);
    let scores = crate::calc_scores(game, state);
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let s0 = scores[0] as f64;
    let conflict = crate::estimate_conflict_map(game, state, models);

    let mut leaders = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai_i64 {
            leaders[p] = true;
        }
    }

    let advisors = advisor_pool(game, state, models, phase, uncertainty);
    let max_value = game.v.iter().flatten().copied().max().unwrap_or(1) as f64;
    let primary: Vec<(usize, usize)> = top2.iter().map(|x| x.0).collect();
    let secondary_cap = if uncertainty >= 0.4 {
        3
    } else if uncertainty >= 0.25 {
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

        let mut moves_p = Vec::with_capacity(game.m);
        moves_p.push(mv);
        moves_p.extend_from_slice(&primary);
        let mut moves_s = Vec::with_capacity(game.m);
        moves_s.push(mv);
        moves_s.extend_from_slice(&secondary);

        let ns_p = crate::simulate_turn(game, state, &moves_p);
        let ns_s = crate::simulate_turn(game, state, &moves_s);
        let s_p = crate::strategic_score(game, &ns_p);
        let s_s = crate::strategic_score(game, &ns_s);

        let mut vote = 0.0_f64;
        for av in &advisors {
            if *av == mv {
                vote += 1.0;
            }
        }
        let front = frontier_pressure(game, state, mv) / max_value;
        let risk = conflict[mv.0][mv.1];
        let gap = (max_ai_i64 as f64 - scores[0] as f64).max(0.0) / s0.max(1.0);

        let val = (0.56 + 0.04 * (1.0 - phase)) * s_p
            + (0.36 + 0.12 * (1.0 - uncertainty)) * s_s
            + (0.08 + 0.10 * (1.0 - gap)) * local
            + 0.25 * vote
            + 8.0 * front
            - 0.90 * risk * (1.0 + phase) * game.v[mv.0][mv.1] as f64;

        if val > best_val {
            best_val = val;
            best_mv = mv;
        }
    }

    if best_val.is_finite() {
        best_mv
    } else {
        x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models)
    }
}
