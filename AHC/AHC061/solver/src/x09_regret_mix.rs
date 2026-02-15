use std::collections::HashSet;

use crate::{
    x02_monte_carlo, x04_macro_route, x06_expert_switch_hybrid, AiModel, FastRng, Game, State,
};

fn collect_candidate_moves(game: &Game, state: &State, models: &[AiModel]) -> Vec<(usize, usize)> {
    let mut ordered = Vec::<(usize, usize)>::new();
    ordered.push(x04_macro_route::choose_move_x04_macro_route(game, state, models));
    ordered.push(x06_expert_switch_hybrid::choose_move_x06_expert_switch(
        game, state, models,
    ));
    if (3..=5).contains(&game.m) {
        ordered.push(x02_monte_carlo::choose_move_monte_carlo(game, state, models));
    }

    let cands = crate::get_candidates(game, state, 0);
    if !cands.is_empty() {
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
                cur,
                &is_leader,
            );
            ranked.push((mv, local));
        }
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        for (mv, _) in ranked.into_iter().take(2) {
            ordered.push(mv);
        }
    }

    let mut uniq = HashSet::<(usize, usize)>::new();
    let mut out = Vec::<(usize, usize)>::new();
    for mv in ordered {
        if uniq.insert(mv) {
            out.push(mv);
        }
    }
    out
}

pub(super) fn choose_move_x09_regret_mix(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
    let candidate_moves = collect_candidate_moves(game, state, models);
    if candidate_moves.is_empty() {
        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models);
    }
    if candidate_moves.len() == 1 {
        return candidate_moves[0];
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

    let mut local_bonus = Vec::with_capacity(candidate_moves.len());
    for &mv in &candidate_moves {
        local_bonus.push(crate::evaluate_local_move(
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
        ));
    }

    let ai_options = crate::build_ai_candidates_and_probs(game, state, models);
    let sample_count = if game.m >= 7 {
        6
    } else if game.m >= 5 {
        8
    } else {
        10
    };
    let seed = ((state.turn as u64 + 1) * 0x9e37_79b9_7f4a_7c15)
        ^ (scores[0] as u64)
        ^ ((game.m as u64) << 32)
        ^ ((game.u as u64) << 48)
        ^ 0x7d61_91b3_2f6c_5d09;
    let mut rng = FastRng::new(seed);

    let mut acc = vec![0.0_f64; candidate_moves.len()];
    let mut acc2 = vec![0.0_f64; candidate_moves.len()];
    for _ in 0..sample_count {
        let mut sampled_ai = Vec::with_capacity(game.m.saturating_sub(1));
        for (cands, probs) in &ai_options {
            let idx = crate::sample_index(probs, &mut rng);
            sampled_ai.push(cands[idx]);
        }

        for (i, &mv) in candidate_moves.iter().enumerate() {
            let mut full_moves = Vec::with_capacity(game.m);
            full_moves.push(mv);
            full_moves.extend_from_slice(&sampled_ai);
            let next_state = crate::simulate_turn(game, state, &full_moves);
            let v = crate::strategic_score(game, &next_state);
            acc[i] += v;
            acc2[i] += v * v;
        }
    }

    let risk_w = if game.m >= 6 { 0.35 } else { 0.26 };
    let mut best_idx = 0;
    let mut best_val = f64::NEG_INFINITY;
    for i in 0..candidate_moves.len() {
        let mean = acc[i] / sample_count as f64;
        let var = (acc2[i] / sample_count as f64 - mean * mean).max(0.0);
        let std = var.sqrt();
        let total = mean - risk_w * std + 0.08 * local_bonus[i];
        if total > best_val {
            best_val = total;
            best_idx = i;
        }
    }
    candidate_moves[best_idx]
}
