pub use __cargo_equip::prelude::*;

use std::env;

use ahc061_solver::{run_with_strategy_bayes, StrategyMode};

fn main() {
    // x365: Narrow Initial + Wide Deepening
    //
    // Insight: x361 (narrow beam 3/4) had noisy initial ranking because
    // iterative deepening only re-evaluated top 3 candidates.
    // Wrong top-3 selection => wrong final move.
    //
    // x365: narrow beam for fast initial pass (evaluate ALL 10 candidates),
    // then deepen top 6 candidates (not just 3) with wider beam.
    // This mitigates wrong-candidate-selection from noisy initial ranking.

    unsafe { env::set_var("AHC_TIME_LIMIT_MS", "2700") };

    // Expectimax params (same as x333)
    unsafe { env::set_var("AHC_X332_DEPTH", "3") };
    unsafe { env::set_var("AHC_X332_DEPTH_M2_U3", "4") };
    unsafe { env::set_var("AHC_X332_BRANCH_TOP", "8") };
    unsafe { env::set_var("AHC_X332_BRANCH_DEEP", "4") };

    // Beam search: narrow initial + wide deepening
    unsafe { env::set_var("AHC_X04_BEAM_WIDTH_FAST", "3") };
    unsafe { env::set_var("AHC_X04_BEAM_WIDTH_SLOW", "4") };
    unsafe { env::set_var("AHC_X04_TARGET_EVAL", "5") };
    unsafe { env::set_var("AHC_X04_CANDIDATE_CAP", "10") };
    unsafe { env::set_var("AHC_X04_RE_EVAL_COUNT", "6") }; // was 3 (default)

    // All other beam params same as x333
    unsafe { env::set_var("AHC_X04_ZONE_BONUS", "1.0") };
    unsafe { env::set_var("AHC_ZONE_SIZE", "4") };
    unsafe { env::set_var("AHC_X04_FALLBACK", "u_aware_v4") };
    unsafe { env::set_var("AHC_X04_ALLOWED_M", "3,4") };
    unsafe { env::set_var("AHC_X04_LEADER_TARGET_BONUS", "0.5") };

    unsafe { env::set_var("AHC_X04_PHASE_CUTOFF", "0.92") };
    unsafe { env::set_var("AHC_X04_PHASE_CUTOFF_M4_U1", "0.92") };
    unsafe { env::set_var("AHC_X04_PHASE_CUTOFF_M4_U2", "0.92") };
    unsafe { env::set_var("AHC_X04_PHASE_CUTOFF_M4_U3", "0.92") };
    unsafe { env::set_var("AHC_X04_PHASE_CUTOFF_M4_U4", "0.20") };
    unsafe { env::set_var("AHC_X04_PHASE_CUTOFF_M4_U5", "0.30") };
    unsafe { env::set_var("AHC_X04_PHASE_CUTOFF_M5", "0.25") };

    unsafe { env::set_var("AHC_X04_PHASE_SPLIT", "0.56") };
    unsafe { env::set_var("AHC_X04_TARGET_COUNT", "9") };
    unsafe { env::set_var("AHC_X04_PLAN_LEN_FAST", "6") };
    unsafe { env::set_var("AHC_X04_PLAN_LEN_SLOW", "8") };
    unsafe { env::set_var("AHC_X04_BRANCH_WIDTH", "2") };
    unsafe { env::set_var("AHC_X04_LOCAL_WEIGHT", "0.11") };
    unsafe { env::set_var("AHC_X04_LOCAL_COEFF", "0.09") };
    unsafe { env::set_var("AHC_X04_ROUTE_COEFF", "46.8") };

    unsafe { env::set_var("AHC_X04_TARGET_PRESSURE_WEIGHT", "0.76") };
    unsafe { env::set_var("AHC_X04_PRESSURE_WEIGHT", "0.36") };
    unsafe { env::set_var("AHC_X04_PRESSURE_WEIGHT_EARLY", "0.02") };
    unsafe { env::set_var("AHC_X04_PRESSURE_WEIGHT_LATE", "1.36") };
    unsafe { env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT", "0.08") };
    unsafe { env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_EARLY", "0.24") };
    unsafe { env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_LATE", "0.72") };
    unsafe { env::set_var("AHC_X04_PRESSURE_PHASE_SPLIT", "0.62") };

    // M=3 params (also narrow initial + wide deepen)
    unsafe { env::set_var("AHC_X04_PHASE_CUTOFF_M3", "0.92") };
    unsafe { env::set_var("AHC_X04_TARGET_COUNT_M3", "8") };
    unsafe { env::set_var("AHC_X04_TARGET_EVAL_M3", "4") };
    unsafe { env::set_var("AHC_X04_CANDIDATE_CAP_M3", "8") };
    unsafe { env::set_var("AHC_X04_PLAN_LEN_FAST_M3", "6") };
    unsafe { env::set_var("AHC_X04_PLAN_LEN_SLOW_M3", "8") };
    unsafe { env::set_var("AHC_X04_BEAM_WIDTH_FAST_M3", "3") };
    unsafe { env::set_var("AHC_X04_BEAM_WIDTH_SLOW_M3", "4") };
    unsafe { env::set_var("AHC_X04_BRANCH_WIDTH_M3", "2") };

    unsafe { env::set_var("AHC_X04_ITER_DEEPEN", "1") };

    run_with_strategy_bayes(StrategyMode::AdaptiveSelect);
}

// The following code was expanded by `cargo-equip`.

///  # Bundled libraries
///
///  - `path+file:///C:/Users/kenji/projects/AtCoder/AHC/AHC061-2/solver#ahc061-solver@0.1.0` published in **missing** licensed under **missing** as `crate::__cargo_equip::crates::ahc061_solver`
#[allow(unused)]
mod __cargo_equip {
    pub(crate) mod crates {
        pub mod ahc061_solver {
            use std::collections::{HashMap, VecDeque};
            use std::io::{self, BufRead, BufReader, BufWriter, Write};
            use std::sync::OnceLock;
            use std::time::Instant;

            mod strategy_mode {
                use std::env;

                use crate::__cargo_equip::crates::ahc061_solver::{
                    x04_macro_route, x332_expectimax, AiModel, Game, State,
                };

                #[derive(Clone, Copy, Debug, PartialEq, Eq)]
                pub enum StrategyMode {
                    AdaptiveSelect,
                }

                pub fn strategy_from_env() -> StrategyMode {
                    let _mode = env::var("AHC_STRATEGY").unwrap_or_default();
                    StrategyMode::AdaptiveSelect
                }

                pub(super) fn choose_move(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    mode: StrategyMode,
                ) -> (usize, usize) {
                    match mode {
                        StrategyMode::AdaptiveSelect => {
                            let use_expectimax = (game.m <= 4 && game.u == 3)
                                || (game.m == 4 && game.u == 2)
                                || (game.m == 2 && game.u == 4);
                            if use_expectimax {
                                x332_expectimax::choose_move_x332_expectimax(game, state, models)
                            } else {
                                x04_macro_route::choose_move_x04_macro_route(game, state, models)
                            }
                        }
                    }
                }
            }
            mod x01_beam_pessimistic {
                use crate::__cargo_equip::crates::ahc061_solver::{AiModel, Game, State};

                type ScoreFn = fn(&Game, &State) -> f64;

                fn resolve_score_fn() -> ScoreFn {
                    if std::env::var("AHC_X01_USE_DENIAL_SCORE")
                        .map(|v| v == "1")
                        .unwrap_or(false)
                    {
                        crate::__cargo_equip::crates::ahc061_solver::denial_score
                    } else if std::env::var("AHC_X01_USE_STRATEGIC_SCORE")
                        .map(|v| v == "1")
                        .unwrap_or(false)
                    {
                        crate::__cargo_equip::crates::ahc061_solver::strategic_score
                    } else if std::env::var("AHC_X01_USE_ZONE_SCORE")
                        .map(|v| v == "1")
                        .unwrap_or(false)
                    {
                        crate::__cargo_equip::crates::ahc061_solver::zone_absolute_score
                    } else {
                        crate::__cargo_equip::crates::ahc061_solver::absolute_score
                    }
                }

                fn env_f64(key: &str, default: f64) -> f64 {
                    std::env::var(key)
                        .ok()
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(default)
                }

                fn best_n_step_score(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    score_fn: ScoreFn,
                    depth: usize,
                ) -> f64 {
                    if depth == 0 || state.turn >= game.t {
                        return score_fn(game, state);
                    }
                    let candidates =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if candidates.is_empty() {
                        return score_fn(game, state);
                    }
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let ai_top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let predicted_primary: Vec<(usize, usize)> =
                        ai_top2.iter().map(|x| x.0).collect();
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&ai_top2);
                    let secondary_cap = if game.m >= 7 && uncertainty >= 0.40 {
                        3
                    } else if game.m >= 6 && uncertainty >= 0.28 {
                        2
                    } else {
                        1
                    };
                    let predicted_secondary =
                        crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(
                            &scores,
                            &ai_top2,
                            secondary_cap,
                        );
                    let risk_w = crate::__cargo_equip::crates::ahc061_solver::pessimism_weight(
                        game,
                        uncertainty,
                    );

                    if candidates.len() == 1 {
                        let mut primary = Vec::with_capacity(game.m);
                        primary.push(candidates[0]);
                        primary.extend_from_slice(&predicted_primary);
                        let next = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &primary,
                        );
                        let score_primary =
                            best_n_step_score(game, &next, models, score_fn, depth - 1);

                        let mut secondary = Vec::with_capacity(game.m);
                        secondary.push(candidates[0]);
                        secondary.extend_from_slice(&predicted_secondary);
                        let score_secondary = best_n_step_score(
                            game,
                            &crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                game, state, &secondary,
                            ),
                            models,
                            score_fn,
                            depth - 1,
                        );
                        return (1.0 - risk_w) * score_primary + risk_w * score_secondary;
                    }

                    let s0 = scores[0] as f64;
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let phase = state.turn as f64 / game.t as f64;
                    let conflict_map =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let cur = state.pos[0];

                    let mut is_leader = vec![false; game.m];
                    for p in 1..game.m {
                        if scores[p] == max_ai_i64 {
                            is_leader[p] = true;
                        }
                    }

                    // For deep search (depth>1), narrow the candidate set to top-4
                    let eval_limit = if depth > 1 {
                        4.min(candidates.len())
                    } else {
                        candidates.len()
                    };

                    let mut ranked: Vec<((usize, usize), f64)> =
                        Vec::with_capacity(candidates.len());
                    for &mv in &candidates {
                        let local_score =
                            crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
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
                        ranked.push((mv, local_score));
                    }
                    ranked
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

                    let mut best_val = f64::NEG_INFINITY;
                    for &(mv, local_score) in ranked.iter().take(eval_limit) {
                        let mut primary = Vec::with_capacity(game.m);
                        primary.push(mv);
                        primary.extend_from_slice(&predicted_primary);
                        let next_primary =
                            crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                game, state, &primary,
                            );
                        let score_primary = if depth > 1 {
                            best_n_step_score(game, &next_primary, models, score_fn, depth - 1)
                        } else {
                            score_fn(game, &next_primary)
                        };

                        let mut secondary = Vec::with_capacity(game.m);
                        secondary.push(mv);
                        secondary.extend_from_slice(&predicted_secondary);
                        let score_secondary = if depth > 1 {
                            best_n_step_score(
                                game,
                                &crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                    game, state, &secondary,
                                ),
                                models,
                                score_fn,
                                depth - 1,
                            )
                        } else {
                            score_fn(
                                game,
                                &crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                    game, state, &secondary,
                                ),
                            )
                        };

                        let rollout = (1.0 - risk_w) * score_primary + risk_w * score_secondary;
                        let total = rollout + 0.12 * local_score;
                        if total > best_val {
                            best_val = total;
                        }
                    }
                    best_val
                }

                pub(super) fn choose_move_x01_beam_pessimistic(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    let candidates =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if candidates.len() == 1 {
                        return candidates[0];
                    }

                    let score_fn = resolve_score_fn();
                    let (local_w, future_w) = if game.m == 2 {
                        // M=2: 1v1で予測精度高→future重視。env varで調整可能
                        (
                            env_f64("AHC_X01_M2_LOCAL_WEIGHT", 0.12),
                            env_f64("AHC_X01_M2_FUTURE_WEIGHT", 0.18),
                        )
                    } else {
                        (
                            env_f64("AHC_X01_LOCAL_WEIGHT", 0.12),
                            env_f64("AHC_X01_FUTURE_WEIGHT", 0.18),
                        )
                    };

                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let s0 = scores[0] as f64;
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let phase = state.turn as f64 / game.t as f64;
                    let conflict_map =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let cur = state.pos[0];
                    let ai_top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let predicted_primary: Vec<(usize, usize)> =
                        ai_top2.iter().map(|x| x.0).collect();
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&ai_top2);
                    let secondary_cap = if game.m >= 7 && uncertainty >= 0.40 {
                        3
                    } else if game.m >= 6 && uncertainty >= 0.28 {
                        2
                    } else {
                        1
                    };
                    let predicted_secondary =
                        crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(
                            &scores,
                            &ai_top2,
                            secondary_cap,
                        );
                    let risk_w = crate::__cargo_equip::crates::ahc061_solver::pessimism_weight(
                        game,
                        uncertainty,
                    );

                    let mut is_leader = vec![false; game.m];
                    for p in 1..game.m {
                        if scores[p] == max_ai_i64 {
                            is_leader[p] = true;
                        }
                    }

                    let mut scored: Vec<((usize, usize), f64, State)> =
                        Vec::with_capacity(candidates.len());

                    for &(x, y) in &candidates {
                        let local_score =
                            crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                game,
                                state,
                                (x, y),
                                &scores,
                                s0,
                                max_ai_i64,
                                phase,
                                &conflict_map,
                                cur,
                                &is_leader,
                            );

                        let mut primary = Vec::with_capacity(game.m);
                        primary.push((x, y));
                        primary.extend_from_slice(&predicted_primary);
                        let next_state = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &primary,
                        );
                        let score_primary = score_fn(game, &next_state);

                        let mut secondary = Vec::with_capacity(game.m);
                        secondary.push((x, y));
                        secondary.extend_from_slice(&predicted_secondary);
                        let score_secondary = score_fn(
                            game,
                            &crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                game, state, &secondary,
                            ),
                        );

                        let rollout_score =
                            (1.0 - risk_w) * score_primary + risk_w * score_secondary;
                        let base_total = rollout_score + local_w * local_score;
                        scored.push(((x, y), base_total, next_state));
                    }

                    scored
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    let mut beam_width = if scored.len() >= 32 {
                        8
                    } else if scored.len() >= 16 {
                        6
                    } else {
                        4
                    };
                    if std::env::var("AHC_X01_FULL_BEAM")
                        .map(|v| v == "1")
                        .unwrap_or(false)
                    {
                        beam_width = scored.len();
                    } else if game.m == 5 && phase <= 0.80 && uncertainty >= 0.18 {
                        beam_width = scored.len();
                    } else if game.m == 6 && phase <= 0.72 && uncertainty >= 0.22 {
                        beam_width = (beam_width + 3).min(scored.len());
                    }

                    let future_depth = if let Ok(v) = std::env::var("AHC_X01_DEPTH") {
                        v.parse::<usize>().unwrap_or(1)
                    } else if game.m == 2 {
                        std::env::var("AHC_X01_M2_DEPTH")
                            .ok()
                            .and_then(|v| v.parse::<usize>().ok())
                            .unwrap_or(1)
                    } else {
                        1
                    };

                    let mut best = scored[0].0;
                    let mut best_total = f64::NEG_INFINITY;
                    for (idx, (mv, base_total, next_state)) in scored.iter().enumerate() {
                        if idx >= beam_width {
                            break;
                        }
                        let future = if state.turn + 1 < game.t {
                            best_n_step_score(game, next_state, models, score_fn, future_depth)
                        } else {
                            0.0
                        };
                        let total = *base_total + future_w * future;
                        if total > best_total {
                            best_total = total;
                            best = *mv;
                        }
                    }
                    best
                }
            }
            mod x02_monte_carlo {
                use crate::__cargo_equip::crates::ahc061_solver::{AiModel, FastRng, Game, State};

                pub(super) fn choose_move_monte_carlo(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    let candidates =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if candidates.len() <= 1 {
                        return candidates.first().copied().unwrap_or(state.pos[0]);
                    }

                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let s0 = scores[0] as f64;
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let phase = state.turn as f64 / game.t as f64;
                    let conflict_map =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let cur = state.pos[0];
                    let mut is_leader = vec![false; game.m];
                    for p in 1..game.m {
                        if scores[p] == max_ai_i64 {
                            is_leader[p] = true;
                        }
                    }

                    let mut ranked: Vec<((usize, usize), f64)> =
                        Vec::with_capacity(candidates.len());
                    for &mv in &candidates {
                        let local =
                            crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
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
                    ranked
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

                    let candidate_cap = if ranked.len() >= 24 {
                        14
                    } else if ranked.len() >= 14 {
                        10
                    } else {
                        ranked.len()
                    };
                    let sample_count_default = if game.m >= 7 {
                        10
                    } else if game.m >= 5 {
                        8
                    } else {
                        6
                    };
                    let sample_count: usize = std::env::var("AHC_X02_SAMPLE_COUNT")
                        .ok()
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(sample_count_default)
                        .max(2)
                        .min(50);

                    let ai_options =
                        crate::__cargo_equip::crates::ahc061_solver::build_ai_candidates_and_probs(
                            game, state, models,
                        );
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
                                let idx = crate::__cargo_equip::crates::ahc061_solver::sample_index(
                                    probs, &mut rng,
                                );
                                sampled.push(cands[idx]);
                            }
                            let next_state =
                                crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                    game, state, &sampled,
                                );
                            let v = if std::env::var("AHC_X02_USE_ZONE_SCORE")
                                .map(|s| s == "1")
                                .unwrap_or(false)
                            {
                                crate::__cargo_equip::crates::ahc061_solver::zone_absolute_score(
                                    game,
                                    &next_state,
                                )
                            } else {
                                crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                    game,
                                    &next_state,
                                )
                            };
                            acc += v;
                            acc2 += v * v;
                        }
                        let mean = acc / sample_count as f64;
                        let var = (acc2 / sample_count as f64 - mean * mean).max(0.0);
                        let std = var.sqrt();
                        let risk_w_default = if game.m >= 6 { 0.40 } else { 0.25 };
                        let risk_w: f64 = std::env::var("AHC_X02_RISK_W")
                            .ok()
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(risk_w_default);
                        let mc_local_w: f64 = std::env::var("AHC_X02_LOCAL_W")
                            .ok()
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(0.09);
                        let total = mean - risk_w * std + mc_local_w * local;
                        if total > best_val {
                            best_val = total;
                            best_mv = mv;
                        }
                    }
                    best_mv
                }
            }
            mod x04_macro_route {
                use crate::__cargo_equip::crates::ahc061_solver::{
                    x01_beam_pessimistic, x02_monte_carlo, x06_expert_switch_hybrid, x174_chokudai,
                    x305_full_horizon, x89_level_fortress, AiModel, Game, State,
                };
                use std::time::{Duration, Instant};

                fn fallback_move(game: &Game, state: &State, models: &[AiModel]) -> (usize, usize) {
                    match std::env::var("AHC_X04_FALLBACK").ok().as_deref() {
                        Some("beam") => x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                            game, state, models,
                        ),
                        Some("mc") => x02_monte_carlo::choose_move_monte_carlo(game, state, models),
                        Some("deep_rollout") => {
                            x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                                game, state, models,
                            )
                        } // x221 disabled
                        Some("x228") => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        ),
                        Some("x86") => x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                            game, state, models,
                        ),
                        Some("x174") => {
                            x174_chokudai::choose_move_x174_chokudai(game, state, models)
                        }
                        Some("x206") => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        ),
                        Some("x89") => {
                            x89_level_fortress::choose_move_x89_level_fortress(game, state, models)
                        }
                        Some("smart") => {
                            // M-aware dispatch: best solver per M based on solver characteristics
                            match game.m {
                                2 => x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                                    game, state, models,
                                ),
                                5 => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                    game, state, models,
                                ),
                                6..=8 => {
                                    x174_chokudai::choose_move_x174_chokudai(game, state, models)
                                }
                                _ => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                    game, state, models,
                                ),
                            }
                        }
                        Some("m6_x174") => {
                            // Only swap M=6 to x174 chokudai, keep x06 default for everything else
                            if game.m == 6 {
                                x174_chokudai::choose_move_x174_chokudai(game, state, models)
                            } else {
                                x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                    game, state, models,
                                )
                            }
                        }
                        Some("m56_x174") => {
                            // Swap M=5 and M=6 to x174 chokudai
                            if game.m == 5 || game.m == 6 {
                                x174_chokudai::choose_move_x174_chokudai(game, state, models)
                            } else {
                                x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                    game, state, models,
                                )
                            }
                        }
                        Some("best_dispatch") => {
                            // Best known dispatch per M value
                            match game.m {
                                2 => x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                                    game, state, models,
                                ),
                                5 => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                    game, state, models,
                                ),
                                6 => x174_chokudai::choose_move_x174_chokudai(game, state, models),
                                7 | 8 => x89_level_fortress::choose_move_x89_level_fortress(
                                    game, state, models,
                                ),
                                _ => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                    game, state, models,
                                ),
                            }
                        }
                        Some("optimal_dispatch") => {
                            // Verified optimal dispatch from x302/x303 testing:
                            // M=2: x01 beam (seed 15: +15k)
                            // M=5: x06 (x174/x228 both worse)
                            // M=6: x174 chokudai (proven, x262)
                            // M=7,8: x89 level fortress (seed 19: +5k)
                            match game.m {
                                2 => x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                                    game, state, models,
                                ),
                                6 => x174_chokudai::choose_move_x174_chokudai(game, state, models),
                                7 | 8 => x89_level_fortress::choose_move_x89_level_fortress(
                                    game, state, models,
                                ),
                                _ => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                    game, state, models,
                                ),
                            }
                        }
                        Some("optimal_v2") => {
                            // Updated dispatch: x305 greedy for M=6 (proven better than x174 on seed 3)
                            // M=2: x01 beam (seed 15: +15k)
                            // M=5: x06 (x174/x228 both worse)
                            // M=6: x305 full horizon greedy (125k vs x174's 81k on seed 3)
                            // M=7,8: x89 level fortress (seed 19: +5k)
                            match game.m {
                                2 => x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                                    game, state, models,
                                ),
                                6 => x305_full_horizon::choose_move_x305_full_horizon(
                                    game, state, models,
                                ),
                                7 | 8 => x89_level_fortress::choose_move_x89_level_fortress(
                                    game, state, models,
                                ),
                                _ => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                    game, state, models,
                                ),
                            }
                        }
                        Some("optimal_v3") => {
                            // Try x305 for both M=5 and M=6
                            match game.m {
                                2 => x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                                    game, state, models,
                                ),
                                5 | 6 => x305_full_horizon::choose_move_x305_full_horizon(
                                    game, state, models,
                                ),
                                7 | 8 => x89_level_fortress::choose_move_x89_level_fortress(
                                    game, state, models,
                                ),
                                _ => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                    game, state, models,
                                ),
                            }
                        }
                        Some("u_aware_v2") => {
                            // U-aware dispatch: x305 greedy for M=3,4 (used when U>=3 causes early cutoff)
                            // Also x305 for M=6 (proven better than x174)
                            match game.m {
                                2 => x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                                    game, state, models,
                                ),
                                3 | 4 | 5 | 6 => x305_full_horizon::choose_move_x305_full_horizon(
                                    game, state, models,
                                ),
                                7 | 8 => x89_level_fortress::choose_move_x89_level_fortress(
                                    game, state, models,
                                ),
                                _ => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                    game, state, models,
                                ),
                            }
                        }
                        Some("u_aware_v3") => {
                            // Refined U-aware dispatch:
                            // - For M=3,4 with U>=3: x305 (beam cutoff sends here, greedy is better for upgrades)
                            // - For M=3,4 with U<=2: x06 (endgame only, x06 handles it well)
                            // - M=6: x305 (proven)
                            // - M=2: x01, M=7,8: x89, else: x06
                            if game.u >= 3 && (game.m == 3 || game.m == 4) {
                                x305_full_horizon::choose_move_x305_full_horizon(
                                    game, state, models,
                                )
                            } else {
                                match game.m {
                                    2 => x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                                        game, state, models,
                                    ),
                                    6 => x305_full_horizon::choose_move_x305_full_horizon(
                                        game, state, models,
                                    ),
                                    7 | 8 => x89_level_fortress::choose_move_x89_level_fortress(
                                        game, state, models,
                                    ),
                                    _ => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                        game, state, models,
                                    ),
                                }
                            }
                        }
                        Some("u_aware_v4") => {
                            // x322: Scoped U-aware dispatch
                            // - M=4 with U>=3: x305 (long greedy portion after beam cutoff)
                            // - M=3: x06 endgame (beam 92%, only 8 turns of fallback — x06 better)
                            // - M=6 with U<=3: x174 chokudai (179k on seed 13, vs x305's 71k)
                            // - M=6 with U>=4: x305 greedy (125k on seed 3, vs x174's 84k)
                            // - M=2: x01 beam (535k on seed 10)
                            // - M=7,8: x89, else: x06
                            if game.u >= 3 && game.m == 4 {
                                x305_full_horizon::choose_move_x305_full_horizon(
                                    game, state, models,
                                )
                            } else {
                                match game.m {
                                    2 => x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                                        game, state, models,
                                    ),
                                    6 if game.u <= 3 => x174_chokudai::choose_move_x174_chokudai(
                                        game, state, models,
                                    ),
                                    6 => x305_full_horizon::choose_move_x305_full_horizon(
                                        game, state, models,
                                    ),
                                    7 | 8 => x89_level_fortress::choose_move_x89_level_fortress(
                                        game, state, models,
                                    ),
                                    _ => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                        game, state, models,
                                    ),
                                }
                            }
                        }
                        Some("u_aware_v5") => {
                            // x354: Same as v4 but M=5 dispatches to x174 chokudai instead of x06
                            // Hypothesis: x174 (iterative deepening beam) handles M=5 better than
                            // x02's 1-step MC (via x06)
                            if game.u >= 3 && game.m == 4 {
                                x305_full_horizon::choose_move_x305_full_horizon(
                                    game, state, models,
                                )
                            } else {
                                match game.m {
                                    2 => x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                                        game, state, models,
                                    ),
                                    5 => x174_chokudai::choose_move_x174_chokudai(
                                        game, state, models,
                                    ),
                                    6 if game.u <= 3 => x174_chokudai::choose_move_x174_chokudai(
                                        game, state, models,
                                    ),
                                    6 => x305_full_horizon::choose_move_x305_full_horizon(
                                        game, state, models,
                                    ),
                                    7 | 8 => x89_level_fortress::choose_move_x89_level_fortress(
                                        game, state, models,
                                    ),
                                    _ => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                        game, state, models,
                                    ),
                                }
                            }
                        }
                        Some("u_aware_v6") => {
                            // x355: Same as v4 but M=5 dispatches to x305 full horizon greedy
                            if game.u >= 3 && game.m == 4 {
                                x305_full_horizon::choose_move_x305_full_horizon(
                                    game, state, models,
                                )
                            } else {
                                match game.m {
                                    2 => x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                                        game, state, models,
                                    ),
                                    5 => x305_full_horizon::choose_move_x305_full_horizon(
                                        game, state, models,
                                    ),
                                    6 if game.u <= 3 => x174_chokudai::choose_move_x174_chokudai(
                                        game, state, models,
                                    ),
                                    6 => x305_full_horizon::choose_move_x305_full_horizon(
                                        game, state, models,
                                    ),
                                    7 | 8 => x89_level_fortress::choose_move_x89_level_fortress(
                                        game, state, models,
                                    ),
                                    _ => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                        game, state, models,
                                    ),
                                }
                            }
                        }
                        Some("smart_v2") => {
                            // Revised M-aware dispatch based on x253-x259 non-beam seed analysis
                            match game.m {
                                2 | 6 => {
                                    x174_chokudai::choose_move_x174_chokudai(game, state, models)
                                }
                                5 | 7 | 8 => x89_level_fortress::choose_move_x89_level_fortress(
                                    game, state, models,
                                ),
                                _ => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                    game, state, models,
                                ),
                            }
                        }
                        Some("u_aware") => {
                            // U-aware dispatch: different strategies for different level caps
                            match game.u {
                                1 => x174_chokudai::choose_move_x174_chokudai(game, state, models),
                                2 => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                    game, state, models,
                                ),
                                _ => x89_level_fortress::choose_move_x89_level_fortress(
                                    game, state, models,
                                ),
                            }
                        }
                        Some("phase") => {
                            // Phase-aware switching: expansion early, defense late
                            let phase = state.turn as f64 / game.t as f64;
                            let switch_point: f64 = std::env::var("AHC_PHASE_SWITCH")
                                .ok()
                                .and_then(|v| v.parse().ok())
                                .unwrap_or(0.55);
                            if phase < switch_point {
                                // Early game: x228 strategic MC (expansion + leader tracking)
                                x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                    game, state, models,
                                )
                            } else {
                                // Late game: x89 level fortress (defend + upgrade)
                                x89_level_fortress::choose_move_x89_level_fortress(
                                    game, state, models,
                                )
                            }
                        }
                        Some("u_aware_v5") => {
                            // x327: Upgrade-sweep fallback — prioritize upgrading high-V cells
                            // After beam cutoff, greedily upgrade the highest-V owned cell each turn.
                            // Only if nothing to upgrade, fall back to u_aware_v4 dispatch.
                            let candidates =
                                crate::__cargo_equip::crates::ahc061_solver::get_candidates(
                                    game, state, 0,
                                );
                            if game.u > 1 {
                                // Find best upgrade target: owned cell with highest V * (U - level)
                                let mut best_upgrade: Option<((usize, usize), f64)> = None;
                                for &(x, y) in &candidates {
                                    if state.owner[x][y] == 0
                                        && (state.level[x][y] as usize) < game.u
                                    {
                                        let v = game.v[x][y] as f64;
                                        let remaining_levels =
                                            (game.u as f64 - state.level[x][y] as f64);
                                        let value = v * remaining_levels;
                                        if best_upgrade.is_none() || value > best_upgrade.unwrap().1
                                        {
                                            best_upgrade = Some(((x, y), value));
                                        }
                                    }
                                }
                                if let Some((target, _)) = best_upgrade {
                                    return target;
                                }
                            }
                            // Nothing to upgrade — use u_aware_v4 dispatch
                            if game.u >= 3 && game.m == 4 {
                                x305_full_horizon::choose_move_x305_full_horizon(
                                    game, state, models,
                                )
                            } else {
                                match game.m {
                                    2 => x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                                        game, state, models,
                                    ),
                                    6 if game.u <= 3 => x174_chokudai::choose_move_x174_chokudai(
                                        game, state, models,
                                    ),
                                    6 => x305_full_horizon::choose_move_x305_full_horizon(
                                        game, state, models,
                                    ),
                                    7 | 8 => x89_level_fortress::choose_move_x89_level_fortress(
                                        game, state, models,
                                    ),
                                    _ => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                        game, state, models,
                                    ),
                                }
                            }
                        }
                        Some("u_aware_v7") => {
                            // x331: M=3 only ratio greedy endgame, everything else u_aware_v4
                            if game.m == 3 {
                                x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                    game, state, models,
                                )
                            } else {
                                // M!=3: original u_aware_v4 dispatch
                                match game.m {
                                    2 => x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                                        game, state, models,
                                    ),
                                    6 if game.u <= 3 => x174_chokudai::choose_move_x174_chokudai(
                                        game, state, models,
                                    ),
                                    6 => x305_full_horizon::choose_move_x305_full_horizon(
                                        game, state, models,
                                    ),
                                    7 | 8 => x89_level_fortress::choose_move_x89_level_fortress(
                                        game, state, models,
                                    ),
                                    _ => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                        game, state, models,
                                    ),
                                }
                            }
                        }
                        Some("u_aware_v6") => {
                            // x330: Beam + Ratio Greedy hybrid fallback
                            // After beam cutoff, use x328 ratio greedy for M=3-4 (where it excels)
                            // For other M values, use u_aware_v4 dispatch
                            if game.m <= 4 {
                                x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                    game, state, models,
                                )
                            } else {
                                // M>=5: use u_aware_v4 dispatch (ratio greedy is bad here)
                                match game.m {
                                    6 if game.u <= 3 => x174_chokudai::choose_move_x174_chokudai(
                                        game, state, models,
                                    ),
                                    6 => x305_full_horizon::choose_move_x305_full_horizon(
                                        game, state, models,
                                    ),
                                    7 | 8 => x89_level_fortress::choose_move_x89_level_fortress(
                                        game, state, models,
                                    ),
                                    _ => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                        game, state, models,
                                    ),
                                }
                            }
                        }
                        _ => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        ),
                    }
                }
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

                fn pressure_weight_by_phase(phase: f64, split: f64, early: f64, late: f64) -> f64 {
                    if phase <= split {
                        early
                    } else {
                        late
                    }
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
                        if crate::__cargo_equip::crates::ahc061_solver::in_bounds(game.n, nx, ny) {
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
                    upgrade_weight: f64,
                ) -> Vec<(usize, usize)> {
                    choose_target_cells_with_bfs(
                        game,
                        state,
                        conflict,
                        max_targets,
                        pressure_weight,
                        phase,
                        upgrade_weight,
                        0.0,
                    )
                }

                fn choose_target_cells_with_bfs(
                    game: &Game,
                    state: &State,
                    conflict: &[Vec<f64>],
                    max_targets: usize,
                    pressure_weight: f64,
                    phase: f64,
                    upgrade_weight: f64,
                    bfs_decay: f64,
                ) -> Vec<(usize, usize)> {
                    let dist_map = if bfs_decay > 0.0 {
                        Some(
                            crate::__cargo_equip::crates::ahc061_solver::bfs_distance_map(
                                game, state, 0,
                            ),
                        )
                    } else {
                        None
                    };
                    // Leader detection for target prioritization
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let leader_target_bonus = env_f64("AHC_X04_LEADER_TARGET_BONUS", 0.0, 0.0, 3.0);
                    // Value zone bonus: prioritize cells in the densest KxK region
                    let zone_bonus = env_f64("AHC_X04_ZONE_BONUS", 0.0, 0.0, 5.0);
                    let use_adaptive_zone = std::env::var("AHC_ZONE_ADAPTIVE")
                        .map(|v| v == "1")
                        .unwrap_or(false);
                    let use_dual_zone = std::env::var("AHC_ZONE_DUAL")
                        .map(|v| v == "1")
                        .unwrap_or(false);
                    let use_proximity_zone = std::env::var("AHC_ZONE_PROXIMITY")
                        .map(|v| v == "1")
                        .unwrap_or(false);
                    let zone = if zone_bonus > 0.0 {
                        Some(if use_dual_zone {
                            crate::__cargo_equip::crates::ahc061_solver::compute_dual_zone(game)
                        } else if use_proximity_zone {
                            crate::__cargo_equip::crates::ahc061_solver::compute_proximity_zone(
                                game, state,
                            )
                        } else if use_adaptive_zone {
                            crate::__cargo_equip::crates::ahc061_solver::compute_adaptive_zone(
                                game, state,
                            )
                        } else {
                            crate::__cargo_equip::crates::ahc061_solver::compute_value_zone(game)
                        })
                    } else {
                        None
                    };

                    let frontier_denial_bonus =
                        env_f64("AHC_X04_FRONTIER_DENIAL_BONUS", 0.0, 0.0, 5.0);
                    let mut scored = Vec::<((usize, usize), f64)>::new();
                    for x in 0..game.n {
                        for y in 0..game.n {
                            let owner = state.owner[x][y];
                            let level = state.level[x][y];
                            let v = game.v[x][y] as f64;
                            let mut w = 0.0;
                            let target_attack_w =
                                env_f64("AHC_X04_TARGET_ATTACK_W", 1.18, 0.5, 3.0);
                            if owner == -1 {
                                w = 1.00 * v;
                                // Frontier denial bonus: empty cells adjacent to leader territory
                                // have dual value — capturing them increases S0 AND denies SA growth
                                if frontier_denial_bonus > 0.0 {
                                    let dirs: [(isize, isize); 4] =
                                        [(0, 1), (0, -1), (1, 0), (-1, 0)];
                                    for (dx, dy) in dirs {
                                        let nx = x as isize + dx;
                                        let ny = y as isize + dy;
                                        if nx >= 0
                                            && nx < game.n as isize
                                            && ny >= 0
                                            && ny < game.n as isize
                                        {
                                            let no = state.owner[nx as usize][ny as usize];
                                            if no > 0 && scores[no as usize] == max_ai_i64 {
                                                w += frontier_denial_bonus * v * phase;
                                                break;
                                            }
                                        }
                                    }
                                }
                            } else if owner > 0 && level == 1 {
                                w = target_attack_w * v;
                                // Extra bonus for targeting the leader's cells
                                if leader_target_bonus > 0.0 && scores[owner as usize] == max_ai_i64
                                {
                                    w += leader_target_bonus * phase * v;
                                }
                            } else if owner == 0 && level < game.u {
                                // Use env-selectable upgrade target formula
                                let upgrade_flat = std::env::var("AHC_X04_UPGRADE_TARGET_FLAT")
                                    .map(|val| val == "1")
                                    .unwrap_or(false);
                                if upgrade_flat {
                                    // Flat formula: each upgrade visit adds V to S0 regardless of level
                                    // Small bonus for cells with more remaining levels (more future visits)
                                    let remaining_bonus =
                                        1.0 + 0.08 * (game.u as f64 - level as f64 - 1.0).max(0.0);
                                    w = upgrade_weight * v * remaining_bonus;
                                } else {
                                    // Original formula: remaining potential proportional
                                    w = upgrade_weight * v * (game.u - level) as f64
                                        / game.u as f64;
                                }
                            }
                            if w > 0.0 {
                                // Zone bonus: cells in the high-value zone get a multiplicative boost
                                if let Some(ref z) = zone {
                                    if z[x][y] {
                                        w *= 1.0 + zone_bonus;
                                    }
                                }
                                if pressure_weight.abs() > f64::EPSILON {
                                    let p = estimate_move_pressure(
                                        game,
                                        state,
                                        conflict,
                                        (x, y),
                                        phase,
                                    );
                                    w += -pressure_weight * p * v;
                                }
                                // BFS distance penalty: discount distant targets
                                if let Some(ref dm) = dist_map {
                                    let d = dm[x][y];
                                    if d == usize::MAX {
                                        // Unreachable through own territory: heavy penalty
                                        let md = manhattan(state.pos[0], (x, y));
                                        w *= 1.0 / (1.0 + bfs_decay * (md as f64) * 3.0);
                                    } else {
                                        w *= 1.0 / (1.0 + bfs_decay * d as f64);
                                    }
                                }
                                scored.push(((x, y), w));
                            }
                        }
                    }
                    if scored.is_empty() {
                        return Vec::new();
                    }
                    scored
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    scored.into_iter().take(max_targets).map(|x| x.0).collect()
                }

                fn predicted_ai_moves(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    step: usize,
                ) -> Vec<(usize, usize)> {
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let primary: Vec<(usize, usize)> = top2.iter().map(|x| x.0).collect();
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2);
                    let secondary_cap = if game.m >= 7 && uncertainty >= 0.40 {
                        3
                    } else if game.m >= 6 && uncertainty >= 0.28 {
                        2
                    } else {
                        1
                    };
                    let secondary =
                        crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(
                            &scores,
                            &top2,
                            secondary_cap,
                        );
                    if uncertainty >= 0.24 && (step % 2 == 1) {
                        secondary
                    } else {
                        primary
                    }
                }

                type RouteScoFn = fn(&Game, &State) -> f64;

                fn resolve_route_score_fn() -> RouteScoFn {
                    match std::env::var("AHC_X04_ROUTE_SCORE").ok().as_deref() {
                        Some("denial") => crate::__cargo_equip::crates::ahc061_solver::denial_score,
                        Some("denial_v2") => {
                            crate::__cargo_equip::crates::ahc061_solver::denial_score_v2
                        }
                        Some("absolute") => {
                            crate::__cargo_equip::crates::ahc061_solver::absolute_score
                        }
                        _ => {
                            // Legacy env var support
                            if std::env::var("AHC_X04_USE_DENIAL_ROUTE")
                                .map(|v| v == "1")
                                .unwrap_or(false)
                            {
                                crate::__cargo_equip::crates::ahc061_solver::denial_score
                            } else {
                                crate::__cargo_equip::crates::ahc061_solver::strategic_score
                            }
                        }
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
                    let score_fn = resolve_route_score_fn();
                    let prev_score = score_fn(game, prev);
                    let next_score = score_fn(game, next);
                    let gain = next_score - prev_score;
                    let d0 = manhattan(prev.pos[0], target) as f64;
                    let d1 = manhattan(mv, target) as f64;
                    let route_bonus = (d0 - d1).clamp(-6.0, 6.0);
                    gain + route_coeff * route_bonus - pressure_weight * pressure
                        + local_coeff * local
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
                    deadline: Option<Instant>,
                ) -> f64 {
                    let mut moves = Vec::with_capacity(game.m);
                    moves.push(first_mv);
                    moves.extend_from_slice(&predicted_ai_moves(game, state, models, 0));
                    let first_state = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                        game, state, &moves,
                    );
                    let first_conflict =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );

                    let first_phase = state.turn as f64 / game.t as f64;
                    let first_pressure =
                        estimate_move_pressure(game, state, &first_conflict, first_mv, first_phase);
                    let first_local =
                        crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                            game,
                            state,
                            first_mv,
                            &crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state),
                            crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state)[0]
                                as f64,
                            crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state)
                                .iter()
                                .skip(1)
                                .copied()
                                .max()
                                .unwrap_or(1)
                                .max(1),
                            state.turn as f64 / game.t as f64,
                            &first_conflict,
                            state.pos[0],
                            &{
                                let mut is_leader = vec![false; game.m];
                                let scores =
                                    crate::__cargo_equip::crates::ahc061_solver::calc_scores(
                                        game, state,
                                    );
                                let max_ai =
                                    scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                                for p in 1..game.m {
                                    if scores[p] == max_ai {
                                        is_leader[p] = true;
                                    }
                                }
                                is_leader
                            },
                        );
                    let first_pressure_weight = pressure_weight_by_phase(
                        first_phase,
                        pressure_phase_split,
                        pressure_weight_early,
                        pressure_weight_late,
                    );

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
                        if let Some(dl) = deadline {
                            if Instant::now() >= dl {
                                break;
                            }
                        }
                        let mut next_beam = Vec::<RouteNode>::new();
                        for node in &beam {
                            let scores = crate::__cargo_equip::crates::ahc061_solver::calc_scores(
                                game,
                                &node.state,
                            );
                            let s0 = scores[0] as f64;
                            let max_ai_i64 =
                                scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                            let phase = node.state.turn as f64 / game.t as f64;
                            let conflict =
                                crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                                    game,
                                    &node.state,
                                    models,
                                );
                            let cur = node.state.pos[0];
                            let mut is_leader = vec![false; game.m];
                            for p in 1..game.m {
                                if scores[p] == max_ai_i64 {
                                    is_leader[p] = true;
                                }
                            }

                            let mut my_cands =
                                crate::__cargo_equip::crates::ahc061_solver::get_candidates(
                                    game,
                                    &node.state,
                                    0,
                                );
                            if my_cands.is_empty() {
                                continue;
                            }
                            my_cands.sort_by(|&a, &b| {
                                let la = crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                    game, &node.state, a, &scores, s0, max_ai_i64, phase, &conflict, cur, &is_leader,
                                ) + 18.0
                                    * (manhattan(cur, target) as f64 - manhattan(a, target) as f64);
                                let lb = crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                    game, &node.state, b, &scores, s0, max_ai_i64, phase, &conflict, cur, &is_leader,
                                ) + 18.0
                                    * (manhattan(cur, target) as f64 - manhattan(b, target) as f64);
                                lb.partial_cmp(&la).unwrap_or(std::cmp::Ordering::Equal)
                            });

                            for &mv in my_cands.iter().take(branch_width) {
                                let local = crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                game, &node.state, mv, &scores, s0, max_ai_i64, phase, &conflict, cur, &is_leader,
                            );
                                let pressure =
                                    estimate_move_pressure(game, &node.state, &conflict, mv, phase);
                                let pressure_weight = pressure_weight_by_phase(
                                    phase,
                                    pressure_phase_split,
                                    pressure_weight_early,
                                    pressure_weight_late,
                                );
                                let mut full_moves = Vec::with_capacity(game.m);
                                full_moves.push(mv);
                                full_moves.extend_from_slice(&predicted_ai_moves(
                                    game,
                                    &node.state,
                                    models,
                                    step,
                                ));
                                let ns = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                    game,
                                    &node.state,
                                    &full_moves,
                                );
                                let discount_base = env_f64("AHC_X04_DISCOUNT", 0.93, 0.80, 0.99);
                                let discount = discount_base.powi(step as i32);
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
                        next_beam.sort_by(|a, b| {
                            b.score
                                .partial_cmp(&a.score)
                                .unwrap_or(std::cmp::Ordering::Equal)
                        });
                        next_beam.truncate(beam_width);
                        beam = next_beam;
                    }

                    let final_eval_w = env_f64("AHC_X04_FINAL_EVAL_W", 0.03, 0.0, 0.50);
                    let score_fn = resolve_route_score_fn();
                    beam.iter()
                        .map(|n| n.score + final_eval_w * score_fn(game, &n.state))
                        .fold(f64::NEG_INFINITY, f64::max)
                }

                pub(super) fn choose_move_x04_macro_route(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    // M帯制限: AHC_X04_ALLOWED_M で許可するM値を指定（カンマ区切り、デフォルト=4のみ）
                    let allowed_m_str =
                        std::env::var("AHC_X04_ALLOWED_M").unwrap_or_else(|_| "4".to_string());
                    let allowed_m: Vec<usize> = allowed_m_str
                        .split(',')
                        .filter_map(|s| s.trim().parse().ok())
                        .collect();
                    if !allowed_m.contains(&game.m) {
                        return fallback_move(game, state, models);
                    }
                    let default_cutoff = env_f64("AHC_X04_PHASE_CUTOFF", 0.65, 0.20, 0.95);
                    // Per-M phase cutoff override: AHC_X04_PHASE_CUTOFF_M2/M3/.../M8
                    let per_m_cutoff_key = format!("AHC_X04_PHASE_CUTOFF_M{}", game.m);
                    let m_cutoff = env_f64(&per_m_cutoff_key, default_cutoff, 0.20, 0.95);
                    // Per-M-U combined cutoff (highest priority): AHC_X04_PHASE_CUTOFF_M2_U3
                    let mu_cutoff_key = format!("AHC_X04_PHASE_CUTOFF_M{}_U{}", game.m, game.u);
                    // Per-U phase cutoff override: AHC_X04_PHASE_CUTOFF_U3/U4/U5
                    let per_u_cutoff_key = format!("AHC_X04_PHASE_CUTOFF_U{}", game.u);
                    let phase_cutoff = if std::env::var(&mu_cutoff_key).is_ok() {
                        env_f64(&mu_cutoff_key, m_cutoff, 0.0, 0.98)
                    } else {
                        env_f64(&per_u_cutoff_key, m_cutoff, 0.0, 0.98)
                    };
                    let phase_split = env_f64("AHC_X04_PHASE_SPLIT", 0.50, 0.20, 0.90);
                    // Per-M beam parameter overrides: e.g. AHC_X04_TARGET_COUNT_M2=4
                    let m_suffix = format!("_M{}", game.m);
                    let target_count = env_u64(
                        &format!("AHC_X04_TARGET_COUNT{}", m_suffix),
                        env_u64("AHC_X04_TARGET_COUNT", 5, 2, 12),
                        2,
                        12,
                    ) as usize;
                    let target_eval = env_u64(
                        &format!("AHC_X04_TARGET_EVAL{}", m_suffix),
                        env_u64("AHC_X04_TARGET_EVAL", 4, 1, 8),
                        1,
                        8,
                    ) as usize;
                    let candidate_cap = env_u64(
                        &format!("AHC_X04_CANDIDATE_CAP{}", m_suffix),
                        env_u64("AHC_X04_CANDIDATE_CAP", 7, 4, 20),
                        4,
                        20,
                    ) as usize;
                    let plan_len_fast = env_u64(
                        &format!("AHC_X04_PLAN_LEN_FAST{}", m_suffix),
                        env_u64("AHC_X04_PLAN_LEN_FAST", 7, 3, 10),
                        3,
                        10,
                    ) as usize;
                    let plan_len_slow = env_u64(
                        &format!("AHC_X04_PLAN_LEN_SLOW{}", m_suffix),
                        env_u64("AHC_X04_PLAN_LEN_SLOW", 8, 3, 10),
                        3,
                        10,
                    ) as usize;
                    let beam_width_fast = env_u64(
                        &format!("AHC_X04_BEAM_WIDTH_FAST{}", m_suffix),
                        env_u64("AHC_X04_BEAM_WIDTH_FAST", 5, 2, 8),
                        2,
                        8,
                    ) as usize;
                    let beam_width_slow = env_u64(
                        &format!("AHC_X04_BEAM_WIDTH_SLOW{}", m_suffix),
                        env_u64("AHC_X04_BEAM_WIDTH_SLOW", 6, 2, 8),
                        2,
                        8,
                    ) as usize;
                    let beam_width_min = env_u64(
                        &format!("AHC_X04_BEAM_WIDTH_MIN{}", m_suffix),
                        env_u64("AHC_X04_BEAM_WIDTH_MIN", 0, 0, 8),
                        0,
                        8,
                    ) as usize;
                    let branch_width = env_u64(
                        &format!("AHC_X04_BRANCH_WIDTH{}", m_suffix),
                        env_u64("AHC_X04_BRANCH_WIDTH", 3, 1, 4),
                        1,
                        4,
                    ) as usize;
                    let local_weight = env_f64("AHC_X04_LOCAL_WEIGHT", 0.10, 0.0, 0.40);
                    let local_coeff = env_f64("AHC_X04_LOCAL_COEFF", 0.07, 0.0, 0.40);
                    let route_coeff = env_f64("AHC_X04_ROUTE_COEFF", 42.0, 10.0, 90.0);
                    let target_pressure_weight =
                        env_f64("AHC_X04_TARGET_PRESSURE_WEIGHT", 0.0, -1.0, 2.0);
                    let pressure_weight = env_f64("AHC_X04_PRESSURE_WEIGHT", 0.00, -1.5, 2.0);
                    let pressure_weight_early =
                        env_f64("AHC_X04_PRESSURE_WEIGHT_EARLY", pressure_weight, -1.5, 2.0);
                    let pressure_weight_late =
                        env_f64("AHC_X04_PRESSURE_WEIGHT_LATE", pressure_weight, -1.5, 2.0);
                    let pressure_route_weight =
                        env_f64("AHC_X04_ROUTE_PRESSURE_WEIGHT", 0.00, -1.5, 2.0);
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
                    let pressure_phase_split =
                        env_f64("AHC_X04_PRESSURE_PHASE_SPLIT", 0.60, 0.20, 0.90);
                    let upgrade_weight_base = env_f64("AHC_X04_UPGRADE_WEIGHT", 0.72, 0.0, 5.0);
                    let upgrade_weight_high_u =
                        env_f64("AHC_X04_UPGRADE_WEIGHT_HIGH_U", -1.0, -1.0, 5.0);
                    let default_for_u = if game.u >= 3 && upgrade_weight_high_u >= 0.0 {
                        upgrade_weight_high_u
                    } else {
                        upgrade_weight_base
                    };
                    // Per-U override: AHC_X04_UPGRADE_WEIGHT_U2/U3/U4/U5
                    let per_u_key = format!("AHC_X04_UPGRADE_WEIGHT_U{}", game.u);
                    let upgrade_weight = env_f64(&per_u_key, default_for_u, 0.0, 5.0);
                    let target_bfs_decay = env_f64("AHC_X04_TARGET_BFS_DECAY", 0.0, 0.0, 2.0);
                    let phase_now = state.turn as f64 / game.t as f64;
                    // Dynamic ratio-based switching: if winning big, switch to fallback (upgrade mode)
                    let ratio_switch = env_f64("AHC_X04_RATIO_SWITCH", 0.0, 0.0, 20.0);
                    if ratio_switch > 0.0 && phase_now > 0.15 {
                        let sc =
                            crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                        let s0 = sc[0].max(1) as f64;
                        let sa = sc.iter().skip(1).copied().max().unwrap_or(1).max(1) as f64;
                        if s0 / sa > ratio_switch {
                            return fallback_move(game, state, models);
                        }
                    }
                    if phase_now > phase_cutoff {
                        return fallback_move(game, state, models);
                    }

                    // Preemptive upgrade: if an adjacent own cell has high upgrade value,
                    // skip beam search and upgrade immediately.
                    // Uses actual score delta (absolute_score) to compare.
                    let preemptive_threshold =
                        env_f64("AHC_X04_PREEMPTIVE_UPGRADE", -1.0, -1.0, 50000.0);
                    if preemptive_threshold >= 0.0 && game.u > 1 {
                        let cur_pos = state.pos[0];
                        let pre_scores =
                            crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                        let pre_s0 = pre_scores[0].max(1) as f64;
                        let pre_sa =
                            pre_scores.iter().skip(1).copied().max().unwrap_or(1).max(1) as f64;
                        let pre_abs = (1.0 + pre_s0 / pre_sa).log2() * 1e5;

                        let mut best_upgrade: Option<((usize, usize), f64)> = None;
                        // Check current cell + adjacent cells
                        let dirs: [(isize, isize); 5] = [(0, 0), (0, 1), (1, 0), (0, -1), (-1, 0)];
                        for (dx, dy) in dirs {
                            let nx = cur_pos.0 as isize + dx;
                            let ny = cur_pos.1 as isize + dy;
                            if nx < 0 || nx >= game.n as isize || ny < 0 || ny >= game.n as isize {
                                continue;
                            }
                            let ux = nx as usize;
                            let uy = ny as usize;
                            if state.owner[ux][uy] == 0 && state.level[ux][uy] < game.u {
                                // Compute actual score delta from upgrading this cell
                                let v = game.v[ux][uy] as i64;
                                let new_s0 = pre_s0 + v as f64;
                                let new_abs = (1.0 + new_s0 / pre_sa).log2() * 1e5;
                                let delta = new_abs - pre_abs;
                                if delta >= preemptive_threshold {
                                    if best_upgrade.is_none() || delta > best_upgrade.unwrap().1 {
                                        best_upgrade = Some(((ux, uy), delta));
                                    }
                                }
                            }
                        }
                        if let Some((target, _)) = best_upgrade {
                            // Check that target is reachable (in candidates)
                            let cands = crate::__cargo_equip::crates::ahc061_solver::get_candidates(
                                game, state, 0,
                            );
                            if cands.contains(&target) {
                                return target;
                            }
                        }
                    }

                    let candidates =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if candidates.len() <= 1 {
                        return candidates.first().copied().unwrap_or(state.pos[0]);
                    }

                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let s0 = scores[0] as f64;
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let phase = state.turn as f64 / game.t as f64;
                    let conflict_map =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let targets = choose_target_cells_with_bfs(
                        game,
                        state,
                        &conflict_map,
                        target_count,
                        target_pressure_weight,
                        phase,
                        upgrade_weight,
                        target_bfs_decay,
                    );
                    if targets.is_empty() {
                        return fallback_move(game, state, models);
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
                        let pressure =
                            estimate_move_pressure(game, state, &conflict_map, mv, phase);
                        let local =
                            crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
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
                        let pressure_weight_now = pressure_weight_by_phase(
                            phase,
                            pressure_phase_split,
                            pressure_weight_early,
                            pressure_weight_late,
                        );
                        ranked.push((mv, local - pressure_weight_now * pressure));
                    }
                    ranked
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    let candidate_cap = candidate_cap.min(ranked.len());
                    let fast_phase = phase_now > phase_split;
                    let plan_len = if fast_phase {
                        plan_len_fast
                    } else {
                        plan_len_slow
                    };
                    let beam_width = if fast_phase {
                        beam_width_fast
                    } else {
                        beam_width_slow
                    };

                    // Time management: compute deadline for this turn
                    let turn_budget = crate::__cargo_equip::crates::ahc061_solver::turn_budget_ms(
                        state.turn, game.t,
                    );
                    let turn_start = Instant::now();
                    let deadline = if turn_budget > 0 {
                        Some(turn_start + Duration::from_millis(turn_budget * 96 / 100))
                    } else {
                        None
                    };

                    let mut best_mv = ranked[0].0;
                    let mut best_score = f64::NEG_INFINITY;

                    // Adaptive beam: decide beam width BEFORE the loop using a timing probe.
                    // This ensures ALL candidates are evaluated with the SAME beam width.
                    let effective_beam = if beam_width_min > 0
                        && beam_width_min < beam_width
                        && turn_budget > 0
                        && !ranked.is_empty()
                        && !targets.is_empty()
                    {
                        let probe_start = Instant::now();
                        let _probe = beam_route_score(
                            game,
                            state,
                            models,
                            ranked[0].0,
                            targets[0],
                            plan_len,
                            beam_width,
                            branch_width,
                            pressure_phase_split,
                            pressure_route_weight_early,
                            pressure_route_weight_late,
                            local_coeff,
                            route_coeff,
                            deadline,
                        );
                        let probe_us = probe_start.elapsed().as_micros() as u64;
                        // Project: probe_us * candidate_cap * target_eval
                        let projected_us = probe_us * candidate_cap as u64 * target_eval as u64;
                        // Allow 55% of deadline for initial pass
                        let budget_us = turn_budget * 960 * 55 / 100; // turn_budget*96%*55% in microseconds
                        if projected_us > budget_us {
                            beam_width_min
                        } else {
                            beam_width
                        }
                    } else {
                        beam_width
                    };

                    for &(mv, local) in ranked.iter().take(candidate_cap) {
                        // Time guard: stop evaluating more candidates if running out of time
                        if let Some(dl) = deadline {
                            if Instant::now() >= dl {
                                break;
                            }
                        }
                        let mut best_target_score = f64::NEG_INFINITY;
                        for &target in targets.iter().take(target_eval) {
                            if let Some(dl) = deadline {
                                if Instant::now() >= dl {
                                    break;
                                }
                            }
                            let route_score = beam_route_score(
                                game,
                                state,
                                models,
                                mv,
                                target,
                                plan_len,
                                effective_beam,
                                branch_width,
                                pressure_phase_split,
                                pressure_route_weight_early,
                                pressure_route_weight_late,
                                local_coeff,
                                route_coeff,
                                deadline,
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

                    // Iterative deepening: loop with progressively wider beams until deadline
                    if let Some(dl) = deadline {
                        let iter_deepen = env_u64("AHC_X04_ITER_DEEPEN", 1, 0, 1);
                        if iter_deepen == 1 {
                            let mut deep_beam = beam_width + 2;
                            let deep_plan = (plan_len * 2).min(16);
                            let re_eval_count = (env_u64("AHC_X04_RE_EVAL_COUNT", 3, 1, 10)
                                as usize)
                                .min(candidate_cap);
                            while Instant::now() < dl && deep_beam <= 20 {
                                for &(mv, local) in ranked.iter().take(re_eval_count) {
                                    if Instant::now() >= dl {
                                        break;
                                    }
                                    let mut best_target_score = f64::NEG_INFINITY;
                                    for &target in targets.iter().take(target_eval) {
                                        if Instant::now() >= dl {
                                            break;
                                        }
                                        let route_score = beam_route_score(
                                            game,
                                            state,
                                            models,
                                            mv,
                                            target,
                                            deep_plan,
                                            deep_beam,
                                            branch_width,
                                            pressure_phase_split,
                                            pressure_route_weight_early,
                                            pressure_route_weight_late,
                                            local_coeff,
                                            route_coeff,
                                            deadline,
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
                                deep_beam += 2;
                            }
                        }
                    }

                    best_mv
                }
            }
            mod x06_expert_switch_hybrid {
                use crate::__cargo_equip::crates::ahc061_solver::{
                    x01_beam_pessimistic, x02_monte_carlo, AiModel, Game, State,
                };

                pub(super) fn choose_move_x06_expert_switch(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    if (3..=5).contains(&game.m) {
                        x02_monte_carlo::choose_move_monte_carlo(game, state, models)
                    } else {
                        x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models)
                    }
                }
            }
            mod x89_level_fortress {

                use crate::__cargo_equip::crates::ahc061_solver::{
                    ai_features, calc_scores, dot, estimate_conflict_map, get_candidates,
                    in_bounds, simulate_turn, AiModel, Game, State,
                };

                const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

                fn predict_ai_moves(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> Vec<(usize, usize)> {
                    let mut moves = Vec::with_capacity(game.m.saturating_sub(1));
                    for ai_idx in 0..game.m.saturating_sub(1) {
                        let player = ai_idx + 1;
                        let cands = get_candidates(game, state, player);
                        if cands.is_empty() {
                            moves.push(state.pos[player]);
                            continue;
                        }
                        if cands.len() == 1 {
                            moves.push(cands[0]);
                            continue;
                        }
                        let mut best_score = f64::NEG_INFINITY;
                        let mut best = cands[0];
                        for &c in &cands {
                            let feat = ai_features(game, state, player, c);
                            let s = dot(&models[ai_idx].w, &feat);
                            if s > best_score {
                                best_score = s;
                                best = c;
                            }
                        }
                        moves.push(best);
                    }
                    moves
                }

                fn defense_urgency(game: &Game, state: &State, x: usize, y: usize) -> f64 {
                    if state.owner[x][y] != 0 {
                        return 0.0;
                    }
                    let v = game.v[x][y] as f64;
                    let level = state.level[x][y];

                    if level >= 2 {
                        // すでに防御済み、さらなるレベル上げの価値
                        if level < game.u {
                            return v * 0.3; // 追加レベルの価値は低め
                        }
                        return 0.0; // max level
                    }

                    // level == 1: 最も脆弱。隣接に敵がいるなら超緊急
                    let mut enemy_adjacent = false;
                    let mut enemy_threat = 0.0_f64;
                    for (dx, dy) in DIRS {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if in_bounds(game.n, nx, ny) {
                            let ux = nx as usize;
                            let uy = ny as usize;
                            if state.owner[ux][uy] > 0 {
                                enemy_adjacent = true;
                                // 敵のレベルが低いほど、こちらを攻撃しやすい
                                enemy_threat += 1.0;
                            }
                            // 敵プレイヤーが隣接セルにいる場合
                            for p in 1..game.m {
                                if state.pos[p] == (ux, uy) {
                                    enemy_threat += 2.0;
                                }
                            }
                        }
                    }

                    if enemy_adjacent {
                        v * 3.0 + enemy_threat * v * 0.5 // 超緊急: 敵が隣にいるlevel1セル
                    } else {
                        v * 1.5 // 緊急: level1は常にリスク
                    }
                }

                fn score_move(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    mv: (usize, usize),
                    scores: &[i64],
                    conflict_map: &[Vec<f64>],
                ) -> f64 {
                    let (x, y) = mv;
                    let owner = state.owner[x][y];
                    let level = state.level[x][y];
                    let v = game.v[x][y] as f64;
                    let phase = state.turn as f64 / game.t as f64;
                    let remaining = (game.t - state.turn) as f64;
                    let s0 = scores[0].max(1) as f64;
                    let sa = scores.iter().skip(1).copied().max().unwrap_or(1).max(1) as f64;

                    let mut score = 0.0_f64;

                    if owner == -1 {
                        // === 中立セル獲得 ===
                        // 即座にスコア +V（level 1で獲得）
                        // 序盤ほど価値が高い（先に取らないとAIに取られる）
                        let expansion_value = v * (1.2 + 0.8 * (1.0 - phase));

                        // 連結性ボーナス: 自領土に隣接していると到達維持が容易
                        let mut adj_own = 0;
                        for (dx, dy) in DIRS {
                            let nx = x as isize + dx;
                            let ny = y as isize + dy;
                            if in_bounds(game.n, nx, ny) {
                                if state.owner[nx as usize][ny as usize] == 0 {
                                    adj_own += 1;
                                }
                            }
                        }
                        let connectivity_bonus = v * 0.15 * adj_own as f64;

                        score += expansion_value + connectivity_bonus;

                        // U > 1 なら、この中立セルは将来 level up 可能 → 追加価値
                        if game.u > 1 {
                            let future_level_value = v * (game.u - 1) as f64 * 0.10;
                            score += future_level_value;
                        }
                    } else if owner == 0 {
                        // === 自領土: レベル上げ ===
                        if level < game.u {
                            // レベル上げの直接的なスコア貢献: +V per level
                            let direct_value = v;

                            // 防御壁化ボーナス: level 1→2 は特に重要（敵の攻撃を弾く）
                            let defense_bonus = if level == 1 {
                                v * 2.5 // level 1→2: 防御の境界線。最も重要
                            } else if level == 2 {
                                v * 1.0 // level 2→3: 追加防御は嬉しいが緊急度は低い
                            } else {
                                v * 0.5 // level 3+→: さらに低い
                            };

                            // 残りターン数に基づく価値: 早期のレベル上げほど長く恩恵を受ける
                            let time_value = direct_value * (remaining / game.t as f64) * 0.3;

                            // 高V地のレベル上げ優先度を大幅に上げる
                            let high_v_bonus = if v > 70.0 {
                                v * 0.8
                            } else if v > 40.0 {
                                v * 0.3
                            } else {
                                0.0
                            };

                            // 敵隣接の場合のlevel upは緊急度が高い
                            let urgency = defense_urgency(game, state, x, y);

                            score +=
                                direct_value + defense_bonus + time_value + high_v_bonus + urgency;

                            // phase に応じた重み調整
                            if phase < 0.25 {
                                // 序盤: 拡張も重要だが高V地のlevel upも重視
                                score *= 1.1;
                            } else if phase < 0.6 {
                                // 中盤: レベル上げ最優先期
                                score *= 1.4;
                            } else {
                                // 終盤: ラストスパートでレベル最大化
                                score *= 1.3;
                            }
                        } else {
                            // max level: 移動の無駄（ただしテレポートの中継点としてはあり）
                            if (x, y) == state.pos[0] {
                                score -= v * 0.3; // その場に留まるのはペナルティ
                            } else {
                                score -= v * 0.05;
                            }
                        }
                    } else {
                        // === 敵領土: 攻撃 ===
                        let opp = owner as usize;
                        let opp_score = scores[opp] as f64;
                        let threat = (opp_score / sa).clamp(0.0, 1.0);

                        if level == 1 {
                            // 即座に奪取可能！
                            // 奪取すると: S0 += V, SA -= V (もし最大プレイヤーの場合)
                            let capture_value = v * (1.5 + 1.0 * threat);

                            // リーダーの領土を奪うとSAが下がる
                            if opp_score >= sa * 0.9 {
                                let sa_reduction_bonus = v * 1.0;
                                score += sa_reduction_bonus;
                            }

                            score += capture_value;
                        } else {
                            // level≥2: 攻撃しても弾かれる → 基本的に無価値
                            // ただし level 2 なら2手で奪取可能
                            if level == 2 && remaining > 20.0 {
                                score += v * 0.15 * threat; // 将来的な投資としてわずかな価値
                            } else {
                                score -= v * 0.2; // 弾かれるだけなので避ける
                            }
                        }
                    }

                    // === 衝突リスク補正 ===
                    let p_any = 1.0 - (-conflict_map[x][y]).exp();
                    let risk = p_any * v;

                    if owner == -1 {
                        // 中立セルでの衝突: 取得失敗 → 大きなペナルティ
                        score -= 0.6 * risk;
                    } else if owner == 0 && level < game.u {
                        // 自領土レベル上げでの衝突: 問題なし（自分が居座るだけ）
                        // 衝突しても自分はowner → 残れる。ただし相手もその場に来る
                        score -= 0.05 * risk; // 微小なリスクのみ
                    } else if owner > 0 && level == 1 {
                        // 敵L1攻撃での衝突: 衝突すると弾かれる可能性
                        score -= 0.4 * risk;
                    }

                    score
                }

                fn one_step_evaluate(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    mv: (usize, usize),
                ) -> f64 {
                    let ai_moves = predict_ai_moves(game, state, models);
                    let mut all_moves = Vec::with_capacity(game.m);
                    all_moves.push(mv);
                    all_moves.extend_from_slice(&ai_moves);
                    let next = simulate_turn(game, state, &all_moves);

                    let scores = calc_scores(game, &next);
                    let s0 = scores[0].max(1) as f64;
                    let sa = scores.iter().skip(1).copied().max().unwrap_or(1).max(1) as f64;

                    // 基本スコア: log ratio
                    let ratio_score = 1e5 * (1.0 + s0 / sa).log2();

                    // 防御完了度ボーナス: level≥2 の自領土の割合
                    let mut total_own_v = 0.0_f64;
                    let mut defended_v = 0.0_f64;
                    let mut level_sum = 0.0_f64;
                    for x in 0..game.n {
                        for y in 0..game.n {
                            if next.owner[x][y] == 0 {
                                let v = game.v[x][y] as f64;
                                total_own_v += v;
                                level_sum += v * next.level[x][y] as f64;
                                if next.level[x][y] >= 2 {
                                    defended_v += v;
                                }
                            }
                        }
                    }

                    let defense_ratio = if total_own_v > 0.0 {
                        defended_v / total_own_v
                    } else {
                        0.0
                    };

                    // 防御完了度が高いほどボーナス
                    let defense_bonus = defense_ratio * 500.0;

                    // レベル密度ボーナス: 高レベルの方が良い
                    let level_density = if total_own_v > 0.0 {
                        level_sum / total_own_v
                    } else {
                        0.0
                    };
                    let level_bonus = level_density * 200.0;

                    ratio_score + defense_bonus + level_bonus
                }

                pub(super) fn choose_move_x89_level_fortress(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    let candidates = get_candidates(game, state, 0);
                    if candidates.is_empty() {
                        return state.pos[0];
                    }
                    if candidates.len() == 1 {
                        return candidates[0];
                    }

                    let scores = calc_scores(game, state);
                    let conflict_map = estimate_conflict_map(game, state, models);

                    // Phase 1: ヒューリスティックスコアで候補をランキング
                    let mut scored: Vec<((usize, usize), f64)> = candidates
                        .iter()
                        .map(|&mv| {
                            let s = score_move(game, state, models, mv, &scores, &conflict_map);
                            (mv, s)
                        })
                        .collect();
                    scored
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

                    // Phase 2: 上位候補のみ1手先読みで精密評価
                    let eval_cap = scored.len().min(8);
                    let mut best_mv = scored[0].0;
                    let mut best_total = f64::NEG_INFINITY;

                    for &(mv, heuristic_score) in scored.iter().take(eval_cap) {
                        let lookahead = one_step_evaluate(game, state, models, mv);
                        let total = lookahead + 0.05 * heuristic_score;
                        if total > best_total {
                            best_total = total;
                            best_mv = mv;
                        }
                    }

                    best_mv
                }
            }
            mod x174_chokudai {
                // x174: Chokudai Search（完全新規設計）
                //
                // 既存モジュール(x04/x01/x06)を一切使わない。
                // 目標: mean ≥ 238,440 (+50% over x58)
                //
                // 設計思想:
                // 1. 時間適応型反復深化: 固定パラメータではなく、時間が許す限り探索を深化
                // 2. ターゲット付きBeam: 高V未占領セルへの経路を探索（方向性確保）
                // 3. 軽量AI予測: M-1人分のBFS+特徴量計算を最小限に
                // 4. Upgrade重視評価: Level倍率(最大5x)をスコアに直結

                use crate::__cargo_equip::crates::ahc061_solver::{
                    in_bounds, AiModel, Game, State,
                };
                use std::cmp::Ordering;
                use std::collections::BinaryHeap;
                use std::time::Instant;

                const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

                // ==================== 軽量ユーティリティ ====================

                fn reachable(game: &Game, state: &State, player: usize) -> Vec<(usize, usize)> {
                    let mut visited = vec![vec![false; game.n]; game.n];
                    let mut queue = std::collections::VecDeque::new();
                    let mut result = Vec::new();
                    let start = state.pos[player];
                    queue.push_back(start);
                    visited[start.0][start.1] = true;

                    while let Some((x, y)) = queue.pop_front() {
                        // Check not occupied by other player
                        let occupied = state
                            .pos
                            .iter()
                            .enumerate()
                            .any(|(i, &(px, py))| i != player && px == x && py == y);
                        if !occupied {
                            result.push((x, y));
                        }
                        if state.owner[x][y] == player as i32 {
                            for (dx, dy) in DIRS {
                                let nx = x as isize + dx;
                                let ny = y as isize + dy;
                                if in_bounds(game.n, nx, ny) {
                                    let ux = nx as usize;
                                    let uy = ny as usize;
                                    if !visited[ux][uy] {
                                        visited[ux][uy] = true;
                                        queue.push_back((ux, uy));
                                    }
                                }
                            }
                        }
                    }
                    result
                }

                fn predict_ai_top1(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> Vec<(usize, usize)> {
                    let mut moves = Vec::with_capacity(game.m.saturating_sub(1));
                    for ai_idx in 0..game.m.saturating_sub(1) {
                        let player = ai_idx + 1;
                        let cands = reachable(game, state, player);
                        if cands.is_empty() {
                            moves.push(state.pos[player]);
                            continue;
                        }
                        let w = &models[ai_idx].w;
                        let mut best = cands[0];
                        let mut best_s = f64::NEG_INFINITY;
                        for &(x, y) in &cands {
                            let v = game.v[x][y] as f64;
                            let owner = state.owner[x][y];
                            let level = state.level[x][y];
                            let feat = if owner == -1 {
                                [v, 0.0, 0.0, 0.0]
                            } else if owner == player as i32 {
                                if level < game.u {
                                    [0.0, v, 0.0, 0.0]
                                } else {
                                    [0.0; 4]
                                }
                            } else if level == 1 {
                                [0.0, 0.0, v, 0.0]
                            } else {
                                [0.0, 0.0, 0.0, v]
                            };
                            let s =
                                w[0] * feat[0] + w[1] * feat[1] + w[2] * feat[2] + w[3] * feat[3];
                            if s > best_s {
                                best_s = s;
                                best = (x, y);
                            }
                        }
                        moves.push(best);
                    }
                    moves
                }

                fn sim_turn(game: &Game, state: &State, all_moves: &[(usize, usize)]) -> State {
                    crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                        game, state, all_moves,
                    )
                }

                fn score_ratio(game: &Game, state: &State) -> (i64, i64) {
                    let mut scores = vec![0_i64; game.m];
                    for x in 0..game.n {
                        for y in 0..game.n {
                            let o = state.owner[x][y];
                            if o >= 0 {
                                scores[o as usize] += game.v[x][y] * state.level[x][y] as i64;
                            }
                        }
                    }
                    let s0 = scores[0];
                    let sa = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    (s0, sa)
                }

                // ==================== 評価関数 ====================

                fn evaluate(game: &Game, state: &State) -> f64 {
                    let (s0, sa) = score_ratio(game, state);
                    let s0 = s0.max(1) as f64;
                    let sa = sa as f64;

                    // Primary: log2-based score (the actual objective)
                    let base = 1e5 * (1.0 + s0 / sa).log2();

                    let remaining = (game.t as f64 - state.turn as f64).max(0.0) / game.t as f64;

                    // Territory quality (single-pass)
                    let mut own_total_v = 0.0_f64;
                    let mut upgrade_room = 0.0_f64; // V * (U - level) / U for own cells
                    let mut frontier_v = 0.0_f64; // V of cells adjacent to our territory
                    let mut defended_v = 0.0_f64; // V of own cells with level >= 2

                    for x in 0..game.n {
                        for y in 0..game.n {
                            let v = game.v[x][y] as f64;
                            if state.owner[x][y] == 0 {
                                own_total_v += v * state.level[x][y] as f64;
                                if state.level[x][y] < game.u {
                                    upgrade_room +=
                                        v * (game.u - state.level[x][y]) as f64 / game.u as f64;
                                }
                                if state.level[x][y] >= 2 {
                                    defended_v += v;
                                }
                                // Frontier: check adjacent non-owned
                                for (dx, dy) in DIRS {
                                    let nx = x as isize + dx;
                                    let ny = y as isize + dy;
                                    if in_bounds(game.n, nx, ny) {
                                        let ux = nx as usize;
                                        let uy = ny as usize;
                                        if state.owner[ux][uy] == -1 {
                                            frontier_v += game.v[ux][uy] as f64;
                                        } else if state.owner[ux][uy] > 0
                                            && state.level[ux][uy] == 1
                                        {
                                            frontier_v += 0.6 * game.v[ux][uy] as f64;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Weights: frontier matters more early, upgrade more late
                    base + remaining * 0.020 * frontier_v
                        + (0.003 + 0.005 * (1.0 - remaining)) * upgrade_room
                        + 0.010 * defended_v
                }

                // ==================== Move Scoring ====================

                fn score_move(game: &Game, state: &State, mv: (usize, usize), phase: f64) -> f64 {
                    let (x, y) = mv;
                    let v = game.v[x][y] as f64;
                    let owner = state.owner[x][y];
                    let level = state.level[x][y];

                    let base = if owner == -1 {
                        // Capture: high value early, moderate later
                        v * (1.2 + 0.6 * (1.0 - phase))
                    } else if owner == 0 {
                        if level < game.u {
                            // Upgrade: increasingly important over time
                            // Level 1→2 gives 100% value increase, 4→5 gives 25%
                            let upgrade_value = v * (1.0 / level as f64);
                            upgrade_value * (0.8 + 0.8 * phase)
                        } else {
                            -0.2 * v // Max level: waste of turn
                        }
                    } else {
                        // Attack enemy
                        if level == 1 {
                            // Can capture in one turn
                            v * (0.9 + 0.5 * phase)
                        } else {
                            // Expensive: need level turns to break through
                            v * 0.15 / level as f64
                        }
                    };

                    // Adjacency bonus: value of expansion opportunities from new position
                    let next_pos = if owner > 0 && level >= 2 {
                        state.pos[0]
                    } else {
                        (x, y)
                    };
                    let mut adj_bonus = 0.0;
                    for (dx, dy) in DIRS {
                        let nx = next_pos.0 as isize + dx;
                        let ny = next_pos.1 as isize + dy;
                        if in_bounds(game.n, nx, ny) {
                            let ux = nx as usize;
                            let uy = ny as usize;
                            let nv = game.v[ux][uy] as f64;
                            if state.owner[ux][uy] != 0 {
                                adj_bonus += 0.06 * nv;
                            }
                        }
                    }

                    base + adj_bonus + v * 1e-7 // tiebreaker
                }

                // ==================== Chokudai Search ====================

                #[derive(Clone)]
                struct SearchNode {
                    first_move: (usize, usize),
                    state: State,
                    score: f64,
                    depth: usize,
                }

                impl PartialEq for SearchNode {
                    fn eq(&self, other: &Self) -> bool {
                        self.score == other.score
                    }
                }
                impl Eq for SearchNode {}
                impl PartialOrd for SearchNode {
                    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                        self.score.partial_cmp(&other.score)
                    }
                }
                impl Ord for SearchNode {
                    fn cmp(&self, other: &Self) -> Ordering {
                        self.partial_cmp(other).unwrap_or(Ordering::Equal)
                    }
                }

                pub(super) fn choose_move_x174_chokudai(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    let start = Instant::now();

                    let candidates = reachable(game, state, 0);
                    if candidates.len() <= 1 {
                        return candidates.first().copied().unwrap_or(state.pos[0]);
                    }

                    let phase = state.turn as f64 / game.t as f64;
                    let remaining_turns = game.t - state.turn;

                    // Time budget: adaptive per turn
                    // Total 1800ms for 100 turns, weighted toward early turns
                    let budget_ms: u64 = if state.turn < 20 {
                        22
                    } else if state.turn < 50 {
                        18
                    } else if state.turn < 80 {
                        15
                    } else {
                        10
                    };

                    // Max search depth based on remaining turns
                    let max_depth = remaining_turns.min(12);

                    // Beam width per depth level (narrower at deeper levels)
                    let base_width = match game.m {
                        2 => 20,
                        3 => 16,
                        4 => 14,
                        5 => 12,
                        6 => 10,
                        _ => 8,
                    };

                    // Branch factor (top-K moves to consider per node)
                    let branch = match game.m {
                        2 => 5,
                        3..=4 => 4,
                        5..=6 => 3,
                        _ => 3,
                    };

                    let base_eval = evaluate(game, state);

                    // Initialize depth-0 beam: all candidate first moves
                    let mut beams: Vec<BinaryHeap<SearchNode>> =
                        vec![BinaryHeap::new(); max_depth + 1];

                    // Score and insert initial candidates
                    for &mv in &candidates {
                        let ai_moves = predict_ai_top1(game, state, models);
                        let mut all_moves = vec![mv];
                        all_moves.extend_from_slice(&ai_moves);
                        let ns = sim_turn(game, state, &all_moves);
                        let eval = evaluate(game, &ns);

                        beams[1].push(SearchNode {
                            first_move: mv,
                            state: ns,
                            score: eval - base_eval,
                            depth: 1,
                        });
                    }

                    // Track best first move seen so far
                    let mut best_move = candidates[0];
                    let mut best_score = f64::NEG_INFINITY;
                    if let Some(top) = beams[1].peek() {
                        best_move = top.first_move;
                        best_score = top.score;
                    }

                    // Chokudai search: iterate over depths, extending beams
                    // Continue until time runs out
                    let mut iteration = 0;
                    'outer: loop {
                        iteration += 1;
                        let elapsed = start.elapsed().as_millis() as u64;
                        if elapsed >= budget_ms {
                            break;
                        }

                        // Try to extend each depth level
                        for d in 1..max_depth {
                            if start.elapsed().as_millis() as u64 >= budget_ms {
                                break 'outer;
                            }

                            // Pop the best node at depth d
                            let node = match beams[d].pop() {
                                Some(n) => n,
                                None => continue,
                            };

                            // Width limit: only process if beam at d+1 isn't too large
                            let width_at_next =
                                (base_width as f64 * 0.85_f64.powi(d as i32)) as usize;
                            let width_at_next = width_at_next.max(4);

                            if node.state.turn >= game.t {
                                // Terminal: update best
                                if node.score > best_score {
                                    best_score = node.score;
                                    best_move = node.first_move;
                                }
                                beams[d].push(node); // Put back
                                continue;
                            }

                            let node_cands = reachable(game, &node.state, 0);
                            if node_cands.is_empty() {
                                beams[d].push(node); // Put back
                                continue;
                            }

                            let node_phase = node.state.turn as f64 / game.t as f64;
                            let discount = 0.93_f64.powi(d as i32);

                            // Score and sort candidates
                            let mut scored: Vec<((usize, usize), f64)> = node_cands
                                .iter()
                                .map(|&mv| (mv, score_move(game, &node.state, mv, node_phase)))
                                .collect();
                            scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));

                            // AI prediction for this state
                            let ai_moves = predict_ai_top1(game, &node.state, models);

                            let node_eval = evaluate(game, &node.state);

                            // Extend top-branch moves
                            for &(mv, _) in scored.iter().take(branch) {
                                let mut all_moves = vec![mv];
                                all_moves.extend_from_slice(&ai_moves);
                                let ns = sim_turn(game, &node.state, &all_moves);
                                let eval = evaluate(game, &ns);
                                let inc = eval - node_eval;

                                let child = SearchNode {
                                    first_move: node.first_move,
                                    state: ns,
                                    score: node.score + discount * inc,
                                    depth: d + 1,
                                };

                                // Update global best
                                if child.score > best_score {
                                    best_score = child.score;
                                    best_move = child.first_move;
                                }

                                // Insert into next depth's beam (with width limit)
                                if beams[d + 1].len() < width_at_next * 2 {
                                    beams[d + 1].push(child);
                                } else if let Some(worst) = beams[d + 1].peek() {
                                    // BinaryHeap is max-heap, we want to keep top-K
                                    // So we need a min-heap for pruning... just keep pushing for now
                                    beams[d + 1].push(child);
                                }
                            }

                            // Put node back (Chokudai style: can be re-expanded with different branches)
                            // Actually, in Chokudai we don't put back - we move on
                            // But we should track that we've expanded this node

                            // Prune beam at d+1 if too large
                            if beams[d + 1].len() > width_at_next * 3 {
                                let mut keep: Vec<SearchNode> = Vec::with_capacity(width_at_next);
                                while keep.len() < width_at_next {
                                    if let Some(n) = beams[d + 1].pop() {
                                        keep.push(n);
                                    } else {
                                        break;
                                    }
                                }
                                beams[d + 1].clear();
                                for n in keep {
                                    beams[d + 1].push(n);
                                }
                            }
                        }

                        // If no more nodes to expand, break
                        let total_nodes: usize = beams.iter().map(|b| b.len()).sum();
                        if total_nodes == 0 {
                            break;
                        }
                    }

                    best_move
                }
            }
            mod x305_full_horizon {
                // x305: Full-Horizon Greedy (FHG) v2
                //
                // Paradigm: For each candidate move, simulate the remaining game (15-30 turns)
                // with deterministic AI (full BFS) and greedy player0 policy.
                // Pick the candidate leading to the highest final absolute_score.
                //
                // v2 changes from v1:
                //   - AI uses full BFS (accurate) instead of neighbor-only (inaccurate)
                //   - Reduced default depth (20 vs 50) for time budget
                //   - Improved greedy policy with distance penalty and frontier awareness
                //   - Configurable depth via AHC_X305_DEPTH env var

                use crate::__cargo_equip::crates::ahc061_solver::{AiModel, Game, State};

                fn ai_best_move(
                    game: &Game,
                    state: &State,
                    player: usize,
                    model: &AiModel,
                ) -> (usize, usize) {
                    let cands = crate::__cargo_equip::crates::ahc061_solver::get_candidates(
                        game, state, player,
                    );
                    if cands.is_empty() {
                        return state.pos[player];
                    }
                    let mut best = cands[0];
                    let mut best_score = f64::NEG_INFINITY;
                    for &c in &cands {
                        let feat = crate::__cargo_equip::crates::ahc061_solver::ai_features(
                            game, state, player, c,
                        );
                        let s = crate::__cargo_equip::crates::ahc061_solver::dot(&model.w, &feat);
                        if s > best_score {
                            best_score = s;
                            best = c;
                        }
                    }
                    best
                }

                fn greedy_p0_move(game: &Game, state: &State) -> (usize, usize) {
                    let cands =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if cands.is_empty() {
                        return state.pos[0];
                    }
                    if cands.len() == 1 {
                        return cands[0];
                    }

                    let phase = state.turn as f64 / game.t as f64;
                    let remaining = game.t - state.turn;
                    let (px, py) = state.pos[0];

                    let mut best = cands[0];
                    let mut best_score = f64::NEG_INFINITY;

                    for &(x, y) in &cands {
                        let v = game.v[x][y] as f64;
                        let owner = state.owner[x][y];
                        let level = state.level[x][y];

                        // Distance from current position (Manhattan)
                        let dist = ((x as isize - px as isize).unsigned_abs()
                            + (y as isize - py as isize).unsigned_abs())
                            as f64;

                        let base = if remaining <= game.u && owner == 0 && level < game.u {
                            // Endgame: upgrade highest-V own cells
                            v * 3.0
                        } else if owner == -1 {
                            // Empty: capture (stronger early)
                            v * (1.4 - 0.4 * phase)
                        } else if owner == 0 {
                            if level < game.u {
                                // Own territory: upgrade (stronger late)
                                v * (0.3 + 1.0 * phase)
                            } else {
                                // Max level: strong penalty (avoid oscillation)
                                -v * 0.5
                            }
                        } else if level == 1 {
                            // Enemy L1: cheap capture
                            v * (1.1 - 0.2 * phase)
                        } else {
                            // Enemy L2+: expensive
                            v * 0.1 / level as f64
                        };

                        // Distance penalty: prefer nearby cells
                        let dist_penalty = dist * 50.0;
                        let s = base - dist_penalty;

                        if s > best_score {
                            best_score = s;
                            best = (x, y);
                        }
                    }
                    best
                }

                fn quick_score(game: &Game, state: &State, x: usize, y: usize) -> f64 {
                    let v = game.v[x][y] as f64;
                    let owner = state.owner[x][y];
                    let level = state.level[x][y];
                    let phase = state.turn as f64 / game.t as f64;
                    let remaining = game.t - state.turn;
                    let (px, py) = state.pos[0];
                    let dist = ((x as isize - px as isize).unsigned_abs()
                        + (y as isize - py as isize).unsigned_abs())
                        as f64;

                    let base = if remaining <= game.u && owner == 0 && level < game.u {
                        v * 3.0
                    } else if owner == -1 {
                        v * (1.4 - 0.4 * phase)
                    } else if owner == 0 {
                        if level < game.u {
                            v * (0.3 + 1.0 * phase)
                        } else {
                            -v * 0.5
                        }
                    } else if level == 1 {
                        v * (1.1 - 0.2 * phase)
                    } else {
                        v * 0.1 / level as f64
                    };

                    base - dist * 50.0
                }

                fn simulate_remaining(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    max_depth: usize,
                ) -> f64 {
                    let mut st = state.clone();
                    let depth = max_depth.min(game.t.saturating_sub(st.turn));

                    for _ in 0..depth {
                        let mut moves = Vec::with_capacity(game.m);
                        // Player0: full BFS greedy
                        moves.push(greedy_p0_move(game, &st));
                        // AI: full BFS deterministic prediction
                        for ai_idx in 0..game.m.saturating_sub(1) {
                            moves.push(ai_best_move(game, &st, ai_idx + 1, &models[ai_idx]));
                        }
                        st = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, &st, &moves,
                        );
                    }
                    crate::__cargo_equip::crates::ahc061_solver::absolute_score(game, &st)
                }

                pub(in crate::__cargo_equip::crates::ahc061_solver) fn choose_move_x305_full_horizon(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    let candidates =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if candidates.len() <= 1 {
                        return candidates.first().copied().unwrap_or(state.pos[0]);
                    }

                    // Configurable depth (default 20)
                    let cfg_depth: usize = std::env::var("AHC_X305_DEPTH")
                        .ok()
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(20);
                    let remaining = game.t.saturating_sub(state.turn);
                    let max_depth = remaining.min(cfg_depth);

                    // Time budget
                    let budget = crate::__cargo_equip::crates::ahc061_solver::turn_budget_ms(
                        state.turn, game.t,
                    );
                    let start = std::time::Instant::now();

                    // Pre-filter to top-K using quick heuristic
                    let cfg_k: usize = std::env::var("AHC_X305_TOP_K")
                        .ok()
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(6);
                    let k = cfg_k.min(candidates.len());
                    let mut scored: Vec<_> = candidates
                        .iter()
                        .map(|&(x, y)| ((x, y), quick_score(game, state, x, y)))
                        .collect();
                    scored
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    scored.truncate(k);

                    // Predict AI moves once (BFS-based, accurate) for the first step
                    let ai_top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let ai_primary: Vec<(usize, usize)> = ai_top2.iter().map(|x| x.0).collect();

                    // Full-horizon evaluation for top-K candidates
                    let mut best = scored[0].0;
                    let mut best_fh_score = f64::NEG_INFINITY;

                    for &(mv, _) in &scored {
                        // Time guard
                        if budget > 0
                            && start.elapsed().as_millis() as u64 > budget.saturating_sub(2)
                        {
                            break;
                        }

                        // Step 1: apply candidate move + accurate AI prediction
                        let mut first_moves = Vec::with_capacity(game.m);
                        first_moves.push(mv);
                        first_moves.extend_from_slice(&ai_primary);
                        let next = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game,
                            state,
                            &first_moves,
                        );

                        // Step 2: forward simulation
                        let fh_score =
                            simulate_remaining(game, &next, models, max_depth.saturating_sub(1));

                        if fh_score > best_fh_score {
                            best_fh_score = fh_score;
                            best = mv;
                        }
                    }

                    best
                }
            }
            mod x332_expectimax {
                // x332: Expectimax Tree Search with Strategic Score
                //
                // Fundamentally different from beam search (x04):
                // - Beam: routes through cells, evaluates incremental strategic_score per step
                // - Expectimax: explores game tree, evaluates strategic_score at leaf states
                //
                // Key differences from x328 ratio greedy:
                // - Uses strategic_score (= absolute_score + frontier_potential) at leaves
                //   instead of pure absolute_score. This captures growth potential.
                // - Default depth 3 (vs x328's depth 2)
                // - Opponent moves re-predicted at each level (not cached from root)
                //
                // Hypothesis: beam search undervalues certain moves because it evaluates
                // incremental gains, missing compound effects. Expectimax evaluates the
                // full resulting state after multi-turn lookahead with opponent responses.

                use crate::__cargo_equip::crates::ahc061_solver::{
                    denial_score_v2, get_candidates, simulate_turn, strategic_score, AiModel, Game,
                    State,
                };
                use std::time::Instant;

                // Evaluation function selector: "strategic" (default) or "denial_v2"
                // denial_v2 penalizes leader's upgrade potential and rewards leader weakness,
                // making it better suited for M>=5 where reducing SA matters as much as raising S0.
                fn eval_fn(game: &Game, state: &State) -> f64 {
                    match std::env::var("AHC_X332_EVAL").ok().as_deref() {
                        Some("denial_v2") => denial_score_v2(game, state),
                        Some("ratio") => {
                            crate::__cargo_equip::crates::ahc061_solver::ratio_eval_score(
                                game, state, 0.05, 0.5, 0.1,
                            )
                        }
                        _ => strategic_score(game, state),
                    }
                }

                fn predict_ai_top1(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> Vec<(usize, usize)> {
                    let mut moves = Vec::with_capacity(game.m.saturating_sub(1));
                    for ai_idx in 0..game.m.saturating_sub(1) {
                        let player = ai_idx + 1;
                        let cands = get_candidates(game, state, player);
                        if cands.is_empty() {
                            moves.push(state.pos[player]);
                            continue;
                        }
                        let w = &models[ai_idx].w;
                        let mut best = cands[0];
                        let mut best_s = f64::NEG_INFINITY;
                        for &(x, y) in &cands {
                            let feat = crate::__cargo_equip::crates::ahc061_solver::ai_features(
                                game,
                                state,
                                player,
                                (x, y),
                            );
                            let s = crate::__cargo_equip::crates::ahc061_solver::dot(w, &feat);
                            if s > best_s {
                                best_s = s;
                                best = (x, y);
                            }
                        }
                        moves.push(best);
                    }
                    moves
                }

                fn env_u64(name: &str, default: u64, min: u64, max: u64) -> u64 {
                    if let Ok(v) = std::env::var(name) {
                        if let Ok(x) = v.parse::<u64>() {
                            return x.max(min).min(max);
                        }
                    }
                    default.max(min).min(max)
                }

                pub(super) fn choose_move_x332_expectimax(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    let candidates = get_candidates(game, state, 0);
                    if candidates.len() <= 1 {
                        return candidates.first().copied().unwrap_or(state.pos[0]);
                    }

                    // Per-(M,U) depth overrides: AHC_X332_DEPTH_M2_U3 > AHC_X332_DEPTH_M2 > AHC_X332_DEPTH
                    let default_depth = env_u64("AHC_X332_DEPTH", 3, 1, 6);
                    let m_depth =
                        env_u64(&format!("AHC_X332_DEPTH_M{}", game.m), default_depth, 1, 6);
                    let depth = env_u64(
                        &format!("AHC_X332_DEPTH_M{}_U{}", game.m, game.u),
                        m_depth,
                        1,
                        6,
                    ) as usize;
                    let branch_top = env_u64("AHC_X332_BRANCH_TOP", 8, 2, 16) as usize;
                    let branch_deep = env_u64("AHC_X332_BRANCH_DEEP", 4, 1, 8) as usize;

                    let turn_budget = crate::__cargo_equip::crates::ahc061_solver::turn_budget_ms(
                        state.turn, game.t,
                    );
                    let start = Instant::now();
                    let deadline_ms = if turn_budget > 0 {
                        turn_budget * 90 / 100
                    } else {
                        18
                    };

                    // Step 1: Quick 1-step evaluation to rank candidates
                    let ai_moves = predict_ai_top1(game, state, models);
                    let mut scored: Vec<((usize, usize), f64)> =
                        Vec::with_capacity(candidates.len());
                    for &mv in &candidates {
                        let mut all_moves = vec![mv];
                        all_moves.extend_from_slice(&ai_moves);
                        let ns = simulate_turn(game, state, &all_moves);
                        let score = eval_fn(game, &ns);
                        scored.push((mv, score));
                    }
                    scored
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

                    if depth <= 1 || state.turn + 1 >= game.t {
                        return scored[0].0;
                    }

                    // Step 2: Deep expectimax on top candidates
                    let mut best_mv = scored[0].0;
                    let mut best_score = f64::NEG_INFINITY;
                    let eval_count = branch_top.min(scored.len());

                    for &(mv, _) in scored.iter().take(eval_count) {
                        if start.elapsed().as_millis() as u64 >= deadline_ms {
                            break;
                        }
                        let mut all_moves = vec![mv];
                        all_moves.extend_from_slice(&ai_moves);
                        let ns = simulate_turn(game, state, &all_moves);

                        let deep_score = expectimax_eval(
                            game,
                            &ns,
                            models,
                            depth - 1,
                            branch_deep,
                            &start,
                            deadline_ms,
                        );
                        if deep_score > best_score {
                            best_score = deep_score;
                            best_mv = mv;
                        }
                    }

                    best_mv
                }

                fn expectimax_eval(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    remaining_depth: usize,
                    branch: usize,
                    start: &Instant,
                    deadline_ms: u64,
                ) -> f64 {
                    // Base cases
                    if remaining_depth == 0 || state.turn >= game.t {
                        return eval_fn(game, state);
                    }
                    if start.elapsed().as_millis() as u64 >= deadline_ms {
                        return eval_fn(game, state);
                    }

                    let candidates = get_candidates(game, state, 0);
                    if candidates.is_empty() {
                        return eval_fn(game, state);
                    }

                    // Re-predict opponent moves at this state (not cached from root)
                    let ai_moves = predict_ai_top1(game, state, models);

                    // Quick 1-step eval to rank candidates for pruning
                    let mut scored: Vec<((usize, usize), f64)> =
                        Vec::with_capacity(candidates.len());
                    for &mv in &candidates {
                        let mut all_moves = vec![mv];
                        all_moves.extend_from_slice(&ai_moves);
                        let ns = simulate_turn(game, state, &all_moves);
                        scored.push((mv, eval_fn(game, &ns)));
                    }
                    scored
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

                    // Narrow branching at deeper levels
                    let effective_branch = if remaining_depth <= 1 {
                        branch.min(3)
                    } else {
                        branch
                    };

                    let mut best = f64::NEG_INFINITY;
                    for &(mv, _) in scored.iter().take(effective_branch) {
                        if start.elapsed().as_millis() as u64 >= deadline_ms {
                            break;
                        }
                        let mut all_moves = vec![mv];
                        all_moves.extend_from_slice(&ai_moves);
                        let ns = simulate_turn(game, state, &all_moves);
                        let score = expectimax_eval(
                            game,
                            &ns,
                            models,
                            remaining_depth - 1,
                            branch.min(3), // narrow deeper
                            start,
                            deadline_ms,
                        );
                        if score > best {
                            best = score;
                        }
                    }

                    if best == f64::NEG_INFINITY {
                        eval_fn(game, state)
                    } else {
                        best
                    }
                }
            }

            use strategy_mode::choose_move;
            pub use strategy_mode::{strategy_from_env, StrategyMode};

            #[derive(Clone)]
            pub(in crate::__cargo_equip::crates::ahc061_solver) struct Game {
                n: usize,
                m: usize,
                t: usize,
                u: usize,
                v: Vec<Vec<i64>>,
            }

            #[derive(Clone)]
            pub(in crate::__cargo_equip::crates::ahc061_solver) struct State {
                pos: Vec<(usize, usize)>,
                owner: Vec<Vec<i32>>,
                level: Vec<Vec<usize>>,
                turn: usize,
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) const FEAT_DIM: usize = 4;

            const FEAT_ANCHOR: [f64; FEAT_DIM] = [0.64, 0.64, 0.64, 0.64];
            const FEAT_CLAMP_LO: [f64; FEAT_DIM] = [0.10, 0.10, 0.10, 0.10];
            const FEAT_CLAMP_HI: [f64; FEAT_DIM] = [2.00, 2.00, 2.00, 2.00];

            #[derive(Clone)]
            pub(in crate::__cargo_equip::crates::ahc061_solver) struct AiModel {
                pub(in crate::__cargo_equip::crates::ahc061_solver) w: [f64; FEAT_DIM],
                pub(in crate::__cargo_equip::crates::ahc061_solver) eps_est: f64,
                seen: u32,
                mismatch: u32,
            }

            impl AiModel {
                fn new() -> Self {
                    Self {
                        w: FEAT_ANCHOR,
                        eps_est: 0.30,
                        seen: 0,
                        mismatch: 0,
                    }
                }
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) struct FastRng {
                state: u64,
            }

            impl FastRng {
                pub(in crate::__cargo_equip::crates::ahc061_solver) fn new(seed: u64) -> Self {
                    Self { state: seed | 1 }
                }

                pub(in crate::__cargo_equip::crates::ahc061_solver) fn next_u64(&mut self) -> u64 {
                    self.state ^= self.state << 7;
                    self.state ^= self.state >> 9;
                    self.state ^= self.state << 8;
                    self.state
                }

                pub(in crate::__cargo_equip::crates::ahc061_solver) fn next_f64(&mut self) -> f64 {
                    let x = self.next_u64() >> 11;
                    (x as f64) * (1.0 / ((1_u64 << 53) as f64))
                }
            }

            // ---- Time Management ----
            // Opt-in via AHC_TIME_LIMIT_MS env var.
            // When set, the game timer tracks total elapsed time and computes per-turn budgets.
            static GAME_TIMER: OnceLock<Instant> = OnceLock::new();

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn init_game_timer() {
                GAME_TIMER.get_or_init(Instant::now);
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn turn_budget_ms(
                turn: usize,
                total_turns: usize,
            ) -> u64 {
                let total_limit: u64 = match std::env::var("AHC_TIME_LIMIT_MS") {
                    Ok(v) => match v.parse() {
                        Ok(ms) => ms,
                        Err(_) => return 0,
                    },
                    Err(_) => return 0,
                };
                let start = match GAME_TIMER.get() {
                    Some(t) => t,
                    None => return 0,
                };
                let elapsed = start.elapsed().as_millis() as u64;
                let remaining_turns = (total_turns - turn).max(1) as u64;
                let remaining_time = total_limit.saturating_sub(elapsed);
                // Reserve 1ms per remaining turn for I/O overhead
                let usable = remaining_time.saturating_sub(remaining_turns);
                usable / remaining_turns
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn game_time_elapsed_ms() -> u64 {
                match GAME_TIMER.get() {
                    Some(t) => t.elapsed().as_millis() as u64,
                    None => 0,
                }
            }

            struct Scanner<R: BufRead> {
                reader: R,
                line: String,
                tokens: VecDeque<String>,
            }

            impl<R: BufRead> Scanner<R> {
                fn new(reader: R) -> Self {
                    Self {
                        reader,
                        line: String::new(),
                        tokens: VecDeque::new(),
                    }
                }

                fn next<T: std::str::FromStr>(&mut self) -> Option<T> {
                    loop {
                        if let Some(tok) = self.tokens.pop_front() {
                            if let Ok(v) = tok.parse::<T>() {
                                return Some(v);
                            }
                            return None;
                        }
                        self.line.clear();
                        let n = self.reader.read_line(&mut self.line).ok()?;
                        if n == 0 {
                            return None;
                        }
                        let s = self.line.trim();
                        if s.is_empty() || s.starts_with('#') {
                            continue;
                        }
                        self.tokens = s
                            .split_whitespace()
                            .map(|x| x.to_owned())
                            .collect::<VecDeque<_>>();
                    }
                }
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn in_bounds(
                n: usize,
                x: isize,
                y: isize,
            ) -> bool {
                x >= 0 && y >= 0 && (x as usize) < n && (y as usize) < n
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn occupied_by_other(
                state: &State,
                player: usize,
                x: usize,
                y: usize,
            ) -> bool {
                for (i, &(px, py)) in state.pos.iter().enumerate() {
                    if i != player && px == x && py == y {
                        return true;
                    }
                }
                false
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn bfs_distance_map(
                game: &Game,
                state: &State,
                player: usize,
            ) -> Vec<Vec<usize>> {
                let mut dist = vec![vec![usize::MAX; game.n]; game.n];
                let mut queue = VecDeque::new();
                let start = state.pos[player];
                dist[start.0][start.1] = 0;
                queue.push_back(start);

                const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                while let Some((x, y)) = queue.pop_front() {
                    let d = dist[x][y];
                    if state.owner[x][y] == player as i32 {
                        for (dx, dy) in DIRS {
                            let nx = x as isize + dx;
                            let ny = y as isize + dy;
                            if in_bounds(game.n, nx, ny) {
                                let ux = nx as usize;
                                let uy = ny as usize;
                                if dist[ux][uy] > d + 1 {
                                    dist[ux][uy] = d + 1;
                                    queue.push_back((ux, uy));
                                }
                            }
                        }
                    }
                }
                dist
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn get_candidates(
                game: &Game,
                state: &State,
                player: usize,
            ) -> Vec<(usize, usize)> {
                let mut reachable = Vec::new();
                let mut visited = vec![vec![false; game.n]; game.n];
                let mut q = VecDeque::new();

                let start = state.pos[player];
                q.push_back(start);
                visited[start.0][start.1] = true;

                // Direction vectors for BFS (from tools/src/lib.rs)
                const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

                while let Some((x, y)) = q.pop_front() {
                    if !occupied_by_other(state, player, x, y) {
                        reachable.push((x, y));
                    }

                    if state.owner[x][y] == player as i32 {
                        for (dx, dy) in DIRS {
                            let nx = x as isize + dx;
                            let ny = y as isize + dy;
                            if in_bounds(game.n, nx, ny) {
                                let ux = nx as usize;
                                let uy = ny as usize;
                                if !visited[ux][uy] {
                                    visited[ux][uy] = true;
                                    q.push_back((ux, uy));
                                }
                            }
                        }
                    }
                }
                reachable
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn calc_scores(
                game: &Game,
                state: &State,
            ) -> Vec<i64> {
                let mut scores = vec![0_i64; game.m];
                for i in 0..game.n {
                    for j in 0..game.n {
                        let owner = state.owner[i][j];
                        if owner >= 0 {
                            scores[owner as usize] += game.v[i][j] * state.level[i][j] as i64;
                        }
                    }
                }
                scores
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn ai_features(
                game: &Game,
                state: &State,
                player: usize,
                target: (usize, usize),
            ) -> [f64; FEAT_DIM] {
                let (x, y) = target;
                let owner = state.owner[x][y];
                let level = state.level[x][y];
                let value = game.v[x][y] as f64;

                if owner == -1 {
                    [value, 0.0, 0.0, 0.0]
                } else if owner == player as i32 {
                    if level < game.u {
                        [0.0, value, 0.0, 0.0]
                    } else {
                        [0.0, 0.0, 0.0, 0.0]
                    }
                } else if level == 1 {
                    [0.0, 0.0, value, 0.0]
                } else {
                    [0.0, 0.0, 0.0, value]
                }
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn dot(
                w: &[f64; FEAT_DIM],
                x: &[f64; FEAT_DIM],
            ) -> f64 {
                let mut s = 0.0;
                for i in 0..FEAT_DIM {
                    s += w[i] * x[i];
                }
                s
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn predict_ai_distribution(
                game: &Game,
                state: &State,
                player: usize,
                model: &AiModel,
                candidates: &[(usize, usize)],
            ) -> Vec<f64> {
                if candidates.is_empty() {
                    return Vec::new();
                }

                let mut est_scores = vec![0.0_f64; candidates.len()];
                for (i, &cand) in candidates.iter().enumerate() {
                    let feat = ai_features(game, state, player, cand);
                    est_scores[i] = dot(&model.w, &feat);
                }

                // Softmax temperature: 0.0 = original winner-take-all, >0 = softmax
                let temp: f64 = std::env::var("AHC_AI_SOFTMAX_TEMP")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0.0);

                if temp > 0.0 {
                    // Softmax distribution with epsilon floor
                    let max_s = est_scores.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                    let exp_scores: Vec<f64> = est_scores
                        .iter()
                        .map(|&s| ((s - max_s) / temp).exp())
                        .collect();
                    let sum: f64 = exp_scores.iter().sum();
                    let eps = model.eps_est.clamp(0.05, 0.60);
                    let floor = eps / candidates.len() as f64;
                    let mut probs: Vec<f64> = exp_scores
                        .iter()
                        .map(|&e| floor + (1.0 - eps) * e / sum)
                        .collect();
                    let total: f64 = probs.iter().sum();
                    if total > 0.0 {
                        for p in &mut probs {
                            *p /= total;
                        }
                    }
                    probs
                } else {
                    // Original winner-take-all with epsilon
                    let max_score = est_scores.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                    let tol = 1e-9 * max_score.abs().max(1.0);
                    let best_idx: Vec<usize> = (0..candidates.len())
                        .filter(|&i| est_scores[i] >= max_score - tol)
                        .collect();

                    let eps = model.eps_est.clamp(0.05, 0.60);
                    let base = eps / candidates.len() as f64;
                    let mut probs = vec![base; candidates.len()];
                    let rem = 1.0 - eps;
                    let share = if best_idx.is_empty() {
                        rem / candidates.len() as f64
                    } else {
                        rem / best_idx.len() as f64
                    };

                    if best_idx.is_empty() {
                        for p in &mut probs {
                            *p += share;
                        }
                    } else {
                        for &i in &best_idx {
                            probs[i] += share;
                        }
                    }
                    probs
                }
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn blended_ai_probs(
                game: &Game,
                state: &State,
                player: usize,
                model: &AiModel,
                candidates: &[(usize, usize)],
            ) -> Vec<f64> {
                if candidates.is_empty() {
                    return Vec::new();
                }
                let model_probs = predict_ai_distribution(game, state, player, model, candidates);
                let uniform_prob = 1.0 / candidates.len() as f64;
                let turns_ratio = (state.turn as f64 / game.t as f64).clamp(0.0, 1.0);
                let seen = model.seen as f64;
                let confidence = (seen / (seen + 10.0)) * (1.0 - model.eps_est).clamp(0.15, 0.95);
                let alpha = (0.05 + 0.90 * turns_ratio * confidence).clamp(0.05, 0.95);
                model_probs
                    .iter()
                    .map(|&p| alpha * p + (1.0 - alpha) * uniform_prob)
                    .collect()
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn estimate_conflict_map(
                game: &Game,
                state: &State,
                models: &[AiModel],
            ) -> Vec<Vec<f64>> {
                let mut map = vec![vec![0.0_f64; game.n]; game.n];
                for ai_idx in 0..(game.m.saturating_sub(1)) {
                    let player = ai_idx + 1;
                    let cands = get_candidates(game, state, player);
                    if cands.is_empty() {
                        continue;
                    }
                    let probs = blended_ai_probs(game, state, player, &models[ai_idx], &cands);
                    for (i, &(x, y)) in cands.iter().enumerate() {
                        map[x][y] += probs[i];
                    }
                }
                map
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn simulate_turn(
                game: &Game,
                state: &State,
                moves: &[(usize, usize)],
            ) -> State {
                let mut next = state.clone();
                let temp_pos = moves.to_vec();
                let mut next_pos = state.pos.clone();
                let mut move_counts = HashMap::<(usize, usize), usize>::new();
                for &mv in moves {
                    *move_counts.entry(mv).or_insert(0) += 1;
                }

                let mut collected = vec![false; game.m];
                for i in 0..game.m {
                    let target = if i < temp_pos.len() {
                        temp_pos[i]
                    } else {
                        state.pos[i]
                    };
                    let collision_count = move_counts.get(&target).copied().unwrap_or(0);
                    if collision_count >= 2 {
                        let owner = next.owner[target.0][target.1];
                        if i as i32 != owner {
                            collected[i] = true;
                        }
                    }
                }

                for i in 0..game.m {
                    if collected[i] {
                        continue;
                    }
                    let (x, y) = if i < temp_pos.len() {
                        temp_pos[i]
                    } else {
                        state.pos[i]
                    };
                    let owner = next.owner[x][y];
                    if owner == -1 {
                        next.owner[x][y] = i as i32;
                        next.level[x][y] = 1;
                    } else if owner == i as i32 {
                        if next.level[x][y] < game.u {
                            next.level[x][y] += 1;
                        }
                    } else {
                        next.level[x][y] -= 1;
                        if next.level[x][y] == 0 {
                            next.owner[x][y] = i as i32;
                            next.level[x][y] = 1;
                        } else {
                            collected[i] = true;
                        }
                    }
                    next_pos[i] = (x, y);
                }

                for i in 0..game.m {
                    if collected[i] {
                        next_pos[i] = state.pos[i];
                    }
                }
                next.pos = next_pos;
                next
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn absolute_score(
                game: &Game,
                state: &State,
            ) -> f64 {
                let scores = calc_scores(game, state);
                let sa = scores.iter().skip(1).copied().max().unwrap_or(1).max(1) as f64;
                let ratio = scores[0] as f64 / sa;
                1e5 * (1.0 + ratio).log2()
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn zone_absolute_score(
                game: &Game,
                state: &State,
            ) -> f64 {
                let base = absolute_score(game, state);
                let zone_w: f64 = std::env::var("AHC_ZONE_SCORE_W")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0.0);
                if zone_w <= 0.0 {
                    return base;
                }
                let zone = compute_value_zone(game);
                let mut zone_val = 0.0_f64;
                for x in 0..game.n {
                    for y in 0..game.n {
                        if zone[x][y] && state.owner[x][y] == 0 {
                            zone_val += game.v[x][y] as f64 * state.level[x][y] as f64;
                        }
                    }
                }
                base + zone_w * zone_val
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn compute_value_zone(
                game: &Game,
            ) -> Vec<Vec<bool>> {
                let zone_size: usize = std::env::var("AHC_ZONE_SIZE")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(4);
                let n = game.n;
                let k = zone_size.min(n);

                // Find the KxK window with maximum total value
                let mut best_sum = 0_i64;
                let mut best_x = 0;
                let mut best_y = 0;
                for x in 0..=(n - k) {
                    for y in 0..=(n - k) {
                        let mut s = 0_i64;
                        for dx in 0..k {
                            for dy in 0..k {
                                s += game.v[x + dx][y + dy];
                            }
                        }
                        if s > best_sum {
                            best_sum = s;
                            best_x = x;
                            best_y = y;
                        }
                    }
                }

                let mut zone = vec![vec![false; n]; n];
                for dx in 0..k {
                    for dy in 0..k {
                        zone[best_x + dx][best_y + dy] = true;
                    }
                }
                zone
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn compute_adaptive_zone(
                game: &Game,
                state: &State,
            ) -> Vec<Vec<bool>> {
                let zone_size: usize = std::env::var("AHC_ZONE_SIZE")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(4);
                let opp_penalty: f64 = std::env::var("AHC_ZONE_OPP_PENALTY")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0.0);
                let n = game.n;
                let k = zone_size.min(n);

                let mut best_score = f64::NEG_INFINITY;
                let mut best_x = 0;
                let mut best_y = 0;
                for x in 0..=(n - k) {
                    for y in 0..=(n - k) {
                        let mut val_sum = 0.0_f64;
                        let mut opp_count = 0_usize;
                        for dx in 0..k {
                            for dy in 0..k {
                                let cx = x + dx;
                                let cy = y + dy;
                                val_sum += game.v[cx][cy] as f64;
                                if state.owner[cx][cy] > 0 {
                                    opp_count += 1;
                                    // Higher-level opponent cells are harder to take
                                    if state.level[cx][cy] >= 2 {
                                        opp_count += 1;
                                    }
                                }
                            }
                        }
                        let score = val_sum - opp_penalty * opp_count as f64;
                        if score > best_score {
                            best_score = score;
                            best_x = x;
                            best_y = y;
                        }
                    }
                }

                let mut zone = vec![vec![false; n]; n];
                for dx in 0..k {
                    for dy in 0..k {
                        zone[best_x + dx][best_y + dy] = true;
                    }
                }
                zone
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn compute_dual_zone(
                game: &Game,
            ) -> Vec<Vec<bool>> {
                let zone_size: usize = std::env::var("AHC_ZONE_SIZE")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(4);
                let n = game.n;
                let k = zone_size.min(n);
                let max_overlap = k * k / 2;

                // Collect all windows with scores
                let mut windows: Vec<(usize, usize, i64)> = Vec::new();
                for x in 0..=(n - k) {
                    for y in 0..=(n - k) {
                        let mut s = 0_i64;
                        for dx in 0..k {
                            for dy in 0..k {
                                s += game.v[x + dx][y + dy];
                            }
                        }
                        windows.push((x, y, s));
                    }
                }
                windows.sort_by(|a, b| b.2.cmp(&a.2));

                // Best zone
                let (bx1, by1, _) = windows[0];

                // Find second-best non-overlapping zone
                let mut bx2 = bx1;
                let mut by2 = by1;
                let mut found_second = false;
                for &(x, y, _) in windows.iter().skip(1) {
                    // Count overlap with first zone
                    let ox_start = x.max(bx1);
                    let ox_end = (x + k).min(bx1 + k);
                    let oy_start = y.max(by1);
                    let oy_end = (y + k).min(by1 + k);
                    let overlap = if ox_start < ox_end && oy_start < oy_end {
                        (ox_end - ox_start) * (oy_end - oy_start)
                    } else {
                        0
                    };
                    if overlap <= max_overlap {
                        bx2 = x;
                        by2 = y;
                        found_second = true;
                        break;
                    }
                }

                let mut zone = vec![vec![false; n]; n];
                for dx in 0..k {
                    for dy in 0..k {
                        zone[bx1 + dx][by1 + dy] = true;
                        if found_second {
                            zone[bx2 + dx][by2 + dy] = true;
                        }
                    }
                }
                zone
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn compute_proximity_zone(
                game: &Game,
                state: &State,
            ) -> Vec<Vec<bool>> {
                let zone_size: usize = std::env::var("AHC_ZONE_SIZE")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(4);
                let prox_decay: f64 = std::env::var("AHC_ZONE_PROX_DECAY")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0.15);
                let n = game.n;
                let k = zone_size.min(n);

                let dist_map = bfs_distance_map(game, state, 0);

                let mut best_score = f64::NEG_INFINITY;
                let mut best_x = 0;
                let mut best_y = 0;
                for x in 0..=(n - k) {
                    for y in 0..=(n - k) {
                        let mut score = 0.0_f64;
                        for dx in 0..k {
                            for dy in 0..k {
                                let cx = x + dx;
                                let cy = y + dy;
                                let v = game.v[cx][cy] as f64;
                                let d = dist_map[cx][cy] as f64;
                                // Nearby cells get full value, distant ones decay
                                let weight = (-prox_decay * d).exp();
                                score += v * weight;
                            }
                        }
                        if score > best_score {
                            best_score = score;
                            best_x = x;
                            best_y = y;
                        }
                    }
                }

                let mut zone = vec![vec![false; n]; n];
                for dx in 0..k {
                    for dy in 0..k {
                        zone[best_x + dx][best_y + dy] = true;
                    }
                }
                zone
            }

            static VALUE_ZONE: OnceLock<Vec<Vec<bool>>> = OnceLock::new();

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn get_value_zone(
                game: &Game,
            ) -> &'static Vec<Vec<bool>> {
                VALUE_ZONE.get_or_init(|| compute_value_zone(game))
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn choose_predicted_ai_top2_moves(
                game: &Game,
                state: &State,
                models: &[AiModel],
            ) -> Vec<((usize, usize), (usize, usize), f64)> {
                let mut moves = Vec::with_capacity(game.m.saturating_sub(1));
                for ai_idx in 0..game.m.saturating_sub(1) {
                    let player = ai_idx + 1;
                    let cands = get_candidates(game, state, player);
                    if cands.is_empty() {
                        let cur = state.pos[player];
                        moves.push((cur, cur, 1.0));
                        continue;
                    }
                    let probs = blended_ai_probs(game, state, player, &models[ai_idx], &cands);
                    let mut order: Vec<usize> = (0..cands.len()).collect();
                    order.sort_by(|&a, &b| {
                        probs[b]
                            .partial_cmp(&probs[a])
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                    let i1 = order[0];
                    let i2 = if order.len() >= 2 { order[1] } else { order[0] };
                    let p1 = probs[i1];
                    let p2 = probs[i2];
                    let conf = if i1 == i2 {
                        1.0
                    } else {
                        p1 / (p1 + p2 + 1e-12)
                    };
                    moves.push((cands[i1], cands[i2], conf.clamp(0.5, 1.0)));
                }
                moves
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn uncertainty_risk(
                top2: &[((usize, usize), (usize, usize), f64)],
            ) -> f64 {
                if top2.is_empty() {
                    return 0.0;
                }
                let mut sum = 0.0;
                for (_, _, conf) in top2 {
                    sum += 1.0 - *conf;
                }
                (sum / top2.len() as f64).clamp(0.0, 0.5)
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn build_secondary_ai_moves(
                scores: &[i64],
                top2: &[((usize, usize), (usize, usize), f64)],
                switch_cap: usize,
            ) -> Vec<(usize, usize)> {
                let mut moves: Vec<(usize, usize)> = top2.iter().map(|x| x.0).collect();
                if top2.is_empty() {
                    return moves;
                }
                let s0 = scores.first().copied().unwrap_or(1).max(1) as f64;
                let mut ranked: Vec<(f64, usize)> = Vec::new();
                for (ai_idx, (p1, p2, conf)) in top2.iter().enumerate() {
                    if p1 == p2 {
                        continue;
                    }
                    let player = ai_idx + 1;
                    let threat_ratio = (scores[player] as f64 / s0).clamp(0.2, 3.0);
                    let threat = (1.0 - *conf) * threat_ratio;
                    ranked.push((threat, ai_idx));
                }
                ranked.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
                let cap = switch_cap.max(1).min(ranked.len());
                for (_, i) in ranked.into_iter().take(cap) {
                    moves[i] = top2[i].1;
                }
                moves
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn sample_index(
                probs: &[f64],
                rng: &mut FastRng,
            ) -> usize {
                if probs.is_empty() {
                    return 0;
                }
                let r = rng.next_f64();
                let mut acc = 0.0;
                for (i, &p) in probs.iter().enumerate() {
                    acc += p.max(0.0);
                    if r <= acc {
                        return i;
                    }
                }
                probs.len() - 1
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn build_ai_candidates_and_probs(
                game: &Game,
                state: &State,
                models: &[AiModel],
            ) -> Vec<(Vec<(usize, usize)>, Vec<f64>)> {
                let mut all = Vec::with_capacity(game.m.saturating_sub(1));
                for ai_idx in 0..game.m.saturating_sub(1) {
                    let player = ai_idx + 1;
                    let cands = get_candidates(game, state, player);
                    if cands.is_empty() {
                        all.push((vec![state.pos[player]], vec![1.0]));
                        continue;
                    }
                    let probs = blended_ai_probs(game, state, player, &models[ai_idx], &cands);
                    all.push((cands, probs));
                }
                all
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn frontier_potential(
                game: &Game,
                state: &State,
            ) -> f64 {
                let mut frontier = 0.0_f64;
                let mut growth = 0.0_f64;
                let mut vulnerability = 0.0_f64;
                const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

                for x in 0..game.n {
                    for y in 0..game.n {
                        if state.owner[x][y] != 0 {
                            continue;
                        }
                        let v = game.v[x][y] as f64;
                        let lv = state.level[x][y] as f64;
                        if state.level[x][y] < game.u {
                            growth += v * (game.u - state.level[x][y]) as f64 / game.u as f64;
                        }
                        for (dx, dy) in DIRS {
                            let nx = x as isize + dx;
                            let ny = y as isize + dy;
                            if !in_bounds(game.n, nx, ny) {
                                continue;
                            }
                            let ux = nx as usize;
                            let uy = ny as usize;
                            let nv = game.v[ux][uy] as f64;
                            let owner = state.owner[ux][uy];
                            if owner == -1 {
                                frontier += 1.00 * nv;
                            } else if owner > 0 {
                                if state.level[ux][uy] == 1 {
                                    frontier += 0.85 * nv;
                                } else {
                                    frontier += 0.35 * nv / state.level[ux][uy] as f64;
                                }
                            } else if state.level[ux][uy] == 1 && lv == 1.0 {
                                vulnerability += 0.45 * v;
                            }
                        }
                    }
                }
                0.022 * frontier + 0.090 * growth - 0.060 * vulnerability
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn strategic_score(
                game: &Game,
                state: &State,
            ) -> f64 {
                let base = absolute_score(game, state) + frontier_potential(game, state);
                let enriched_w: f64 = std::env::var("AHC_ENRICHED_SCORE_W")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0.0);
                let opp_frontier_w: f64 = std::env::var("AHC_OPP_FRONTIER_W")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0.0);
                let mut score = base;
                if enriched_w > 0.0 {
                    score += enriched_w * territory_enrichment(game, state);
                }
                if opp_frontier_w > 0.0 {
                    score -= opp_frontier_w * leader_frontier_strength(game, state);
                }
                score
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn denial_score(
                game: &Game,
                state: &State,
            ) -> f64 {
                let scores = calc_scores(game, state);
                let s0 = scores[0] as f64;
                let sa = scores.iter().skip(1).copied().max().unwrap_or(1).max(1) as f64;
                let ratio = s0 / sa;
                let base = 1e5 * (1.0 + ratio).log2();

                let phase = state.turn as f64 / game.t as f64;
                let remaining = (1.0 - phase).max(0.0);

                let leader = (1..game.m).max_by_key(|&p| scores[p]).unwrap_or(1);

                let mut own_upgrade_pot = 0.0_f64;
                let mut leader_upgrade_pot = 0.0_f64;
                let mut leader_weak_cells = 0.0_f64;

                for x in 0..game.n {
                    for y in 0..game.n {
                        let v = game.v[x][y] as f64;
                        let owner = state.owner[x][y];
                        let level = state.level[x][y];

                        if owner == 0 && level < game.u {
                            own_upgrade_pot += v * (game.u - level) as f64;
                        } else if owner == leader as i32 {
                            if level < game.u {
                                leader_upgrade_pot += v * (game.u - level) as f64;
                            }
                            if level == 1 {
                                leader_weak_cells += v;
                            }
                        }
                    }
                }

                // Weights: scale by remaining game time and score ratio
                // own_pot_w: value of our upgrade potential (higher early, fades late)
                let own_pot_w = 0.30 * remaining;
                // deny_pot_w: penalty for leader's upgrade potential (higher when we're ahead)
                let deny_pot_w = 0.15 * remaining * ratio.clamp(0.5, 3.0);
                // weak_w: bonus for states where leader has many vulnerable (level-1) cells
                // (negative because more leader weakness = better for us, so subtract * negative = add)
                let weak_bonus_w = 0.10 * remaining * ratio.clamp(0.5, 2.0);

                base + own_pot_w * own_upgrade_pot - deny_pot_w * leader_upgrade_pot
                    + weak_bonus_w * leader_weak_cells
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn denial_score_v2(
                game: &Game,
                state: &State,
            ) -> f64 {
                let scores = calc_scores(game, state);
                let s0 = scores[0] as f64;
                let sa = scores.iter().skip(1).copied().max().unwrap_or(1).max(1) as f64;
                let ratio = s0 / sa;
                let base = 1e5 * (1.0 + ratio).log2();

                let phase = state.turn as f64 / game.t as f64;
                let remaining = (1.0 - phase).max(0.0);

                let leader = (1..game.m).max_by_key(|&p| scores[p]).unwrap_or(1);

                let mut own_upgrade_pot = 0.0_f64;
                let mut leader_upgrade_pot = 0.0_f64;
                let mut leader_weak_cells = 0.0_f64;

                for x in 0..game.n {
                    for y in 0..game.n {
                        let v = game.v[x][y] as f64;
                        let owner = state.owner[x][y];
                        let level = state.level[x][y];

                        if owner == 0 && level < game.u {
                            own_upgrade_pot += v * (game.u - level) as f64;
                        } else if owner == leader as i32 {
                            if level < game.u {
                                leader_upgrade_pot += v * (game.u - level) as f64;
                            }
                            if level == 1 {
                                leader_weak_cells += v;
                            }
                        }
                    }
                }

                // FIX: penalize own remaining potential (incentivize upgrading)
                let own_pot_w = 0.20 * remaining;
                let deny_pot_w = 0.15 * remaining * ratio.clamp(0.5, 3.0);
                let weak_bonus_w = 0.10 * remaining * ratio.clamp(0.5, 2.0);

                // Add frontier for territory awareness (from strategic_score)
                let frontier = frontier_potential(game, state);

                base + frontier - own_pot_w * own_upgrade_pot - deny_pot_w * leader_upgrade_pot
                    + weak_bonus_w * leader_weak_cells
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn ratio_eval_score(
                game: &Game,
                state: &State,
                equalization_w: f64,
                frontier_w: f64,
                denial_w: f64,
            ) -> f64 {
                let scores = calc_scores(game, state);
                let s0 = scores[0] as f64;
                let sa = scores.iter().skip(1).copied().max().unwrap_or(1).max(1) as f64;
                let base = 1e5 * (1.0 + s0 / sa).log2();

                let phase = state.turn as f64 / game.t as f64;
                let remaining = (1.0 - phase).max(0.0);

                // Equalization bonus: prefer states where opponents are balanced
                let opp_scores: Vec<f64> =
                    scores.iter().skip(1).copied().map(|s| s as f64).collect();
                let n_opp = opp_scores.len().max(1) as f64;
                let opp_mean = opp_scores.iter().sum::<f64>() / n_opp;
                let opp_var = opp_scores
                    .iter()
                    .map(|&s| (s - opp_mean).powi(2))
                    .sum::<f64>()
                    / n_opp;
                let equalization = -equalization_w * opp_var.sqrt() * remaining;

                // Frontier growth potential
                let n = game.n;
                let mut frontier = 0.0_f64;
                for x in 0..n {
                    for y in 0..n {
                        if state.owner[x][y] == 0 {
                            let v = game.v[x][y] as f64;
                            if state.level[x][y] < game.u {
                                frontier +=
                                    v * (game.u - state.level[x][y]) as f64 / game.u as f64 * 0.3;
                            }
                            for &(dx, dy) in &[(0isize, 1isize), (1, 0), (0, -1), (-1, 0)] {
                                let nx = x as isize + dx;
                                let ny = y as isize + dy;
                                if nx >= 0 && nx < n as isize && ny >= 0 && ny < n as isize {
                                    let ux = nx as usize;
                                    let uy = ny as usize;
                                    if state.owner[ux][uy] == -1 {
                                        frontier += game.v[ux][uy] as f64 * 0.2;
                                    }
                                }
                            }
                        }
                    }
                }

                // Leader denial: bonus for leader having L1 cells (attackable)
                let leader = (1..game.m).max_by_key(|&p| scores[p]).unwrap_or(1);
                let mut leader_weak = 0.0_f64;
                for x in 0..n {
                    for y in 0..n {
                        if state.owner[x][y] == leader as i32 && state.level[x][y] == 1 {
                            leader_weak += game.v[x][y] as f64;
                        }
                    }
                }

                base + frontier_w * frontier + equalization + denial_w * leader_weak * remaining
            }

            fn leader_frontier_strength(game: &Game, state: &State) -> f64 {
                let scores = calc_scores(game, state);
                let max_ai = scores.iter().skip(1).copied().max().unwrap_or(0);
                if max_ai <= 0 {
                    return 0.0;
                }
                // Find which player is the leader
                let leader = (1..game.m).find(|&p| scores[p] == max_ai).unwrap_or(1);
                let mut frontier_v = 0.0_f64;
                const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                for x in 0..game.n {
                    for y in 0..game.n {
                        if state.owner[x][y] != leader as i32 {
                            continue;
                        }
                        for (dx, dy) in DIRS {
                            let nx = x as isize + dx;
                            let ny = y as isize + dy;
                            if !in_bounds(game.n, nx, ny) {
                                continue;
                            }
                            let ux = nx as usize;
                            let uy = ny as usize;
                            let nv = game.v[ux][uy] as f64;
                            let owner = state.owner[ux][uy];
                            if owner == -1 {
                                frontier_v += nv;
                            } else if owner != leader as i32 && state.level[ux][uy] == 1 {
                                frontier_v += 0.7 * nv;
                            }
                        }
                    }
                }
                frontier_v
            }

            fn territory_enrichment(game: &Game, state: &State) -> f64 {
                let mut visited = vec![vec![false; game.n]; game.n];
                let mut queue = VecDeque::new();
                let (sx, sy) = state.pos[0];
                visited[sx][sy] = true;
                queue.push_back((sx, sy));

                let mut reachable_value = 0.0_f64;
                let mut defended_value = 0.0_f64;
                let mut frontier_access = 0.0_f64;
                let mut total_owned = 0_u32;

                const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                while let Some((x, y)) = queue.pop_front() {
                    let v = game.v[x][y] as f64;
                    let lv = state.level[x][y];
                    reachable_value += v * lv as f64;
                    total_owned += 1;
                    if lv >= 2 {
                        defended_value += v;
                    }

                    for (dx, dy) in DIRS {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if !in_bounds(game.n, nx, ny) {
                            continue;
                        }
                        let ux = nx as usize;
                        let uy = ny as usize;
                        if visited[ux][uy] {
                            continue;
                        }
                        if state.owner[ux][uy] == 0 {
                            visited[ux][uy] = true;
                            queue.push_back((ux, uy));
                        } else {
                            visited[ux][uy] = true;
                            // Non-owned neighbor = frontier access opportunity
                            let nv = game.v[ux][uy] as f64;
                            if state.owner[ux][uy] == -1 {
                                frontier_access += nv;
                            } else if state.level[ux][uy] == 1 {
                                frontier_access += 0.8 * nv;
                            }
                        }
                    }
                }

                // Count total owned cells (including disconnected ones) for fragmentation penalty
                let mut total_owned_all = 0_u32;
                for x in 0..game.n {
                    for y in 0..game.n {
                        if state.owner[x][y] == 0 {
                            total_owned_all += 1;
                        }
                    }
                }
                let fragmentation = if total_owned_all > 0 {
                    1.0 - (total_owned as f64 / total_owned_all as f64)
                } else {
                    0.0
                };

                // Combine: reachable territory + defense bonus + frontier access - fragmentation penalty
                0.003 * reachable_value + 0.010 * defended_value + 0.008 * frontier_access
                    - 50.0 * fragmentation
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn pessimism_weight(
                game: &Game,
                uncertainty: f64,
            ) -> f64 {
                if uncertainty < 0.08 {
                    return 0.0;
                }
                let m_factor = ((game.m as f64 - 2.0) / 6.0).clamp(0.0, 1.0);
                (0.05 + 0.22 * uncertainty + 0.10 * m_factor).clamp(0.05, 0.32)
            }

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn evaluate_local_move(
                game: &Game,
                state: &State,
                cand: (usize, usize),
                scores: &[i64],
                s0: f64,
                max_ai_i64: i64,
                phase: f64,
                conflict_map: &[Vec<f64>],
                cur: (usize, usize),
                is_leader: &[bool],
            ) -> f64 {
                let (x, y) = cand;
                let owner = state.owner[x][y];
                let level = state.level[x][y];
                let value = game.v[x][y] as f64;
                let max_ai = max_ai_i64 as f64;
                let mut score = 0.0_f64;

                if owner == -1 {
                    score += value;
                    score += (1.0 - phase) * 0.52 * value;
                } else if owner == 0 {
                    if level < game.u {
                        // Phase-adaptive + U-adaptive upgrade weight
                        // Early game: expand territory (low upgrade weight)
                        // Late game: upgrade fortress cells (high upgrade weight)
                        // U=1: no upgrade possible, u_factor=0 keeps weight at early level
                        let upgrade_early: f64 = std::env::var("AHC_UPGRADE_EARLY")
                            .ok()
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(-1.0);
                        let upgrade_late: f64 = std::env::var("AHC_UPGRADE_LATE")
                            .ok()
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(-1.0);
                        let upgrade_base: f64 = if upgrade_early >= 0.0 && upgrade_late >= 0.0 {
                            // Fortress mode: phase-adaptive with U scaling
                            let u_factor = if game.u <= 1 {
                                0.0
                            } else {
                                ((game.u as f64 - 1.0) / 4.0).clamp(0.0, 1.0)
                            };
                            upgrade_early + (upgrade_late - upgrade_early) * phase * u_factor
                        } else {
                            // Legacy mode: flat upgrade_base
                            std::env::var("AHC_UPGRADE_BASE")
                                .ok()
                                .and_then(|v| v.parse().ok())
                                .unwrap_or(0.90)
                        };
                        let upgrade_defense: f64 = std::env::var("AHC_UPGRADE_DEFENSE")
                            .ok()
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(0.0);
                        score += upgrade_base * value;
                        score += 0.18 * value * (game.u - level) as f64 / game.u as f64;
                        // Defense bonus: higher for early upgrades (1->2 is most defensive)
                        score += upgrade_defense * value * phase / level as f64;
                    } else if (x, y) == cur {
                        score -= 0.15 * value;
                    } else {
                        score -= 0.05 * value;
                    }
                } else {
                    let opp = owner as usize;
                    let threat = ((scores[opp] as f64 - s0).max(0.0)) / max_ai;
                    let m5_focus = (1.0 - ((game.m as f64 - 5.0).abs() / 2.0)).clamp(0.0, 1.0);
                    let leader_bonus: f64 = std::env::var("AHC_LEADER_ATTACK_BONUS")
                        .ok()
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0.45);
                    if level == 1 {
                        score += (1.25 + 0.85 * threat) * value;
                        if is_leader[opp] {
                            score += leader_bonus * phase * value;
                            score += (0.10 + 0.20 * phase) * m5_focus * (0.5 + threat) * value;
                        }
                    } else {
                        score += (0.32 + 0.45 * threat) * value / level as f64;
                        score -= 0.11 * value;
                        if is_leader[opp] {
                            score += 0.20 * phase * value / level as f64;
                        }
                    }
                }

                let next_pos = if owner > 0 && level >= 2 { cur } else { (x, y) };
                const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                for (dx, dy) in DIRS {
                    let nx = next_pos.0 as isize + dx;
                    let ny = next_pos.1 as isize + dy;
                    if in_bounds(game.n, nx, ny) {
                        let ux = nx as usize;
                        let uy = ny as usize;
                        let vv = game.v[ux][uy] as f64;
                        if state.owner[ux][uy] != 0 {
                            score += 0.07 * vv;
                        } else if state.level[ux][uy] < game.u {
                            score += 0.03 * vv;
                        }
                    }
                }

                let p_any = 1.0 - (-conflict_map[x][y]).exp();
                let multi_factor = ((game.m as f64 - 2.0) / 6.0).clamp(0.0, 1.0);
                let risk_scale = 1.0 + 0.35 * multi_factor + 0.20 * phase;
                if owner == -1 {
                    score -= 0.75 * risk_scale * p_any * value;
                } else if owner == 0 {
                    score += 0.08 * p_any * value / risk_scale;
                } else if level == 1 {
                    score -= 0.30 * risk_scale * p_any * value;
                } else {
                    score -= 0.18 * risk_scale * p_any * value;
                }

                // Zone bonus: small directional bias toward high-value-density zone
                // Affects all strategies (x04 beam, x01 pessimistic, x02 mc)
                let eval_zone_bonus: f64 = std::env::var("AHC_EVAL_ZONE_BONUS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0.0);
                if eval_zone_bonus > 0.0 {
                    let zone = get_value_zone(game);
                    if zone[x][y] {
                        score += eval_zone_bonus * value;
                    }
                }

                score + value * 1e-6 - (x as f64 * 31.0 + y as f64) * 1e-9
            }

            fn update_model_for_player(
                game: &Game,
                state_before: &State,
                player: usize,
                observed: (usize, usize),
                model: &mut AiModel,
            ) {
                let cands = get_candidates(game, state_before, player);
                if cands.is_empty() {
                    return;
                }

                let obs_idx = match cands.iter().position(|&x| x == observed) {
                    Some(v) => v,
                    None => return,
                };

                let mut est_scores = vec![0.0_f64; cands.len()];
                let mut feats = vec![[0.0_f64; FEAT_DIM]; cands.len()];
                for (i, &cand) in cands.iter().enumerate() {
                    let f = ai_features(game, state_before, player, cand);
                    feats[i] = f;
                    est_scores[i] = dot(&model.w, &f);
                }

                let max_score = est_scores.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                let tol = 1e-9 * max_score.abs().max(1.0);
                let best_set: Vec<usize> = (0..cands.len())
                    .filter(|&i| est_scores[i] >= max_score - tol)
                    .collect();
                let pred_idx = best_set.first().copied().unwrap_or(0);

                let informative = best_set.len() < cands.len();
                if informative {
                    model.seen += 1;
                    let matched = best_set.contains(&obs_idx);
                    if !matched {
                        model.mismatch += 1;
                    }

                    let raw_eps = model.mismatch as f64 / model.seen.max(1) as f64;
                    model.eps_est = (0.70 * model.eps_est + 0.30 * raw_eps).clamp(0.05, 0.60);

                    if !matched {
                        let lr: f64 = std::env::var("AHC_MODEL_LR")
                            .ok()
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(0.12);
                        for k in 0..FEAT_DIM {
                            let diff = (feats[obs_idx][k] - feats[pred_idx][k]) / 1000.0;
                            model.w[k] =
                                (model.w[k] + lr * diff).clamp(FEAT_CLAMP_LO[k], FEAT_CLAMP_HI[k]);
                        }
                    }
                }

                let reg: f64 = std::env::var("AHC_MODEL_REG")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(0.995);
                for k in 0..FEAT_DIM {
                    model.w[k] = reg * model.w[k] + (1.0 - reg) * FEAT_ANCHOR[k];
                }
            }

            fn update_models(
                game: &Game,
                state_before: &State,
                selected: &[(usize, usize)],
                models: &mut [AiModel],
            ) {
                for ai_idx in 0..models.len() {
                    let player = ai_idx + 1;
                    update_model_for_player(
                        game,
                        state_before,
                        player,
                        selected[player],
                        &mut models[ai_idx],
                    );
                }
            }

            fn read_initial<R: BufRead>(sc: &mut Scanner<R>) -> Option<(Game, State)> {
                let n = sc.next::<usize>()?;
                let m = sc.next::<usize>()?;
                let t = sc.next::<usize>()?;
                let u = sc.next::<usize>()?;

                let mut v = vec![vec![0_i64; n]; n];
                for row in &mut v {
                    for val in row.iter_mut() {
                        *val = sc.next::<i64>()?;
                    }
                }

                let mut pos = vec![(0_usize, 0_usize); m];
                for p in &mut pos {
                    let x = sc.next::<usize>()?;
                    let y = sc.next::<usize>()?;
                    *p = (x, y);
                }

                let mut owner = vec![vec![-1_i32; n]; n];
                let mut level = vec![vec![0_usize; n]; n];
                for (i, &(x, y)) in pos.iter().enumerate() {
                    owner[x][y] = i as i32;
                    level[x][y] = 1;
                }

                let game = Game { n, m, t, u, v };
                let state = State {
                    pos,
                    owner,
                    level,
                    turn: 0,
                };
                Some((game, state))
            }

            fn read_feedback<R: BufRead>(
                sc: &mut Scanner<R>,
                game: &Game,
                state: &mut State,
            ) -> Option<Vec<(usize, usize)>> {
                let mut selected = vec![(0_usize, 0_usize); game.m];
                for s in &mut selected {
                    let x = sc.next::<usize>()?;
                    let y = sc.next::<usize>()?;
                    *s = (x, y);
                }

                for p in 0..game.m {
                    let x = sc.next::<usize>()?;
                    let y = sc.next::<usize>()?;
                    state.pos[p] = (x, y);
                }
                for i in 0..game.n {
                    for j in 0..game.n {
                        state.owner[i][j] = sc.next::<i32>()?;
                    }
                }
                for i in 0..game.n {
                    for j in 0..game.n {
                        state.level[i][j] = sc.next::<usize>()?;
                    }
                }
                state.turn += 1;
                Some(selected)
            }

            // ========== Bayesian Opponent Modeling ==========

            const BAYES_GRID_SIZE: usize = 200;
            const BAYES_DIM: usize = FEAT_DIM + 1; // w0,w1,w2,w3 + eps
            const BAYES_PRUNE_LOG_THRESHOLD: f64 = -30.0;
            const BAYES_PRIOR_SIGMA: f64 = 0.50; // Gaussian prior sigma around FEAT_ANCHOR

            pub(in crate::__cargo_equip::crates::ahc061_solver) struct BayesGrid {
                points: Vec<[f64; BAYES_DIM]>,
                log_weights: Vec<f64>,
                active: Vec<bool>,
                n_active: usize,
            }

            fn generate_lhs_grid(rng: &mut FastRng, n: usize) -> Vec<[f64; BAYES_DIM]> {
                let ranges: [(f64, f64); BAYES_DIM] = [
                    (0.10, 2.00), // w0: empty cell capture
                    (0.10, 2.00), // w1: own cell level-up
                    (0.10, 2.00), // w2: attack lv1
                    (0.10, 2.00), // w3: attack lv2+
                    (0.05, 0.60), // eps
                ];

                // Generate permutations for each dimension (Fisher-Yates)
                let mut perms = vec![vec![0usize; n]; BAYES_DIM];
                for d in 0..BAYES_DIM {
                    for i in 0..n {
                        perms[d][i] = i;
                    }
                    for i in (1..n).rev() {
                        let j = (rng.next_u64() % (i as u64 + 1)) as usize;
                        perms[d].swap(i, j);
                    }
                }

                let mut grid = Vec::with_capacity(n);
                for i in 0..n {
                    let mut point = [0.0_f64; BAYES_DIM];
                    for d in 0..BAYES_DIM {
                        let u = (perms[d][i] as f64 + rng.next_f64()) / n as f64;
                        point[d] = ranges[d].0 + u * (ranges[d].1 - ranges[d].0);
                    }
                    grid.push(point);
                }
                grid
            }

            impl BayesGrid {
                fn new(rng: &mut FastRng) -> Self {
                    let n = BAYES_GRID_SIZE;
                    let points = generate_lhs_grid(rng, n);

                    // Gaussian prior centered on FEAT_ANCHOR
                    let anchor = [
                        FEAT_ANCHOR[0],
                        FEAT_ANCHOR[1],
                        FEAT_ANCHOR[2],
                        FEAT_ANCHOR[3],
                        0.30,
                    ];
                    let inv_2sigma2 = 1.0 / (2.0 * BAYES_PRIOR_SIGMA * BAYES_PRIOR_SIGMA);
                    let mut log_weights = Vec::with_capacity(n);
                    for p in &points {
                        let mut dist2 = 0.0_f64;
                        for d in 0..BAYES_DIM {
                            let diff = p[d] - anchor[d];
                            dist2 += diff * diff;
                        }
                        log_weights.push(-dist2 * inv_2sigma2);
                    }

                    Self {
                        points,
                        log_weights,
                        active: vec![true; n],
                        n_active: n,
                    }
                }

                fn update(
                    &mut self,
                    game: &Game,
                    state: &State,
                    player: usize,
                    observed: (usize, usize),
                ) {
                    let cands = get_candidates(game, state, player);
                    if cands.len() <= 1 {
                        return;
                    }

                    let obs_idx = match cands.iter().position(|&c| c == observed) {
                        Some(v) => v,
                        None => return,
                    };

                    // Pre-compute features for all candidates
                    let feats: Vec<[f64; FEAT_DIM]> = cands
                        .iter()
                        .map(|&c| ai_features(game, state, player, c))
                        .collect();
                    let k = cands.len() as f64;

                    for g in 0..self.points.len() {
                        if !self.active[g] {
                            continue;
                        }

                        let w = [
                            self.points[g][0],
                            self.points[g][1],
                            self.points[g][2],
                            self.points[g][3],
                        ];
                        let eps = self.points[g][4];

                        // Compute scores under this grid point's weights
                        let mut max_score = f64::NEG_INFINITY;
                        let mut obs_score = 0.0_f64;
                        let mut n_ties = 0usize;
                        for (i, feat) in feats.iter().enumerate() {
                            let s = dot(&w, feat);
                            if s > max_score + 1e-12 {
                                max_score = s;
                                n_ties = 1;
                            } else if s >= max_score - 1e-12 {
                                n_ties += 1;
                            }
                            if i == obs_idx {
                                obs_score = s;
                            }
                        }

                        // Likelihood: P(observed | w, eps)
                        let likelihood = if obs_score >= max_score - 1e-12 {
                            // Observed is argmax
                            (1.0 - eps) / n_ties as f64 + eps / k
                        } else {
                            // Observed is not argmax: only possible via epsilon
                            eps / k
                        };

                        self.log_weights[g] += likelihood.max(1e-300).ln();
                    }

                    // Prune low-probability grid points
                    self.prune();
                }

                fn prune(&mut self) {
                    let max_lw = self
                        .log_weights
                        .iter()
                        .zip(self.active.iter())
                        .filter(|(_, a)| **a)
                        .map(|(&w, _)| w)
                        .fold(f64::NEG_INFINITY, f64::max);

                    for g in 0..self.points.len() {
                        if self.active[g]
                            && self.log_weights[g] < max_lw + BAYES_PRUNE_LOG_THRESHOLD
                        {
                            self.active[g] = false;
                            self.n_active -= 1;
                        }
                    }
                }

                fn map_estimate(&self) -> ([f64; FEAT_DIM], f64) {
                    let mut best_g = 0;
                    let mut best_lw = f64::NEG_INFINITY;
                    for g in 0..self.points.len() {
                        if self.active[g] && self.log_weights[g] > best_lw {
                            best_lw = self.log_weights[g];
                            best_g = g;
                        }
                    }
                    let p = &self.points[best_g];
                    ([p[0], p[1], p[2], p[3]], p[4])
                }

                fn posterior_mean(&self) -> ([f64; FEAT_DIM], f64) {
                    // Compute normalized weights (in probability space)
                    let max_lw = self
                        .log_weights
                        .iter()
                        .zip(self.active.iter())
                        .filter(|(_, a)| **a)
                        .map(|(&w, _)| w)
                        .fold(f64::NEG_INFINITY, f64::max);

                    let mut sum_w = 0.0_f64;
                    let mut mean = [0.0_f64; BAYES_DIM];

                    for g in 0..self.points.len() {
                        if !self.active[g] {
                            continue;
                        }
                        let w = (self.log_weights[g] - max_lw).exp();
                        sum_w += w;
                        for d in 0..BAYES_DIM {
                            mean[d] += w * self.points[g][d];
                        }
                    }

                    if sum_w > 0.0 {
                        for d in 0..BAYES_DIM {
                            mean[d] /= sum_w;
                        }
                    } else {
                        return (FEAT_ANCHOR, 0.30);
                    }

                    ([mean[0], mean[1], mean[2], mean[3]], mean[4])
                }

                fn thompson_sample(&self, rng: &mut FastRng) -> ([f64; FEAT_DIM], f64) {
                    // Sample a grid point proportional to posterior weights
                    let max_lw = self
                        .log_weights
                        .iter()
                        .zip(self.active.iter())
                        .filter(|(_, a)| **a)
                        .map(|(&w, _)| w)
                        .fold(f64::NEG_INFINITY, f64::max);

                    let mut cum_weights = Vec::with_capacity(self.n_active);
                    let mut indices = Vec::with_capacity(self.n_active);
                    let mut cumsum = 0.0_f64;

                    for g in 0..self.points.len() {
                        if !self.active[g] {
                            continue;
                        }
                        let w = (self.log_weights[g] - max_lw).exp();
                        cumsum += w;
                        cum_weights.push(cumsum);
                        indices.push(g);
                    }

                    if cumsum <= 0.0 || indices.is_empty() {
                        return (FEAT_ANCHOR, 0.30);
                    }

                    let u = rng.next_f64() * cumsum;
                    let idx = match cum_weights.binary_search_by(|w| {
                        w.partial_cmp(&u).unwrap_or(std::cmp::Ordering::Equal)
                    }) {
                        Ok(i) => i,
                        Err(i) => i.min(indices.len() - 1),
                    };
                    let g = indices[idx];
                    let p = &self.points[g];
                    ([p[0], p[1], p[2], p[3]], p[4])
                }

                fn write_to_model(&self, model: &mut AiModel) {
                    let (w, eps) = self.posterior_mean();
                    model.w = w;
                    model.eps_est = eps.clamp(0.05, 0.60);
                }
            }

            fn bayes_switch_turn() -> usize {
                std::env::var("AHC_BAYES_SWITCH_TURN")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(8)
            }
            fn bayes_max_m() -> usize {
                std::env::var("AHC_BAYES_MAX_M")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(5)
            }

            fn update_bayes_models(
                game: &Game,
                state_before: &State,
                selected: &[(usize, usize)],
                models: &mut [AiModel],
                grids: &mut [BayesGrid],
            ) {
                for ai_idx in 0..models.len() {
                    let player = ai_idx + 1;
                    // Always update gradient descent
                    update_model_for_player(
                        game,
                        state_before,
                        player,
                        selected[player],
                        &mut models[ai_idx],
                    );

                    // Only use Bayesian for small M (more predictable, less noise)
                    if game.m <= bayes_max_m() {
                        grids[ai_idx].update(game, state_before, player, selected[player]);
                        if state_before.turn >= bayes_switch_turn() {
                            grids[ai_idx].write_to_model(&mut models[ai_idx]);
                        }
                    }
                }
            }

            pub fn run_with_strategy_bayes(strategy: StrategyMode) {
                init_game_timer();

                let stdin = io::stdin();
                let mut sc = Scanner::new(BufReader::new(stdin.lock()));
                let stdout = io::stdout();
                let mut out = BufWriter::new(stdout.lock());

                let (game, mut state) = match read_initial(&mut sc) {
                    Some(v) => v,
                    None => return,
                };

                let mut models = vec![AiModel::new(); game.m.saturating_sub(1)];
                let mut rng = FastRng::new(42);
                let mut grids: Vec<BayesGrid> = (0..game.m.saturating_sub(1))
                    .map(|_| BayesGrid::new(&mut rng))
                    .collect();

                for _ in 0..game.t {
                    let prev_state = state.clone();
                    let (x, y) = choose_move(&game, &prev_state, &models, strategy);

                    if writeln!(out, "{} {}", x, y).is_err() {
                        return;
                    }
                    if out.flush().is_err() {
                        return;
                    }

                    let selected = match read_feedback(&mut sc, &game, &mut state) {
                        Some(v) => v,
                        None => return,
                    };
                    update_bayes_models(&game, &prev_state, &selected, &mut models, &mut grids);
                }
            }

            pub fn run_with_strategy(strategy: StrategyMode) {
                init_game_timer();

                let stdin = io::stdin();
                let mut sc = Scanner::new(BufReader::new(stdin.lock()));
                let stdout = io::stdout();
                let mut out = BufWriter::new(stdout.lock());

                let (game, mut state) = match read_initial(&mut sc) {
                    Some(v) => v,
                    None => return,
                };

                let mut models = vec![AiModel::new(); game.m.saturating_sub(1)];

                for _ in 0..game.t {
                    let prev_state = state.clone();
                    let (x, y) = choose_move(&game, &prev_state, &models, strategy);

                    if writeln!(out, "{} {}", x, y).is_err() {
                        return;
                    }
                    if out.flush().is_err() {
                        return;
                    }

                    let selected = match read_feedback(&mut sc, &game, &mut state) {
                        Some(v) => v,
                        None => return,
                    };
                    update_models(&game, &prev_state, &selected, &mut models);
                }
            }

            pub fn run_prediction_verify() {
                init_game_timer();

                let stdin = io::stdin();
                let mut sc = Scanner::new(BufReader::new(stdin.lock()));
                let stdout = io::stdout();
                let mut out = BufWriter::new(stdout.lock());

                let (game, mut state) = match read_initial(&mut sc) {
                    Some(v) => v,
                    None => return,
                };

                let n_opponents = game.m.saturating_sub(1);
                let mut models = vec![AiModel::new(); n_opponents];
                let mut rng = FastRng::new(42);
                let mut grids: Vec<BayesGrid> =
                    (0..n_opponents).map(|_| BayesGrid::new(&mut rng)).collect();

                // Per-opponent tracking
                let mut top1_hits = vec![0u32; n_opponents];
                let mut top2_hits = vec![0u32; n_opponents];
                let mut total = vec![0u32; n_opponents];
                let mut phase_hits = vec![[0u32; 3]; n_opponents]; // early/mid/late
                let mut phase_total = vec![[0u32; 3]; n_opponents];
                let mut cand_sum = vec![0u64; n_opponents];
                let mut unique_best_count = vec![0u32; n_opponents];

                for turn in 0..game.t {
                    let prev_state = state.clone();

                    // Determine phase: 0=early(0-29), 1=mid(30-69), 2=late(70-99)
                    let phase = if turn < 30 {
                        0
                    } else if turn < 70 {
                        1
                    } else {
                        2
                    };

                    // === PREDICTION PHASE ===
                    // For each opponent, record prediction before seeing actual move
                    struct PredRecord {
                        best_set: Vec<(usize, usize)>,
                        top2_set: Vec<(usize, usize)>,
                        n_cands: usize,
                        unique_best: bool,
                    }

                    let mut preds = Vec::with_capacity(n_opponents);
                    for ai_idx in 0..n_opponents {
                        let player = ai_idx + 1;
                        let candidates = get_candidates(&game, &prev_state, player);
                        let n_cands = candidates.len();

                        if candidates.is_empty() {
                            preds.push(PredRecord {
                                best_set: Vec::new(),
                                top2_set: Vec::new(),
                                n_cands: 0,
                                unique_best: false,
                            });
                            continue;
                        }

                        // Score each candidate
                        let mut scored: Vec<(f64, (usize, usize))> = candidates
                            .iter()
                            .map(|&c| {
                                let feat = ai_features(&game, &prev_state, player, c);
                                (dot(&models[ai_idx].w, &feat), c)
                            })
                            .collect();

                        // Sort descending by score
                        scored.sort_by(|a, b| {
                            b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal)
                        });

                        let best_score = scored[0].0;
                        let tol = 1e-9;

                        // Argmax set (all within tolerance of best)
                        let best_set: Vec<(usize, usize)> = scored
                            .iter()
                            .filter(|(s, _)| best_score - s < tol)
                            .map(|&(_, c)| c)
                            .collect();

                        let unique_best = best_set.len() == 1;

                        // Top-2 set: best_set + second-best tier
                        let mut top2_set = best_set.clone();
                        if scored.len() > best_set.len() {
                            let second_score = scored[best_set.len()].0;
                            for &(s, c) in &scored[best_set.len()..] {
                                if second_score - s < tol {
                                    top2_set.push(c);
                                } else {
                                    break;
                                }
                            }
                        }

                        preds.push(PredRecord {
                            best_set,
                            top2_set,
                            n_cands,
                            unique_best,
                        });
                    }

                    // === OUR MOVE ===
                    // Simple greedy: pick highest-value available cell
                    let my_cands = get_candidates(&game, &prev_state, 0);
                    let (mx, my) = if my_cands.is_empty() {
                        prev_state.pos[0]
                    } else {
                        *my_cands
                            .iter()
                            .max_by_key(|&&(x, y)| {
                                let v = game.v[x][y];
                                let owner = prev_state.owner[x][y];
                                if owner == -1 {
                                    // Neutral high-value cell
                                    v * 10
                                } else if owner == 0 {
                                    // Own cell upgrade
                                    if prev_state.level[x][y] < game.u {
                                        v * 5
                                    } else {
                                        0
                                    }
                                } else {
                                    // Enemy cell attack
                                    v * 3
                                }
                            })
                            .unwrap()
                    };

                    if writeln!(out, "{} {}", mx, my).is_err() {
                        return;
                    }
                    if out.flush().is_err() {
                        return;
                    }

                    // Read feedback (actual moves)
                    let selected = match read_feedback(&mut sc, &game, &mut state) {
                        Some(v) => v,
                        None => return,
                    };

                    // === COMPARISON ===
                    for ai_idx in 0..n_opponents {
                        let player = ai_idx + 1;
                        let actual = selected[player];
                        let pred = &preds[ai_idx];

                        total[ai_idx] += 1;
                        phase_total[ai_idx][phase] += 1;
                        cand_sum[ai_idx] += pred.n_cands as u64;
                        if pred.unique_best {
                            unique_best_count[ai_idx] += 1;
                        }

                        let hit1 = pred.best_set.contains(&actual);
                        let hit2 = pred.top2_set.contains(&actual);

                        if hit1 {
                            top1_hits[ai_idx] += 1;
                            phase_hits[ai_idx][phase] += 1;
                        }
                        if hit2 {
                            top2_hits[ai_idx] += 1;
                        }

                        eprintln!(
                            "T={:3}|P{}|cands={:3}|best_sz={:2}|hit1={}|hit2={}|eps={:.3}",
                            turn,
                            player,
                            pred.n_cands,
                            pred.best_set.len(),
                            if hit1 { 1 } else { 0 },
                            if hit2 { 1 } else { 0 },
                            models[ai_idx].eps_est,
                        );
                    }

                    // === UPDATE MODELS ===
                    update_bayes_models(&game, &prev_state, &selected, &mut models, &mut grids);
                }

                // === SUMMARY OUTPUT ===
                eprintln!("HEADER|M={}|U={}", game.m, game.u,);

                let mut overall_top1 = 0u32;
                let mut overall_top2 = 0u32;
                let mut overall_total = 0u32;

                for ai_idx in 0..n_opponents {
                    let player = ai_idx + 1;
                    let t = total[ai_idx].max(1) as f64;
                    let top1_rate = top1_hits[ai_idx] as f64 / t;
                    let top2_rate = top2_hits[ai_idx] as f64 / t;

                    let early_rate = if phase_total[ai_idx][0] > 0 {
                        phase_hits[ai_idx][0] as f64 / phase_total[ai_idx][0] as f64
                    } else {
                        0.0
                    };
                    let mid_rate = if phase_total[ai_idx][1] > 0 {
                        phase_hits[ai_idx][1] as f64 / phase_total[ai_idx][1] as f64
                    } else {
                        0.0
                    };
                    let late_rate = if phase_total[ai_idx][2] > 0 {
                        phase_hits[ai_idx][2] as f64 / phase_total[ai_idx][2] as f64
                    } else {
                        0.0
                    };

                    let mean_cands = cand_sum[ai_idx] as f64 / t;

                    eprintln!(
                        "PLAYER_{}|top1={:.3}|top2={:.3}|early={:.3}|mid={:.3}|late={:.3}|eps={:.3}|n={}|mean_cands={:.1}|unique_best={}",
                        player,
                        top1_rate,
                        top2_rate,
                        early_rate,
                        mid_rate,
                        late_rate,
                        models[ai_idx].eps_est,
                        total[ai_idx],
                        mean_cands,
                        unique_best_count[ai_idx],
                    );

                    overall_top1 += top1_hits[ai_idx];
                    overall_top2 += top2_hits[ai_idx];
                    overall_total += total[ai_idx];
                }

                let ot = overall_total.max(1) as f64;
                eprintln!(
                    "OVERALL|top1={:.3}|top2={:.3}",
                    overall_top1 as f64 / ot,
                    overall_top2 as f64 / ot,
                );
            }
        }
    }

    pub(crate) mod macros {
        pub mod ahc061_solver {}
    }

    pub(crate) mod prelude {
        pub use crate::__cargo_equip::crates::*;
    }

    mod preludes {
        pub mod ahc061_solver {}
    }
}
