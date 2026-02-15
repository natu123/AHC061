use crate::{x06_expert_switch_hybrid, AiModel, Game, State};

fn frontier_pressure(game: &Game, state: &State, mv: (usize, usize)) -> f64 {
    let (x, y) = mv;
    let owner = state.owner[x][y];
    let level = state.level[x][y];
    let value = game.v[x][y] as f64;

    let mut score = 0.0;
    if owner == -1 {
        score += 1.00 * value;
    } else if owner > 0 && level == 1 {
        score += 1.20 * value;
    } else if owner == 0 && level < game.u {
        score += 0.70 * value * (game.u - level) as f64 / game.u as f64;
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
        let o = state.owner[ux][uy];
        let lv = state.level[ux][uy].max(1) as f64;
        if o == -1 {
            score += 0.10 * ov;
        } else if o > 0 {
            if state.level[ux][uy] == 1 {
                score += 0.28 * ov;
            } else {
                score += 0.14 * ov / lv;
            }
        } else if state.level[ux][uy] < game.u {
            score += 0.08 * ov;
        }
    }
    score
}

pub(super) fn choose_move_x08_pressure_frontier(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
    if !(3..=6).contains(&game.m) {
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
    let mut is_leader = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai_i64 {
            is_leader[p] = true;
        }
    }

    let top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
    let uncertainty = crate::uncertainty_risk(&top2);
    let primary: Vec<(usize, usize)> = top2.iter().map(|x| x.0).collect();
    let secondary_cap = if game.m >= 7 && uncertainty >= 0.40 {
        3
    } else if game.m >= 6 && uncertainty >= 0.28 {
        2
    } else {
        1
    };
    let secondary = crate::build_secondary_ai_moves(&scores, &top2, secondary_cap);

    let candidate_cap = if candidates.len() >= 24 {
        10
    } else {
        candidates.len().min(7)
    };

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

    let mut best_mv = ranked[0].0;
    let mut best_total = f64::NEG_INFINITY;
    for &(mv, local) in ranked.iter().take(candidate_cap) {
        let mut m1 = Vec::with_capacity(game.m);
        m1.push(mv);
        m1.extend_from_slice(&primary);
        let ns1 = crate::simulate_turn(game, state, &m1);
        let gain1 = crate::strategic_score(game, &ns1) - crate::strategic_score(game, state);

        let mut m2 = Vec::with_capacity(game.m);
        m2.push(mv);
        m2.extend_from_slice(&secondary);
        let ns2 = crate::simulate_turn(game, state, &m2);
        let gain2 = crate::strategic_score(game, &ns2) - crate::strategic_score(game, state);

        let pressure = frontier_pressure(game, state, mv);
        let risk_penalty = conflict_map[mv.0][mv.1] * game.v[mv.0][mv.1] as f64 * 0.35;
        let total = 0.58 * gain1
            + 0.42 * gain2
            + (0.68 + 0.08 * (1.0 - phase)) * local
            + 0.20 * pressure
            - risk_penalty;
        if total > best_total {
            best_total = total;
            best_mv = mv;
        }
    }
    best_mv
}
