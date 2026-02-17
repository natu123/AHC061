use crate::{
    x01_beam_pessimistic, x04_macro_route, x06_expert_switch_hybrid, x11_contest_frontier_recovery,
    AiModel, Game, State,
};

fn frontier_sweep_score(game: &Game, state: &State, mv: (usize, usize)) -> f64 {
    let (x, y) = mv;
    let owner = state.owner[x][y];
    let level = state.level[x][y];
    let value = game.v[x][y] as f64;
    let mut score = 0.0_f64;

    if owner == -1 {
        score += 1.1 * value;
    } else if owner == 0 && level < game.u {
        score += 0.70 * value * (game.u - level) as f64 / game.u as f64;
    } else if owner > 0 && level == 1 {
        score += 0.80 * value;
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
        let o_owner = state.owner[ux][uy];
        if o_owner == 0 {
            score += 0.13 * ov;
            if state.level[ux][uy] == 1 {
                score += 0.11 * ov;
            }
        } else if o_owner == -1 {
            score += 0.09 * ov;
        } else if state.level[ux][uy] < game.u {
            score += 0.06 * ov / state.level[ux][uy] as f64;
        }
    }
    score
}

fn opponent_set(
    game: &Game,
    state: &State,
    top2: &[((usize, usize), (usize, usize), f64)],
    secondary: bool,
) -> Vec<(usize, usize)> {
    let mut out = Vec::with_capacity(game.m);
    out.push((0, 0));
    for ai_idx in 0..(game.m.saturating_sub(1)) {
        let cur_top2 = top2[ai_idx];
        let fallback = state.pos[ai_idx + 1];
        if secondary && cur_top2.1 != fallback {
            out.push(cur_top2.1);
        } else {
            out.push(cur_top2.0);
        }
        if out.len() == game.m {
            break;
        }
    }
    while out.len() < game.m {
        out.push(state.pos[0]);
    }
    out.truncate(game.m);
    out
}

fn sweep_rollout(
    game: &Game,
    state: &State,
    models: &[AiModel],
    first_move: (usize, usize),
    steps: usize,
) -> f64 {
    let mut cur = state.clone();
    for step in 0..steps {
        let mut moves = Vec::with_capacity(game.m);
        if step == 0 {
            moves.push(first_move);
        } else {
            let cands = crate::get_candidates(game, &cur, 0);
            if cands.is_empty() {
                break;
            }
            let scores = crate::calc_scores(game, &cur);
            let top2 = crate::choose_predicted_ai_top2_moves(game, &cur, models);
            let s0 = scores[0] as f64;
            let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
            let phase = cur.turn as f64 / game.t as f64;
            let conflict = crate::estimate_conflict_map(game, &cur, models);
            let mut leaders = vec![false; game.m];
            for p in 1..game.m {
                if scores[p] == max_ai {
                    leaders[p] = true;
                }
            }
            let mut best = cands[0];
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
                    best = mv;
                }
            }
            moves.push(best);
            let uncertain = crate::uncertainty_risk(&top2);
            if uncertain >= 0.22 {
                moves.extend(crate::build_secondary_ai_moves(&scores, &top2, 1));
            } else {
                moves.extend(top2.iter().map(|x| x.0));
            }
        }
        cur = crate::simulate_turn(game, &cur, &moves);
    }
    crate::strategic_score(game, &cur) - crate::strategic_score(game, state)
}

pub(super) fn choose_move_x19_frontier_recovery_sweep(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
    if game.m <= 4 {
        return x04_macro_route::choose_move_x04_macro_route(game, state, models);
    }
    if game.m >= 7 {
        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
    }

    let candidates = crate::get_candidates(game, state, 0);
    if candidates.is_empty() {
        return state.pos[0];
    }
    if candidates.len() == 1 {
        return candidates[0];
    }

    let top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
    let scores = crate::calc_scores(game, state);
    let uncertainty = crate::uncertainty_risk(&top2);
    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let s0 = scores[0] as f64;
    let phase = state.turn as f64 / game.t as f64;
    let conflict = crate::estimate_conflict_map(game, state, models);
    let secondary = crate::build_secondary_ai_moves(&scores, &top2, 1);
    let mut is_leader = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai {
            is_leader[p] = true;
        }
    }

    let advisor_mvs = [
        x04_macro_route::choose_move_x04_macro_route(game, state, models),
        x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models),
        x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models),
        x11_contest_frontier_recovery::choose_move_x11_contest_frontier_recovery(game, state, models),
    ];

    let mut ranked = Vec::<((usize, usize), f64)>::new();
    for &mv in &candidates {
        let local = crate::evaluate_local_move(
            game,
            state,
            mv,
            &scores,
            s0,
            max_ai,
            phase,
            &conflict,
            state.pos[0],
            &is_leader,
        );
        let fs = frontier_sweep_score(game, state, mv);
        let mut defense = 0.0_f64;
        if state.owner[mv.0][mv.1] == 0 {
            defense += 0.05 * game.v[mv.0][mv.1] as f64;
        }
        ranked.push((mv, local + 0.06 * fs + 0.10 * defense));
    }
    for &mv in &advisor_mvs {
        if !candidates.contains(&mv) {
            continue;
        }
        ranked.push((mv, 0.90 * ranked[0].1));
    }
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    ranked.dedup_by_key(|x| x.0);
    if ranked.is_empty() {
        return state.pos[0];
    }

    let primary = opponent_set(game, state, &top2, false);
    let secondary_set = opponent_set(game, state, &top2, true);
    let mut best_mv = ranked[0].0;
    let mut best_v = f64::NEG_INFINITY;

    let sweep_cap = ranked.len().min(9);
    for &(mv, local) in ranked.iter().take(sweep_cap) {
        let mut primary_moves = Vec::with_capacity(game.m);
        primary_moves.push(mv);
        primary_moves.extend_from_slice(&primary[1..game.m]);
        let mut secondary_moves = Vec::with_capacity(game.m);
        secondary_moves.push(mv);
        secondary_moves.extend_from_slice(&secondary_set[1..game.m]);

        let s_primary = crate::simulate_turn(game, state, &primary_moves);
        let s_secondary = crate::simulate_turn(game, state, &secondary_moves);
        let p_score = 0.62 * crate::strategic_score(game, &s_primary);
        let s_score = 0.38 * crate::strategic_score(game, &s_secondary);
        let rollout = 0.12 * sweep_rollout(game, state, models, mv, 2);
        let conflict_penalty = conflict[mv.0][mv.1] * game.v[mv.0][mv.1] as f64;
        let frontier_bonus = crate::frontier_potential(game, &crate::simulate_turn(game, state, &primary_moves)) * 0.25;
        let uncertainty_bonus = uncertainty * 0.08 * local;
        let alt_gain = if game.m >= 6 && uncertainty >= 0.30 {
            0.25 * local + 0.12 * frontier_bonus
        } else {
            0.15 * local
        };

        let value = p_score + s_score + rollout + alt_gain + uncertainty_bonus - 0.70 * conflict_penalty;
        if secondary.iter().any(|&s| s == mv) {
            let secondary_value = value + 0.03 * local;
            if secondary_value > best_v {
                best_v = secondary_value;
                best_mv = mv;
            }
        } else if value > best_v {
            best_v = value;
            best_mv = mv;
        }
    }

    if best_v.is_finite() {
        best_mv
    } else if uncertainty >= 0.35 {
        x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models)
    } else {
        x04_macro_route::choose_move_x04_macro_route(game, state, models)
    }
}
