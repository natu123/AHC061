use crate::{AiModel, FastRng, Game, State};

pub(super) fn choose_move_monte_carlo(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
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

    let mut ranked: Vec<((usize, usize), f64)> = Vec::with_capacity(candidates.len());
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

    let candidate_cap = if ranked.len() >= 24 {
        14
    } else if ranked.len() >= 14 {
        10
    } else {
        ranked.len()
    };
    let sample_count = if game.m >= 7 {
        10
    } else if game.m >= 5 {
        8
    } else {
        6
    };

    let ai_options = crate::build_ai_candidates_and_probs(game, state, models);
    let seed = ((state.turn as u64 + 1) * 0x9e37_79b9_7f4a_7c15)
        ^ (scores[0] as u64)
        ^ ((game.m as u64) << 32)
        ^ ((game.u as u64) << 48);
    let mut rng = FastRng::new(seed);

    let mut best_mv = ranked[0].0;
    let mut best_val = f64::NEG_INFINITY;
    for &(mv, local) in ranked.iter().take(candidate_cap) {
        let mut acc = 0.0_f64;
        let mut acc2 = 0.0_f64;
        for _ in 0..sample_count {
            let mut sampled = Vec::with_capacity(game.m);
            sampled.push(mv);
            for (cands, probs) in &ai_options {
                let idx = crate::sample_index(probs, &mut rng);
                sampled.push(cands[idx]);
            }
            let next_state = crate::simulate_turn(game, state, &sampled);
            let v = crate::strategic_score(game, &next_state);
            acc += v;
            acc2 += v * v;
        }
        let mean = acc / sample_count as f64;
        let var = (acc2 / sample_count as f64 - mean * mean).max(0.0);
        let std = var.sqrt();
        let risk_w = if game.m >= 6 { 0.40 } else { 0.25 };
        let total = mean - risk_w * std + 0.09 * local;
        if total > best_val {
            best_val = total;
            best_mv = mv;
        }
    }
    best_mv
}
