use std::collections::HashMap;

use crate::{
    x01_beam_pessimistic, x04_macro_route, x06_expert_switch_hybrid, x08_pressure_frontier,
    x18_robust_minmax_guard, x19_frontier_recovery_sweep, AiModel, Game, State,
};

fn frontier_signal(game: &Game, state: &State, mv: (usize, usize)) -> f64 {
    let (x, y) = mv;
    let mut s = 0.0_f64;
    let owner = state.owner[x][y];
    let value = game.v[x][y] as f64;
    let level = state.level[x][y] as f64;
    s += 0.35 * value;
    if owner == -1 {
        s += 0.45 * value;
    } else if owner == 0 {
        s += 0.20 * value / (level + 1.0);
        if level >= game.u as f64 {
            s -= 0.12 * value;
        }
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
        let v = game.v[ux][uy] as f64;
        match state.owner[ux][uy] {
            -1 => s += 0.04 * v,
            0 => {
                if state.level[ux][uy] < game.u {
                    s += 0.03 * v;
                }
            }
            _ => {
                if state.level[ux][uy] == 1 {
                    s += 0.06 * v;
                }
            }
        }
    }
    s
}

fn weighted_votes(
    game: &Game,
    state: &State,
    models: &[AiModel],
    uncertainty: f64,
    phase: f64,
    conf: f64,
    gap: f64,
) -> Vec<((usize, usize), f64)> {
    let mut w01 = 0.16 + 0.12 * (1.0 - phase).min(1.0);
    let mut w04 = 0.34 + 0.18 * (1.0 - uncertainty);
    let mut w06 = 0.24 + 0.14 * phase;
    let mut w08 = 0.12 + 0.16 * (1.0 - conf);
    let mut w19 = 0.06 + 0.08 * (gap / 100_000.0).min(1.0);
    let mut w18 = 0.08 + 0.04 * conf;

    if conf > 1.2 {
        w18 += 0.12;
        w04 *= 0.90;
        w19 += 0.04;
    }
    if uncertainty >= 0.30 {
        w18 += 0.08;
        w06 += 0.04;
        w08 -= 0.03;
    }
    if phase >= 0.65 {
        w18 += 0.06;
        w06 += 0.05;
        w04 *= 0.92;
    }
    if game.m == 5 {
        w06 -= 0.02;
        w01 += 0.02;
    }

    let total = (w01 + w04 + w06 + w08 + w19 + w18).max(1e-12);
    let mut votes = vec![
        (x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models), w01 / total),
        (x04_macro_route::choose_move_x04_macro_route(game, state, models), w04 / total),
        (
            x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models),
            w06 / total,
        ),
        (
            x08_pressure_frontier::choose_move_x08_pressure_frontier(game, state, models),
            w08 / total,
        ),
        (
            x19_frontier_recovery_sweep::choose_move_x19_frontier_recovery_sweep(game, state, models),
            w19 / total,
        ),
        (x18_robust_minmax_guard::choose_move_x18_robust_minmax_guard(game, state, models), w18 / total),
    ];
    if uncertainty < 0.22 && game.m <= 5 {
        let idx = votes.len();
        votes[idx - 1].1 += 0.02;
        let sum = votes.iter().map(|x| x.1).sum::<f64>();
        for x in votes.iter_mut() {
            x.1 /= sum;
        }
    }
    votes
}

fn two_step_probe(
    game: &Game,
    state: &State,
    models: &[AiModel],
    first_mv: (usize, usize),
    top2: &[((usize, usize), (usize, usize), f64)],
) -> f64 {
    let mut cur = state.clone();
    let scores = crate::calc_scores(game, state);
    let mut leaders = vec![false; game.m];
    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    for p in 1..game.m {
        if scores[p] == max_ai {
            leaders[p] = true;
        }
    }
    let mut total = 0.0_f64;
    for step in 0..2 {
        let mut moves = Vec::with_capacity(game.m);
        if step == 0 {
            moves.push(first_mv);
        } else {
            let cands = crate::get_candidates(game, &cur, 0);
            if cands.is_empty() {
                break;
            }
            let conflict = crate::estimate_conflict_map(game, &cur, models);
            let s0 = scores[0] as f64;
            let mut next_mv = cands[0];
            let mut best = f64::NEG_INFINITY;
            for &mv in &cands {
                let mut v = crate::evaluate_local_move(
                    game,
                    &cur,
                    mv,
                    &scores,
                    s0,
                    max_ai,
                    cur.turn as f64 / game.t as f64,
                    &conflict,
                    cur.pos[0],
                    &leaders,
                );
                v -= 0.18 * conflict[mv.0][mv.1];
                if v > best {
                    best = v;
                    next_mv = mv;
                }
            }
            moves.push(next_mv);
            if step > 0 {
                moves.extend(top2.iter().map(|x| x.0).take(game.m - 1));
            }
        }
        cur = crate::simulate_turn(game, &cur, &moves);
        total += (0.90_f64).powi(step as i32) * crate::strategic_score(game, &cur);
    }
    total
}

pub(super) fn choose_move_x26_reactive_frontier_pressure(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
    if !(4..=6).contains(&game.m) {
        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
    }

    let top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
    let uncertainty = crate::uncertainty_risk(&top2);
    let scores = crate::calc_scores(game, state);
    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let phase = state.turn as f64 / game.t as f64;
    let gap = (max_ai as f64 - scores[0] as f64).abs().min(100_000.0);
    let conflict = crate::estimate_conflict_map(game, state, models);
    let conf = conflict.iter().flatten().sum::<f64>() / (game.n as f64 * game.n as f64);

    let cands = crate::get_candidates(game, state, 0);
    if cands.is_empty() {
        return state.pos[0];
    }
    if cands.len() <= 3 {
        return cands[0];
    }

    let mut votes: HashMap<(usize, usize), f64> = HashMap::new();
    for (mv, w) in weighted_votes(game, state, models, uncertainty, phase, conf, gap) {
        *votes.entry(mv).or_insert(0.0) += w;
    }

    let mut leaders = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai {
            leaders[p] = true;
        }
    }
    let s0 = scores[0] as f64;
    for &mv in cands.iter().take(20) {
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
            &leaders,
        );
        let frontier = frontier_signal(game, state, mv);
        let entry = votes.entry(mv).or_insert(0.0);
        *entry += 0.01 * local + 0.000_02 * frontier;
    }

    let mut pool: Vec<_> = votes.into_iter().collect();
    if pool.is_empty() {
        return x04_macro_route::choose_move_x04_macro_route(game, state, models);
    }
    pool.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    pool.truncate(pool.len().max(8).min(16));

    let mut best_mv = pool[0].0;
    let mut best = f64::NEG_INFINITY;
    for &(mv, w) in pool.iter().take(12) {
        let mut primary = Vec::with_capacity(game.m);
        primary.push(mv);
        primary.extend(top2.iter().map(|x| x.0).take(game.m - 1));
        let mut secondary = Vec::with_capacity(game.m);
        secondary.push(mv);
        secondary.extend(crate::build_secondary_ai_moves(&scores, &top2, 1).into_iter().take(game.m - 1));
        let ns1 = crate::simulate_turn(game, state, &primary);
        let ns2 = crate::simulate_turn(game, state, &secondary);
        let core = 0.53 * crate::strategic_score(game, &ns1) + 0.24 * crate::strategic_score(game, &ns2);
        let probe = 0.18 * two_step_probe(game, state, models, mv, &top2);
        let pressure = frontier_signal(game, state, mv) / 1_000.0;
        let risk = 0.16 * conflict[mv.0][mv.1];
        let score = core + probe + pressure + 1.8 * w - risk;
        if score > best {
            best = score;
            best_mv = mv;
        }
    }

    if best.is_finite() {
        best_mv
    } else {
        x18_robust_minmax_guard::choose_move_x18_robust_minmax_guard(game, state, models)
    }
}
