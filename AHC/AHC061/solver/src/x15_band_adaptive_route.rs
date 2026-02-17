use crate::{
    x04_macro_route, x06_expert_switch_hybrid, AiModel, Game, State,
};

#[derive(Clone)]
struct RouteNode {
    state: State,
    score: f64,
}

fn manhattan(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn targets_for_band(game: &Game, state: &State, max_targets: usize) -> Vec<(usize, usize)> {
    let mut scored = Vec::<((usize, usize), f64)>::new();
    for x in 0..game.n {
        for y in 0..game.n {
            let owner = state.owner[x][y];
            let level = state.level[x][y];
            let v = game.v[x][y] as f64;
            let mut w = 0.0;
            if owner == -1 {
                w = 1.00 * v;
            } else if owner > 0 && level == 1 {
                w = 1.22 * v;
            } else if owner == 0 && level < game.u {
                w = 0.75 * v * (game.u - level) as f64 / game.u as f64;
            }
            if w > 0.0 {
                scored.push(((x, y), w));
            }
        }
    }
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scored
        .into_iter()
        .take(max_targets)
        .map(|x| x.0)
        .collect()
}

fn predicted_ai_moves(game: &Game, state: &State, models: &[AiModel], step: usize, uncertainty: f64) -> Vec<(usize, usize)> {
    let top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
    let primary: Vec<(usize, usize)> = top2.iter().map(|x| x.0).collect();
    if step >= 3 {
        return primary;
    }
    let secondary_cap = if game.m >= 7 && uncertainty >= 0.36 {
        3
    } else if game.m >= 6 && uncertainty >= 0.24 {
        2
    } else {
        1
    };
    let scores = crate::calc_scores(game, state);
    let secondary = crate::build_secondary_ai_moves(&scores, &top2, secondary_cap);
    if uncertainty >= 0.18 && step % 2 == 1 {
        secondary
    } else {
        primary
    }
}

fn route_increment(game: &Game, prev: &State, next: &State, mv: (usize, usize), target: (usize, usize), local: f64) -> f64 {
    let prev_score = crate::strategic_score(game, prev);
    let next_score = crate::strategic_score(game, next);
    let gain = next_score - prev_score;
    let d0 = manhattan(prev.pos[0], target) as f64;
    let d1 = manhattan(mv, target) as f64;
    let route_bonus = (d0 - d1).clamp(-8.0, 8.0);
    gain + 48.0 * route_bonus + 0.05 * local
}

fn beam_route_score(
    game: &Game,
    state: &State,
    models: &[AiModel],
    first_mv: (usize, usize),
    target: (usize, usize),
    plan_len: usize,
    beam_width: usize,
    branch_width: usize,
    uncertainty: f64,
) -> f64 {
    let scores = crate::calc_scores(game, state);
    let s0 = scores[0] as f64;
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let phase = state.turn as f64 / game.t as f64;
    let conflict = crate::estimate_conflict_map(game, state, models);
    let cur = state.pos[0];
    let mut is_leader = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai_i64 {
            is_leader[p] = true;
        }
    }
    let local = crate::evaluate_local_move(
        game,
        state,
        first_mv,
        &scores,
        s0,
        max_ai_i64,
        phase,
        &conflict,
        cur,
        &is_leader,
    );

    let first_moves = {
        let mut m = Vec::with_capacity(game.m);
        m.push(first_mv);
        m.extend_from_slice(&predicted_ai_moves(game, state, models, 0, uncertainty));
        crate::simulate_turn(game, state, &m)
    };

    let first_inc = route_increment(game, state, &first_moves, first_mv, target, local);
    let mut beam = vec![RouteNode {
        state: first_moves,
        score: first_inc,
    }];

    for step in 1..plan_len {
        let mut next_beam = Vec::<RouteNode>::new();
        for node in &beam {
            let scores = crate::calc_scores(game, &node.state);
            let max_i = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
            let phase_cur = node.state.turn as f64 / game.t as f64;
            let conflict_cur = crate::estimate_conflict_map(game, &node.state, models);
            let now = node.state.pos[0];
            let mut is_leader = vec![false; game.m];
            for p in 1..game.m {
                if scores[p] == max_i {
                    is_leader[p] = true;
                }
            }
            let mut cands = crate::get_candidates(game, &node.state, 0);
            if cands.is_empty() {
                continue;
            }
            let s0_cur = scores[0] as f64;
            cands.sort_by(|&a, &b| {
                let la = crate::evaluate_local_move(
                    game,
                    &node.state,
                    a,
                    &scores,
                    s0_cur,
                    max_i,
                    phase_cur,
                    &conflict_cur,
                    now,
                    &is_leader,
                );
                let lb = crate::evaluate_local_move(
                    game,
                    &node.state,
                    b,
                    &scores,
                    s0_cur,
                    max_i,
                    phase_cur,
                    &conflict_cur,
                    now,
                    &is_leader,
                );
                lb.partial_cmp(&la).unwrap_or(std::cmp::Ordering::Equal)
            });

            for &mv in cands.iter().take(branch_width) {
                let mut full_moves = Vec::with_capacity(game.m);
                full_moves.push(mv);
                full_moves.extend_from_slice(&predicted_ai_moves(game, &node.state, models, step, uncertainty));
                let ns = crate::simulate_turn(game, &node.state, &full_moves);
                let inc = route_increment(game, &node.state, &ns, mv, target, local);
                let discount = (0.94_f64).powi(step as i32);
                let route_bias = crate::strategic_score(game, &ns) * 0.01;
                next_beam.push(RouteNode {
                    state: ns,
                    score: node.score + discount * inc + route_bias,
                });
            }
        }
        if next_beam.is_empty() {
            break;
        }
        next_beam.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        next_beam.truncate(beam_width);
        beam = next_beam;
    }

    beam.iter()
        .map(|n| n.score + 0.02 * crate::strategic_score(game, &n.state))
        .fold(f64::NEG_INFINITY, f64::max)
}

fn band_plan(_game: &Game, m: usize, phase: f64, uncertainty: f64, conflict: f64) -> (f64, usize, usize, usize, usize) {
    match m {
        4 => {
            let cutoff = if uncertainty >= 0.24 { 0.74 } else { 0.82 };
            if phase >= cutoff {
                (1.0, 5, 2, 2, 10)
            } else if phase >= 0.55 {
                (0.7, 6, 2, 2, 8)
            } else {
                let w = if phase <= 0.25 { 0.4 } else { 0.7 };
                let target_cap = if conflict <= 0.30 { 5 } else { 4 };
                (w, 7, 2, 2, target_cap)
            }
        }
        5 => {
            let conflict_hi = if conflict >= 0.90 { 0.5 } else { 0.65 };
            if phase >= 0.66 || uncertainty >= 0.38 {
                (0.6, 5, 2, 1, 5)
            } else if phase >= 0.45 {
                (0.7, 6, 2, 2, 4)
            } else {
                (conflict_hi, 5, 2, 2, 5)
            }
        }
        6 => {
            if phase >= 0.60 || uncertainty >= 0.36 {
                (0.3, 4, 1, 1, 4)
            } else if phase >= 0.40 {
                (0.5, 5, 1, 2, 4)
            } else {
                (0.6, 5, 1, 2, 4)
            }
        }
        _ => (0.0, 0, 0, 0, 0),
    }
}

pub(super) fn choose_move_x15_band_adaptive_route(
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
    let conflict_map = crate::estimate_conflict_map(game, state, models);
    let mut is_leader = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai_i64 {
            is_leader[p] = true;
        }
    }

    let cands = crate::get_candidates(game, state, 0);
    if cands.len() <= 1 {
        return cands.first().copied().unwrap_or(state.pos[0]);
    }

    let (weight, plan_len, branch_width, target_cap, local_cap) = band_plan(
        game,
        game.m,
        phase,
        uncertainty,
        conflict_map[state.pos[0].0][state.pos[0].1],
    );
    if weight <= 0.0 {
        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
    }

    let mut ranked = Vec::<((usize, usize), f64)>::new();
    for &mv in &cands {
        let local = crate::evaluate_local_move(
            game,
            state,
            mv,
            &scores,
            s0,
            max_ai_i64,
            phase,
            &conflict_map,
            state.pos[0],
            &is_leader,
        );
        ranked.push((mv, local));
    }
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let mut use_route = false;
    if game.m == 4 && phase <= 0.82 {
        use_route = true;
    } else if game.m == 5 && phase <= 0.66 {
        use_route = true;
    } else if game.m == 6 && phase <= 0.58 && uncertainty < 0.36 {
        use_route = true;
    }
    if game.m == 5 {
        if uncertainty >= 0.35 || phase > 0.64 {
            use_route = false;
        }
    }

    let target_count = target_cap.max(1).min(8);
    let targets = targets_for_band(game, state, target_count);
    if !use_route || targets.is_empty() {
        let mut fallback = ranked[0].0;
        let mut fallback_score = f64::NEG_INFINITY;
        for &(mv, local) in ranked.iter().take(local_cap.max(3).min(ranked.len())) {
            let mut m = Vec::with_capacity(game.m);
            m.push(mv);
            let top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
            let uncertainty = crate::uncertainty_risk(&top2);
            if uncertainty > 0.22 {
                let secondary = crate::build_secondary_ai_moves(&scores, &top2, 1);
                m.extend(secondary);
            } else {
                m.extend(top2.iter().map(|x| x.0));
            }
            let next = crate::simulate_turn(game, state, &m);
            let value = crate::strategic_score(game, &next) + 0.08 * local + 0.10 * (crate::strategic_score(game, &next) - crate::strategic_score(game, state));
            if value > fallback_score {
                fallback_score = value;
                fallback = mv;
            }
        }
        return if fallback_score.is_finite() {
            fallback
        } else {
            x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models)
        };
    }

    let mut best_mv = ranked[0].0;
    let mut best_score = f64::NEG_INFINITY;
    let beam_width = if game.m == 6 { 4 } else if phase > 0.5 { 5 } else { 6 };
    for &(mv, local) in ranked.iter().take(local_cap.min(ranked.len())) {
        let mut best_target = f64::NEG_INFINITY;
        for &target in targets.iter().take(4) {
            let rs = beam_route_score(
                game,
                state,
                models,
                mv,
                target,
                plan_len,
                beam_width.max(2),
                branch_width.max(1),
                uncertainty,
            );
            if rs > best_target {
                best_target = rs;
            }
        }
        let route_bonus = if game.m == 4 { 1.0 } else { 0.82 };
        let conflict = conflict_map[mv.0][mv.1];
        let total = weight * best_target + route_bonus * local + 0.08 * local - 0.70 * conflict * game.v[mv.0][mv.1] as f64;
        if total > best_score {
            best_score = total;
            best_mv = mv;
        }
    }

    if best_score.is_finite() {
        best_mv
    } else {
        let fallback = x04_macro_route::choose_move_x04_macro_route(game, state, models);
        if game.m <= 5 || fallback == state.pos[0] {
            x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models)
        } else {
            fallback
        }
    }
}
