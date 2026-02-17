use std::collections::HashMap;
use std::env;

use crate::{
    x01_beam_pessimistic, x04_macro_route, x06_expert_switch_hybrid, x10_phase_adaptive_mix,
    x11_contest_frontier_recovery, x13_frontier_consensus, x14_adaptive_risk_lane,
    x15_band_adaptive_route, x19_frontier_recovery_sweep, x26_reactive_frontier_pressure, AiModel,
    Game, State,
};

fn env_f64(key: &str, default: f64, min: f64, max: f64) -> f64 {
    env::var(key)
        .ok()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(default)
        .clamp(min, max)
}

fn env_u64(key: &str, default: u64, min: u64, max: u64) -> usize {
    env::var(key)
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(default)
        .clamp(min, max) as usize
}

fn frontier_opportunity(game: &Game, state: &State, mv: (usize, usize)) -> f64 {
    let (x, y) = mv;
    let owner = state.owner[x][y];
    let level = state.level[x][y];
    let value = game.v[x][y] as f64;

    let mut score = 0.0_f64;
    if owner == -1 {
        score += 1.20 * value;
    } else if owner > 0 && level == 1 {
        score += 0.95 * value;
    } else if owner == 0 && level < game.u {
        score += 0.65 * value * (game.u - level) as f64 / game.u as f64;
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
        match state.owner[ux][uy] {
            -1 => score += 0.08 * ov,
            0 => {
                if state.level[ux][uy] < game.u {
                    score += 0.05 * ov;
                }
            }
            _ => {
                if state.level[ux][uy] == 1 {
                    score += 0.10 * ov;
                }
            }
        }
    }
    score
}

fn advisor_votes(
    game: &Game,
    state: &State,
    models: &[AiModel],
    phase: f64,
    uncertainty: f64,
    gap: f64,
) -> Vec<((usize, usize), f64)> {
    let w_x01 = env_f64("AHC_X64_W_X01", 0.75, 0.0, 3.0) + 0.10 * (1.0 - phase);
    let w_x04 = env_f64("AHC_X64_W_X04", 1.10, 0.0, 3.0) + 0.15 * (1.0 - uncertainty);
    let w_x06 = env_f64("AHC_X64_W_X06", 1.00, 0.0, 3.0) + 0.10 * phase;
    let w_x10 = env_f64("AHC_X64_W_X10", 0.55, 0.0, 3.0) + 0.08 * uncertainty;
    let w_x11 = env_f64("AHC_X64_W_X11", 0.70, 0.0, 3.0) + 0.10 * gap;
    let w_x13 = env_f64("AHC_X64_W_X13", 0.85, 0.0, 3.0) + 0.05 * (1.0 - uncertainty);
    let w_x14 = env_f64("AHC_X64_W_X14", 0.75, 0.0, 3.0) + 0.10 * uncertainty;
    let w_x19 = env_f64("AHC_X64_W_X19", 0.80, 0.0, 3.0) + 0.08 * gap;
    let w_x26 = env_f64("AHC_X64_W_X26", 0.90, 0.0, 3.0) + 0.10 * uncertainty;
    let w_x15 = env_f64("AHC_X64_W_X15", 0.55, 0.0, 3.0) + 0.05 * (1.0 - phase);

    let mut votes = vec![
        (
            x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models),
            w_x01,
        ),
        (
            x04_macro_route::choose_move_x04_macro_route(game, state, models),
            w_x04,
        ),
        (
            x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models),
            w_x06,
        ),
        (
            x10_phase_adaptive_mix::choose_move_x10_phase_adaptive_mix(game, state, models),
            w_x10,
        ),
        (
            x11_contest_frontier_recovery::choose_move_x11_contest_frontier_recovery(
                game, state, models,
            ),
            w_x11,
        ),
        (
            x13_frontier_consensus::choose_move_x13_frontier_consensus(game, state, models),
            w_x13,
        ),
        (
            x14_adaptive_risk_lane::choose_move_x14_adaptive_risk_lane(game, state, models),
            w_x14,
        ),
        (
            x19_frontier_recovery_sweep::choose_move_x19_frontier_recovery_sweep(
                game, state, models,
            ),
            w_x19,
        ),
        (
            x26_reactive_frontier_pressure::choose_move_x26_reactive_frontier_pressure(
                game, state, models,
            ),
            w_x26,
        ),
    ];

    if (4..=6).contains(&game.m) {
        votes.push((
            x15_band_adaptive_route::choose_move_x15_band_adaptive_route(game, state, models),
            w_x15,
        ));
    }

    let sum = votes.iter().map(|x| x.1).sum::<f64>().max(1e-9);
    for v in &mut votes {
        v.1 /= sum;
    }
    votes
}

pub(super) fn choose_move_x64_portfolio_mixer(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
    let candidates = crate::get_candidates(game, state, 0);
    if candidates.is_empty() {
        return state.pos[0];
    }
    if candidates.len() == 1 {
        return candidates[0];
    }

    let phase = state.turn as f64 / game.t as f64;
    let top2 = crate::choose_predicted_ai_top2_moves(game, state, models);
    let uncertainty = crate::uncertainty_risk(&top2);
    let scores = crate::calc_scores(game, state);
    let s0 = scores[0] as f64;
    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let gap = ((max_ai as f64 - s0).max(0.0) / s0.max(1.0)).clamp(0.0, 1.5);
    let conflict = crate::estimate_conflict_map(game, state, models);

    let mut leaders = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai {
            leaders[p] = true;
        }
    }

    let advisor_votes = advisor_votes(game, state, models, phase, uncertainty, gap);
    let mut vote_map: HashMap<(usize, usize), f64> = HashMap::new();
    for (mv, w) in advisor_votes {
        *vote_map.entry(mv).or_insert(0.0) += w;
    }

    let mut local_rank = Vec::<((usize, usize), f64)>::new();
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
            &leaders,
        );
        local_rank.push((mv, local));
    }
    local_rank.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let mut pool = Vec::<(usize, usize)>::new();
    for (mv, _) in &local_rank {
        if !pool.contains(mv) {
            pool.push(*mv);
        }
        if pool.len() >= 8 {
            break;
        }
    }
    let mut voted_moves: Vec<((usize, usize), f64)> = vote_map.iter().map(|(k, v)| (*k, *v)).collect();
    voted_moves.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    for (mv, _) in voted_moves.into_iter().take(8) {
        if !pool.contains(&mv) {
            pool.push(mv);
        }
    }
    if pool.is_empty() {
        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
    }

    let sec_cap = env_u64("AHC_X64_SECONDARY_CAP", if game.m >= 6 { 2 } else { 1 }, 1, 3);
    let secondary = crate::build_secondary_ai_moves(&scores, &top2, sec_cap);
    let primary: Vec<(usize, usize)> = top2.iter().map(|x| x.0).collect();

    let aggro_gap = env_f64("AHC_X64_AGGRO_GAP", 0.20, 0.0, 1.0);
    let aggro = if gap >= aggro_gap { 1.0 } else { 0.0 };
    let w_primary = env_f64("AHC_X64_W_PRIMARY", 0.56, 0.20, 0.90) + 0.08 * aggro;
    let w_secondary = env_f64("AHC_X64_W_SECONDARY", 0.34, 0.05, 0.70) - 0.05 * aggro;
    let w_local = env_f64("AHC_X64_W_LOCAL", 0.12, 0.00, 0.50);
    let w_vote = env_f64("AHC_X64_W_VOTE", 22.0, 0.0, 60.0);
    let w_frontier = env_f64("AHC_X64_W_FRONTIER", 0.0016, 0.0, 0.01);
    let w_risk = env_f64("AHC_X64_W_RISK", 0.82, 0.0, 3.0) - 0.22 * aggro;
    let w_gap_bonus = env_f64("AHC_X64_W_GAP_BONUS", 14.0, 0.0, 50.0);
    let w_uncertainty = env_f64("AHC_X64_W_UNCERTAINTY", 0.06, 0.0, 0.40);

    let mut best_mv = pool[0];
    let mut best_val = f64::NEG_INFINITY;

    for &mv in &pool {
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
        let mut moves_primary = Vec::with_capacity(game.m);
        moves_primary.push(mv);
        moves_primary.extend_from_slice(&primary);
        let mut moves_secondary = Vec::with_capacity(game.m);
        moves_secondary.push(mv);
        moves_secondary.extend_from_slice(&secondary);

        let ns_primary = crate::simulate_turn(game, state, &moves_primary);
        let ns_secondary = crate::simulate_turn(game, state, &moves_secondary);
        let s_primary = crate::strategic_score(game, &ns_primary);
        let s_secondary = crate::strategic_score(game, &ns_secondary);

        let vote = *vote_map.get(&mv).unwrap_or(&0.0);
        let frontier = frontier_opportunity(game, state, mv);
        let risk = conflict[mv.0][mv.1] * game.v[mv.0][mv.1] as f64;
        let gap_bonus = if scores[0] < max_ai { gap * w_gap_bonus } else { 0.0 };

        let total = w_primary * s_primary
            + w_secondary * s_secondary
            + w_local * local
            + w_vote * vote
            + w_frontier * frontier
            + gap_bonus
            + w_uncertainty * uncertainty * local
            - w_risk * risk;

        if total > best_val {
            best_val = total;
            best_mv = mv;
        }
    }

    if best_val.is_finite() {
        best_mv
    } else {
        x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models)
    }
}
