use crate::{x04_macro_route, x06_expert_switch_hybrid, AiModel, Game, State};

fn opponent_set_by_mode(
    game: &Game,
    state: &State,
    models: &[AiModel],
    mode: u8,
    fallback: &Vec<Vec<(usize, usize)>>,
) -> Vec<Vec<(usize, usize)> > {
    let top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
    let mut out: Vec<Vec<(usize, usize)>> = Vec::with_capacity(game.m.saturating_sub(1));
    for ai_idx in 0..(game.m.saturating_sub(1)) {
        let player = ai_idx + 1;
        let cands = crate::get_candidates(game, state, player);
        if cands.is_empty() {
            out.push(vec![state.pos[player]]);
            continue;
        }
        let scores = crate::calc_scores(game, state);
        let secondary = crate::build_secondary_ai_moves(&scores, &top2, 1);
        let mv = match mode {
            0 => top2[ai_idx].0,
            1 => {
                if !secondary.is_empty() {
                    secondary[ai_idx]
                } else {
                    top2[ai_idx].0
                }
            }
            _ => {
                let f = &fallback[ai_idx];
                if f.is_empty() {
                    state.pos[player]
                } else {
                    f[0]
                }
            }
        };
        out.push(vec![mv]);
    }
    out
}

fn simulate_with_set(
    game: &Game,
    state: &State,
    my_mv: (usize, usize),
    opponent_moves: &[Vec<(usize, usize)>],
) -> State {
    let mut moves = Vec::with_capacity(game.m);
    moves.push(my_mv);
    for ai_idx in 0..(game.m.saturating_sub(1)) {
        let mv = opponent_moves[ai_idx][0];
        moves.push(mv);
    }
    crate::simulate_turn(game, state, &moves)
}

fn short_rollout(game: &Game, state: &State, models: &[AiModel], first_mv: (usize, usize), steps: usize) -> f64 {
    let mut cur = state.clone();
    let _top2 = crate::choose_predicted_ai_top2_moves(game, &cur, models);
    for step in 0..steps {
        let mut moves = Vec::with_capacity(game.m);
        if step == 0 {
            moves.push(first_mv);
        } else {
            let cands = crate::get_candidates(game, &cur, 0);
            if cands.is_empty() {
                break;
            }
            let scores = crate::calc_scores(game, &cur);
            let s0 = scores[0] as f64;
            let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
            let phase = cur.turn as f64 / game.t as f64;
            let conflict = crate::estimate_conflict_map(game, &cur, models);
            let mut leaders = vec![false; game.m];
            for p in 1..game.m {
                if scores[p] == max_ai_i64 {
                    leaders[p] = true;
                }
            }
            let mut best_mv = cands[0];
            let mut best_score = f64::NEG_INFINITY;
            for &mv in &cands {
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
                    &leaders,
                );
                if v > best_score {
                    best_score = v;
                    best_mv = mv;
                }
            }
            moves.push(best_mv);
        }
        let cur_top2 = crate::choose_predicted_ai_top2_moves(game, &cur, models);
        let primary: Vec<(usize, usize)> = cur_top2.iter().map(|x| x.0).collect();
        let uncertainty = crate::uncertainty_risk(&cur_top2);
        moves.extend(
            if uncertainty >= 0.24 {
                let secondary = crate::build_secondary_ai_moves(
                    &crate::calc_scores(game, &cur),
                    &cur_top2,
                    1,
                );
                secondary.into_iter().take(1)
            } else {
                primary.into_iter().take(1)
            },
        );
        cur = crate::simulate_turn(game, &cur, &moves);
    }
    crate::strategic_score(game, &cur) - crate::strategic_score(game, state)
}

pub(super) fn choose_move_x18_robust_minmax_guard(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
    if !(4..=6).contains(&game.m) {
        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
    }

    let candidates = crate::get_candidates(game, state, 0);
    if candidates.len() <= 1 {
        return candidates.first().copied().unwrap_or(state.pos[0]);
    }

    let top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
    let uncertainty = crate::uncertainty_risk(&top2);
    let scores = crate::calc_scores(game, state);
    let s0 = scores[0] as f64;
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let phase = state.turn as f64 / game.t as f64;
    let conflict = crate::estimate_conflict_map(game, state, models);
    let mut leaders = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai_i64 {
            leaders[p] = true;
        }
    }

    let primary_set = opponent_set_by_mode(game, state, models, 0, &Vec::new());
    let secondary_set = opponent_set_by_mode(game, state, models, 1, &Vec::new());

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
            &conflict,
            state.pos[0],
            &leaders,
        );
        ranked.push((mv, local));
    }
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let mut best_mv = ranked[0].0;
    let mut best_val = f64::NEG_INFINITY;
    let candidate_cap = ranked.len().min(7);

    for &(mv, local) in ranked.iter().take(candidate_cap) {
        let p_state = simulate_with_set(game, state, mv, &primary_set);
        let s_state = simulate_with_set(game, state, mv, &secondary_set);
        let p_score = crate::strategic_score(game, &p_state);
        let s_score = crate::strategic_score(game, &s_state);

        let mut worst_score = p_score.min(s_score);
        for ai_idx in 0..(game.m.saturating_sub(1)) {
            let mut alt_set = secondary_set.clone();
            if alt_set[ai_idx] != primary_set[ai_idx] {
                let p2 = vec![secondary_set[ai_idx][0]];
                alt_set[ai_idx] = p2;
                let alt_state = simulate_with_set(game, state, mv, &alt_set);
                let alt_score = crate::strategic_score(game, &alt_state);
                if alt_score < worst_score {
                    worst_score = alt_score;
                }
            }
        }

        let short = short_rollout(game, state, models, mv, 2);
        let base_adv = local * 0.12;
        let conflict_penalty = (1.0 - (-conflict[mv.0][mv.1]).exp()) * game.v[mv.0][mv.1] as f64;
        let robust = 0.60 * worst_score + 0.30 * ((p_score + s_score) * 0.5) + 0.10 * short;
        let val = robust + base_adv - 0.45 * conflict_penalty;
        let val = if uncertainty >= 0.30 {
            val - 0.20 * conflict_penalty
        } else {
            val
        };

        if val > best_val {
            best_val = val;
            best_mv = mv;
        }
    }

    if best_val.is_finite() {
        best_mv
    } else if phase <= 0.45 {
        x04_macro_route::choose_move_x04_macro_route(game, state, models)
    } else {
        x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models)
    }
}
