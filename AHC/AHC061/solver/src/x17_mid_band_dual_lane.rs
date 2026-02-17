use crate::{x04_macro_route, x06_expert_switch_hybrid, x07_dual_horizon_route, AiModel, Game, State};

fn lane_candidates(
    game: &Game,
    state: &State,
    models: &[AiModel],
    phase: f64,
    uncertainty: f64,
) -> Vec<(usize, usize)> {
    let mut cands = Vec::<(usize, usize)>::new();
    let local = crate::get_candidates(game, state, 0);
    for mv in local {
        if !cands.contains(&mv) {
            cands.push(mv);
        }
    }

    if phase <= 0.70 {
        let route_mv = x04_macro_route::choose_move_x04_macro_route(game, state, models);
        if !cands.contains(&route_mv) {
            cands.push(route_mv);
        }
    }

    if phase <= 0.42 || uncertainty >= 0.28 {
        let dual_mv = x07_dual_horizon_route::choose_move_x07_dual_horizon_route(game, state, models);
        if !cands.contains(&dual_mv) {
            cands.push(dual_mv);
        }
    }

    if phase > 0.76 {
        let stable_mv = x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
        if !cands.contains(&stable_mv) {
            cands.push(stable_mv);
        }
    }

    cands
}

fn quick_rollout(
    game: &Game,
    state: &State,
    models: &[AiModel],
    first_mv: (usize, usize),
    steps: usize,
) -> f64 {
    let mut cur = state.clone();
    for step in 0..steps {
        let mut moves = Vec::with_capacity(game.m);
        if step == 0 {
            moves.push(first_mv);
            let top2 = crate::choose_predicted_ai_top2_moves(game, &cur, models);
            let uncertainty = crate::uncertainty_risk(&top2);
            moves.extend(top2.iter().take(2).map(|x| x.0));
            if uncertainty >= 0.24 {
                moves.pop();
                let secondary = crate::build_secondary_ai_moves(
                    &crate::calc_scores(game, &cur),
                    &top2,
                    2,
                );
                moves.extend_from_slice(&secondary);
            }
        } else {
            let scores = crate::calc_scores(game, &cur);
            let s0 = scores[0] as f64;
            let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
            let phase = cur.turn as f64 / game.t as f64;
            let conflict = crate::estimate_conflict_map(game, &cur, models);
            let mut best = cur.pos[0];
            let mut best_v = f64::NEG_INFINITY;
            let mut is_leader = vec![false; game.m];
            for p in 1..game.m {
                if scores[p] == max_ai_i64 {
                    is_leader[p] = true;
                }
            }
            for mv in crate::get_candidates(game, &cur, 0) {
                let v = crate::evaluate_local_move(
                    game,
                    &cur,
                    mv,
                    &scores,
                    s0,
                    max_ai_i64,
                    phase,
                    &conflict,
                    cur.pos[0],
                    &is_leader,
                );
                if v > best_v {
                    best_v = v;
                    best = mv;
                }
            }
            moves.push(best);
            let top2 = crate::choose_predicted_ai_top2_moves(game, &cur, models);
            moves.extend(top2.iter().take(1).map(|x| x.0));
        }
        cur = crate::simulate_turn(game, &cur, &moves);
    }
    crate::strategic_score(game, &cur) - crate::strategic_score(game, state)
}

fn conflict_ratio(conflict: &[Vec<f64>], game: &Game, mv: (usize, usize)) -> f64 {
    let p_any = 1.0 - (-conflict[mv.0][mv.1]).exp();
    p_any * 0.2 + conflict[mv.0][mv.1] * 0.8 + 1e-6 * game.v[mv.0][mv.1] as f64
}

pub(super) fn choose_move_x17_mid_band_dual_lane(
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
    let conflict = crate::estimate_conflict_map(game, state, models);
    let mut is_leader = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai_i64 {
            is_leader[p] = true;
        }
    }

    let base_score = crate::strategic_score(game, state);
    let cands = lane_candidates(game, state, models, phase, uncertainty);
    if cands.is_empty() {
        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
    }

    let mut best_mv = cands[0];
    let mut best_val = f64::NEG_INFINITY;
    for mv in cands {
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

        let short = quick_rollout(game, state, models, mv, 2);
        let long = quick_rollout(game, state, models, mv, 4);
        let risk = conflict[mv.0][mv.1];
        let mut bonus = 0.0;
        for d in [0_i32, 1, -1] {
            for e in [0_i32, 1, -1] {
                let nx = mv.0 as i32 + d;
                let ny = mv.1 as i32 + e;
                if nx < 0 || ny < 0 || nx >= game.n as i32 || ny >= game.n as i32 {
                    continue;
                }
                let v = game.v[nx as usize][ny as usize] as f64;
                if state.owner[nx as usize][ny as usize] == 0 {
                    bonus += 0.005 * v;
                } else if state.owner[nx as usize][ny as usize] > 0 {
                    bonus += 0.018 * v;
                } else {
                    bonus += 0.009 * v;
                }
            }
        }
        let mix = 0.35 + 0.20 * phase + 0.10 * (1.0 - uncertainty);
        let v = mix * short + (1.0 - mix) * long + 0.10 * local + 0.30 * bonus - 0.75 * risk * conflict_ratio(&conflict, game, mv);

        if v > best_val {
            best_val = v;
            best_mv = mv;
        }
    }

    if best_val.is_finite() {
        let next_state_val = crate::strategic_score(game, state) + best_val;
        if next_state_val < base_score {
            x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models)
        } else {
            best_mv
        }
    } else {
        x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models)
    }
}
