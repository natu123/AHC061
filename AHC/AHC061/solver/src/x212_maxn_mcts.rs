use std::time::{Duration, Instant};

use crate::{
    build_ai_candidates_and_probs, calc_scores, estimate_conflict_map, evaluate_local_move,
    get_candidates, sample_index, simulate_turn, strategic_score, AiModel, FastRng, Game, State,
};

struct MctsParams {
    ucb1_c: f64,
    rollout_depth: usize,
    candidate_k: usize,
}

fn get_mcts_params(m: usize) -> MctsParams {
    match m {
        2 => MctsParams {
            ucb1_c: 2.0,
            rollout_depth: 6,
            candidate_k: 8,
        },
        3 | 4 => MctsParams {
            ucb1_c: 1.5,
            rollout_depth: 4,
            candidate_k: 6,
        },
        5 | 6 => MctsParams {
            ucb1_c: 1.2,
            rollout_depth: 3,
            candidate_k: 5,
        },
        _ => MctsParams {
            ucb1_c: 1.0,
            rollout_depth: 2,
            candidate_k: 4,
        },
    }
}

struct MctsChild {
    mv: (usize, usize),
    visits: u32,
    total_score: f64,
}

/// Greedy rollout using evaluate_local_move (with zero conflict map for speed).
fn greedy_rollout(
    game: &Game,
    state: &State,
    ai_cp: &[(Vec<(usize, usize)>, Vec<f64>)],
    rng: &mut FastRng,
    depth: usize,
) -> f64 {
    let zero_conflict = vec![vec![0.0_f64; game.n]; game.n];
    let mut current = state.clone();

    for _ in 0..depth {
        let my_cands = get_candidates(game, &current, 0);
        if my_cands.is_empty() {
            break;
        }

        let scores = calc_scores(game, &current);
        let s0 = scores[0] as f64;
        let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
        let phase = current.turn as f64 / game.t as f64;
        let cur = current.pos[0];
        let mut is_leader = vec![false; game.m];
        for p in 1..game.m {
            if scores[p] == max_ai_i64 {
                is_leader[p] = true;
            }
        }

        // Pick best move by evaluate_local_move (zero conflict for speed)
        let mut best_mv = my_cands[0];
        let mut best_score = f64::NEG_INFINITY;
        for &c in &my_cands {
            let score = evaluate_local_move(
                game,
                &current,
                c,
                &scores,
                s0,
                max_ai_i64,
                phase,
                &zero_conflict,
                cur,
                &is_leader,
            );
            if score > best_score {
                best_score = score;
                best_mv = c;
            }
        }

        // Sample AI moves from pre-computed blended probabilities
        let mut moves = Vec::with_capacity(game.m);
        moves.push(best_mv);
        for (cands, probs) in ai_cp {
            let idx = sample_index(probs, rng);
            moves.push(cands[idx]);
        }
        current = simulate_turn(game, &current, &moves);
        current.turn += 1;
    }

    strategic_score(game, &current)
}

pub(super) fn choose_move_x212_maxn_mcts(
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

    let params = get_mcts_params(game.m);
    let mut rng = FastRng::new(state.turn as u64 * 98765 + 37);

    let candidates = get_candidates(game, state, 0);
    if candidates.len() <= 1 {
        return candidates.first().copied().unwrap_or(state.pos[0]);
    }

    // Rank candidates using full evaluate_local_move at root
    let scores = calc_scores(game, state);
    let s0 = scores[0] as f64;
    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
    let phase = state.turn as f64 / game.t as f64;
    let conflict_map = estimate_conflict_map(game, state, models);
    let cur = state.pos[0];
    let mut is_leader = vec![false; game.m];
    for p in 1..game.m {
        if scores[p] == max_ai_i64 {
            is_leader[p] = true;
        }
    }

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
    ranked.truncate(params.candidate_k);

    let ai_cp = build_ai_candidates_and_probs(game, state, models);

    // Initialize MCTS children
    let mut children: Vec<MctsChild> = ranked
        .iter()
        .map(|&(mv, _)| MctsChild {
            mv,
            visits: 0,
            total_score: 0.0,
        })
        .collect();

    let c = params.ucb1_c;

    // MCTS main loop
    while Instant::now() < deadline {
        // UCB1 selection
        let total_visits: u32 = children.iter().map(|ch| ch.visits).sum();
        let ln_total = (total_visits.max(1) as f64).ln();

        let selected = children
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                let ua = if a.visits == 0 {
                    f64::INFINITY
                } else {
                    a.total_score / a.visits as f64 + c * (ln_total / a.visits as f64).sqrt()
                };
                let ub = if b.visits == 0 {
                    f64::INFINITY
                } else {
                    b.total_score / b.visits as f64 + c * (ln_total / b.visits as f64).sqrt()
                };
                ua.partial_cmp(&ub).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
            .unwrap();

        // Simulate one turn with selected move + sampled AI moves
        let mv = children[selected].mv;
        let mut moves = Vec::with_capacity(game.m);
        moves.push(mv);
        for (cands, probs) in &ai_cp {
            let idx = sample_index(probs, &mut rng);
            moves.push(cands[idx]);
        }
        let mut next = simulate_turn(game, state, &moves);
        next.turn = state.turn + 1;

        // Rollout from the resulting state
        let val = greedy_rollout(game, &next, &ai_cp, &mut rng, params.rollout_depth);

        // Backpropagate
        children[selected].visits += 1;
        children[selected].total_score += val;
    }

    // Select best child by visit count
    children
        .iter()
        .max_by_key(|ch| ch.visits)
        .map(|ch| ch.mv)
        .unwrap_or(state.pos[0])
}
