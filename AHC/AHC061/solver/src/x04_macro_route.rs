use crate::{x06_expert_switch_hybrid, AiModel, Game, State};
#[derive(Clone)]
struct RouteNode {
    state: State,
    score: f64,
}

fn env_u64(name: &str, default: u64, min: u64, max: u64) -> u64 {
    if let Ok(v) = std::env::var(name) {
        if let Ok(x) = v.parse::<u64>() {
            return x.max(min).min(max);
        }
    }
    default.max(min).min(max)
}

fn env_f64(name: &str, default: f64, min: f64, max: f64) -> f64 {
    if let Ok(v) = std::env::var(name) {
        if let Ok(x) = v.parse::<f64>() {
            return x.max(min).min(max);
        }
    }
    default.max(min).min(max)
}

fn manhattan(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn clamp01(x: f64) -> f64 {
    x.clamp(0.0, 1.0)
}

fn pressure_weight_by_phase(
    phase: f64,
    split: f64,
    early: f64,
    late: f64,
) -> f64 {
    if phase <= split { early } else { late }
}

fn estimate_move_pressure(
    game: &Game,
    state: &State,
    conflict: &[Vec<f64>],
    mv: (usize, usize),
    phase: f64,
) -> f64 {
    const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut neigh = 0.0;
    let mut neigh_n = 0_u32;
    for (dx, dy) in DIRS {
        let nx = mv.0 as isize + dx;
        let ny = mv.1 as isize + dy;
        if crate::in_bounds(game.n, nx, ny) {
            neigh += conflict[nx as usize][ny as usize];
            neigh_n += 1;
        }
    }
    let neigh = if neigh_n > 0 {
        neigh / neigh_n as f64
    } else {
        0.0
    };
    let owner = state.owner[mv.0][mv.1];
    let owner_factor = if owner == -1 {
        0.22
    } else if owner == 0 {
        0.48
    } else {
        1.0 + (state.level[mv.0][mv.1] as f64) / (game.u as f64 + 1.0)
    };
    let base = 0.75 * clamp01(conflict[mv.0][mv.1]) + 0.25 * clamp01(neigh);
    base * owner_factor * (1.0 + 0.5 * phase).clamp(0.5, 1.5)
}

fn choose_target_cells(
    game: &Game,
    state: &State,
    conflict: &[Vec<f64>],
    max_targets: usize,
    pressure_weight: f64,
    phase: f64,
) -> Vec<(usize, usize)> {
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
                if pressure_weight.abs() > f64::EPSILON {
                    let p = estimate_move_pressure(game, state, conflict, (x, y), phase);
                    w += -pressure_weight * p * v;
                }
                scored.push(((x, y), w));
            }
        }
    }
    if scored.is_empty() {
        return Vec::new();
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
    pressure: f64,
    pressure_weight: f64,
    local_coeff: f64,
    route_coeff: f64,
) -> f64 {
    let prev_score = crate::strategic_score(game, prev);
    let next_score = crate::strategic_score(game, next);
    let gain = next_score - prev_score;
    let d0 = manhattan(prev.pos[0], target) as f64;
    let d1 = manhattan(mv, target) as f64;
    let route_bonus = (d0 - d1).clamp(-6.0, 6.0);
    gain + route_coeff * route_bonus - pressure_weight * pressure + local_coeff * local
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
    pressure_phase_split: f64,
    pressure_weight_early: f64,
    pressure_weight_late: f64,
    local_coeff: f64,
    route_coeff: f64,
) -> f64 {
    let mut moves = Vec::with_capacity(game.m);
    moves.push(first_mv);
    moves.extend_from_slice(&predicted_ai_moves(game, state, models, 0));
    let first_state = crate::simulate_turn(game, state, &moves);
    let first_conflict = crate::estimate_conflict_map(game, state, models);

    let first_phase = state.turn as f64 / game.t as f64;
    let first_pressure = estimate_move_pressure(game, state, &first_conflict, first_mv, first_phase);
    let first_local = crate::evaluate_local_move(
        game,
        state,
        first_mv,
        &crate::calc_scores(game, state),
        crate::calc_scores(game, state)[0] as f64,
        crate::calc_scores(game, state).iter().skip(1).copied().max().unwrap_or(1).max(1),
        state.turn as f64 / game.t as f64,
        &first_conflict,
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
    let first_pressure_weight =
        pressure_weight_by_phase(first_phase, pressure_phase_split, pressure_weight_early, pressure_weight_late);

    let base_gain = route_increment(
        game,
        state,
        &first_state,
        first_mv,
        target,
        first_local,
        first_pressure,
        first_pressure_weight,
        local_coeff,
        route_coeff,
    );
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
            let pressure = estimate_move_pressure(game, &node.state, &conflict, mv, phase);
            let pressure_weight =
                pressure_weight_by_phase(phase, pressure_phase_split, pressure_weight_early, pressure_weight_late);
            let mut full_moves = Vec::with_capacity(game.m);
            full_moves.push(mv);
            full_moves.extend_from_slice(&predicted_ai_moves(game, &node.state, models, step));
            let ns = crate::simulate_turn(game, &node.state, &full_moves);
            let discount = (0.93_f64).powi(step as i32);
            let inc = route_increment(
                game,
                &node.state,
                &ns,
                mv,
                target,
                local,
                pressure,
                pressure_weight,
                local_coeff,
                route_coeff,
            );
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
    let phase_cutoff = env_f64("AHC_X04_PHASE_CUTOFF", 0.65, 0.20, 0.95);
    let phase_split = env_f64("AHC_X04_PHASE_SPLIT", 0.50, 0.20, 0.90);
    let target_count = env_u64("AHC_X04_TARGET_COUNT", 5, 2, 12) as usize;
    let target_eval = env_u64("AHC_X04_TARGET_EVAL", 4, 1, 8) as usize;
    let candidate_cap = env_u64("AHC_X04_CANDIDATE_CAP", 7, 4, 20) as usize;
    let plan_len_fast = env_u64("AHC_X04_PLAN_LEN_FAST", 6, 3, 10) as usize;
    let plan_len_slow = env_u64("AHC_X04_PLAN_LEN_SLOW", 7, 3, 10) as usize;
    let beam_width_fast = env_u64("AHC_X04_BEAM_WIDTH_FAST", 4, 2, 8) as usize;
    let beam_width_slow = env_u64("AHC_X04_BEAM_WIDTH_SLOW", 5, 2, 8) as usize;
    let branch_width = env_u64("AHC_X04_BRANCH_WIDTH", 2, 1, 4) as usize;
    let local_weight = env_f64("AHC_X04_LOCAL_WEIGHT", 0.10, 0.0, 0.40);
    let local_coeff = env_f64("AHC_X04_LOCAL_COEFF", 0.07, 0.0, 0.40);
    let route_coeff = env_f64("AHC_X04_ROUTE_COEFF", 42.0, 10.0, 90.0);
    let target_pressure_weight = env_f64("AHC_X04_TARGET_PRESSURE_WEIGHT", 0.0, -1.0, 2.0);
    let pressure_weight = env_f64("AHC_X04_PRESSURE_WEIGHT", 0.00, -1.5, 2.0);
    let pressure_weight_early = env_f64(
        "AHC_X04_PRESSURE_WEIGHT_EARLY",
        pressure_weight,
        -1.5,
        2.0,
    );
    let pressure_weight_late = env_f64(
        "AHC_X04_PRESSURE_WEIGHT_LATE",
        pressure_weight,
        -1.5,
        2.0,
    );
    let pressure_route_weight = env_f64("AHC_X04_ROUTE_PRESSURE_WEIGHT", 0.00, -1.5, 2.0);
    let pressure_route_weight_early = env_f64(
        "AHC_X04_ROUTE_PRESSURE_WEIGHT_EARLY",
        pressure_route_weight,
        -1.5,
        2.0,
    );
    let pressure_route_weight_late = env_f64(
        "AHC_X04_ROUTE_PRESSURE_WEIGHT_LATE",
        pressure_route_weight,
        -1.5,
        2.0,
    );
    let pressure_phase_split = env_f64("AHC_X04_PRESSURE_PHASE_SPLIT", 0.60, 0.20, 0.90);
    let phase_now = state.turn as f64 / game.t as f64;
    if phase_now > phase_cutoff {
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
    let targets = choose_target_cells(
        game,
        state,
        &conflict_map,
        target_count,
        target_pressure_weight,
        phase,
    );
    if targets.is_empty() {
        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
    }
    let cur = state.pos[0];
    let mut is_leader = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai_i64 {
            is_leader[p] = true;
        }
    }

    let mut ranked = Vec::<((usize, usize), f64)>::new();
    for &mv in &candidates {
        let pressure = estimate_move_pressure(game, state, &conflict_map, mv, phase);
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
        let pressure_weight_now =
            pressure_weight_by_phase(phase, pressure_phase_split, pressure_weight_early, pressure_weight_late);
        ranked.push((mv, local - pressure_weight_now * pressure));
    }
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let candidate_cap = candidate_cap.min(ranked.len());
    let fast_phase = phase_now > phase_split;
    let plan_len = if fast_phase { plan_len_fast } else { plan_len_slow };
    let beam_width = if fast_phase { beam_width_fast } else { beam_width_slow };

    let mut best_mv = ranked[0].0;
    let mut best_score = f64::NEG_INFINITY;
    for &(mv, local) in ranked.iter().take(candidate_cap) {
        let mut best_target_score = f64::NEG_INFINITY;
        for &target in targets.iter().take(target_eval) {
        let route_score = beam_route_score(
            game,
            state,
            models,
            mv,
            target,
            plan_len,
            beam_width,
            branch_width,
            pressure_phase_split,
            pressure_route_weight_early,
            pressure_route_weight_late,
            local_coeff,
            route_coeff,
        );
            if route_score > best_target_score {
                best_target_score = route_score;
            }
        }
        let total = best_target_score + local_weight * local;
        if total > best_score {
            best_score = total;
            best_mv = mv;
        }
    }
    best_mv
}
