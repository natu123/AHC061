use crate::{
    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route, x06_expert_switch_hybrid,
    x07_dual_horizon_route, x08_pressure_frontier, x09_regret_mix, AiModel, Game, State,
};

fn frontier_recovery_pressure(game: &Game, state: &State, mv: (usize, usize)) -> f64 {
    let (x, y) = mv;
    let owner = state.owner[x][y];
    let level = state.level[x][y];
    let value = game.v[x][y] as f64;
    let mut score = 0.0_f64;

    if owner == -1 {
        score += 1.2 * value;
    } else if owner > 0 && level == 1 {
        score += 0.9 * value;
    } else if owner == 0 && level < game.u {
        score += 0.7 * value * (game.u - level) as f64 / game.u as f64;
    }

    const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for (dx, dy) in DIRS {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if !crate::in_bounds(game.n, nx, ny) {
            continue;
        }
        let ox = nx as usize;
        let oy = ny as usize;
        let ov = game.v[ox][oy] as f64;
        let oowner = state.owner[ox][oy];
        let olv = state.level[ox][oy].max(1) as f64;
        if oowner == -1 {
            score += 0.11 * ov;
        } else if oowner > 0 {
            if state.level[ox][oy] == 1 {
                score += 0.26 * ov;
            } else {
                score += 0.09 * ov / olv;
            }
        } else if state.level[ox][oy] < game.u {
            score += 0.06 * ov;
        }
    }
    score
}

fn choose_advisors(game: &Game, state: &State, models: &[AiModel], phase: f64) -> Vec<(usize, usize)> {
    let mut ordered = Vec::<(usize, usize)>::new();
    if game.m == 4 {
        ordered.push(x04_macro_route::choose_move_x04_macro_route(game, state, models));
    }
    if phase <= 0.68 && (3..=5).contains(&game.m) {
        ordered.push(x02_monte_carlo::choose_move_monte_carlo(game, state, models));
    }
    ordered.push(x07_dual_horizon_route::choose_move_x07_dual_horizon_route(
        game,
        state,
        models,
    ));
    ordered.push(x06_expert_switch_hybrid::choose_move_x06_expert_switch(
        game,
        state,
        models,
    ));
    ordered.push(x08_pressure_frontier::choose_move_x08_pressure_frontier(game, state, models));
    ordered.push(x09_regret_mix::choose_move_x09_regret_mix(game, state, models));
    ordered.push(x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
        game,
        state,
        models,
    ));

    let mut uniq = Vec::with_capacity(ordered.len());
    for mv in ordered {
        if !uniq.contains(&mv) {
            uniq.push(mv);
        }
    }
    uniq
}

pub(super) fn choose_move_x11_contest_frontier_recovery(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
    let phase = state.turn as f64 / game.t as f64;
    if game.m == 7 && phase >= 0.90 {
        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
    }

    let candidates = crate::get_candidates(game, state, 0);
    if candidates.len() <= 1 {
        return candidates.first().copied().unwrap_or(state.pos[0]);
    }

    let scores = crate::calc_scores(game, state);
    let s0 = scores[0] as f64;
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let conflict = crate::estimate_conflict_map(game, state, models);
    let my_pos = state.pos[0];
    let mut is_leader = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai_i64 {
            is_leader[p] = true;
        }
    }
    let top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
    let uncertainty = crate::uncertainty_risk(&top2);

    let mut ranked_candidates = Vec::<((usize, usize), f64)>::new();
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
            my_pos,
            &is_leader,
        );
        let front = frontier_recovery_pressure(game, state, mv);
        ranked_candidates.push((mv, local + front * 0.05));
    }
    ranked_candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    ranked_candidates.truncate(12);

    let advisors = choose_advisors(game, state, models, phase);
    let advisor_set: Vec<(usize, usize)> = advisors
        .iter()
        .copied()
        .filter(|mv| conflict[mv.0][mv.1] < 0.95)
        .collect();

    let mut best_mv = ranked_candidates[0].0;
    let mut best_score = f64::NEG_INFINITY;
    for &(mv, local_score) in &ranked_candidates {
        let mut primary = Vec::<(usize, usize)>::with_capacity(game.m);
        primary.push(mv);
        let mut secondary = primary.clone();

        for &((a0x, a0y), (a1x, a1y), _) in &top2 {
            primary.push((a0x, a0y));
            if secondary.len() < game.m {
                secondary.push((a1x, a1y));
            }
        }
        for x in top2.iter().skip(2) {
            if secondary.len() < game.m {
                secondary.push(x.1);
            }
        }
        if secondary.len() > game.m {
            secondary.truncate(game.m);
        }
        while secondary.len() < game.m {
            secondary.push(state.pos[0]);
        }
        let next_primary = crate::simulate_turn(game, state, &primary);
        let next_secondary = crate::simulate_turn(game, state, &secondary);
        let s_primary = crate::strategic_score(game, &next_primary);
        let s_secondary = crate::strategic_score(game, &next_secondary);

        let mut votes = 0.0_f64;
        for av in &advisor_set {
            if *av == mv {
                votes += 1.0;
            }
        }
        let leader_gap = (max_ai_i64 as f64 - scores[0] as f64).max(0.0) / s0.max(1.0);
        let recovery = if scores[0] < max_ai_i64 {
            1.2
        } else {
            0.8
        };
        let frontier = frontier_recovery_pressure(game, state, mv);
        let risk_penalty = conflict[mv.0][mv.1] * game.v[mv.0][mv.1] as f64;
        let total = (0.54 + 0.15 * recovery) * s_primary
            + (0.38 - 0.15 * recovery) * s_secondary
            + 0.08 * local_score
            + 0.05 * votes
            + (0.10 + 0.12 * uncertainty) * frontier
            + (leader_gap * 12.0)
            - 0.75 * risk_penalty;
        if total > best_score {
            best_score = total;
            best_mv = mv;
        }
    }
    best_mv
}



