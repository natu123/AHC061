pub use __cargo_equip::prelude::*;

use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    unsafe {
        env::set_var("AHC_X04_PHASE_CUTOFF", "0.79");
        env::set_var("AHC_X04_PHASE_SPLIT", "0.54");
        env::set_var("AHC_X04_TARGET_COUNT", "8");
        env::set_var("AHC_X04_TARGET_EVAL", "7");
        env::set_var("AHC_X04_CANDIDATE_CAP", "9");
        env::set_var("AHC_X04_PLAN_LEN_FAST", "6");
        env::set_var("AHC_X04_PLAN_LEN_SLOW", "8");
        env::set_var("AHC_X04_BEAM_WIDTH_FAST", "5");
        env::set_var("AHC_X04_BEAM_WIDTH_SLOW", "7");
        env::set_var("AHC_X04_BRANCH_WIDTH", "2");
        env::set_var("AHC_X04_LOCAL_WEIGHT", "0.10");
        env::set_var("AHC_X04_LOCAL_COEFF", "0.09");
        env::set_var("AHC_X04_ROUTE_COEFF", "46.0");

        env::set_var("AHC_X04_TARGET_PRESSURE_WEIGHT", "0.65");
        env::set_var("AHC_X04_PRESSURE_WEIGHT", "0.30");
        env::set_var("AHC_X04_PRESSURE_WEIGHT_EARLY", "-0.05");
        env::set_var("AHC_X04_PRESSURE_WEIGHT_LATE", "1.10");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT", "-0.10");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_EARLY", "0.18");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_LATE", "0.38");
        env::set_var("AHC_X04_PRESSURE_PHASE_SPLIT", "0.64");
    }

    run_with_strategy(StrategyMode::MacroRoute);
}

// The following code was expanded by `cargo-equip`.

///  # Bundled libraries
///
///  - `path+file:///C:/Users/kenji/projects/AtCoder/AHC/AHC061/solver#ahc061-solver@0.1.0` published in **missing** licensed under **missing** as `crate::__cargo_equip::crates::ahc061_solver`
#[allow(unused)]
mod __cargo_equip {
    pub(crate) mod crates {
        pub mod ahc061_solver {
            use std::collections::{HashMap, VecDeque};
            use std::io::{self, BufRead, BufReader, BufWriter, Write};

            mod strategy_mode {
                use std::env;

                use crate::__cargo_equip::crates::ahc061_solver::{
                    x01_beam_pessimistic, x02_monte_carlo, x03_particle_cvar, x04_macro_route,
                    x05_adaptive_racing_mc, x06_expert_switch_hybrid, AiModel, Game, State,
                };

                #[derive(Clone, Copy, Debug, PartialEq, Eq)]
                pub enum StrategyMode {
                    Champion,
                    MonteCarloExplore,
                    ParticleCvar,
                    MacroRoute,
                    HybridMidMc,
                    AdaptiveRacingMc,
                    ExpertSwitchHybrid,
                }

                pub fn strategy_from_env() -> StrategyMode {
                    match env::var("AHC_STRATEGY").ok().as_deref() {
                        Some("champion") => StrategyMode::Champion,
                        Some("mc") | Some("mc_sample") | Some("monte_carlo") => {
                            StrategyMode::MonteCarloExplore
                        }
                        Some("x03") | Some("particle_cvar") | Some("particle") => {
                            StrategyMode::ParticleCvar
                        }
                        Some("x04") | Some("macro_route") | Some("macro") => {
                            StrategyMode::MacroRoute
                        }
                        Some("hybrid_mid_mc") | Some("mid_mc") => StrategyMode::HybridMidMc,
                        Some("x05") | Some("adaptive_racing_mc") | Some("armc") => {
                            StrategyMode::AdaptiveRacingMc
                        }
                        Some("x06") | Some("expert_switch_hybrid") | Some("expert_switch") => {
                            StrategyMode::ExpertSwitchHybrid
                        }
                        _ => StrategyMode::HybridMidMc,
                    }
                }

                pub(in crate::__cargo_equip::crates::ahc061_solver) fn choose_move(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    strategy: StrategyMode,
                ) -> (usize, usize) {
                    match strategy {
                        StrategyMode::Champion => {
                            x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                                game, state, models,
                            )
                        }
                        StrategyMode::MonteCarloExplore => {
                            x02_monte_carlo::choose_move_monte_carlo(game, state, models)
                        }
                        StrategyMode::ParticleCvar => {
                            x03_particle_cvar::choose_move_x03_particle_cvar(game, state, models)
                        }
                        StrategyMode::MacroRoute => {
                            x04_macro_route::choose_move_x04_macro_route(game, state, models)
                        }
                        StrategyMode::HybridMidMc => {
                            if (3..=5).contains(&game.m) {
                                x02_monte_carlo::choose_move_monte_carlo(game, state, models)
                            } else {
                                x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                                    game, state, models,
                                )
                            }
                        }
                        StrategyMode::AdaptiveRacingMc => {
                            x05_adaptive_racing_mc::choose_move_x05_adaptive_racing(
                                game, state, models,
                            )
                        }
                        StrategyMode::ExpertSwitchHybrid => {
                            x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                game, state, models,
                            )
                        }
                    }
                }
            }
            mod x01_beam_pessimistic {
                use crate::__cargo_equip::crates::ahc061_solver::{AiModel, Game, State};

                fn best_one_step_score(game: &Game, state: &State, models: &[AiModel]) -> f64 {
                    let candidates =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if candidates.is_empty() {
                        return 0.0;
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
                        let score_primary =
                            crate::__cargo_equip::crates::ahc061_solver::absolute_score(
                                game,
                                &crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                    game, state, &primary,
                                ),
                            );

                        let mut secondary = Vec::with_capacity(game.m);
                        secondary.push(candidates[0]);
                        secondary.extend_from_slice(&predicted_secondary);
                        let score_secondary =
                            crate::__cargo_equip::crates::ahc061_solver::absolute_score(
                                game,
                                &crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                    game, state, &secondary,
                                ),
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

                    let mut best_val = f64::NEG_INFINITY;
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

                        let mut primary = Vec::with_capacity(game.m);
                        primary.push(mv);
                        primary.extend_from_slice(&predicted_primary);
                        let score_primary =
                            crate::__cargo_equip::crates::ahc061_solver::absolute_score(
                                game,
                                &crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                    game, state, &primary,
                                ),
                            );

                        let mut secondary = Vec::with_capacity(game.m);
                        secondary.push(mv);
                        secondary.extend_from_slice(&predicted_secondary);
                        let score_secondary =
                            crate::__cargo_equip::crates::ahc061_solver::absolute_score(
                                game,
                                &crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                    game, state, &secondary,
                                ),
                            );

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
                        let score_primary =
                            crate::__cargo_equip::crates::ahc061_solver::absolute_score(
                                game,
                                &next_state,
                            );

                        let mut secondary = Vec::with_capacity(game.m);
                        secondary.push((x, y));
                        secondary.extend_from_slice(&predicted_secondary);
                        let score_secondary =
                            crate::__cargo_equip::crates::ahc061_solver::absolute_score(
                                game,
                                &crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                    game, state, &secondary,
                                ),
                            );

                        let rollout_score =
                            (1.0 - risk_w) * score_primary + risk_w * score_secondary;
                        let base_total = rollout_score + 0.12 * local_score;
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
                    if game.m == 5 && phase <= 0.80 && uncertainty >= 0.18 {
                        beam_width = scored.len();
                    } else if game.m == 6 && phase <= 0.72 && uncertainty >= 0.22 {
                        beam_width = (beam_width + 3).min(scored.len());
                    }

                    let mut best = scored[0].0;
                    let mut best_total = f64::NEG_INFINITY;
                    for (idx, (mv, base_total, next_state)) in scored.iter().enumerate() {
                        if idx >= beam_width {
                            break;
                        }
                        let future = if state.turn + 1 < game.t {
                            best_one_step_score(game, next_state, models)
                        } else {
                            0.0
                        };
                        let total = *base_total + 0.18 * future;
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
                    let sample_count = if game.m >= 7 {
                        10
                    } else if game.m >= 5 {
                        8
                    } else {
                        6
                    };

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
                            let v = crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game,
                                &next_state,
                            );
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
            }
            mod x03_particle_cvar {
                use crate::__cargo_equip::crates::ahc061_solver::{
                    x06_expert_switch_hybrid, AiModel, FastRng, Game, State,
                };

                #[derive(Clone)]
                struct ParticleBank {
                    candidates: Vec<(usize, usize)>,
                    probs_by_particle: Vec<Vec<f64>>,
                    particle_weights: Vec<f64>,
                }

                fn normalize_probs(mut probs: Vec<f64>) -> Vec<f64> {
                    if probs.is_empty() {
                        return probs;
                    }
                    let sum: f64 = probs.iter().copied().sum();
                    if !sum.is_finite() || sum <= 0.0 {
                        return vec![1.0 / probs.len() as f64; probs.len()];
                    }
                    for p in &mut probs {
                        *p = (*p / sum).clamp(0.0, 1.0);
                    }
                    probs
                }

                fn make_particle_model(
                    base: &AiModel,
                    weight_scale: [f64; 4],
                    eps_shift: f64,
                ) -> AiModel {
                    let mut model = base.clone();
                    for (k, scale) in weight_scale.into_iter().enumerate() {
                        model.w[k] = (model.w[k] * scale).clamp(0.08, 2.30);
                    }
                    model.eps_est = (model.eps_est + eps_shift).clamp(0.05, 0.70);
                    model
                }

                fn particle_models_and_weights(base: &AiModel) -> (Vec<AiModel>, Vec<f64>) {
                    let confidence = base.seen as f64 / (base.seen as f64 + 12.0);
                    let uncertainty = base.eps_est.clamp(0.05, 0.70);

                    let models = vec![
                        make_particle_model(base, [1.00, 1.00, 1.00, 1.00], 0.00),
                        make_particle_model(base, [1.12, 0.94, 1.20, 0.86], 0.03),
                        make_particle_model(base, [0.94, 1.14, 0.90, 1.18], 0.09),
                        make_particle_model(base, [1.18, 0.90, 1.08, 0.78], -0.04),
                        make_particle_model(base, [0.86, 0.86, 0.86, 0.86], 0.18),
                    ];

                    let base_w = 0.30 + 0.22 * confidence * (1.0 - uncertainty);
                    let attack_w = 0.17 + 0.08 * (1.0 - uncertainty);
                    let defense_w = 0.17 + 0.07 * uncertainty;
                    let greedy_w = 0.14 + 0.06 * confidence;
                    let noisy_w = 0.10 + 0.28 * (1.0 - confidence) + 0.20 * uncertainty;
                    let weights =
                        normalize_probs(vec![base_w, attack_w, defense_w, greedy_w, noisy_w]);

                    (models, weights)
                }

                fn build_particle_bank(
                    game: &Game,
                    state: &State,
                    player: usize,
                    base: &AiModel,
                ) -> ParticleBank {
                    let candidates = crate::__cargo_equip::crates::ahc061_solver::get_candidates(
                        game, state, player,
                    );
                    if candidates.is_empty() {
                        return ParticleBank {
                            candidates: vec![state.pos[player]],
                            probs_by_particle: vec![vec![1.0]],
                            particle_weights: vec![1.0],
                        };
                    }

                    let (particles, weights) = particle_models_and_weights(base);
                    let mut probs_by_particle = Vec::with_capacity(particles.len());
                    for model in &particles {
                        let probs = crate::__cargo_equip::crates::ahc061_solver::blended_ai_probs(
                            game,
                            state,
                            player,
                            model,
                            &candidates,
                        );
                        probs_by_particle.push(normalize_probs(probs));
                    }

                    ParticleBank {
                        candidates,
                        probs_by_particle,
                        particle_weights: weights,
                    }
                }

                fn cvar_lower(samples: &mut [f64], alpha: f64) -> f64 {
                    if samples.is_empty() {
                        return f64::NEG_INFINITY;
                    }
                    samples.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                    let tail_count = ((samples.len() as f64 * alpha).ceil() as usize)
                        .max(1)
                        .min(samples.len());
                    let mut sum = 0.0;
                    for &v in samples.iter().take(tail_count) {
                        sum += v;
                    }
                    sum / tail_count as f64
                }

                pub(super) fn choose_move_x03_particle_cvar(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    // 初版観測では中人数帯で優位だったため、適用帯を限定する。
                    if !(3..=5).contains(&game.m) {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
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
                    let sample_count = if game.m >= 5 { 16 } else { 12 };
                    let tail_alpha = if game.m >= 5 { 0.30 } else { 0.25 };
                    let cvar_w = if game.m >= 5 { 0.58 } else { 0.45 };
                    let local_w = 0.07;

                    let mut particle_banks = Vec::with_capacity(game.m.saturating_sub(1));
                    for ai_idx in 0..game.m.saturating_sub(1) {
                        particle_banks.push(build_particle_bank(
                            game,
                            state,
                            ai_idx + 1,
                            &models[ai_idx],
                        ));
                    }

                    let seed = ((state.turn as u64 + 1) * 0x517c_c1b7_2722_0a95)
                        ^ (scores[0] as u64)
                        ^ ((game.m as u64) << 32)
                        ^ ((game.u as u64) << 48)
                        ^ 0x3f15_9d7e_a42c_55b1;
                    let mut rng = FastRng::new(seed);

                    let mut best_mv = ranked[0].0;
                    let mut best_val = f64::NEG_INFINITY;
                    for &(mv, local) in ranked.iter().take(candidate_cap) {
                        let mut samples = Vec::with_capacity(sample_count);
                        for _ in 0..sample_count {
                            let mut sampled_moves = Vec::with_capacity(game.m);
                            sampled_moves.push(mv);
                            for bank in &particle_banks {
                                let pidx =
                                    crate::__cargo_equip::crates::ahc061_solver::sample_index(
                                        &bank.particle_weights,
                                        &mut rng,
                                    );
                                let probs = &bank.probs_by_particle[pidx];
                                let midx =
                                    crate::__cargo_equip::crates::ahc061_solver::sample_index(
                                        probs, &mut rng,
                                    );
                                sampled_moves.push(bank.candidates[midx]);
                            }
                            let next_state =
                                crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                    game,
                                    state,
                                    &sampled_moves,
                                );
                            samples.push(
                                crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                    game,
                                    &next_state,
                                ),
                            );
                        }

                        let mean = samples.iter().copied().sum::<f64>() / sample_count as f64;
                        let cvar = cvar_lower(&mut samples, tail_alpha);
                        let total = (1.0 - cvar_w) * mean + cvar_w * cvar + local_w * local;
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
                    x06_expert_switch_hybrid, AiModel, Game, State,
                };

                #[derive(Clone)]
                struct RouteNode {
                    state: State,
                    score: f64,
                }

                fn manhattan(a: (usize, usize), b: (usize, usize)) -> usize {
                    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
                }

                fn choose_target_cells(
                    game: &Game,
                    state: &State,
                    max_targets: usize,
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
                                scored.push(((x, y), w));
                            }
                        }
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

                fn route_increment(
                    game: &Game,
                    prev: &State,
                    next: &State,
                    mv: (usize, usize),
                    target: (usize, usize),
                    local: f64,
                ) -> f64 {
                    let prev_score =
                        crate::__cargo_equip::crates::ahc061_solver::strategic_score(game, prev);
                    let next_score =
                        crate::__cargo_equip::crates::ahc061_solver::strategic_score(game, next);
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
                    let first_state = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                        game, state, &moves,
                    );

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
                            &crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                                game, state, models,
                            ),
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

                    let base_gain =
                        route_increment(game, state, &first_state, first_mv, target, first_local);
                    let mut beam = vec![RouteNode {
                        state: first_state,
                        score: base_gain,
                    }];

                    for step in 1..plan_len {
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
                                let discount = (0.93_f64).powi(step as i32);
                                let inc =
                                    route_increment(game, &node.state, &ns, mv, target, local);
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

                    beam.iter()
                        .map(|n| {
                            n.score
                                + 0.03
                                    * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                        game, &n.state,
                                    )
                        })
                        .fold(f64::NEG_INFINITY, f64::max)
                }

                pub(super) fn choose_move_x04_macro_route(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    // full結果の帯別分析で M=4 のみ優位が大きかったため、適用帯を限定する。
                    if game.m != 4 {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }
                    let phase_cutoff = std::env::var("AHC_X04_PHASE_CUTOFF")
                        .ok()
                        .and_then(|x| x.parse::<f64>().ok())
                        .unwrap_or(0.65);
                    let phase_now = state.turn as f64 / game.t as f64;
                    if phase_now > phase_cutoff {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }

                    let candidates =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if candidates.len() <= 1 {
                        return candidates.first().copied().unwrap_or(state.pos[0]);
                    }

                    let targets = choose_target_cells(game, state, 5);
                    if targets.is_empty() {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
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

                    let mut ranked = Vec::<((usize, usize), f64)>::new();
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
            }
            mod x05_adaptive_racing_mc {
                use crate::__cargo_equip::crates::ahc061_solver::{
                    x01_beam_pessimistic, AiModel, FastRng, Game, State,
                };

                #[derive(Clone, Copy, Debug)]
                struct RacingStat {
                    sum: f64,
                    sum2: f64,
                    count: usize,
                    downside_count: usize,
                }

                impl RacingStat {
                    fn new() -> Self {
                        Self {
                            sum: 0.0,
                            sum2: 0.0,
                            count: 0,
                            downside_count: 0,
                        }
                    }

                    fn push(&mut self, v: f64, is_downside: bool) {
                        self.sum += v;
                        self.sum2 += v * v;
                        self.count += 1;
                        if is_downside {
                            self.downside_count += 1;
                        }
                    }

                    fn mean(self) -> f64 {
                        if self.count == 0 {
                            return f64::NEG_INFINITY;
                        }
                        self.sum / self.count as f64
                    }

                    fn std(self) -> f64 {
                        if self.count <= 1 {
                            return 0.0;
                        }
                        let mean = self.mean();
                        let var = (self.sum2 / self.count as f64 - mean * mean).max(0.0);
                        var.sqrt()
                    }

                    fn stderr(self) -> f64 {
                        if self.count <= 1 {
                            return f64::INFINITY;
                        }
                        self.std() / (self.count as f64).sqrt()
                    }

                    fn downside_prob(self) -> f64 {
                        if self.count == 0 {
                            return 1.0;
                        }
                        self.downside_count as f64 / self.count as f64
                    }
                }

                pub(super) fn choose_move_x05_adaptive_racing(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    // x05は中人数帯を対象にし、それ以外はx01の安定版を再利用する。
                    if !(3..=5).contains(&game.m) {
                        return x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                            game, state, models,
                        );
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
                    let max_rounds = if game.m == 5 { 12 } else { 14 };
                    let min_rounds = 3usize;
                    let confidence_z = 1.05_f64;
                    let base_score =
                        crate::__cargo_equip::crates::ahc061_solver::strategic_score(game, state);
                    let downside_drop = if game.m == 5 { 4200.0 } else { 3200.0 };

                    let ai_options =
                        crate::__cargo_equip::crates::ahc061_solver::build_ai_candidates_and_probs(
                            game, state, models,
                        );
                    let seed = ((state.turn as u64 + 1) * 0x9e37_79b9_7f4a_7c15)
                        ^ (scores[0] as u64)
                        ^ ((game.m as u64) << 32)
                        ^ ((game.u as u64) << 48)
                        ^ 0xa5a5_5a5a_cc33_33cc;
                    let mut rng = FastRng::new(seed);

                    let mut active: Vec<usize> = (0..candidate_cap).collect();
                    let mut stats = vec![RacingStat::new(); candidate_cap];
                    for round in 0..max_rounds {
                        let mut sampled_ai_moves = Vec::with_capacity(game.m.saturating_sub(1));
                        for (cands, probs) in &ai_options {
                            let idx = crate::__cargo_equip::crates::ahc061_solver::sample_index(
                                probs, &mut rng,
                            );
                            sampled_ai_moves.push(cands[idx]);
                        }

                        for &idx in &active {
                            let mv = ranked[idx].0;
                            let mut sampled = Vec::with_capacity(game.m);
                            sampled.push(mv);
                            sampled.extend_from_slice(&sampled_ai_moves);
                            let next_state =
                                crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                    game, state, &sampled,
                                );
                            let v = crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game,
                                &next_state,
                            );
                            let is_downside = v + downside_drop < base_score;
                            stats[idx].push(v, is_downside);
                        }

                        if round + 1 < min_rounds || active.len() <= 3 {
                            continue;
                        }

                        let mut best_lcb = f64::NEG_INFINITY;
                        for &idx in &active {
                            let st = stats[idx];
                            let mean = st.mean();
                            let se = st.stderr();
                            let lcb = if se.is_finite() {
                                mean - confidence_z * se
                            } else {
                                mean - 1.5 * st.std()
                            };
                            if lcb > best_lcb {
                                best_lcb = lcb;
                            }
                        }
                        let prune_margin = 900.0 + 0.0020 * best_lcb.abs();
                        let mut kept = Vec::with_capacity(active.len());
                        for &idx in &active {
                            let st = stats[idx];
                            let mean = st.mean();
                            let se = st.stderr();
                            let ucb = if se.is_finite() {
                                mean + confidence_z * se
                            } else {
                                mean + 1.5 * st.std()
                            };
                            if ucb + prune_margin >= best_lcb {
                                kept.push(idx);
                            }
                        }
                        if kept.len() < 2 {
                            let mut by_mean = active.clone();
                            by_mean.sort_by(|&a, &b| {
                                stats[b]
                                    .mean()
                                    .partial_cmp(&stats[a].mean())
                                    .unwrap_or(std::cmp::Ordering::Equal)
                            });
                            kept = by_mean.into_iter().take(2).collect();
                        }
                        active = kept;
                        if active.len() == 1 {
                            break;
                        }
                    }

                    let risk_w = if game.m == 5 { 0.24 } else { 0.22 };
                    let downside_w = 480.0;
                    let local_w = 0.09;
                    let mut best_idx = active[0];
                    let mut best_total = f64::NEG_INFINITY;
                    for &idx in &active {
                        let st = stats[idx];
                        let mean = st.mean();
                        let std = st.std();
                        let downside_prob = st.downside_prob();
                        let local = ranked[idx].1;
                        let total =
                            mean - risk_w * std - downside_w * downside_prob + local_w * local;
                        if total > best_total {
                            best_total = total;
                            best_idx = idx;
                        }
                    }
                    ranked[best_idx].0
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

            use self::strategy_mode::choose_move;
            pub use self::strategy_mode::{strategy_from_env, StrategyMode};

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

            #[derive(Clone)]
            pub(in crate::__cargo_equip::crates::ahc061_solver) struct AiModel {
                w: [f64; 4],
                eps_est: f64,
                seen: u32,
                mismatch: u32,
            }

            impl AiModel {
                fn new() -> Self {
                    Self {
                        w: [0.64, 0.64, 0.64, 0.64],
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

                // tools/src/lib.rs の近傍順を維持。
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
            ) -> [f64; 4] {
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

            pub(in crate::__cargo_equip::crates::ahc061_solver) fn dot4(
                w: &[f64; 4],
                x: &[f64; 4],
            ) -> f64 {
                w[0] * x[0] + w[1] * x[1] + w[2] * x[2] + w[3] * x[3]
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
                    est_scores[i] = dot4(&model.w, &feat);
                }

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
                let mut temp_pos = moves.to_vec();
                let mut move_counts = HashMap::<(usize, usize), usize>::new();
                for &mv in moves {
                    *move_counts.entry(mv).or_insert(0) += 1;
                }

                let mut collected = vec![false; game.m];
                for i in 0..game.m {
                    let target = temp_pos[i];
                    if move_counts[&target] >= 2 {
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
                    let (x, y) = temp_pos[i];
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
                }

                for i in 0..game.m {
                    if collected[i] {
                        temp_pos[i] = state.pos[i];
                    }
                }
                next.pos = temp_pos;
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
                absolute_score(game, state) + frontier_potential(game, state)
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
                        score += 0.90 * value;
                        score += 0.18 * value * (game.u - level) as f64 / game.u as f64;
                    } else if (x, y) == cur {
                        score -= 0.15 * value;
                    } else {
                        score -= 0.05 * value;
                    }
                } else {
                    let opp = owner as usize;
                    let threat = ((scores[opp] as f64 - s0).max(0.0)) / max_ai;
                    let m5_focus = (1.0 - ((game.m as f64 - 5.0).abs() / 2.0)).clamp(0.0, 1.0);
                    if level == 1 {
                        score += (1.25 + 0.85 * threat) * value;
                        if is_leader[opp] {
                            score += 0.45 * phase * value;
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
                let mut feats = vec![[0.0_f64; 4]; cands.len()];
                for (i, &cand) in cands.iter().enumerate() {
                    let f = ai_features(game, state_before, player, cand);
                    feats[i] = f;
                    est_scores[i] = dot4(&model.w, &f);
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
                        for k in 0..4 {
                            let diff = (feats[obs_idx][k] - feats[pred_idx][k]) / 1000.0;
                            model.w[k] = (model.w[k] + 0.12 * diff).clamp(0.10, 2.00);
                        }
                    }
                }

                for k in 0..4 {
                    model.w[k] = 0.995 * model.w[k] + 0.005 * 0.64;
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

            pub fn run_with_strategy(strategy: StrategyMode) {
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
