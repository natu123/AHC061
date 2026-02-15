use crate::{x06_expert_switch_hybrid, AiModel, Game, State};

#[derive(Clone)]
struct RouteNode {
    state: State,
    score: f64,
}

fn manhattan(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn choose_target_cells(game: &Game, state: &State, max_targets: usize) -> Vec<(usize, usize)> {
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
                w = 1.18 * v;
            } else if owner == 0 && level < game.u {
                w = 0.72 * v * (game.u - level) as f64 / game.u as f64;
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

fn route_increment(
    game: &Game,
    prev: &State,
    next: &State,
    mv: (usize, usize),
    target: (usize, usize),
    local: f64,
) -> f64 {
    let prev_score = crate::strategic_score(game, prev);
    let next_score = crate::strategic_score(game, next);
    let gain = next_score - prev_score;
    let d0 = manhattan(prev.pos[0], target) as f64;
    let d1 = manhattan(mv, target) as f64;
    let route_bonus = (d0 - d1).clamp(-6.0, 6.0);
    gain + 42.0 * route_bonus + 0.07 * local
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
) -> f64 {
    let mut moves = Vec::with_capacity(game.m);
    moves.push(first_mv);
    moves.extend_from_slice(&predicted_ai_moves(game, state, models, 0));
    let first_state = crate::simulate_turn(game, state, &moves);

    let first_local = crate::evaluate_local_move(
        game,
        state,
        first_mv,
        &crate::calc_scores(game, state),
        crate::calc_scores(game, state)[0] as f64,
        crate::calc_scores(game, state).iter().skip(1).copied().max().unwrap_or(1).max(1),
        state.turn as f64 / game.t as f64,
        &crate::estimate_conflict_map(game, state, models),
        state.pos[0],
        &{
            let mut is_leader = vec![false; game.m];
            let scores = crate::calc_scores(game, state);
            let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
            for p in 1..game.m {
                if scores[p] == max_ai {
                    is_leader[p] = true;
                }
            }
            is_leader
        },
    );

    let base_gain = route_increment(game, state, &first_state, first_mv, target, first_local);
    let mut beam = vec![RouteNode {
        state: first_state,
        score: base_gain,
    }];

    for step in 1..plan_len {
        let mut next_beam = Vec::<RouteNode>::new();
        for node in &beam {
            let scores = crate::calc_scores(game, &node.state);
            let s0 = scores[0] as f64;
            let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
            let phase = node.state.turn as f64 / game.t as f64;
            let conflict = crate::estimate_conflict_map(game, &node.state, models);
            let cur = node.state.pos[0];
            let mut is_leader = vec![false; game.m];
            for p in 1..game.m {
                if scores[p] == max_ai_i64 {
                    is_leader[p] = true;
                }
            }

            let mut my_cands = crate::get_candidates(game, &node.state, 0);
            if my_cands.is_empty() {
                continue;
            }
            my_cands.sort_by(|&a, &b| {
                let la = crate::evaluate_local_move(
                    game, &node.state, a, &scores, s0, max_ai_i64, phase, &conflict, cur, &is_leader,
                ) + 18.0
                    * (manhattan(cur, target) as f64 - manhattan(a, target) as f64);
                let lb = crate::evaluate_local_move(
                    game, &node.state, b, &scores, s0, max_ai_i64, phase, &conflict, cur, &is_leader,
                ) + 18.0
                    * (manhattan(cur, target) as f64 - manhattan(b, target) as f64);
                lb.partial_cmp(&la).unwrap_or(std::cmp::Ordering::Equal)
            });

            for &mv in my_cands.iter().take(branch_width) {
                let local = crate::evaluate_local_move(
                    game, &node.state, mv, &scores, s0, max_ai_i64, phase, &conflict, cur, &is_leader,
                );
                let mut full_moves = Vec::with_capacity(game.m);
                full_moves.push(mv);
                full_moves.extend_from_slice(&predicted_ai_moves(game, &node.state, models, step));
                let ns = crate::simulate_turn(game, &node.state, &full_moves);
                let discount = (0.93_f64).powi(step as i32);
                let inc = route_increment(game, &node.state, &ns, mv, target, local);
                next_beam.push(RouteNode {
                    state: ns,
                    score: node.score + discount * inc,
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
        .map(|n| n.score + 0.03 * crate::strategic_score(game, &n.state))
        .fold(f64::NEG_INFINITY, f64::max)
}

pub(super) fn choose_move_x04_macro_route(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
    // full結果の帯別分析で M=4 のみ優位が大きかったため、適用帯を限定する。
    if game.m != 4 {
        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
    }
    let phase_cutoff = std::env::var("AHC_X04_PHASE_CUTOFF")
        .ok()
        .and_then(|x| x.parse::<f64>().ok())
        .unwrap_or(0.65);
    let phase_now = state.turn as f64 / game.t as f64;
    if phase_now > phase_cutoff {
        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
    }

    let candidates = crate::get_candidates(game, state, 0);
    if candidates.len() <= 1 {
        return candidates.first().copied().unwrap_or(state.pos[0]);
    }

    let targets = choose_target_cells(game, state, 5);
    if targets.is_empty() {
        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
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
    let candidate_cap = if ranked.len() >= 20 {
        7
    } else {
        ranked.len().min(5)
    };

    let fast_phase = phase_now > 0.50;
    let plan_len = if fast_phase { 6 } else { 7 };
    let beam_width = if fast_phase { 4 } else { 5 };
    let branch_width = 2;

    let mut best_mv = ranked[0].0;
    let mut best_score = f64::NEG_INFINITY;
    for &(mv, local) in ranked.iter().take(candidate_cap) {
        let mut best_target_score = f64::NEG_INFINITY;
        for &target in targets.iter().take(4) {
            let route_score = beam_route_score(
                game,
                state,
                models,
                mv,
                target,
                plan_len,
                beam_width,
                branch_width,
            );
            if route_score > best_target_score {
                best_target_score = route_score;
            }
        }
        let total = best_target_score + 0.10 * local;
        if total > best_score {
            best_score = total;
            best_mv = mv;
        }
    }
    best_mv
}
