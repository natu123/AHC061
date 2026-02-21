use std::time::{Duration, Instant};

use crate::{
    build_ai_candidates_and_probs, calc_scores, estimate_conflict_map, evaluate_local_move,
    get_candidates, sample_index, simulate_turn, strategic_score, AiModel, FastRng, Game, State,
};

struct SearchParams {
    max_depth: usize,
    root_top_k: usize,
    root_samples: usize,
    deeper_top_k_base: usize,
    deeper_samples_base: usize,
}

fn get_search_params(m: usize) -> SearchParams {
    match m {
        2 => SearchParams {
            max_depth: 5,
            root_top_k: 7,
            root_samples: 6,
            deeper_top_k_base: 4,
            deeper_samples_base: 4,
        },
        3 | 4 => SearchParams {
            max_depth: 4,
            root_top_k: 6,
            root_samples: 5,
            deeper_top_k_base: 3,
            deeper_samples_base: 3,
        },
        5 | 6 => SearchParams {
            max_depth: 3,
            root_top_k: 5,
            root_samples: 5,
            deeper_top_k_base: 3,
            deeper_samples_base: 3,
        },
        _ => SearchParams {
            max_depth: 2,
            root_top_k: 5,
            root_samples: 6,
            deeper_top_k_base: 3,
            deeper_samples_base: 3,
        },
    }
}

fn top_k_at_depth(params: &SearchParams, depth: usize) -> usize {
    if depth == 0 {
        params.root_top_k
    } else {
        params
            .deeper_top_k_base
            .saturating_sub(depth.saturating_sub(1))
            .max(1)
    }
}

fn samples_at_depth(params: &SearchParams, depth: usize) -> usize {
    if depth == 0 {
        params.root_samples
    } else {
        params
            .deeper_samples_base
            .saturating_sub(depth.saturating_sub(1))
            .max(1)
    }
}

/// Simplified candidate scoring for deeper search nodes (avoids expensive conflict_map).
fn quick_rank_score(game: &Game, state: &State, cand: (usize, usize)) -> f64 {
    let (x, y) = cand;
    let owner = state.owner[x][y];
    let level = state.level[x][y];
    let v = game.v[x][y] as f64;
    let phase = state.turn as f64 / game.t as f64;

    if owner == -1 {
        v * (1.0 + (1.0 - phase) * 0.5)
    } else if owner == 0 {
        if level < game.u {
            v * 0.85
        } else {
            v * (-0.05)
        }
    } else if level == 1 {
        v * (1.25 + 0.4 * phase)
    } else {
        v * (0.3 + 0.15 * phase) / level as f64
    }
}

fn expectimax_inner(
    game: &Game,
    state: &State,
    ai_cp: &[(Vec<(usize, usize)>, Vec<f64>)],
    depth: usize,
    max_depth: usize,
    params: &SearchParams,
    rng: &mut FastRng,
    deadline: Instant,
) -> f64 {
    if depth >= max_depth || Instant::now() >= deadline {
        return strategic_score(game, state);
    }

    let my_cands = get_candidates(game, state, 0);
    if my_cands.is_empty() {
        return strategic_score(game, state);
    }

    let top_k = top_k_at_depth(params, depth);
    let mut ranked: Vec<((usize, usize), f64)> = my_cands
        .iter()
        .map(|&c| (c, quick_rank_score(game, state, c)))
        .collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    ranked.truncate(top_k);

    let num_samples = samples_at_depth(params, depth);
    let mut best_val = f64::NEG_INFINITY;

    for &(mv, _) in &ranked {
        if Instant::now() >= deadline {
            break;
        }
        let mut total = 0.0;
        for _ in 0..num_samples {
            let mut moves = Vec::with_capacity(game.m);
            moves.push(mv);
            for (cands, probs) in ai_cp {
                let idx = sample_index(probs, rng);
                moves.push(cands[idx]);
            }
            let mut next = simulate_turn(game, state, &moves);
            next.turn = state.turn + 1;
            let val =
                expectimax_inner(game, &next, ai_cp, depth + 1, max_depth, params, rng, deadline);
            total += val;
        }
        let avg = total / num_samples as f64;
        if avg > best_val {
            best_val = avg;
        }
    }

    if best_val == f64::NEG_INFINITY {
        strategic_score(game, state)
    } else {
        best_val
    }
}

pub(super) fn choose_move_x211_deep_mc_expectimax(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
    let time_limit_ms: u64 = std::env::var("AHC_TIME_LIMIT_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(2000);
    let per_turn = time_limit_ms / game.t as u64;
    let budget_ms = (per_turn * 85 / 100).max(5);
    let deadline = Instant::now() + Duration::from_millis(budget_ms);
    let fast_mode = per_turn <= 25; // online: ~20ms/turn

    let mut rng = FastRng::new(state.turn as u64 * 54321 + 13);

    let candidates = get_candidates(game, state, 0);
    if candidates.len() <= 1 {
        return candidates.first().copied().unwrap_or(state.pos[0]);
    }

    let scores = calc_scores(game, state);
    let s0 = scores[0] as f64;
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let phase = state.turn as f64 / game.t as f64;
    let cur = state.pos[0];
    let mut is_leader = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai_i64 {
            is_leader[p] = true;
        }
    }

    if fast_mode {
        // Fast mode: skip conflict_map, use quick_rank_score, 1-step MC with minimal samples
        let top_k = 3usize.min(candidates.len());
        let mut ranked: Vec<((usize, usize), f64)> = candidates
            .iter()
            .map(|&c| (c, quick_rank_score(game, state, c)))
            .collect();
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        ranked.truncate(top_k);

        let ai_cp = build_ai_candidates_and_probs(game, state, models);
        let num_samples = 2usize;

        let mut best_mv = ranked[0].0;
        let mut best_val = f64::NEG_INFINITY;

        for &(mv, _) in &ranked {
            if Instant::now() >= deadline {
                break;
            }
            let mut total = 0.0;
            for _ in 0..num_samples {
                let mut moves = Vec::with_capacity(game.m);
                moves.push(mv);
                for (cands, probs) in &ai_cp {
                    let idx = sample_index(probs, &mut rng);
                    moves.push(cands[idx]);
                }
                let next = simulate_turn(game, state, &moves);
                total += strategic_score(game, &next);
            }
            let avg = total / num_samples as f64;
            if avg > best_val {
                best_val = avg;
                best_mv = mv;
            }
        }
        return best_mv;
    }

    // Full mode: conflict_map + evaluate_local_move + iterative deepening
    let params = get_search_params(game.m);
    let conflict_map = estimate_conflict_map(game, state, models);

    let mut ranked: Vec<((usize, usize), f64)> = candidates
        .iter()
        .map(|&c| {
            (
                c,
                evaluate_local_move(
                    game,
                    state,
                    c,
                    &scores,
                    s0,
                    max_ai_i64,
                    phase,
                    &conflict_map,
                    cur,
                    &is_leader,
                ),
            )
        })
        .collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    ranked.truncate(params.root_top_k);

    let ai_cp = build_ai_candidates_and_probs(game, state, models);

    // Iterative deepening
    let mut best_mv = ranked[0].0;

    for max_depth in 1..=params.max_depth {
        if Instant::now() >= deadline {
            break;
        }

        let mut depth_best_mv = ranked[0].0;
        let mut depth_best_val = f64::NEG_INFINITY;

        for &(mv, local_score) in &ranked {
            if Instant::now() >= deadline {
                break;
            }

            let mut total = 0.0;
            for _ in 0..params.root_samples {
                let mut moves = Vec::with_capacity(game.m);
                moves.push(mv);
                for (cands, probs) in &ai_cp {
                    let idx = sample_index(probs, &mut rng);
                    moves.push(cands[idx]);
                }
                let mut next = simulate_turn(game, state, &moves);
                next.turn = state.turn + 1;
                let val = expectimax_inner(
                    game, &next, &ai_cp, 1, max_depth, &params, &mut rng, deadline,
                );
                total += val;
            }
            let avg = total / params.root_samples as f64;
            let combined = avg + 0.05 * local_score;

            if combined > depth_best_val {
                depth_best_val = combined;
                depth_best_mv = mv;
            }
        }

        // Only update if this depth search completed
        if Instant::now() < deadline {
            best_mv = depth_best_mv;
        }
    }

    best_mv
}
