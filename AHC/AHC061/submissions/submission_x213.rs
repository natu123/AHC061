pub use __cargo_equip::prelude::*;

use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    unsafe {
        // x47 champion params (for M=4 via x04, and baseline for M=3 via x210)
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
        // Allow x04 beam for M=3 (via x210) and M=4
        env::set_var("AHC_X04_ALLOWED_M", "3,4");
    }
    run_with_strategy(StrategyMode::HybridPortfolio);
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
                    x05_adaptive_racing_mc, x06_expert_switch_hybrid, x07_dual_horizon_route,
                    x08_pressure_frontier, x09_regret_mix, x10_phase_adaptive_mix,
                    x11_contest_frontier_recovery, x12_advisor_vote_ensemble,
                    x13_frontier_consensus, x14_adaptive_risk_lane, x15_band_adaptive_route,
                    x16_safe_recovery_route, x17_mid_band_dual_lane, x18_robust_minmax_guard,
                    x19_frontier_recovery_sweep, x20_band_stage_ensemble, x210_adaptive_beam_route,
                    x211_deep_mc_expectimax, x212_maxn_mcts, x213_hybrid_portfolio,
                    x21_band_stage_adaptive_guard, x22_band_stage_recovery_boost,
                    x23_band_stage_frontier_guard, x24_band_stage_adaptive_switch,
                    x25_race_adaptive_recovery, x26_reactive_frontier_pressure,
                    x64_portfolio_mixer, x67_gear_shift_hybrid, x73_selective_unlocked_macro,
                    x75_risk_gated_unlocked_macro, x76_crossband_route_hybrid, AiModel, Game,
                    State,
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
                    DualHorizonRoute,
                    PressureFrontier,
                    RegretMix,
                    FrontierConsensus,
                    AdaptiveRiskLane,
                    PhaseAdaptiveMix,
                    ContestFrontierRecovery,
                    AdvisorVoteEnsemble,
                    BandAdaptiveRoute,
                    SafeRecoveryRoute,
                    MidBandDualLane,
                    RobustMinmaxGuard,
                    FrontierRecoverySweep,
                    BandStageEnsemble,
                    BandStageAdaptiveGuard,
                    BandStageRecoveryBoost,
                    BandStageFrontierGuard,
                    BandStageAdaptiveSwitch,
                    RaceAdaptiveRecovery,
                    ReactiveFrontierPressure,
                    PortfolioMixer,
                    GearShiftHybrid,
                    SelectiveUnlockedMacro,
                    RiskGatedUnlockedMacro,
                    CrossbandRouteHybrid,
                    AdaptiveBeamRoute,
                    DeepMcExpectimax,
                    MaxnMcts,
                    HybridPortfolio,
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
                        Some("x07") | Some("dual_horizon_route") | Some("dual_horizon") => {
                            StrategyMode::DualHorizonRoute
                        }
                        Some("x08") | Some("pressure_frontier") | Some("frontier") => {
                            StrategyMode::PressureFrontier
                        }
                        Some("x09") | Some("regret_mix") | Some("regret") => {
                            StrategyMode::RegretMix
                        }
                        Some("x10") | Some("phase_adaptive_mix") | Some("phase_mix") => {
                            StrategyMode::PhaseAdaptiveMix
                        }
                        Some("x11")
                        | Some("contest_frontier_recovery")
                        | Some("frontier_recovery") => StrategyMode::ContestFrontierRecovery,
                        Some("x12") | Some("advisor_vote_ensemble") | Some("advisor_vote") => {
                            StrategyMode::AdvisorVoteEnsemble
                        }
                        Some("x13") | Some("frontier_consensus") | Some("consensus") => {
                            StrategyMode::FrontierConsensus
                        }
                        Some("x14") | Some("adaptive_risk_lane") | Some("risk_lane") => {
                            StrategyMode::AdaptiveRiskLane
                        }
                        Some("x15") | Some("band_adaptive_route") | Some("band_route") => {
                            StrategyMode::BandAdaptiveRoute
                        }
                        Some("x16") | Some("safe_recovery_route") | Some("recovery_route") => {
                            StrategyMode::SafeRecoveryRoute
                        }
                        Some("x17") | Some("mid_band_dual_lane") | Some("dual_lane") => {
                            StrategyMode::MidBandDualLane
                        }
                        Some("x18") | Some("robust_minmax_guard") | Some("minmax_guard") => {
                            StrategyMode::RobustMinmaxGuard
                        }
                        Some("x19") | Some("frontier_recovery_sweep") | Some("recovery_sweep") => {
                            StrategyMode::FrontierRecoverySweep
                        }
                        Some("x20") | Some("band_stage_ensemble") | Some("stage_ensemble") => {
                            StrategyMode::BandStageEnsemble
                        }
                        Some("x21")
                        | Some("band_stage_adaptive_guard")
                        | Some("adaptive_guard") => StrategyMode::BandStageAdaptiveGuard,
                        Some("x22")
                        | Some("band_stage_recovery_boost")
                        | Some("recovery_boost") => StrategyMode::BandStageRecoveryBoost,
                        Some("x23")
                        | Some("band_stage_frontier_guard")
                        | Some("frontier_guard") => StrategyMode::BandStageFrontierGuard,
                        Some("x24")
                        | Some("band_stage_adaptive_switch")
                        | Some("adaptive_switch") => StrategyMode::BandStageAdaptiveSwitch,
                        Some("x25")
                        | Some("race_adaptive_recovery")
                        | Some("adaptive_recovery") => StrategyMode::RaceAdaptiveRecovery,
                        Some("x26")
                        | Some("reactive_frontier_pressure")
                        | Some("frontier_pressure")
                        | Some("reactive_pressure") => StrategyMode::ReactiveFrontierPressure,
                        Some("x64")
                        | Some("portfolio_mixer")
                        | Some("portfolio")
                        | Some("meta_portfolio") => StrategyMode::PortfolioMixer,
                        Some("x67")
                        | Some("gear_shift_hybrid")
                        | Some("gear_shift")
                        | Some("novel_hybrid") => StrategyMode::GearShiftHybrid,
                        Some("x73") | Some("selective_unlocked_macro") | Some("sel_unlocked") => {
                            StrategyMode::SelectiveUnlockedMacro
                        }
                        Some("x75")
                        | Some("risk_gated_unlocked_macro")
                        | Some("gated_unlocked") => StrategyMode::RiskGatedUnlockedMacro,
                        Some("x76") | Some("crossband_route_hybrid") | Some("crossband") => {
                            StrategyMode::CrossbandRouteHybrid
                        }
                        Some("x210") | Some("adaptive_beam_route") | Some("adaptive_beam") => {
                            StrategyMode::AdaptiveBeamRoute
                        }
                        Some("x211") | Some("deep_mc_expectimax") | Some("expectimax") => {
                            StrategyMode::DeepMcExpectimax
                        }
                        Some("x212") | Some("maxn_mcts") | Some("mcts_accurate") => {
                            StrategyMode::MaxnMcts
                        }
                        Some("x213") | Some("hybrid_portfolio") | Some("portfolio_hybrid") => {
                            StrategyMode::HybridPortfolio
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
                        StrategyMode::Champion => x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models),
                        StrategyMode::MonteCarloExplore => {
                            x02_monte_carlo::choose_move_monte_carlo(game, state, models)
                        }
                        StrategyMode::ParticleCvar => {
                            x03_particle_cvar::choose_move_x03_particle_cvar(game, state, models)
                        }
                        StrategyMode::MacroRoute => x04_macro_route::choose_move_x04_macro_route(game, state, models),
                        StrategyMode::HybridMidMc => {
                            if (3..=5).contains(&game.m) {
                                x02_monte_carlo::choose_move_monte_carlo(game, state, models)
                            } else {
                                x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models)
                            }
                        }
                        StrategyMode::AdaptiveRacingMc => {
                            x05_adaptive_racing_mc::choose_move_x05_adaptive_racing(game, state, models)
                        }
                        StrategyMode::ExpertSwitchHybrid => {
                            x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models)
                        }
                        StrategyMode::DualHorizonRoute => {
                            x07_dual_horizon_route::choose_move_x07_dual_horizon_route(game, state, models)
                        }
                        StrategyMode::PressureFrontier => {
                            x08_pressure_frontier::choose_move_x08_pressure_frontier(game, state, models)
                        }
                        StrategyMode::RegretMix => x09_regret_mix::choose_move_x09_regret_mix(game, state, models),
                        StrategyMode::PhaseAdaptiveMix => {
                            x10_phase_adaptive_mix::choose_move_x10_phase_adaptive_mix(game, state, models)
                        }
                        StrategyMode::ContestFrontierRecovery => {
                            x11_contest_frontier_recovery::choose_move_x11_contest_frontier_recovery(
                                game, state, models,
                            )
                        }
                        StrategyMode::AdvisorVoteEnsemble => {
                            x12_advisor_vote_ensemble::choose_move_x12_advisor_vote_ensemble(game, state, models)
                        }
                        StrategyMode::FrontierConsensus => {
                            x13_frontier_consensus::choose_move_x13_frontier_consensus(game, state, models)
                        }
                        StrategyMode::AdaptiveRiskLane => {
                            x14_adaptive_risk_lane::choose_move_x14_adaptive_risk_lane(game, state, models)
                        }
                        StrategyMode::BandAdaptiveRoute => {
                            x15_band_adaptive_route::choose_move_x15_band_adaptive_route(game, state, models)
                        }
                        StrategyMode::SafeRecoveryRoute => {
                            x16_safe_recovery_route::choose_move_x16_safe_recovery_route(game, state, models)
                        }
                        StrategyMode::MidBandDualLane => {
                            x17_mid_band_dual_lane::choose_move_x17_mid_band_dual_lane(game, state, models)
                        }
                        StrategyMode::RobustMinmaxGuard => {
                            x18_robust_minmax_guard::choose_move_x18_robust_minmax_guard(game, state, models)
                        }
                        StrategyMode::FrontierRecoverySweep => {
                            x19_frontier_recovery_sweep::choose_move_x19_frontier_recovery_sweep(
                                game, state, models,
                            )
                        }
                        StrategyMode::BandStageEnsemble => {
                            x20_band_stage_ensemble::choose_move_x20_band_stage_ensemble(game, state, models)
                        }
                        StrategyMode::BandStageAdaptiveGuard => {
                            x21_band_stage_adaptive_guard::choose_move_x21_band_stage_adaptive_guard(
                                game,
                                state,
                                models,
                            )
                        }
                        StrategyMode::BandStageRecoveryBoost => {
                            x22_band_stage_recovery_boost::choose_move_x22_band_stage_recovery_boost(
                                game,
                                state,
                                models,
                            )
                        }
                        StrategyMode::BandStageFrontierGuard => {
                            x23_band_stage_frontier_guard::choose_move_x23_band_stage_frontier_guard(
                                game,
                                state,
                                models,
                            )
                        }
                        StrategyMode::BandStageAdaptiveSwitch => {
                            x24_band_stage_adaptive_switch::choose_move_x24_band_stage_adaptive_switch(
                                game,
                                state,
                                models,
                            )
                        }
                        StrategyMode::RaceAdaptiveRecovery => {
                            x25_race_adaptive_recovery::choose_move_x25_race_adaptive_recovery(game, state, models)
                        }
                        StrategyMode::ReactiveFrontierPressure => {
                            x26_reactive_frontier_pressure::choose_move_x26_reactive_frontier_pressure(
                                game,
                                state,
                                models,
                            )
                        }
                        StrategyMode::PortfolioMixer => {
                            x64_portfolio_mixer::choose_move_x64_portfolio_mixer(game, state, models)
                        }
                        StrategyMode::GearShiftHybrid => {
                            x67_gear_shift_hybrid::choose_move_x67_gear_shift_hybrid(game, state, models)
                        }
                        StrategyMode::SelectiveUnlockedMacro => {
                            x73_selective_unlocked_macro::choose_move_x73_selective_unlocked_macro(game, state, models)
                        }
                        StrategyMode::RiskGatedUnlockedMacro => {
                            x75_risk_gated_unlocked_macro::choose_move_x75_risk_gated_unlocked_macro(game, state, models)
                        }
                        StrategyMode::CrossbandRouteHybrid => {
                            x76_crossband_route_hybrid::choose_move_x76_crossband_route_hybrid(game, state, models)
                        }
                        StrategyMode::AdaptiveBeamRoute => {
                            x210_adaptive_beam_route::choose_move_x210_adaptive_beam_route(game, state, models)
                        }
                        StrategyMode::DeepMcExpectimax => {
                            if game.m == 4 {
                                x04_macro_route::choose_move_x04_macro_route(game, state, models)
                            } else {
                                x211_deep_mc_expectimax::choose_move_x211_deep_mc_expectimax(game, state, models)
                            }
                        }
                        StrategyMode::MaxnMcts => {
                            if game.m == 4 {
                                x04_macro_route::choose_move_x04_macro_route(game, state, models)
                            } else {
                                x212_maxn_mcts::choose_move_x212_maxn_mcts(game, state, models)
                            }
                        }
                        StrategyMode::HybridPortfolio => {
                            x213_hybrid_portfolio::choose_move_x213_hybrid_portfolio(game, state, models)
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
                    // Historical runs showed advantages mainly in mid-player bands, so gate usage.
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
                use std::time::Instant;

                use crate::__cargo_equip::crates::ahc061_solver::{
                    x06_expert_switch_hybrid, AiModel, Game, State,
                };
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
                                    let p = estimate_move_pressure(
                                        game,
                                        state,
                                        conflict,
                                        (x, y),
                                        phase,
                                    );
                                    w += -pressure_weight * p * v;
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
                    let prev_score =
                        crate::__cargo_equip::crates::ahc061_solver::strategic_score(game, prev);
                    let next_score =
                        crate::__cargo_equip::crates::ahc061_solver::strategic_score(game, next);
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
                    deadline: Instant,
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
                        if Instant::now() >= deadline {
                            break;
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
                    // AHC_X04_ALLOWED_M で許可された M 値のみ beam search を適用（未設定時は M=4 のみ）
                    let m_allowed = match std::env::var("AHC_X04_ALLOWED_M") {
                        Ok(val) => val
                            .split(',')
                            .any(|s| s.trim().parse::<usize>().map_or(false, |v| v == game.m)),
                        Err(_) => game.m == 4,
                    };
                    if !m_allowed {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
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
                    let time_limit_ms: u64 = std::env::var("AHC_TIME_LIMIT_MS")
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(2000);
                    let per_turn = time_limit_ms / game.t as u64;
                    let budget_ms = (per_turn * 85 / 100).max(5);
                    let deadline = Instant::now() + std::time::Duration::from_millis(budget_ms);

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

                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let s0 = scores[0] as f64;
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let phase = state.turn as f64 / game.t as f64;
                    let conflict_map =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let targets = choose_target_cells(
                        game,
                        state,
                        &conflict_map,
                        target_count,
                        target_pressure_weight,
                        phase,
                    );
                    if targets.is_empty() {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
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

                    let mut best_mv = ranked[0].0;
                    let mut best_score = f64::NEG_INFINITY;
                    for &(mv, local) in ranked.iter().take(candidate_cap) {
                        if Instant::now() >= deadline {
                            break;
                        }
                        let mut best_target_score = f64::NEG_INFINITY;
                        for &target in targets.iter().take(target_eval) {
                            if Instant::now() >= deadline {
                                break;
                            }
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
                    // Keep x05 on mid-player bands and reuse x01 elsewhere for stability.
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
            mod x07_dual_horizon_route {
                use crate::__cargo_equip::crates::ahc061_solver::{
                    x06_expert_switch_hybrid, AiModel, Game, State,
                };

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

                fn leader_flags(scores: &[i64], m: usize) -> Vec<bool> {
                    let mut is_leader = vec![false; m];
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    for p in 1..m {
                        if scores[p] == max_ai_i64 {
                            is_leader[p] = true;
                        }
                    }
                    is_leader
                }

                fn best_local_move(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> ((usize, usize), f64) {
                    let cands =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if cands.is_empty() {
                        return (state.pos[0], f64::NEG_INFINITY);
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
                    let is_leader = leader_flags(&scores, game.m);

                    let mut best_mv = cands[0];
                    let mut best_local = f64::NEG_INFINITY;
                    for mv in cands {
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
                        if local > best_local {
                            best_local = local;
                            best_mv = mv;
                        }
                    }
                    (best_mv, best_local)
                }

                fn rollout_value(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    first_mv: (usize, usize),
                    steps: usize,
                ) -> f64 {
                    let mut cur = state.clone();
                    let mut total = 0.0;
                    for step in 0..steps {
                        let my_mv = if step == 0 {
                            first_mv
                        } else {
                            best_local_move(game, &cur, models).0
                        };
                        let mut moves = Vec::with_capacity(game.m);
                        moves.push(my_mv);
                        moves.extend_from_slice(&predicted_ai_moves(game, &cur, models, step));
                        let next = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, &cur, &moves,
                        );
                        let gain = crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                            game, &next,
                        ) - crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                            game, &cur,
                        );
                        total += (0.94_f64).powi(step as i32) * gain;
                        cur = next;
                    }
                    total
                }

                pub(super) fn choose_move_x07_dual_horizon_route(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    if game.m != 4 {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }
                    let phase_now = state.turn as f64 / game.t as f64;
                    if phase_now > 0.72 {
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
                    let is_leader = leader_flags(&scores, game.m);

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
                    let candidate_cap = ranked.len().min(6);

                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2);
                    let blend = (0.35 + 1.40 * uncertainty + 0.30 * phase_now).clamp(0.35, 0.88);
                    let long_steps = if phase_now > 0.48 { 6 } else { 7 };

                    let mut best_mv = ranked[0].0;
                    let mut best_score = f64::NEG_INFINITY;
                    for &(mv, local) in ranked.iter().take(candidate_cap) {
                        let short_v = rollout_value(game, state, models, mv, 3);
                        let long_v = rollout_value(game, state, models, mv, long_steps);
                        let total = (1.0 - blend) * short_v + blend * long_v + 0.10 * local;
                        if total > best_score {
                            best_score = total;
                            best_mv = mv;
                        }
                    }
                    best_mv
                }
            }
            mod x08_pressure_frontier {
                use crate::__cargo_equip::crates::ahc061_solver::{
                    x06_expert_switch_hybrid, AiModel, Game, State,
                };

                fn frontier_pressure(game: &Game, state: &State, mv: (usize, usize)) -> f64 {
                    let (x, y) = mv;
                    let owner = state.owner[x][y];
                    let level = state.level[x][y];
                    let value = game.v[x][y] as f64;

                    let mut score = 0.0;
                    if owner == -1 {
                        score += 1.00 * value;
                    } else if owner > 0 && level == 1 {
                        score += 1.20 * value;
                    } else if owner == 0 && level < game.u {
                        score += 0.70 * value * (game.u - level) as f64 / game.u as f64;
                    }

                    const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                    for (dx, dy) in DIRS {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if !crate::__cargo_equip::crates::ahc061_solver::in_bounds(game.n, nx, ny) {
                            continue;
                        }
                        let ux = nx as usize;
                        let uy = ny as usize;
                        let ov = game.v[ux][uy] as f64;
                        let o = state.owner[ux][uy];
                        let lv = state.level[ux][uy].max(1) as f64;
                        if o == -1 {
                            score += 0.10 * ov;
                        } else if o > 0 {
                            if state.level[ux][uy] == 1 {
                                score += 0.28 * ov;
                            } else {
                                score += 0.14 * ov / lv;
                            }
                        } else if state.level[ux][uy] < game.u {
                            score += 0.08 * ov;
                        }
                    }
                    score
                }

                pub(super) fn choose_move_x08_pressure_frontier(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    if !(3..=6).contains(&game.m) {
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

                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2);
                    let primary: Vec<(usize, usize)> = top2.iter().map(|x| x.0).collect();
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

                    let candidate_cap = if candidates.len() >= 24 {
                        10
                    } else {
                        candidates.len().min(7)
                    };

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

                    let mut best_mv = ranked[0].0;
                    let mut best_total = f64::NEG_INFINITY;
                    for &(mv, local) in ranked.iter().take(candidate_cap) {
                        let mut m1 = Vec::with_capacity(game.m);
                        m1.push(mv);
                        m1.extend_from_slice(&primary);
                        let ns1 = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &m1,
                        );
                        let gain1 = crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                            game, &ns1,
                        )
                            - crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game, state,
                            );

                        let mut m2 = Vec::with_capacity(game.m);
                        m2.push(mv);
                        m2.extend_from_slice(&secondary);
                        let ns2 = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &m2,
                        );
                        let gain2 = crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                            game, &ns2,
                        )
                            - crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game, state,
                            );

                        let pressure = frontier_pressure(game, state, mv);
                        let risk_penalty =
                            conflict_map[mv.0][mv.1] * game.v[mv.0][mv.1] as f64 * 0.35;
                        let total = 0.58 * gain1
                            + 0.42 * gain2
                            + (0.68 + 0.08 * (1.0 - phase)) * local
                            + 0.20 * pressure
                            - risk_penalty;
                        if total > best_total {
                            best_total = total;
                            best_mv = mv;
                        }
                    }
                    best_mv
                }
            }
            mod x09_regret_mix {
                use std::collections::HashSet;

                use crate::__cargo_equip::crates::ahc061_solver::{
                    x02_monte_carlo, x04_macro_route, x06_expert_switch_hybrid, AiModel, FastRng,
                    Game, State,
                };

                fn collect_candidate_moves(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> Vec<(usize, usize)> {
                    let mut ordered = Vec::<(usize, usize)>::new();
                    ordered.push(x04_macro_route::choose_move_x04_macro_route(
                        game, state, models,
                    ));
                    ordered.push(x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                        game, state, models,
                    ));
                    if (3..=5).contains(&game.m) {
                        ordered.push(x02_monte_carlo::choose_move_monte_carlo(
                            game, state, models,
                        ));
                    }

                    let cands =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if !cands.is_empty() {
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
                        for &mv in &cands {
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
                        ranked.sort_by(|a, b| {
                            b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
                        });
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
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }
                    if candidate_moves.len() == 1 {
                        return candidate_moves[0];
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

                    let mut local_bonus = Vec::with_capacity(candidate_moves.len());
                    for &mv in &candidate_moves {
                        local_bonus.push(
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
                            ),
                        );
                    }

                    let ai_options =
                        crate::__cargo_equip::crates::ahc061_solver::build_ai_candidates_and_probs(
                            game, state, models,
                        );
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
                            let idx = crate::__cargo_equip::crates::ahc061_solver::sample_index(
                                probs, &mut rng,
                            );
                            sampled_ai.push(cands[idx]);
                        }

                        for (i, &mv) in candidate_moves.iter().enumerate() {
                            let mut full_moves = Vec::with_capacity(game.m);
                            full_moves.push(mv);
                            full_moves.extend_from_slice(&sampled_ai);
                            let next_state =
                                crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                    game,
                                    state,
                                    &full_moves,
                                );
                            let v = crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game,
                                &next_state,
                            );
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
            }
            mod x10_phase_adaptive_mix {
                use crate::__cargo_equip::crates::ahc061_solver::{
                    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route,
                    x06_expert_switch_hybrid, x07_dual_horizon_route, AiModel, Game, State,
                };

                fn top_move_candidates(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    phase: f64,
                ) -> Vec<(usize, usize)> {
                    let mut ordered = Vec::<(usize, usize)>::new();
                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2);
                    let cands =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if cands.is_empty() {
                        return ordered;
                    }

                    let mut local_rank = Vec::<((usize, usize), f64)>::new();
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let s0 = scores[0] as f64;
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let conflict =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let is_leader = {
                        let mut v = vec![false; game.m];
                        for p in 1..game.m {
                            if scores[p] == max_ai_i64 {
                                v[p] = true;
                            }
                        }
                        v
                    };

                    for &mv in &cands {
                        let local =
                            crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                game,
                                state,
                                mv,
                                &scores,
                                s0,
                                max_ai_i64,
                                phase,
                                &conflict,
                                state.pos[0],
                                &is_leader,
                            );
                        local_rank.push((mv, local));
                    }
                    local_rank
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    for &(mv, _) in local_rank.iter().take(2) {
                        ordered.push(mv);
                    }

                    if game.m == 4 {
                        if phase <= 0.72 {
                            ordered.push(x04_macro_route::choose_move_x04_macro_route(
                                game, state, models,
                            ));
                        }
                        if uncertainty >= 0.14 {
                            ordered.push(
                                x07_dual_horizon_route::choose_move_x07_dual_horizon_route(
                                    game, state, models,
                                ),
                            );
                        }
                    } else if game.m <= 5 {
                        ordered.push(x02_monte_carlo::choose_move_monte_carlo(
                            game, state, models,
                        ));
                    } else {
                        ordered.push(x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                            game, state, models,
                        ));
                    }

                    if phase >= 0.40 || uncertainty >= 0.18 {
                        ordered.push(x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        ));
                    }
                    if phase >= 0.65 || uncertainty >= 0.33 {
                        ordered.push(x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                            game, state, models,
                        ));
                    }

                    let mut uniq = Vec::with_capacity(ordered.len());
                    for mv in ordered {
                        if !uniq.contains(&mv) {
                            uniq.push(mv);
                        }
                    }
                    uniq
                }

                fn eval_candidate(
                    game: &Game,
                    state: &State,
                    _models: &[AiModel],
                    mv: (usize, usize),
                    top2: &[((usize, usize), (usize, usize), f64)],
                    scores: &[i64],
                    conflict_map: &[Vec<f64>],
                    phase: f64,
                    is_leader: &[bool],
                ) -> f64 {
                    let s0 = scores[0] as f64;
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(top2);

                    let local = crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                        game,
                        state,
                        mv,
                        scores,
                        s0,
                        max_ai_i64,
                        phase,
                        conflict_map,
                        state.pos[0],
                        is_leader,
                    );

                    let primary: Vec<(usize, usize)> = top2.iter().map(|x| x.0).collect();
                    let secondary_cap = if game.m >= 7 && uncertainty >= 0.40 {
                        3
                    } else if game.m >= 6 && uncertainty >= 0.28 {
                        2
                    } else {
                        1
                    };
                    let secondary =
                        crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(
                            scores,
                            top2,
                            secondary_cap,
                        );

                    let mut moves1 = Vec::with_capacity(game.m);
                    moves1.push(mv);
                    moves1.extend_from_slice(&primary);
                    let mut moves2 = Vec::with_capacity(game.m);
                    moves2.push(mv);
                    moves2.extend_from_slice(&secondary);

                    let ns1 = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                        game, state, &moves1,
                    );
                    let ns2 = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                        game, state, &moves2,
                    );
                    let s1 =
                        crate::__cargo_equip::crates::ahc061_solver::strategic_score(game, &ns1);
                    let s2 =
                        crate::__cargo_equip::crates::ahc061_solver::strategic_score(game, &ns2);
                    let base_owner = state.owner[mv.0][mv.1];
                    let lv = state.level[mv.0][mv.1] as f64;
                    let v = game.v[mv.0][mv.1] as f64;
                    let recovery_boost = if base_owner == 0 && lv >= 1.0 {
                        0.12 * v * (game.u as f64 - lv + 1.0) / game.u as f64
                    } else {
                        0.0
                    };
                    let risk = (1.0 - (-conflict_map[mv.0][mv.1]).exp()).clamp(0.0, 1.0);
                    let phase_w = 0.60 + 0.20 * phase;

                    (1.0 - phase_w) * s1 + phase_w * s2 + 0.10 * local + recovery_boost
                        - 0.80 * risk * v
                }

                pub(super) fn choose_move_x10_phase_adaptive_mix(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    let phase = state.turn as f64 / game.t as f64;
                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let candidates = top_move_candidates(game, state, models, phase);
                    if candidates.is_empty() {
                        return state.pos[0];
                    }

                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let conflict_map =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let mut is_leader = vec![false; game.m];
                    for p in 1..game.m {
                        if scores[p] == max_ai_i64 {
                            is_leader[p] = true;
                        }
                    }

                    let mut best_mv = candidates[0];
                    let mut best_val = f64::NEG_INFINITY;
                    for mv in &candidates {
                        let v = eval_candidate(
                            game,
                            state,
                            models,
                            *mv,
                            &top2,
                            &scores,
                            &conflict_map,
                            phase,
                            &is_leader,
                        );
                        if v > best_val {
                            best_val = v;
                            best_mv = *mv;
                        }
                    }
                    best_mv
                }
            }
            mod x11_contest_frontier_recovery {
                use crate::__cargo_equip::crates::ahc061_solver::{
                    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route,
                    x06_expert_switch_hybrid, x07_dual_horizon_route, x08_pressure_frontier,
                    x09_regret_mix, AiModel, Game, State,
                };

                fn frontier_recovery_pressure(
                    game: &Game,
                    state: &State,
                    mv: (usize, usize),
                ) -> f64 {
                    let (x, y) = mv;
                    let owner = state.owner[x][y];
                    let level = state.level[x][y];
                    let value = game.v[x][y] as f64;
                    let mut score = 0.0_f64;

                    if owner == -1 {
                        score += 1.2 * value;
                    } else if owner > 0 && level == 1 {
                        score += 0.9 * value;
                    } else if owner == 0 && level < game.u {
                        score += 0.7 * value * (game.u - level) as f64 / game.u as f64;
                    }

                    const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                    for (dx, dy) in DIRS {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if !crate::__cargo_equip::crates::ahc061_solver::in_bounds(game.n, nx, ny) {
                            continue;
                        }
                        let ox = nx as usize;
                        let oy = ny as usize;
                        let ov = game.v[ox][oy] as f64;
                        let oowner = state.owner[ox][oy];
                        let olv = state.level[ox][oy].max(1) as f64;
                        if oowner == -1 {
                            score += 0.11 * ov;
                        } else if oowner > 0 {
                            if state.level[ox][oy] == 1 {
                                score += 0.26 * ov;
                            } else {
                                score += 0.09 * ov / olv;
                            }
                        } else if state.level[ox][oy] < game.u {
                            score += 0.06 * ov;
                        }
                    }
                    score
                }

                fn choose_advisors(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    phase: f64,
                ) -> Vec<(usize, usize)> {
                    let mut ordered = Vec::<(usize, usize)>::new();
                    if game.m == 4 {
                        ordered.push(x04_macro_route::choose_move_x04_macro_route(
                            game, state, models,
                        ));
                    }
                    if phase <= 0.68 && (3..=5).contains(&game.m) {
                        ordered.push(x02_monte_carlo::choose_move_monte_carlo(
                            game, state, models,
                        ));
                    }
                    ordered.push(x07_dual_horizon_route::choose_move_x07_dual_horizon_route(
                        game, state, models,
                    ));
                    ordered.push(x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                        game, state, models,
                    ));
                    ordered.push(x08_pressure_frontier::choose_move_x08_pressure_frontier(
                        game, state, models,
                    ));
                    ordered.push(x09_regret_mix::choose_move_x09_regret_mix(
                        game, state, models,
                    ));
                    ordered.push(x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                        game, state, models,
                    ));

                    let mut uniq = Vec::with_capacity(ordered.len());
                    for mv in ordered {
                        if !uniq.contains(&mv) {
                            uniq.push(mv);
                        }
                    }
                    uniq
                }

                pub(super) fn choose_move_x11_contest_frontier_recovery(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    let phase = state.turn as f64 / game.t as f64;
                    if game.m == 7 && phase >= 0.90 {
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
                    let conflict =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let my_pos = state.pos[0];
                    let mut is_leader = vec![false; game.m];
                    for p in 1..game.m {
                        if scores[p] == max_ai_i64 {
                            is_leader[p] = true;
                        }
                    }
                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2);

                    let mut ranked_candidates = Vec::<((usize, usize), f64)>::new();
                    for &mv in &candidates {
                        let local =
                            crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                game, state, mv, &scores, s0, max_ai_i64, phase, &conflict, my_pos,
                                &is_leader,
                            );
                        let front = frontier_recovery_pressure(game, state, mv);
                        ranked_candidates.push((mv, local + front * 0.05));
                    }
                    ranked_candidates
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    ranked_candidates.truncate(12);

                    let advisors = choose_advisors(game, state, models, phase);
                    let advisor_set: Vec<(usize, usize)> = advisors
                        .iter()
                        .copied()
                        .filter(|mv| conflict[mv.0][mv.1] < 0.95)
                        .collect();

                    let mut best_mv = ranked_candidates[0].0;
                    let mut best_score = f64::NEG_INFINITY;
                    for &(mv, local_score) in &ranked_candidates {
                        let mut primary = Vec::<(usize, usize)>::with_capacity(game.m);
                        primary.push(mv);
                        let mut secondary = primary.clone();

                        for &((a0x, a0y), (a1x, a1y), _) in &top2 {
                            primary.push((a0x, a0y));
                            if secondary.len() < game.m {
                                secondary.push((a1x, a1y));
                            }
                        }
                        for x in top2.iter().skip(2) {
                            if secondary.len() < game.m {
                                secondary.push(x.1);
                            }
                        }
                        if secondary.len() > game.m {
                            secondary.truncate(game.m);
                        }
                        while secondary.len() < game.m {
                            secondary.push(state.pos[0]);
                        }
                        let next_primary =
                            crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                game, state, &primary,
                            );
                        let next_secondary =
                            crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                game, state, &secondary,
                            );
                        let s_primary =
                            crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game,
                                &next_primary,
                            );
                        let s_secondary =
                            crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game,
                                &next_secondary,
                            );

                        let mut votes = 0.0_f64;
                        for av in &advisor_set {
                            if *av == mv {
                                votes += 1.0;
                            }
                        }
                        let leader_gap =
                            (max_ai_i64 as f64 - scores[0] as f64).max(0.0) / s0.max(1.0);
                        let recovery = if scores[0] < max_ai_i64 { 1.2 } else { 0.8 };
                        let frontier = frontier_recovery_pressure(game, state, mv);
                        let risk_penalty = conflict[mv.0][mv.1] * game.v[mv.0][mv.1] as f64;
                        let total = (0.54 + 0.15 * recovery) * s_primary
                            + (0.38 - 0.15 * recovery) * s_secondary
                            + 0.08 * local_score
                            + 0.05 * votes
                            + (0.10 + 0.12 * uncertainty) * frontier
                            + (leader_gap * 12.0)
                            - 0.75 * risk_penalty;
                        if total > best_score {
                            best_score = total;
                            best_mv = mv;
                        }
                    }
                    best_mv
                }
            }
            mod x12_advisor_vote_ensemble {
                use crate::__cargo_equip::crates::ahc061_solver::{
                    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route,
                    x06_expert_switch_hybrid, x07_dual_horizon_route, x08_pressure_frontier,
                    x09_regret_mix, AiModel, Game, State,
                };

                fn advisor_votes(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    phase: f64,
                    uncertainty: f64,
                ) -> Vec<((usize, usize), f64)> {
                    let mut votes = Vec::<((usize, usize), f64)>::new();
                    let mut add_vote = |mv: (usize, usize), w: f64| {
                        votes.push((mv, w));
                    };

                    if game.m == 4 {
                        add_vote(
                            x04_macro_route::choose_move_x04_macro_route(game, state, models),
                            1.35 + 0.15 * (1.0 - phase),
                        );
                    }
                    add_vote(
                        x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        ),
                        1.00 + 0.10 * uncertainty,
                    );
                    add_vote(
                        x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models),
                        0.85 + 0.10 * (1.0 - phase),
                    );
                    if (3..=5).contains(&game.m) {
                        add_vote(
                            x02_monte_carlo::choose_move_monte_carlo(game, state, models),
                            0.80 + 0.20 * uncertainty,
                        );
                    }
                    add_vote(
                        x07_dual_horizon_route::choose_move_x07_dual_horizon_route(
                            game, state, models,
                        ),
                        0.90,
                    );
                    add_vote(
                        x08_pressure_frontier::choose_move_x08_pressure_frontier(
                            game, state, models,
                        ),
                        0.80,
                    );
                    add_vote(
                        x09_regret_mix::choose_move_x09_regret_mix(game, state, models),
                        0.95,
                    );
                    if phase <= 0.70 {
                        add_vote(
                            x09_regret_mix::choose_move_x09_regret_mix(game, state, models),
                            0.35,
                        );
                    }

                    votes
                }

                fn collect_candidate_moves(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    phase: f64,
                    uncertainty: f64,
                ) -> Vec<(usize, usize)> {
                    let mut ranked_advisors =
                        advisor_votes(game, state, models, phase, uncertainty);
                    ranked_advisors
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

                    let mut candidates = Vec::<(usize, usize)>::new();
                    for (mv, _) in ranked_advisors {
                        if !candidates.contains(&mv) {
                            candidates.push(mv);
                        }
                        if candidates.len() >= 8 {
                            break;
                        }
                    }

                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let s0 = scores[0] as f64;
                    let conflict =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let mut leaders = vec![false; game.m];
                    for p in 1..game.m {
                        if scores[p] == max_ai_i64 {
                            leaders[p] = true;
                        }
                    }

                    for &(mv, local) in
                        &crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0)
                            .iter()
                            .map(|&mv| {
                                (
                                mv,
                                crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                    game,
                                    state,
                                    mv,
                                    &scores,
                                    s0,
                                    max_ai_i64,
                                    phase,
                                    &conflict,
                                    state.pos[0],
                                    &leaders,
                                ),
                            )
                            })
                            .collect::<Vec<_>>()
                    {
                        let _ = local;
                        if candidates.len() < 14 && !candidates.contains(&mv) {
                            candidates.push(mv);
                        }
                    }
                    if candidates.is_empty() {
                        candidates.push(state.pos[0]);
                    }
                    candidates
                }

                pub(super) fn choose_move_x12_advisor_vote_ensemble(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    let phase = state.turn as f64 / game.t as f64;
                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2);
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let s0 = scores[0] as f64;
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let conflict =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let leaders = {
                        let mut flags = vec![false; game.m];
                        for p in 1..game.m {
                            if scores[p] == max_ai_i64 {
                                flags[p] = true;
                            }
                        }
                        flags
                    };

                    let advisor_votes = advisor_votes(game, state, models, phase, uncertainty);
                    let candidates =
                        collect_candidate_moves(game, state, models, phase, uncertainty);
                    let mut best_mv = candidates[0];
                    let mut best_val = f64::NEG_INFINITY;

                    let primary: Vec<(usize, usize)> = top2.iter().map(|x| x.0).collect();
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
                    let base_gap = (max_ai_i64 as f64 - scores[0] as f64).max(0.0) / s0.max(1.0);

                    for &mv in &candidates {
                        let mut vote_weight = 0.0_f64;
                        for &(av, w) in &advisor_votes {
                            if av == mv {
                                vote_weight += w;
                            }
                        }
                        let local =
                            crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                game,
                                state,
                                mv,
                                &scores,
                                s0,
                                max_ai_i64,
                                phase,
                                &conflict,
                                state.pos[0],
                                &leaders,
                            );
                        let mut moves1 = Vec::with_capacity(game.m);
                        moves1.push(mv);
                        moves1.extend_from_slice(&primary);
                        let mut moves2 = Vec::with_capacity(game.m);
                        moves2.push(mv);
                        moves2.extend_from_slice(&secondary);

                        let ns1 = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &moves1,
                        );
                        let ns2 = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &moves2,
                        );
                        let s1 = crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                            game, &ns1,
                        );
                        let s2 = crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                            game, &ns2,
                        );
                        let frontier_penalty = conflict[mv.0][mv.1] * game.v[mv.0][mv.1] as f64;
                        let recover_boost = (1.0 - phase) * 12.0 + (1.0 - uncertainty) * 8.0;
                        let vote_pressure = (0.70 + 0.10 * uncertainty) * vote_weight;

                        let total = (0.68 + 0.12 * (1.0 - base_gap)) * s1
                            + (0.32 - 0.12 * (1.0 - base_gap)) * s2
                            + 0.12 * local
                            + 0.50 * vote_pressure
                            + recover_boost
                            - frontier_penalty
                            + if scores[0] < max_ai_i64 { 18.0 } else { 0.0 };
                        if total > best_val {
                            best_val = total;
                            best_mv = mv;
                        }
                    }
                    best_mv
                }
            }
            mod x13_frontier_consensus {
                use crate::__cargo_equip::crates::ahc061_solver::{
                    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route,
                    x06_expert_switch_hybrid, x07_dual_horizon_route, x08_pressure_frontier,
                    x09_regret_mix, AiModel, Game, State,
                };

                fn frontier_pressure(game: &Game, state: &State, mv: (usize, usize)) -> f64 {
                    let (x, y) = mv;
                    let owner = state.owner[x][y];
                    let level = state.level[x][y];
                    let value = game.v[x][y] as f64;

                    let mut score = 0.0_f64;
                    if owner == -1 {
                        score += 1.1 * value;
                    } else if owner > 0 && level == 1 {
                        score += 0.7 * value;
                    } else if owner == 0 && level < game.u {
                        score += 0.5 * value * (game.u - level) as f64 / game.u as f64;
                    }

                    const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                    for (dx, dy) in DIRS {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if !crate::__cargo_equip::crates::ahc061_solver::in_bounds(game.n, nx, ny) {
                            continue;
                        }
                        let ux = nx as usize;
                        let uy = ny as usize;
                        let ov = game.v[ux][uy] as f64;
                        let oowner = state.owner[ux][uy];
                        if oowner == -1 {
                            score += 0.08 * ov;
                        } else if oowner == 0 {
                            if state.level[ux][uy] < game.u {
                                score += 0.05 * ov;
                            }
                        } else if state.level[ux][uy] == 1 {
                            score += 0.18 * ov;
                        } else {
                            score += 0.03 * ov / state.level[ux][uy] as f64;
                        }
                    }

                    score
                }

                fn advisor_pool(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    phase: f64,
                    uncertainty: f64,
                ) -> Vec<(usize, usize)> {
                    let mut ordered = Vec::<(usize, usize)>::new();
                    ordered.push(x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                        game, state, models,
                    ));
                    ordered.push(x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                        game, state, models,
                    ));

                    if (3..=5).contains(&game.m) {
                        ordered.push(x02_monte_carlo::choose_move_monte_carlo(
                            game, state, models,
                        ));
                    }

                    if game.m == 4 {
                        ordered.push(x04_macro_route::choose_move_x04_macro_route(
                            game, state, models,
                        ));
                    }

                    if phase <= 0.7 {
                        ordered.push(x07_dual_horizon_route::choose_move_x07_dual_horizon_route(
                            game, state, models,
                        ));
                    }

                    if uncertainty >= 0.2 {
                        ordered.push(x08_pressure_frontier::choose_move_x08_pressure_frontier(
                            game, state, models,
                        ));
                    }

                    if game.m >= 6 {
                        ordered.push(x09_regret_mix::choose_move_x09_regret_mix(
                            game, state, models,
                        ));
                    }

                    let mut uniq = Vec::with_capacity(ordered.len());
                    for mv in ordered {
                        if !uniq.contains(&mv) {
                            uniq.push(mv);
                        }
                    }
                    uniq
                }

                pub(super) fn choose_move_x13_frontier_consensus(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    let phase = state.turn as f64 / game.t as f64;
                    let candidates =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if candidates.is_empty() {
                        return state.pos[0];
                    }

                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2);
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let s0 = scores[0] as f64;
                    let conflict =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );

                    let mut leaders = vec![false; game.m];
                    for p in 1..game.m {
                        if scores[p] == max_ai_i64 {
                            leaders[p] = true;
                        }
                    }

                    let advisors = advisor_pool(game, state, models, phase, uncertainty);
                    let max_value = game.v.iter().flatten().copied().max().unwrap_or(1) as f64;
                    let primary: Vec<(usize, usize)> = top2.iter().map(|x| x.0).collect();
                    let secondary_cap = if uncertainty >= 0.4 {
                        3
                    } else if uncertainty >= 0.25 {
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

                    let mut best_mv = candidates[0];
                    let mut best_val = f64::NEG_INFINITY;

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
                                &conflict,
                                state.pos[0],
                                &leaders,
                            );

                        let mut moves_p = Vec::with_capacity(game.m);
                        moves_p.push(mv);
                        moves_p.extend_from_slice(&primary);
                        let mut moves_s = Vec::with_capacity(game.m);
                        moves_s.push(mv);
                        moves_s.extend_from_slice(&secondary);

                        let ns_p = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &moves_p,
                        );
                        let ns_s = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &moves_s,
                        );
                        let s_p = crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                            game, &ns_p,
                        );
                        let s_s = crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                            game, &ns_s,
                        );

                        let mut vote = 0.0_f64;
                        for av in &advisors {
                            if *av == mv {
                                vote += 1.0;
                            }
                        }
                        let front = frontier_pressure(game, state, mv) / max_value;
                        let risk = conflict[mv.0][mv.1];
                        let gap = (max_ai_i64 as f64 - scores[0] as f64).max(0.0) / s0.max(1.0);

                        let val = (0.56 + 0.04 * (1.0 - phase)) * s_p
                            + (0.36 + 0.12 * (1.0 - uncertainty)) * s_s
                            + (0.08 + 0.10 * (1.0 - gap)) * local
                            + 0.25 * vote
                            + 8.0 * front
                            - 0.90 * risk * (1.0 + phase) * game.v[mv.0][mv.1] as f64;

                        if val > best_val {
                            best_val = val;
                            best_mv = mv;
                        }
                    }

                    if best_val.is_finite() {
                        best_mv
                    } else {
                        x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models)
                    }
                }
            }
            mod x14_adaptive_risk_lane {
                use crate::__cargo_equip::crates::ahc061_solver::{
                    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route,
                    x06_expert_switch_hybrid, x07_dual_horizon_route, x08_pressure_frontier,
                    x09_regret_mix, AiModel, Game, State,
                };

                fn build_candidates(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    phase: f64,
                    uncertainty: f64,
                ) -> Vec<(usize, usize)> {
                    let mut cand = Vec::<(usize, usize)>::new();

                    let base =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    for mv in base {
                        if !cand.contains(&mv) {
                            cand.push(mv);
                        }
                    }

                    if phase <= 0.70 {
                        let m6 = x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                        let m1 = x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                            game, state, models,
                        );
                        if !cand.contains(&m6) {
                            cand.push(m6);
                        }
                        if !cand.contains(&m1) {
                            cand.push(m1);
                        }
                    } else if !cand.contains(&state.pos[0]) {
                        cand.push(state.pos[0]);
                    }

                    if (3..=5).contains(&game.m) {
                        let mv2 = x02_monte_carlo::choose_move_monte_carlo(game, state, models);
                        if !cand.contains(&mv2) {
                            cand.push(mv2);
                        }
                    }

                    if game.m == 4 && phase <= 0.80 {
                        let mv4 = x04_macro_route::choose_move_x04_macro_route(game, state, models);
                        if !cand.contains(&mv4) {
                            cand.push(mv4);
                        }
                    }

                    if phase <= 0.55 {
                        let mv7 = x07_dual_horizon_route::choose_move_x07_dual_horizon_route(
                            game, state, models,
                        );
                        if !cand.contains(&mv7) {
                            cand.push(mv7);
                        }
                    }

                    if phase >= 0.45 || uncertainty >= 0.22 {
                        let mv8 = x08_pressure_frontier::choose_move_x08_pressure_frontier(
                            game, state, models,
                        );
                        if !cand.contains(&mv8) {
                            cand.push(mv8);
                        }
                    }

                    if uncertainty >= 0.16 {
                        let mv9 = x09_regret_mix::choose_move_x09_regret_mix(game, state, models);
                        if !cand.contains(&mv9) {
                            cand.push(mv9);
                        }
                    }

                    if cand.len() > 16 {
                        cand.truncate(16);
                    }
                    cand
                }

                pub(super) fn choose_move_x14_adaptive_risk_lane(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    let phase = state.turn as f64 / game.t as f64;
                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2);
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let conflict =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let s0 = scores[0] as f64;

                    let mut leaders = vec![false; game.m];
                    for p in 1..game.m {
                        if scores[p] == max_ai_i64 {
                            leaders[p] = true;
                        }
                    }

                    let candidates = build_candidates(game, state, models, phase, uncertainty);
                    if candidates.is_empty() {
                        return state.pos[0];
                    }

                    let primary: Vec<(usize, usize)> = top2.iter().map(|x| x.0).collect();
                    let secondary_cap = if game.m >= 8 {
                        3
                    } else if game.m >= 6 {
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

                    let mut best_mv = candidates[0];
                    let mut best_val = f64::NEG_INFINITY;

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
                                &conflict,
                                state.pos[0],
                                &leaders,
                            );

                        let mut m1 = Vec::with_capacity(game.m);
                        m1.push(mv);
                        m1.extend_from_slice(&primary);
                        let mut m2 = Vec::with_capacity(game.m);
                        m2.push(mv);
                        m2.extend_from_slice(&secondary);

                        let ns1 = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &m1,
                        );
                        let ns2 = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &m2,
                        );
                        let s1 = crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                            game, &ns1,
                        );
                        let s2 = crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                            game, &ns2,
                        );

                        let risk = conflict[mv.0][mv.1];
                        let value = (0.68 - 0.12 * phase) * s1
                            + (0.32 + 0.12 * phase) * s2
                            + (0.05 + 0.20 * uncertainty) * local
                            - (0.90 + 0.40 * uncertainty) * risk * game.v[mv.0][mv.1] as f64;

                        if value > best_val {
                            best_val = value;
                            best_mv = mv;
                        }
                    }

                    if best_val.is_finite() {
                        best_mv
                    } else {
                        x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models)
                    }
                }
            }
            mod x15_band_adaptive_route {
                use crate::__cargo_equip::crates::ahc061_solver::{
                    x04_macro_route, x06_expert_switch_hybrid, AiModel, Game, State,
                };

                #[derive(Clone)]
                struct RouteNode {
                    state: State,
                    score: f64,
                }

                fn manhattan(a: (usize, usize), b: (usize, usize)) -> usize {
                    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
                }

                fn targets_for_band(
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
                                w = 1.22 * v;
                            } else if owner == 0 && level < game.u {
                                w = 0.75 * v * (game.u - level) as f64 / game.u as f64;
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
                    uncertainty: f64,
                ) -> Vec<(usize, usize)> {
                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let primary: Vec<(usize, usize)> = top2.iter().map(|x| x.0).collect();
                    if step >= 3 {
                        return primary;
                    }
                    let secondary_cap = if game.m >= 7 && uncertainty >= 0.36 {
                        3
                    } else if game.m >= 6 && uncertainty >= 0.24 {
                        2
                    } else {
                        1
                    };
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let secondary =
                        crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(
                            &scores,
                            &top2,
                            secondary_cap,
                        );
                    if uncertainty >= 0.18 && step % 2 == 1 {
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
                    let route_bonus = (d0 - d1).clamp(-8.0, 8.0);
                    gain + 48.0 * route_bonus + 0.05 * local
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
                    uncertainty: f64,
                ) -> f64 {
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let s0 = scores[0] as f64;
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let phase = state.turn as f64 / game.t as f64;
                    let conflict =
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
                    let local = crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                        game, state, first_mv, &scores, s0, max_ai_i64, phase, &conflict, cur,
                        &is_leader,
                    );

                    let first_moves = {
                        let mut m = Vec::with_capacity(game.m);
                        m.push(first_mv);
                        m.extend_from_slice(&predicted_ai_moves(
                            game,
                            state,
                            models,
                            0,
                            uncertainty,
                        ));
                        crate::__cargo_equip::crates::ahc061_solver::simulate_turn(game, state, &m)
                    };

                    let first_inc =
                        route_increment(game, state, &first_moves, first_mv, target, local);
                    let mut beam = vec![RouteNode {
                        state: first_moves,
                        score: first_inc,
                    }];

                    for step in 1..plan_len {
                        let mut next_beam = Vec::<RouteNode>::new();
                        for node in &beam {
                            let scores = crate::__cargo_equip::crates::ahc061_solver::calc_scores(
                                game,
                                &node.state,
                            );
                            let max_i = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                            let phase_cur = node.state.turn as f64 / game.t as f64;
                            let conflict_cur =
                                crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                                    game,
                                    &node.state,
                                    models,
                                );
                            let now = node.state.pos[0];
                            let mut is_leader = vec![false; game.m];
                            for p in 1..game.m {
                                if scores[p] == max_i {
                                    is_leader[p] = true;
                                }
                            }
                            let mut cands =
                                crate::__cargo_equip::crates::ahc061_solver::get_candidates(
                                    game,
                                    &node.state,
                                    0,
                                );
                            if cands.is_empty() {
                                continue;
                            }
                            let s0_cur = scores[0] as f64;
                            cands.sort_by(|&a, &b| {
                                let la = crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                    game,
                                    &node.state,
                                    a,
                                    &scores,
                                    s0_cur,
                                    max_i,
                                    phase_cur,
                                    &conflict_cur,
                                    now,
                                    &is_leader,
                                );
                                let lb = crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                    game,
                                    &node.state,
                                    b,
                                    &scores,
                                    s0_cur,
                                    max_i,
                                    phase_cur,
                                    &conflict_cur,
                                    now,
                                    &is_leader,
                                );
                                lb.partial_cmp(&la).unwrap_or(std::cmp::Ordering::Equal)
                            });

                            for &mv in cands.iter().take(branch_width) {
                                let mut full_moves = Vec::with_capacity(game.m);
                                full_moves.push(mv);
                                full_moves.extend_from_slice(&predicted_ai_moves(
                                    game,
                                    &node.state,
                                    models,
                                    step,
                                    uncertainty,
                                ));
                                let ns = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                    game,
                                    &node.state,
                                    &full_moves,
                                );
                                let inc =
                                    route_increment(game, &node.state, &ns, mv, target, local);
                                let discount = (0.94_f64).powi(step as i32);
                                let route_bias =
                                    crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                        game, &ns,
                                    ) * 0.01;
                                next_beam.push(RouteNode {
                                    state: ns,
                                    score: node.score + discount * inc + route_bias,
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
                                + 0.02
                                    * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                        game, &n.state,
                                    )
                        })
                        .fold(f64::NEG_INFINITY, f64::max)
                }

                fn band_plan(
                    _game: &Game,
                    m: usize,
                    phase: f64,
                    uncertainty: f64,
                    conflict: f64,
                ) -> (f64, usize, usize, usize, usize) {
                    match m {
                        4 => {
                            let cutoff = if uncertainty >= 0.24 { 0.74 } else { 0.82 };
                            if phase >= cutoff {
                                (1.0, 5, 2, 2, 10)
                            } else if phase >= 0.55 {
                                (0.7, 6, 2, 2, 8)
                            } else {
                                let w = if phase <= 0.25 { 0.4 } else { 0.7 };
                                let target_cap = if conflict <= 0.30 { 5 } else { 4 };
                                (w, 7, 2, 2, target_cap)
                            }
                        }
                        5 => {
                            let conflict_hi = if conflict >= 0.90 { 0.5 } else { 0.65 };
                            if phase >= 0.66 || uncertainty >= 0.38 {
                                (0.6, 5, 2, 1, 5)
                            } else if phase >= 0.45 {
                                (0.7, 6, 2, 2, 4)
                            } else {
                                (conflict_hi, 5, 2, 2, 5)
                            }
                        }
                        6 => {
                            if phase >= 0.60 || uncertainty >= 0.36 {
                                (0.3, 4, 1, 1, 4)
                            } else if phase >= 0.40 {
                                (0.5, 5, 1, 2, 4)
                            } else {
                                (0.6, 5, 1, 2, 4)
                            }
                        }
                        _ => (0.0, 0, 0, 0, 0),
                    }
                }

                pub(super) fn choose_move_x15_band_adaptive_route(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    if !(4..=6).contains(&game.m) {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }

                    let phase = state.turn as f64 / game.t as f64;
                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2);
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let s0 = scores[0] as f64;
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let conflict_map =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let mut is_leader = vec![false; game.m];
                    for p in 1..game.m {
                        if scores[p] == max_ai_i64 {
                            is_leader[p] = true;
                        }
                    }

                    let cands =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if cands.len() <= 1 {
                        return cands.first().copied().unwrap_or(state.pos[0]);
                    }

                    let (weight, plan_len, branch_width, target_cap, local_cap) = band_plan(
                        game,
                        game.m,
                        phase,
                        uncertainty,
                        conflict_map[state.pos[0].0][state.pos[0].1],
                    );
                    if weight <= 0.0 {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }

                    let mut ranked = Vec::<((usize, usize), f64)>::new();
                    for &mv in &cands {
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
                                state.pos[0],
                                &is_leader,
                            );
                        ranked.push((mv, local));
                    }
                    ranked
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

                    let mut use_route = false;
                    if game.m == 4 && phase <= 0.82 {
                        use_route = true;
                    } else if game.m == 5 && phase <= 0.66 {
                        use_route = true;
                    } else if game.m == 6 && phase <= 0.58 && uncertainty < 0.36 {
                        use_route = true;
                    }
                    if game.m == 5 {
                        if uncertainty >= 0.35 || phase > 0.64 {
                            use_route = false;
                        }
                    }

                    let target_count = target_cap.max(1).min(8);
                    let targets = targets_for_band(game, state, target_count);
                    if !use_route || targets.is_empty() {
                        let mut fallback = ranked[0].0;
                        let mut fallback_score = f64::NEG_INFINITY;
                        for &(mv, local) in ranked.iter().take(local_cap.max(3).min(ranked.len())) {
                            let mut m = Vec::with_capacity(game.m);
                            m.push(mv);
                            let top2 = crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(game, state, models);
                            let uncertainty =
                                crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(
                                    &top2,
                                );
                            if uncertainty > 0.22 {
                                let secondary = crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(&scores, &top2, 1);
                                m.extend(secondary);
                            } else {
                                m.extend(top2.iter().map(|x| x.0));
                            }
                            let next = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                game, state, &m,
                            );
                            let value = crate::__cargo_equip::crates::ahc061_solver::strategic_score(game, &next) + 0.08 * local + 0.10 * (crate::__cargo_equip::crates::ahc061_solver::strategic_score(game, &next) - crate::__cargo_equip::crates::ahc061_solver::strategic_score(game, state));
                            if value > fallback_score {
                                fallback_score = value;
                                fallback = mv;
                            }
                        }
                        return if fallback_score.is_finite() {
                            fallback
                        } else {
                            x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                game, state, models,
                            )
                        };
                    }

                    let mut best_mv = ranked[0].0;
                    let mut best_score = f64::NEG_INFINITY;
                    let beam_width = if game.m == 6 {
                        4
                    } else if phase > 0.5 {
                        5
                    } else {
                        6
                    };
                    for &(mv, local) in ranked.iter().take(local_cap.min(ranked.len())) {
                        let mut best_target = f64::NEG_INFINITY;
                        for &target in targets.iter().take(4) {
                            let rs = beam_route_score(
                                game,
                                state,
                                models,
                                mv,
                                target,
                                plan_len,
                                beam_width.max(2),
                                branch_width.max(1),
                                uncertainty,
                            );
                            if rs > best_target {
                                best_target = rs;
                            }
                        }
                        let route_bonus = if game.m == 4 { 1.0 } else { 0.82 };
                        let conflict = conflict_map[mv.0][mv.1];
                        let total = weight * best_target + route_bonus * local + 0.08 * local
                            - 0.70 * conflict * game.v[mv.0][mv.1] as f64;
                        if total > best_score {
                            best_score = total;
                            best_mv = mv;
                        }
                    }

                    if best_score.is_finite() {
                        best_mv
                    } else {
                        let fallback =
                            x04_macro_route::choose_move_x04_macro_route(game, state, models);
                        if game.m <= 5 || fallback == state.pos[0] {
                            x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                game, state, models,
                            )
                        } else {
                            fallback
                        }
                    }
                }
            }
            mod x16_safe_recovery_route {
                use crate::__cargo_equip::crates::ahc061_solver::{
                    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route,
                    x06_expert_switch_hybrid, AiModel, Game, State,
                };

                fn recovery_pressure(game: &Game, state: &State, cand: (usize, usize)) -> f64 {
                    let (x, y) = cand;
                    let owner = state.owner[x][y];
                    let level = state.level[x][y];
                    let value = game.v[x][y] as f64;
                    let mut score = 0.0;

                    if owner == -1 {
                        score += 0.9 * value;
                    } else if owner == 0 {
                        if level < game.u {
                            score +=
                                (0.60 + 0.20 * (game.u - level) as f64 / game.u as f64) * value;
                        } else {
                            score -= 0.04 * value;
                        }
                    } else if level == 1 {
                        score += 1.05 * value;
                    } else {
                        score += 0.20 * value;
                    }
                    score
                }

                fn candidate_pool(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    phase: f64,
                    uncertainty: f64,
                ) -> Vec<(usize, usize)> {
                    let mut out = Vec::<(usize, usize)>::new();
                    let base =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    let mut base_local = Vec::<((usize, usize), f64)>::new();
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let s0 = scores[0] as f64;
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let conflict =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let mut leaders = vec![false; game.m];
                    for p in 1..game.m {
                        if scores[p] == max_ai_i64 {
                            leaders[p] = true;
                        }
                    }
                    for &mv in base.iter() {
                        let local =
                            crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                game,
                                state,
                                mv,
                                &scores,
                                s0,
                                max_ai_i64,
                                phase,
                                &conflict,
                                state.pos[0],
                                &leaders,
                            );
                        base_local.push((mv, local));
                    }
                    base_local
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    for (mv, _) in base_local.into_iter().take(7) {
                        if !out.contains(&mv) {
                            out.push(mv);
                        }
                    }

                    let x04_mv = x04_macro_route::choose_move_x04_macro_route(game, state, models);
                    if phase <= 0.72 {
                        if !out.contains(&x04_mv) {
                            out.push(x04_mv);
                        }
                    }

                    if game.m == 5 || uncertainty >= 0.20 {
                        let x02_mv = x02_monte_carlo::choose_move_monte_carlo(game, state, models);
                        if !out.contains(&x02_mv) {
                            out.push(x02_mv);
                        }
                    }

                    if uncertainty >= 0.30 {
                        let x06_mv = x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                        if !out.contains(&x06_mv) {
                            out.push(x06_mv);
                        }
                    } else {
                        let x01_mv = x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                            game, state, models,
                        );
                        if !out.contains(&x01_mv) {
                            out.push(x01_mv);
                        }
                    }

                    out
                }

                pub(super) fn choose_move_x16_safe_recovery_route(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    if !(4..=6).contains(&game.m) {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }
                    let phase = state.turn as f64 / game.t as f64;
                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2);
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let s0 = scores[0] as f64;
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let current_score =
                        crate::__cargo_equip::crates::ahc061_solver::strategic_score(game, state);
                    let conflict =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let mut leaders = vec![false; game.m];
                    for p in 1..game.m {
                        if scores[p] == max_ai_i64 {
                            leaders[p] = true;
                        }
                    }

                    let primary: Vec<(usize, usize)> = top2.iter().map(|x| x.0).collect();
                    let secondary =
                        crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(
                            &scores, &top2, 2,
                        );

                    let mut best_mv = state.pos[0];
                    let mut best_val = f64::NEG_INFINITY;
                    for mv in candidate_pool(game, state, models, phase, uncertainty) {
                        let local =
                            crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                game,
                                state,
                                mv,
                                &scores,
                                s0,
                                max_ai_i64,
                                phase,
                                &conflict,
                                state.pos[0],
                                &leaders,
                            );

                        let mut m1 = Vec::with_capacity(game.m);
                        m1.push(mv);
                        m1.extend_from_slice(&primary);
                        let mut m2 = Vec::with_capacity(game.m);
                        m2.push(mv);
                        m2.extend_from_slice(&secondary);
                        let ns1 = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &m1,
                        );
                        let ns2 = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &m2,
                        );
                        let gain1 = crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                            game, &ns1,
                        ) - current_score;
                        let gain2 = crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                            game, &ns2,
                        ) - current_score;

                        let recovery = recovery_pressure(game, state, mv);
                        let conflict_penalty = (1.0 - (-conflict[mv.0][mv.1]).exp())
                            * game.v[mv.0][mv.1] as f64
                            * if game.m >= 6 { 1.10 } else { 0.78 };
                        let future_risk = -0.18
                            * ((game.v[mv.0][mv.1] as f64).min(500.0) / 500.0)
                            * (0.30 + phase);

                        let val = if phase < 0.45 {
                            0.68 * gain1 + 0.22 * gain2 + 0.14 * local + 0.24 * recovery
                                - conflict_penalty
                                + future_risk
                        } else {
                            0.56 * gain1 + 0.28 * gain2 + 0.10 * local + 0.16 * recovery
                                - 0.95 * conflict_penalty
                        };

                        if val > best_val {
                            best_val = val;
                            best_mv = mv;
                        }
                    }

                    if best_val.is_finite() {
                        best_mv
                    } else {
                        x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models)
                    }
                }
            }
            mod x17_mid_band_dual_lane {
                use crate::__cargo_equip::crates::ahc061_solver::{
                    x04_macro_route, x06_expert_switch_hybrid, x07_dual_horizon_route, AiModel,
                    Game, State,
                };

                fn lane_candidates(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    phase: f64,
                    uncertainty: f64,
                ) -> Vec<(usize, usize)> {
                    let mut cands = Vec::<(usize, usize)>::new();
                    let local =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    for mv in local {
                        if !cands.contains(&mv) {
                            cands.push(mv);
                        }
                    }

                    if phase <= 0.70 {
                        let route_mv =
                            x04_macro_route::choose_move_x04_macro_route(game, state, models);
                        if !cands.contains(&route_mv) {
                            cands.push(route_mv);
                        }
                    }

                    if phase <= 0.42 || uncertainty >= 0.28 {
                        let dual_mv = x07_dual_horizon_route::choose_move_x07_dual_horizon_route(
                            game, state, models,
                        );
                        if !cands.contains(&dual_mv) {
                            cands.push(dual_mv);
                        }
                    }

                    if phase > 0.76 {
                        let stable_mv = x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                        if !cands.contains(&stable_mv) {
                            cands.push(stable_mv);
                        }
                    }

                    cands
                }

                fn quick_rollout(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    first_mv: (usize, usize),
                    steps: usize,
                ) -> f64 {
                    let mut cur = state.clone();
                    for step in 0..steps {
                        let mut moves = Vec::with_capacity(game.m);
                        if step == 0 {
                            moves.push(first_mv);
                            let top2 = crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(game, &cur, models);
                            let uncertainty =
                                crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(
                                    &top2,
                                );
                            moves.extend(top2.iter().take(2).map(|x| x.0));
                            if uncertainty >= 0.24 {
                                moves.pop();
                                let secondary = crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(
                                    &crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, &cur),
                                    &top2,
                                    2,
                                );
                                moves.extend_from_slice(&secondary);
                            }
                        } else {
                            let scores = crate::__cargo_equip::crates::ahc061_solver::calc_scores(
                                game, &cur,
                            );
                            let s0 = scores[0] as f64;
                            let max_ai_i64 =
                                scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                            let phase = cur.turn as f64 / game.t as f64;
                            let conflict =
                                crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                                    game, &cur, models,
                                );
                            let mut best = cur.pos[0];
                            let mut best_v = f64::NEG_INFINITY;
                            let mut is_leader = vec![false; game.m];
                            for p in 1..game.m {
                                if scores[p] == max_ai_i64 {
                                    is_leader[p] = true;
                                }
                            }
                            for mv in crate::__cargo_equip::crates::ahc061_solver::get_candidates(
                                game, &cur, 0,
                            ) {
                                let v = crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                    game,
                                    &cur,
                                    mv,
                                    &scores,
                                    s0,
                                    max_ai_i64,
                                    phase,
                                    &conflict,
                                    cur.pos[0],
                                    &is_leader,
                                );
                                if v > best_v {
                                    best_v = v;
                                    best = mv;
                                }
                            }
                            moves.push(best);
                            let top2 = crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(game, &cur, models);
                            moves.extend(top2.iter().take(1).map(|x| x.0));
                        }
                        cur = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, &cur, &moves,
                        );
                    }
                    crate::__cargo_equip::crates::ahc061_solver::strategic_score(game, &cur)
                        - crate::__cargo_equip::crates::ahc061_solver::strategic_score(game, state)
                }

                fn conflict_ratio(conflict: &[Vec<f64>], game: &Game, mv: (usize, usize)) -> f64 {
                    let p_any = 1.0 - (-conflict[mv.0][mv.1]).exp();
                    p_any * 0.2 + conflict[mv.0][mv.1] * 0.8 + 1e-6 * game.v[mv.0][mv.1] as f64
                }

                pub(super) fn choose_move_x17_mid_band_dual_lane(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    if !(4..=6).contains(&game.m) {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }

                    let phase = state.turn as f64 / game.t as f64;
                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2);
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let s0 = scores[0] as f64;
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let conflict =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let mut is_leader = vec![false; game.m];
                    for p in 1..game.m {
                        if scores[p] == max_ai_i64 {
                            is_leader[p] = true;
                        }
                    }

                    let base_score =
                        crate::__cargo_equip::crates::ahc061_solver::strategic_score(game, state);
                    let cands = lane_candidates(game, state, models, phase, uncertainty);
                    if cands.is_empty() {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }

                    let mut best_mv = cands[0];
                    let mut best_val = f64::NEG_INFINITY;
                    for mv in cands {
                        let local =
                            crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                game,
                                state,
                                mv,
                                &scores,
                                s0,
                                max_ai_i64,
                                phase,
                                &conflict,
                                state.pos[0],
                                &is_leader,
                            );

                        let short = quick_rollout(game, state, models, mv, 2);
                        let long = quick_rollout(game, state, models, mv, 4);
                        let risk = conflict[mv.0][mv.1];
                        let mut bonus = 0.0;
                        for d in [0_i32, 1, -1] {
                            for e in [0_i32, 1, -1] {
                                let nx = mv.0 as i32 + d;
                                let ny = mv.1 as i32 + e;
                                if nx < 0 || ny < 0 || nx >= game.n as i32 || ny >= game.n as i32 {
                                    continue;
                                }
                                let v = game.v[nx as usize][ny as usize] as f64;
                                if state.owner[nx as usize][ny as usize] == 0 {
                                    bonus += 0.005 * v;
                                } else if state.owner[nx as usize][ny as usize] > 0 {
                                    bonus += 0.018 * v;
                                } else {
                                    bonus += 0.009 * v;
                                }
                            }
                        }
                        let mix = 0.35 + 0.20 * phase + 0.10 * (1.0 - uncertainty);
                        let v = mix * short + (1.0 - mix) * long + 0.10 * local + 0.30 * bonus
                            - 0.75 * risk * conflict_ratio(&conflict, game, mv);

                        if v > best_val {
                            best_val = v;
                            best_mv = mv;
                        }
                    }

                    if best_val.is_finite() {
                        let next_state_val =
                            crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game, state,
                            ) + best_val;
                        if next_state_val < base_score {
                            x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                game, state, models,
                            )
                        } else {
                            best_mv
                        }
                    } else {
                        x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models)
                    }
                }
            }
            mod x18_robust_minmax_guard {
                use crate::__cargo_equip::crates::ahc061_solver::{
                    x04_macro_route, x06_expert_switch_hybrid, AiModel, Game, State,
                };

                fn opponent_set_by_mode(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    mode: u8,
                    fallback: &Vec<Vec<(usize, usize)>>,
                ) -> Vec<Vec<(usize, usize)>> {
                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let mut out: Vec<Vec<(usize, usize)>> =
                        Vec::with_capacity(game.m.saturating_sub(1));
                    for ai_idx in 0..(game.m.saturating_sub(1)) {
                        let player = ai_idx + 1;
                        let cands = crate::__cargo_equip::crates::ahc061_solver::get_candidates(
                            game, state, player,
                        );
                        if cands.is_empty() {
                            out.push(vec![state.pos[player]]);
                            continue;
                        }
                        let scores =
                            crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                        let secondary =
                            crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(
                                &scores, &top2, 1,
                            );
                        let mv = match mode {
                            0 => top2[ai_idx].0,
                            1 => {
                                if !secondary.is_empty() {
                                    secondary[ai_idx]
                                } else {
                                    top2[ai_idx].0
                                }
                            }
                            _ => {
                                let f = &fallback[ai_idx];
                                if f.is_empty() {
                                    state.pos[player]
                                } else {
                                    f[0]
                                }
                            }
                        };
                        out.push(vec![mv]);
                    }
                    out
                }

                fn simulate_with_set(
                    game: &Game,
                    state: &State,
                    my_mv: (usize, usize),
                    opponent_moves: &[Vec<(usize, usize)>],
                ) -> State {
                    let mut moves = Vec::with_capacity(game.m);
                    moves.push(my_mv);
                    for ai_idx in 0..(game.m.saturating_sub(1)) {
                        let mv = opponent_moves[ai_idx][0];
                        moves.push(mv);
                    }
                    crate::__cargo_equip::crates::ahc061_solver::simulate_turn(game, state, &moves)
                }

                fn short_rollout(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    first_mv: (usize, usize),
                    steps: usize,
                ) -> f64 {
                    let mut cur = state.clone();
                    let _top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, &cur, models,
                        );
                    for step in 0..steps {
                        let mut moves = Vec::with_capacity(game.m);
                        if step == 0 {
                            moves.push(first_mv);
                        } else {
                            let cands = crate::__cargo_equip::crates::ahc061_solver::get_candidates(
                                game, &cur, 0,
                            );
                            if cands.is_empty() {
                                break;
                            }
                            let scores = crate::__cargo_equip::crates::ahc061_solver::calc_scores(
                                game, &cur,
                            );
                            let s0 = scores[0] as f64;
                            let max_ai_i64 =
                                scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                            let phase = cur.turn as f64 / game.t as f64;
                            let conflict =
                                crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                                    game, &cur, models,
                                );
                            let mut leaders = vec![false; game.m];
                            for p in 1..game.m {
                                if scores[p] == max_ai_i64 {
                                    leaders[p] = true;
                                }
                            }
                            let mut best_mv = cands[0];
                            let mut best_score = f64::NEG_INFINITY;
                            for &mv in &cands {
                                let v = crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                    game,
                                    &cur,
                                    mv,
                                    &scores,
                                    s0,
                                    max_ai_i64,
                                    phase,
                                    &conflict,
                                    cur.pos[0],
                                    &leaders,
                                );
                                if v > best_score {
                                    best_score = v;
                                    best_mv = mv;
                                }
                            }
                            moves.push(best_mv);
                        }
                        let cur_top2 = crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(game, &cur, models);
                        let primary: Vec<(usize, usize)> = cur_top2.iter().map(|x| x.0).collect();
                        let uncertainty =
                            crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(
                                &cur_top2,
                            );
                        moves.extend(
                            if uncertainty >= 0.24 {
                                let secondary = crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(
                                    &crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, &cur),
                                    &cur_top2,
                                    1,
                                );
                                secondary.into_iter().take(1)
                            } else {
                                primary.into_iter().take(1)
                            },
                        );
                        cur = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, &cur, &moves,
                        );
                    }
                    crate::__cargo_equip::crates::ahc061_solver::strategic_score(game, &cur)
                        - crate::__cargo_equip::crates::ahc061_solver::strategic_score(game, state)
                }

                pub(super) fn choose_move_x18_robust_minmax_guard(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    if !(4..=6).contains(&game.m) {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }

                    let candidates =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if candidates.len() <= 1 {
                        return candidates.first().copied().unwrap_or(state.pos[0]);
                    }

                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2);
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let s0 = scores[0] as f64;
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let phase = state.turn as f64 / game.t as f64;
                    let conflict =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let mut leaders = vec![false; game.m];
                    for p in 1..game.m {
                        if scores[p] == max_ai_i64 {
                            leaders[p] = true;
                        }
                    }

                    let primary_set = opponent_set_by_mode(game, state, models, 0, &Vec::new());
                    let secondary_set = opponent_set_by_mode(game, state, models, 1, &Vec::new());

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
                                &conflict,
                                state.pos[0],
                                &leaders,
                            );
                        ranked.push((mv, local));
                    }
                    ranked
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

                    let mut best_mv = ranked[0].0;
                    let mut best_val = f64::NEG_INFINITY;
                    let candidate_cap = ranked.len().min(7);

                    for &(mv, local) in ranked.iter().take(candidate_cap) {
                        let p_state = simulate_with_set(game, state, mv, &primary_set);
                        let s_state = simulate_with_set(game, state, mv, &secondary_set);
                        let p_score = crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                            game, &p_state,
                        );
                        let s_score = crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                            game, &s_state,
                        );

                        let mut worst_score = p_score.min(s_score);
                        for ai_idx in 0..(game.m.saturating_sub(1)) {
                            let mut alt_set = secondary_set.clone();
                            if alt_set[ai_idx] != primary_set[ai_idx] {
                                let p2 = vec![secondary_set[ai_idx][0]];
                                alt_set[ai_idx] = p2;
                                let alt_state = simulate_with_set(game, state, mv, &alt_set);
                                let alt_score =
                                    crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                        game, &alt_state,
                                    );
                                if alt_score < worst_score {
                                    worst_score = alt_score;
                                }
                            }
                        }

                        let short = short_rollout(game, state, models, mv, 2);
                        let base_adv = local * 0.12;
                        let conflict_penalty =
                            (1.0 - (-conflict[mv.0][mv.1]).exp()) * game.v[mv.0][mv.1] as f64;
                        let robust =
                            0.60 * worst_score + 0.30 * ((p_score + s_score) * 0.5) + 0.10 * short;
                        let val = robust + base_adv - 0.45 * conflict_penalty;
                        let val = if uncertainty >= 0.30 {
                            val - 0.20 * conflict_penalty
                        } else {
                            val
                        };

                        if val > best_val {
                            best_val = val;
                            best_mv = mv;
                        }
                    }

                    if best_val.is_finite() {
                        best_mv
                    } else if phase <= 0.45 {
                        x04_macro_route::choose_move_x04_macro_route(game, state, models)
                    } else {
                        x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models)
                    }
                }
            }
            mod x19_frontier_recovery_sweep {
                use crate::__cargo_equip::crates::ahc061_solver::{
                    x01_beam_pessimistic, x04_macro_route, x06_expert_switch_hybrid,
                    x11_contest_frontier_recovery, AiModel, Game, State,
                };

                fn frontier_sweep_score(game: &Game, state: &State, mv: (usize, usize)) -> f64 {
                    let (x, y) = mv;
                    let owner = state.owner[x][y];
                    let level = state.level[x][y];
                    let value = game.v[x][y] as f64;
                    let mut score = 0.0_f64;

                    if owner == -1 {
                        score += 1.1 * value;
                    } else if owner == 0 && level < game.u {
                        score += 0.70 * value * (game.u - level) as f64 / game.u as f64;
                    } else if owner > 0 && level == 1 {
                        score += 0.80 * value;
                    }

                    const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                    for (dx, dy) in DIRS {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if !crate::__cargo_equip::crates::ahc061_solver::in_bounds(game.n, nx, ny) {
                            continue;
                        }
                        let ux = nx as usize;
                        let uy = ny as usize;
                        let ov = game.v[ux][uy] as f64;
                        let o_owner = state.owner[ux][uy];
                        if o_owner == 0 {
                            score += 0.13 * ov;
                            if state.level[ux][uy] == 1 {
                                score += 0.11 * ov;
                            }
                        } else if o_owner == -1 {
                            score += 0.09 * ov;
                        } else if state.level[ux][uy] < game.u {
                            score += 0.06 * ov / state.level[ux][uy] as f64;
                        }
                    }
                    score
                }

                fn opponent_set(
                    game: &Game,
                    state: &State,
                    top2: &[((usize, usize), (usize, usize), f64)],
                    secondary: bool,
                ) -> Vec<(usize, usize)> {
                    let mut out = Vec::with_capacity(game.m);
                    out.push((0, 0));
                    for ai_idx in 0..(game.m.saturating_sub(1)) {
                        let cur_top2 = top2[ai_idx];
                        let fallback = state.pos[ai_idx + 1];
                        if secondary && cur_top2.1 != fallback {
                            out.push(cur_top2.1);
                        } else {
                            out.push(cur_top2.0);
                        }
                        if out.len() == game.m {
                            break;
                        }
                    }
                    while out.len() < game.m {
                        out.push(state.pos[0]);
                    }
                    out.truncate(game.m);
                    out
                }

                fn sweep_rollout(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    first_move: (usize, usize),
                    steps: usize,
                ) -> f64 {
                    let mut cur = state.clone();
                    for step in 0..steps {
                        let mut moves = Vec::with_capacity(game.m);
                        if step == 0 {
                            moves.push(first_move);
                        } else {
                            let cands = crate::__cargo_equip::crates::ahc061_solver::get_candidates(
                                game, &cur, 0,
                            );
                            if cands.is_empty() {
                                break;
                            }
                            let scores = crate::__cargo_equip::crates::ahc061_solver::calc_scores(
                                game, &cur,
                            );
                            let top2 = crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(game, &cur, models);
                            let s0 = scores[0] as f64;
                            let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                            let phase = cur.turn as f64 / game.t as f64;
                            let conflict =
                                crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                                    game, &cur, models,
                                );
                            let mut leaders = vec![false; game.m];
                            for p in 1..game.m {
                                if scores[p] == max_ai {
                                    leaders[p] = true;
                                }
                            }
                            let mut best = cands[0];
                            let mut best_v = f64::NEG_INFINITY;
                            for &mv in &cands {
                                let v = crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                    game,
                                    &cur,
                                    mv,
                                    &scores,
                                    s0,
                                    max_ai,
                                    phase,
                                    &conflict,
                                    cur.pos[0],
                                    &leaders,
                                );
                                if v > best_v {
                                    best_v = v;
                                    best = mv;
                                }
                            }
                            moves.push(best);
                            let uncertain =
                                crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(
                                    &top2,
                                );
                            if uncertain >= 0.22 {
                                moves.extend(crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(&scores, &top2, 1));
                            } else {
                                moves.extend(top2.iter().map(|x| x.0));
                            }
                        }
                        cur = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, &cur, &moves,
                        );
                    }
                    crate::__cargo_equip::crates::ahc061_solver::strategic_score(game, &cur)
                        - crate::__cargo_equip::crates::ahc061_solver::strategic_score(game, state)
                }

                pub(super) fn choose_move_x19_frontier_recovery_sweep(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    if game.m <= 4 {
                        return x04_macro_route::choose_move_x04_macro_route(game, state, models);
                    }
                    if game.m >= 7 {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }

                    let candidates =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if candidates.is_empty() {
                        return state.pos[0];
                    }
                    if candidates.len() == 1 {
                        return candidates[0];
                    }

                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2);
                    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let s0 = scores[0] as f64;
                    let phase = state.turn as f64 / game.t as f64;
                    let conflict =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let secondary =
                        crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(
                            &scores, &top2, 1,
                        );
                    let mut is_leader = vec![false; game.m];
                    for p in 1..game.m {
                        if scores[p] == max_ai {
                            is_leader[p] = true;
                        }
                    }

                    let advisor_mvs = [
                        x04_macro_route::choose_move_x04_macro_route(game, state, models),
                        x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models),
                        x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        ),
                        x11_contest_frontier_recovery::choose_move_x11_contest_frontier_recovery(
                            game, state, models,
                        ),
                    ];

                    let mut ranked = Vec::<((usize, usize), f64)>::new();
                    for &mv in &candidates {
                        let local =
                            crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                game,
                                state,
                                mv,
                                &scores,
                                s0,
                                max_ai,
                                phase,
                                &conflict,
                                state.pos[0],
                                &is_leader,
                            );
                        let fs = frontier_sweep_score(game, state, mv);
                        let mut defense = 0.0_f64;
                        if state.owner[mv.0][mv.1] == 0 {
                            defense += 0.05 * game.v[mv.0][mv.1] as f64;
                        }
                        ranked.push((mv, local + 0.06 * fs + 0.10 * defense));
                    }
                    for &mv in &advisor_mvs {
                        if !candidates.contains(&mv) {
                            continue;
                        }
                        ranked.push((mv, 0.90 * ranked[0].1));
                    }
                    ranked
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    ranked.dedup_by_key(|x| x.0);
                    if ranked.is_empty() {
                        return state.pos[0];
                    }

                    let primary = opponent_set(game, state, &top2, false);
                    let secondary_set = opponent_set(game, state, &top2, true);
                    let mut best_mv = ranked[0].0;
                    let mut best_v = f64::NEG_INFINITY;

                    let sweep_cap = ranked.len().min(9);
                    for &(mv, local) in ranked.iter().take(sweep_cap) {
                        let mut primary_moves = Vec::with_capacity(game.m);
                        primary_moves.push(mv);
                        primary_moves.extend_from_slice(&primary[1..game.m]);
                        let mut secondary_moves = Vec::with_capacity(game.m);
                        secondary_moves.push(mv);
                        secondary_moves.extend_from_slice(&secondary_set[1..game.m]);

                        let s_primary = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game,
                            state,
                            &primary_moves,
                        );
                        let s_secondary =
                            crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                game,
                                state,
                                &secondary_moves,
                            );
                        let p_score = 0.62
                            * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game, &s_primary,
                            );
                        let s_score = 0.38
                            * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game,
                                &s_secondary,
                            );
                        let rollout = 0.12 * sweep_rollout(game, state, models, mv, 2);
                        let conflict_penalty = conflict[mv.0][mv.1] * game.v[mv.0][mv.1] as f64;
                        let frontier_bonus =
                            crate::__cargo_equip::crates::ahc061_solver::frontier_potential(
                                game,
                                &crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                    game,
                                    state,
                                    &primary_moves,
                                ),
                            ) * 0.25;
                        let uncertainty_bonus = uncertainty * 0.08 * local;
                        let alt_gain = if game.m >= 6 && uncertainty >= 0.30 {
                            0.25 * local + 0.12 * frontier_bonus
                        } else {
                            0.15 * local
                        };

                        let value = p_score + s_score + rollout + alt_gain + uncertainty_bonus
                            - 0.70 * conflict_penalty;
                        if secondary.iter().any(|&s| s == mv) {
                            let secondary_value = value + 0.03 * local;
                            if secondary_value > best_v {
                                best_v = secondary_value;
                                best_mv = mv;
                            }
                        } else if value > best_v {
                            best_v = value;
                            best_mv = mv;
                        }
                    }

                    if best_v.is_finite() {
                        best_mv
                    } else if uncertainty >= 0.35 {
                        x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models)
                    } else {
                        x04_macro_route::choose_move_x04_macro_route(game, state, models)
                    }
                }
            }
            mod x20_band_stage_ensemble {
                use std::collections::HashMap;

                use crate::__cargo_equip::crates::ahc061_solver::{
                    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route,
                    x06_expert_switch_hybrid, x11_contest_frontier_recovery, AiModel, Game, State,
                };

                fn propose_weights(
                    game: &Game,
                    _state: &State,
                    uncertainty: f64,
                    phase: f64,
                    phase_gap: f64,
                ) -> (f64, f64, f64, f64, f64) {
                    let late_penalty = if phase >= 0.70 { 0.25 } else { 0.0 };
                    let uncertainty_penalty = (0.40 - uncertainty).max(0.0);
                    let mut w01 = 0.08 + 0.12 * uncertainty + 0.05 * (phase_gap / 1e5).min(1.0);
                    let mut w04 = 0.40 + 0.25 * uncertainty_penalty + 0.20 * (1.0 - phase);
                    let mut w06 = 0.38 + 0.20 * uncertainty + 0.10 * late_penalty;
                    let mut w02 = 0.08;
                    let mut w11 = 0.0;

                    if game.m == 4 {
                        w04 += 0.05;
                        w01 *= 0.95;
                        w06 *= 1.02;
                        if phase < 0.35 {
                            w02 += 0.12;
                            w04 -= 0.08;
                        }
                    } else if game.m == 5 {
                        w04 *= 1.05;
                        w06 += 0.06;
                        w11 += 0.01;
                    } else if game.m == 6 {
                        w01 += 0.04;
                        w04 *= 0.95;
                        w06 *= 1.06;
                        if phase > 0.60 {
                            w11 += 0.10;
                        }
                    }

                    let total = (w01 + w04 + w06 + w02 + w11).max(1e-12);
                    (
                        w01 / total,
                        w04 / total,
                        w06 / total,
                        w02 / total,
                        w11 / total,
                    )
                }

                fn weighted_votes(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    uncertainty: f64,
                    phase: f64,
                    phase_gap: f64,
                ) -> Vec<((usize, usize), f64)> {
                    let (w01, w04, w06, w02, w11) =
                        propose_weights(game, state, uncertainty, phase, phase_gap);
                    let mv1 =
                        x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models);
                    let mv2 = x04_macro_route::choose_move_x04_macro_route(game, state, models);
                    let mv6 = x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                        game, state, models,
                    );
                    let mut mvs: Vec<((usize, usize), f64)> =
                        vec![(mv1, w01), (mv2, w04), (mv6, w06)];
                    let mv0 = x02_monte_carlo::choose_move_monte_carlo(game, state, models);
                    if game.m <= 5 && phase < 0.55 {
                        mvs.push((mv0, w02));
                    }
                    if game.m == 6 && phase > 0.55 {
                        let mv11 = x11_contest_frontier_recovery::choose_move_x11_contest_frontier_recovery(
                            game, state, models,
                        );
                        mvs.push((mv11, w11));
                    }
                    mvs
                }

                fn evaluate_rollout(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    first_mv: (usize, usize),
                    conflict_penalty: f64,
                ) -> f64 {
                    let mut cur = state.clone();
                    for step in 0..2 {
                        let mut moves = Vec::with_capacity(game.m);
                        if step == 0 {
                            moves.push(first_mv);
                        } else {
                            let cands = crate::__cargo_equip::crates::ahc061_solver::get_candidates(
                                game, &cur, 0,
                            );
                            if cands.is_empty() {
                                break;
                            }
                            let top2 = crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(game, &cur, models);
                            let scores = crate::__cargo_equip::crates::ahc061_solver::calc_scores(
                                game, &cur,
                            );
                            let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                            let s0 = scores[0] as f64;
                            let phase = cur.turn as f64 / game.t as f64;
                            let conflict =
                                crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                                    game, &cur, models,
                                );
                            let mut leaders = vec![false; game.m];
                            for p in 1..game.m {
                                if scores[p] == max_ai {
                                    leaders[p] = true;
                                }
                            }
                            let mut best_mv = cands[0];
                            let mut best_v = f64::NEG_INFINITY;
                            for &mv in &cands {
                                let v = crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                    game,
                                    &cur,
                                    mv,
                                    &scores,
                                    s0,
                                    max_ai,
                                    phase,
                                    &conflict,
                                    cur.pos[0],
                                    &leaders,
                                );
                                if v > best_v {
                                    best_v = v;
                                    best_mv = mv;
                                }
                            }
                            moves.push(best_mv);
                            if crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2)
                                >= 0.22
                            {
                                moves.extend(crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(&scores, &top2, 1));
                            } else {
                                moves.extend(top2.iter().map(|x| x.0));
                            }
                        }
                        cur = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, &cur, &moves,
                        );
                    }
                    let growth =
                        crate::__cargo_equip::crates::ahc061_solver::strategic_score(game, &cur)
                            - crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game, state,
                            );
                    growth - 0.45 * conflict_penalty
                }

                pub(super) fn choose_move_x20_band_stage_ensemble(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    if !(4..=6).contains(&game.m) {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }

                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2);
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let mut max_enemy = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let phase = state.turn as f64 / game.t as f64;
                    let phase_score_gap = (max_enemy as f64 - scores[0] as f64).abs();
                    let conflict_map =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );

                    let cands =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if cands.is_empty() {
                        return state.pos[0];
                    }
                    if cands.len() <= 3 {
                        return cands[0];
                    }
                    max_enemy = max_enemy.max(1);
                    let mut vote_map: HashMap<(usize, usize), f64> = HashMap::new();
                    for (mv, w) in
                        weighted_votes(game, state, models, uncertainty, phase, phase_score_gap)
                    {
                        *vote_map.entry(mv).or_insert(0.0) += w;
                    }

                    let mut leaders = vec![false; game.m];
                    for p in 1..game.m {
                        if scores[p] == max_enemy {
                            leaders[p] = true;
                        }
                    }

                    let s0 = scores[0] as f64;
                    for &mv in cands.iter().take(12) {
                        let local =
                            crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                game,
                                state,
                                mv,
                                &scores,
                                s0,
                                max_enemy,
                                phase,
                                &conflict_map,
                                state.pos[0],
                                &leaders,
                            );
                        let base = *vote_map.get(&mv).unwrap_or(&0.0);
                        let entry = vote_map.entry(mv).or_insert(0.0);
                        *entry = base + local * 0.02;
                    }

                    let mut pool = vote_map.into_iter().collect::<Vec<_>>();
                    if pool.is_empty() {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }
                    pool.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    pool.truncate(pool.len().min(12).max(4));

                    let top_candidates = cands.iter().take(14).collect::<Vec<_>>();
                    let mut append_pool = Vec::<((usize, usize), f64)>::new();
                    for (mv, w) in pool.iter() {
                        if !top_candidates.contains(&mv) && *w > 0.0 {
                            append_pool.push((*mv, *w * 0.5));
                        }
                        if pool.len() > 14 {
                            break;
                        }
                    }
                    pool.extend_from_slice(&append_pool);

                    let mut best_mv = pool[0].0;
                    let mut best_score = f64::NEG_INFINITY;
                    for (mv, vote_w) in pool.iter().take(12) {
                        let mut moves_primary = Vec::with_capacity(game.m);
                        moves_primary.push(*mv);
                        moves_primary.extend(top2.iter().map(|x| x.0).take(game.m - 1));
                        let mut moves_secondary = Vec::with_capacity(game.m);
                        moves_secondary.push(*mv);
                        moves_secondary.extend(
                            crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(
                                &scores, &top2, 1,
                            )
                            .into_iter()
                            .take(game.m - 1),
                        );
                        let s_primary = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game,
                            state,
                            &moves_primary,
                        );
                        let s_secondary =
                            crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                game,
                                state,
                                &moves_secondary,
                            );
                        let core =
                            0.50 * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game, &s_primary,
                            ) + 0.28
                                * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                    game,
                                    &s_secondary,
                                );
                        let two_step = 0.16
                            * evaluate_rollout(game, state, models, *mv, conflict_map[mv.0][mv.1]);
                        let frontier = 0.18
                            * crate::__cargo_equip::crates::ahc061_solver::frontier_potential(
                                game, &s_primary,
                            );
                        let risk = 0.09 * phase_score_gap;
                        let stability = if uncertainty >= 0.30 { 0.7 } else { 1.0 };
                        let score = (core + two_step + frontier + risk)
                            + vote_w * 2.0
                            + 0.12 * (state.level[mv.0][mv.1] as f64)
                            - (1.0 - stability)
                                * 0.10
                                * conflict_map[mv.0][mv.1]
                                * game.v[mv.0][mv.1] as f64;
                        if score > best_score {
                            best_score = score;
                            best_mv = *mv;
                        }
                    }

                    if best_score.is_finite() {
                        best_mv
                    } else if uncertainty >= 0.32 {
                        x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models)
                    } else {
                        x04_macro_route::choose_move_x04_macro_route(game, state, models)
                    }
                }
            }
            mod x21_band_stage_adaptive_guard {
                use std::collections::HashMap;

                use crate::__cargo_equip::crates::ahc061_solver::{
                    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route,
                    x06_expert_switch_hybrid, x11_contest_frontier_recovery,
                    x18_robust_minmax_guard, x19_frontier_recovery_sweep, AiModel, Game, State,
                };

                fn propose_weights(
                    game: &Game,
                    uncertainty: f64,
                    phase: f64,
                    conflict_pressure: f64,
                    gap: f64,
                ) -> (f64, f64, f64, f64, f64, f64) {
                    let w01 = 0.12 + 0.22 * (1.0 - phase).max(0.0).min(1.0);
                    let mut w04 = 0.39 + 0.36 * (1.0 - uncertainty).max(0.0).min(1.0);
                    let mut w06 = 0.30 + 0.10 * phase;
                    let mut w18 = 0.07 + 0.28 * uncertainty * conflict_pressure;
                    let w19 = 0.04 + 0.20 * (gap / 1_00_000.0).min(1.0);
                    let mut w02 = 0.01;

                    if game.m <= 5 && phase < 0.55 {
                        w02 += 0.10;
                        w04 += 0.05;
                    }
                    if conflict_pressure > 1.3 {
                        w18 += 0.14;
                        w06 -= 0.05;
                    }
                    if phase > 0.72 {
                        w18 += 0.12;
                        w04 *= 0.90;
                    }
                    if uncertainty < 0.18 {
                        w04 += 0.07;
                        w02 *= 1.4;
                    }

                    let total = (w01 + w04 + w06 + w18 + w19 + w02).max(1e-12);
                    (
                        w01 / total,
                        w04 / total,
                        w06 / total,
                        w18 / total,
                        w19 / total,
                        w02 / total,
                    )
                }

                fn weighted_votes(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    uncertainty: f64,
                    phase: f64,
                    conflict_pressure: f64,
                    gap: f64,
                ) -> Vec<((usize, usize), f64)> {
                    let (w01, w04, w06, w18, w19, w02) =
                        propose_weights(game, uncertainty, phase, conflict_pressure, gap);
                    let mv1 =
                        x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models);
                    let mv4 = x04_macro_route::choose_move_x04_macro_route(game, state, models);
                    let mv6 = x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                        game, state, models,
                    );
                    let mut votes = vec![(mv1, w01), (mv4, w04), (mv6, w06)];

                    if uncertainty > 0.18 || conflict_pressure > 1.2 {
                        let mv18 = x18_robust_minmax_guard::choose_move_x18_robust_minmax_guard(
                            game, state, models,
                        );
                        votes.push((mv18, w18));
                        if phase > 0.55 {
                            let mv19 = x19_frontier_recovery_sweep::choose_move_x19_frontier_recovery_sweep(
                                game,
                                state,
                                models,
                            );
                            votes.push((mv19, w19));
                        }
                    }
                    if game.m <= 5 && phase < 0.58 && uncertainty < 0.30 {
                        let mv2 = x02_monte_carlo::choose_move_monte_carlo(game, state, models);
                        votes.push((mv2, w02));
                    }
                    if phase < 0.35 && uncertainty > 0.22 {
                        let mv11 = x11_contest_frontier_recovery::choose_move_x11_contest_frontier_recovery(
                            game, state, models,
                        );
                        votes.push((mv11, 0.04));
                    }
                    votes
                }

                fn conflict_pressure_value(
                    conflict_map: &[Vec<f64>],
                    mv: (usize, usize),
                    game: &Game,
                    state: &State,
                ) -> f64 {
                    let p = conflict_map[mv.0][mv.1];
                    let v = game.v[mv.0][mv.1] as f64;
                    let lvl = state.level[mv.0][mv.1] as f64 + 1.0;
                    p * v / lvl
                }

                fn evaluate_rollout(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    first_mv: (usize, usize),
                    guard: bool,
                ) -> f64 {
                    let mut cur = state.clone();
                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let conflict =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let s0 = scores[0] as f64;
                    let mut leaders = vec![false; game.m];
                    for p in 1..game.m {
                        if scores[p] == max_ai {
                            leaders[p] = true;
                        }
                    }
                    let mut total = 0.0;
                    for step in 0..2 {
                        let mut moves = Vec::with_capacity(game.m);
                        if step == 0 {
                            moves.push(first_mv);
                        } else {
                            let cands = crate::__cargo_equip::crates::ahc061_solver::get_candidates(
                                game, &cur, 0,
                            );
                            if cands.is_empty() {
                                break;
                            }
                            let phase = cur.turn as f64 / game.t as f64;
                            let mut best_mv = cands[0];
                            let mut best_v = f64::NEG_INFINITY;
                            for &mv in &cands {
                                let mut v = crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                    game,
                                    &cur,
                                    mv,
                                    &scores,
                                    s0,
                                    max_ai,
                                    phase,
                                    &conflict,
                                    cur.pos[0],
                                    &leaders,
                                );
                                if guard {
                                    v -= 0.22 * conflict[mv.0][mv.1] * game.v[mv.0][mv.1] as f64
                                        / 100_000.0;
                                }
                                if v > best_v {
                                    best_v = v;
                                    best_mv = mv;
                                }
                            }
                            moves.push(best_mv);
                            if crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2)
                                >= 0.22
                            {
                                moves.extend(crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(&scores, &top2, 1));
                            } else {
                                moves.extend(top2.iter().map(|x| x.0));
                            }
                        }
                        cur = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, &cur, &moves,
                        );
                        total += (0.88_f64).powi(step as i32)
                            * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game, &cur,
                            );
                    }
                    total
                }

                pub(super) fn choose_move_x21_band_stage_adaptive_guard(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    if !(4..=6).contains(&game.m) {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }

                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2);
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let phase = state.turn as f64 / game.t as f64;
                    let gap = ((max_ai_i64 as f64 - scores[0] as f64).abs()).min(100_000.0);
                    let conflict_map =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let conf = (conflict_map.iter().flat_map(|r| r.iter()).sum::<f64>()
                        / (game.n as f64 * game.n as f64))
                        .min(10.0);

                    let cands =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if cands.is_empty() {
                        return state.pos[0];
                    }
                    if cands.len() <= 3 {
                        return cands[0];
                    }

                    let mut vote: HashMap<(usize, usize), f64> = HashMap::new();
                    for (mv, w) in
                        weighted_votes(game, state, models, uncertainty, phase, conf, gap)
                    {
                        *vote.entry(mv).or_insert(0.0) += w;
                    }

                    let mut is_leader = vec![false; game.m];
                    for p in 1..game.m {
                        if scores[p] == max_ai_i64 {
                            is_leader[p] = true;
                        }
                    }
                    let s0 = scores[0] as f64;
                    for &mv in cands.iter().take(18) {
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
                                state.pos[0],
                                &is_leader,
                            );
                        let guard = if conf > 1.2 { -0.06 } else { 0.00 };
                        let conflict_pen = conflict_pressure_value(&conflict_map, mv, game, state);
                        let bonus = if conflict_pen > 0.35 { -0.14 } else { 0.0 };
                        let entry = vote.entry(mv).or_insert(0.0);
                        *entry += 0.02 * local + bonus + conf * 0.0 + guard;
                    }

                    let mut pool: Vec<_> = vote.into_iter().collect();
                    if pool.is_empty() {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }
                    pool.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    pool.truncate(pool.len().max(8).min(16));

                    let mut best_mv = pool[0].0;
                    let mut best_score = f64::NEG_INFINITY;
                    for &(mv, wv) in pool.iter().take(12) {
                        let mut primary = Vec::with_capacity(game.m);
                        primary.push(mv);
                        primary.extend(top2.iter().map(|x| x.0).take(game.m - 1));
                        let mut secondary = Vec::with_capacity(game.m);
                        secondary.push(mv);
                        secondary.extend(
                            crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(
                                &scores, &top2, 1,
                            ),
                        );

                        let s_primary = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &primary,
                        );
                        let s_secondary =
                            crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                game, state, &secondary,
                            );
                        let core =
                            0.56 * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game, &s_primary,
                            ) + 0.26
                                * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                    game,
                                    &s_secondary,
                                );
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
                                state.pos[0],
                                &is_leader,
                            );
                        let rollout = 0.20
                            * evaluate_rollout(
                                game,
                                state,
                                models,
                                mv,
                                uncertainty > 0.30 && conf > 1.1,
                            );
                        let frontier = 0.10
                            * crate::__cargo_equip::crates::ahc061_solver::frontier_potential(
                                game, &s_primary,
                            );
                        let risk = 0.12 * conflict_map[mv.0][mv.1];
                        let score = core + rollout + frontier + 0.02 * local + 2.0 * wv - risk;
                        if score > best_score {
                            best_score = score;
                            best_mv = mv;
                        }
                    }

                    if best_score.is_finite() {
                        best_mv
                    } else {
                        x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models)
                    }
                }
            }
            mod x22_band_stage_recovery_boost {
                use std::collections::HashMap;

                use crate::__cargo_equip::crates::ahc061_solver::{
                    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route,
                    x06_expert_switch_hybrid, x11_contest_frontier_recovery,
                    x18_robust_minmax_guard, x19_frontier_recovery_sweep, AiModel, Game, State,
                };

                fn phase_weights(
                    phase: f64,
                    uncertainty: f64,
                    gap: f64,
                    m: usize,
                ) -> (f64, f64, f64, f64, f64, f64) {
                    let base = 0.45 + 0.2 * (1.0 - phase);
                    let mut w04 = base * (1.0 - 0.25 * uncertainty);
                    let mut w01 = 0.20 + 0.12 * uncertainty;
                    let mut w06 = 0.28 + 0.18 * phase;
                    let mut w18 = 0.08 + (0.15 * uncertainty);
                    let mut w19 = 0.05 + (0.20 * gap / 100_000.0).min(0.2);
                    let w02 = if m <= 5 && phase < 0.60 { 0.12 } else { 0.02 };
                    if phase > 0.70 {
                        w18 += 0.10;
                        w04 *= 0.85;
                        w19 += 0.02;
                    }
                    if uncertainty > 0.30 && phase > 0.45 {
                        w18 += 0.10;
                        w06 += 0.06;
                    }
                    if gap < 2500.0 {
                        w01 += 0.08;
                        w04 += 0.04;
                    }
                    let total = (w01 + w04 + w06 + w18 + w19 + w02).max(1e-12);
                    (
                        w01 / total,
                        w04 / total,
                        w06 / total,
                        w18 / total,
                        w19 / total,
                        w02 / total,
                    )
                }

                fn weighted_votes(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    uncertainty: f64,
                    phase: f64,
                    gap: f64,
                    conf: f64,
                ) -> Vec<((usize, usize), f64)> {
                    let (w01, w04, w06, w18, w19, w02) =
                        phase_weights(phase, uncertainty, gap, game.m);
                    let mut votes = vec![
                        (
                            x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                                game, state, models,
                            ),
                            w01,
                        ),
                        (
                            x04_macro_route::choose_move_x04_macro_route(game, state, models),
                            w04,
                        ),
                        (
                            x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                game, state, models,
                            ),
                            w06,
                        ),
                    ];
                    if uncertainty > 0.16 || conf > 1.0 {
                        votes.push((
                            x18_robust_minmax_guard::choose_move_x18_robust_minmax_guard(
                                game, state, models,
                            ),
                            w18,
                        ));
                    }
                    if phase > 0.40 && gap > 2_000.0 {
                        votes.push((
                            x19_frontier_recovery_sweep::choose_move_x19_frontier_recovery_sweep(
                                game, state, models,
                            ),
                            w19,
                        ));
                    }
                    if phase >= 0.40 && phase <= 0.82 && uncertainty < 0.36 {
                        votes.push((
                            x11_contest_frontier_recovery::choose_move_x11_contest_frontier_recovery(game, state, models),
                            0.07,
                        ));
                    }
                    if gap < 1200.0 && game.m <= 5 {
                        votes.push((
                            x02_monte_carlo::choose_move_monte_carlo(game, state, models),
                            w02,
                        ));
                    }
                    votes
                }

                fn recovery_probe(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    first_mv: (usize, usize),
                    phase: f64,
                ) -> f64 {
                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let mut leaders = vec![false; game.m];
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    for p in 1..game.m {
                        if scores[p] == max_ai_i64 {
                            leaders[p] = true;
                        }
                    }
                    let mut cur = state.clone();
                    let mut total = 0.0;
                    for step in 0..2 {
                        let mut moves = vec![first_mv];
                        let s0 = scores[0] as f64;
                        if step > 0 {
                            moves.pop();
                            let cands = crate::__cargo_equip::crates::ahc061_solver::get_candidates(
                                game, &cur, 0,
                            );
                            if cands.is_empty() {
                                break;
                            }
                            let conf =
                                crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                                    game, &cur, models,
                                );
                            let cm = crate::__cargo_equip::crates::ahc061_solver::calc_scores(
                                game, &cur,
                            );
                            let phase_now = cur.turn as f64 / game.t as f64;
                            let mut best = cands[0];
                            let mut best_v = f64::NEG_INFINITY;
                            for &mv in &cands {
                                let v = crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                    game,
                                    &cur,
                                    mv,
                                    &cm,
                                    s0,
                                    max_ai_i64,
                                    phase_now,
                                    &conf,
                                    cur.pos[0],
                                    &leaders,
                                );
                                if v > best_v {
                                    best_v = v;
                                    best = mv;
                                }
                            }
                            moves.push(best);
                            if crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2)
                                >= 0.20
                            {
                                moves.extend(crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(&cm, &top2, 1));
                            } else {
                                moves.extend(top2.iter().map(|x| x.0));
                            }
                        }
                        cur = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, &cur, &moves,
                        );
                        let gain = crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                            game, &cur,
                        );
                        if phase > 0.6 {
                            total += (0.82_f64).powi(step as i32) * gain;
                        } else {
                            total += (0.90_f64).powi(step as i32) * gain;
                        }
                    }
                    total
                }

                fn local_gap_bias(
                    conflict_map: &[Vec<f64>],
                    mv: (usize, usize),
                    game: &Game,
                ) -> f64 {
                    let x = mv.0 as f64;
                    let y = mv.1 as f64;
                    let center = ((4.0 - x).abs() + (4.0 - y).abs()) / 8.0;
                    let conflict = conflict_map[mv.0][mv.1];
                    conflict * 12.0 + center * 5.0 + 0.000_01 * game.v[mv.0][mv.1] as f64
                }

                pub(super) fn choose_move_x22_band_stage_recovery_boost(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    if !(4..=6).contains(&game.m) {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }

                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2);
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let phase = state.turn as f64 / game.t as f64;
                    let gap = ((max_ai_i64 - scores[0]).abs() as f64).min(100_000.0);
                    let conflict_map =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let conf =
                        conflict_map.iter().flatten().sum::<f64>() / ((game.n * game.n) as f64);

                    let cands =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if cands.is_empty() {
                        return state.pos[0];
                    }
                    if cands.len() <= 3 {
                        return cands[0];
                    }

                    let mut vote_map: HashMap<(usize, usize), f64> = HashMap::new();
                    for (mv, w) in
                        weighted_votes(game, state, models, uncertainty, phase, gap, conf)
                    {
                        *vote_map.entry(mv).or_insert(0.0) += w;
                    }

                    let mut leaders = vec![false; game.m];
                    for p in 1..game.m {
                        if scores[p] == max_ai_i64 {
                            leaders[p] = true;
                        }
                    }
                    let s0 = scores[0] as f64;
                    for &mv in cands.iter().take(16) {
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
                                state.pos[0],
                                &leaders,
                            );
                        let base = vote_map.entry(mv).or_insert(0.0);
                        *base +=
                            0.018 * local + 0.06 / (1.0 + local_gap_bias(&conflict_map, mv, game));
                    }

                    let mut pool: Vec<_> = vote_map.into_iter().collect();
                    if pool.is_empty() {
                        return x04_macro_route::choose_move_x04_macro_route(game, state, models);
                    }
                    pool.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    pool.truncate(pool.len().max(6).min(14));

                    let mut best_mv = pool[0].0;
                    let mut best = f64::NEG_INFINITY;
                    for &(mv, base_w) in pool.iter().take(10) {
                        let mut p1 = vec![mv];
                        p1.extend(top2.iter().map(|x| x.0).take(game.m - 1));
                        let mut p2 = vec![mv];
                        p2.extend(
                            crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(
                                &scores, &top2, 1,
                            )
                            .into_iter()
                            .take(game.m - 1),
                        );
                        let s1 = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &p1,
                        );
                        let s2 = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &p2,
                        );
                        let score1 = 0.50
                            * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game, &s1,
                            );
                        let score2 = 0.30
                            * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game, &s2,
                            );
                        let rollout = 0.20 * recovery_probe(game, state, models, mv, phase);
                        let front = 0.10
                            * crate::__cargo_equip::crates::ahc061_solver::frontier_potential(
                                game, &s1,
                            );
                        let risk = 0.08 * conflict_map[mv.0][mv.1];
                        let score = score1 + score2 + rollout + front + base_w * 2.0 - risk;
                        if score > best {
                            best = score;
                            best_mv = mv;
                        }
                    }

                    if best.is_finite() {
                        best_mv
                    } else {
                        x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models)
                    }
                }
            }
            mod x23_band_stage_frontier_guard {
                use std::collections::HashMap;

                use crate::__cargo_equip::crates::ahc061_solver::{
                    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route,
                    x06_expert_switch_hybrid, x11_contest_frontier_recovery,
                    x18_robust_minmax_guard, AiModel, Game, State,
                };

                fn frontier_signal(game: &Game, state: &State, mv: (usize, usize)) -> f64 {
                    let mut sig = 0.0;
                    let x = mv.0 as isize;
                    let y = mv.1 as isize;
                    const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                    for (dx, dy) in DIRS {
                        let nx = x + dx;
                        let ny = y + dy;
                        if crate::__cargo_equip::crates::ahc061_solver::in_bounds(game.n, nx, ny) {
                            let ux = nx as usize;
                            let uy = ny as usize;
                            let owner = state.owner[ux][uy];
                            let v = game.v[ux][uy] as f64;
                            if owner == -1 {
                                sig += v * 0.95;
                            } else if owner == 0 {
                                sig += 0.40 * v;
                            } else if owner == 0 && state.level[ux][uy] == 1 {
                                sig += 0.25 * v;
                            }
                            if state.level[ux][uy] == 1 && owner != 0 {
                                sig += 0.10 * v;
                            }
                        }
                    }
                    sig
                }

                fn weighted_votes(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    uncertainty: f64,
                    phase: f64,
                    conflict_peak: f64,
                ) -> Vec<((usize, usize), f64)> {
                    let w01 = 0.18 + 0.10 * (1.0 - uncertainty);
                    let mut w04 = 0.40 + 0.35 * (1.0 - phase);
                    let mut w06 = 0.28 + 0.15 * phase;
                    let mut w18 = 0.08 + 0.18 * uncertainty * conflict_peak;
                    let w11 = 0.04 + 0.20 * (phase * (1.0 - uncertainty)).min(1.0);
                    let w02 = if game.m <= 5 { 0.05 } else { 0.01 };
                    if phase > 0.65 {
                        w18 += 0.10;
                        w04 *= 0.90;
                    }
                    if uncertainty > 0.30 && conflict_peak > 0.6 {
                        w18 += 0.12;
                        w06 -= 0.05;
                    }
                    let total = (w01 + w04 + w06 + w18 + w11 + w02).max(1e-12);
                    let mut votes = vec![
                        (
                            x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models),
                            w01 / total,
                        ),
                        (
                            x04_macro_route::choose_move_x04_macro_route(game, state, models),
                            w04 / total,
                        ),
                        (
                            x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models),
                            w06 / total,
                        ),
                        (
                            x18_robust_minmax_guard::choose_move_x18_robust_minmax_guard(game, state, models),
                            w18 / total,
                        ),
                        (
                            x11_contest_frontier_recovery::choose_move_x11_contest_frontier_recovery(game, state, models),
                            w11 / total,
                        ),
                    ];
                    if game.m <= 5 {
                        votes.push((
                            x02_monte_carlo::choose_move_monte_carlo(game, state, models),
                            w02 / total,
                        ));
                    }
                    votes
                }

                fn score_with_guard(
                    game: &Game,
                    state: &State,
                    mv: (usize, usize),
                    scores: &[i64],
                    phase: f64,
                    phase_gap: f64,
                    conflict_map: &[Vec<f64>],
                    local: f64,
                    leaders: &[bool],
                    phase_gap_risk: f64,
                ) -> f64 {
                    let s0 = scores[0] as f64;
                    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let frontier = frontier_signal(game, state, mv);
                    let conflict = conflict_map[mv.0][mv.1];
                    let risk = conflict * (2.0 + phase_gap * 1e-5) * (1.0 + phase);
                    let safety = -1.2 * (state.level[mv.0][mv.1] as f64).powf(1.1) * risk.min(5.0);
                    let move_pressure = if phase_gap_risk > 1_000.0 {
                        0.0012
                    } else {
                        0.0
                    };
                    let mut score =
                        local + 0.03 * frontier + 0.02 * phase_gap + move_pressure + safety;
                    if mv == state.pos[0] && phase > 0.6 {
                        score -= 0.30;
                    }
                    score += crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                        game,
                        state,
                        mv,
                        scores,
                        s0,
                        max_ai,
                        phase,
                        conflict_map,
                        state.pos[0],
                        leaders,
                    ) * 0.01;
                    score
                }

                fn rollout_mix(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    first_mv: (usize, usize),
                    top2: &[((usize, usize), (usize, usize), f64)],
                    phase: f64,
                ) -> f64 {
                    let mut cur = state.clone();
                    let mut gain = 0.0;
                    for step in 0..2 {
                        let mut moves = Vec::with_capacity(game.m);
                        if step == 0 {
                            moves.push(first_mv);
                        } else {
                            let cands = crate::__cargo_equip::crates::ahc061_solver::get_candidates(
                                game, &cur, 0,
                            );
                            if cands.is_empty() {
                                break;
                            }
                            let conflict =
                                crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                                    game, &cur, models,
                                );
                            let scores = crate::__cargo_equip::crates::ahc061_solver::calc_scores(
                                game, &cur,
                            );
                            let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                            let s0 = scores[0] as f64;
                            let mut leaders = vec![false; game.m];
                            for p in 1..game.m {
                                if scores[p] == max_ai {
                                    leaders[p] = true;
                                }
                            }
                            let mut best_mv = cands[0];
                            let mut best_v = f64::NEG_INFINITY;
                            for &mv in &cands {
                                let mut v = crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
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
                                v -= 0.0015 * frontier_signal(game, &cur, mv);
                                if v > best_v {
                                    best_v = v;
                                    best_mv = mv;
                                }
                            }
                            moves.push(best_mv);
                            if phase < 0.70 {
                                moves.extend(crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(&scores, top2, 1));
                            } else {
                                moves.extend(top2.iter().map(|x| x.0));
                            }
                        }
                        cur = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, &cur, &moves,
                        );
                        gain += (0.9_f64).powi(step as i32)
                            * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game, &cur,
                            );
                    }
                    gain
                }

                pub(super) fn choose_move_x23_band_stage_frontier_guard(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    if !(4..=6).contains(&game.m) {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }

                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2);
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let phase = state.turn as f64 / game.t as f64;
                    let phase_gap = ((max_ai as f64 - scores[0] as f64).abs()).min(100_000.0);
                    let conflict_map =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let conflict_peak =
                        conflict_map
                            .iter()
                            .flatten()
                            .fold(0.0_f64, |a, b| if *b > a { *b } else { a });

                    let mut leaders = vec![false; game.m];
                    for p in 1..game.m {
                        if scores[p] == max_ai {
                            leaders[p] = true;
                        }
                    }

                    let cands =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if cands.is_empty() {
                        return state.pos[0];
                    }
                    if cands.len() <= 4 {
                        return cands[0];
                    }

                    let mut vote_map: HashMap<(usize, usize), f64> = HashMap::new();
                    for (mv, w) in
                        weighted_votes(game, state, models, uncertainty, phase, conflict_peak)
                    {
                        *vote_map.entry(mv).or_insert(0.0) += w;
                    }

                    let s0 = scores[0] as f64;
                    for &mv in cands.iter().take(20) {
                        let local =
                            crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                game,
                                state,
                                mv,
                                &scores,
                                s0,
                                max_ai,
                                phase,
                                &conflict_map,
                                state.pos[0],
                                &leaders,
                            );
                        let risk_guard = score_with_guard(
                            game,
                            state,
                            mv,
                            &scores,
                            phase,
                            phase_gap,
                            &conflict_map,
                            local,
                            &leaders,
                            phase_gap,
                        );
                        let entry = vote_map.entry(mv).or_insert(0.0);
                        *entry += 0.02 * local
                            + 0.001 * risk_guard
                            + 0.000_02 * frontier_signal(game, state, mv);
                    }

                    let mut pool: Vec<_> = vote_map.into_iter().collect();
                    if pool.is_empty() {
                        return x04_macro_route::choose_move_x04_macro_route(game, state, models);
                    }
                    pool.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    pool.truncate(pool.len().max(8).min(16));

                    let mut best_mv = pool[0].0;
                    let mut best_score = f64::NEG_INFINITY;
                    for &(mv, w) in pool.iter().take(10) {
                        let mut primary = Vec::with_capacity(game.m);
                        primary.push(mv);
                        primary.extend(top2.iter().map(|x| x.0).take(game.m - 1));
                        let mut secondary = Vec::with_capacity(game.m);
                        secondary.push(mv);
                        secondary.extend(
                            crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(
                                &scores, &top2, 1,
                            )
                            .into_iter()
                            .take(game.m - 1),
                        );
                        let s1 = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &primary,
                        );
                        let s2 = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &secondary,
                        );
                        let core =
                            0.54 * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game, &s1,
                            ) + 0.28
                                * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                    game, &s2,
                                );
                        let rollout = 0.18 * rollout_mix(game, state, models, mv, &top2, phase);
                        let frontier = 0.12 * frontier_signal(game, state, mv);
                        let risk = 0.15 * conflict_map[mv.0][mv.1]
                            + 0.02 * (state.level[mv.0][mv.1] as f64);
                        let score = core + rollout + frontier + w - risk;
                        if score > best_score {
                            best_score = score;
                            best_mv = mv;
                        }
                    }

                    if best_score.is_finite() {
                        best_mv
                    } else {
                        x18_robust_minmax_guard::choose_move_x18_robust_minmax_guard(
                            game, state, models,
                        )
                    }
                }
            }
            mod x24_band_stage_adaptive_switch {
                use std::collections::HashMap;

                use crate::__cargo_equip::crates::ahc061_solver::{
                    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route,
                    x06_expert_switch_hybrid, x11_contest_frontier_recovery,
                    x18_robust_minmax_guard, x19_frontier_recovery_sweep, AiModel, Game, State,
                };

                fn score_weights(
                    game: &Game,
                    uncertainty: f64,
                    phase: f64,
                    conf: f64,
                    leader_gap: f64,
                ) -> (f64, f64, f64, f64, f64) {
                    let w01 = 0.18 + 0.08 * conf.min(1.5);
                    let mut w04 = 0.42 + 0.25 * (1.0 - phase).max(0.0).min(1.0);
                    let mut w06 = 0.30 + 0.10 * phase + 0.15 * conf;
                    let mut w18 = 0.08 + 0.18 * uncertainty;
                    let mut w11 = 0.02 + 0.05 * (leader_gap / 1200.0).min(1.0);

                    if game.m == 4 && uncertainty > 0.18 {
                        w04 += 0.04;
                        w06 += 0.06;
                    }
                    if phase > 0.72 {
                        w18 += 0.09;
                        w04 *= 0.88;
                    }
                    if conf > 1.3 {
                        w18 += 0.12;
                        w11 += 0.02;
                        w06 -= 0.06;
                        w04 *= 0.94;
                    }
                    if game.m >= 6 && phase < 0.35 {
                        w11 += 0.02;
                    }

                    let total = (w01 + w04 + w06 + w18 + w11).max(1e-12);
                    (
                        w01 / total,
                        w04 / total,
                        w06 / total,
                        w18 / total,
                        w11 / total,
                    )
                }

                fn weighted_votes(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    uncertainty: f64,
                    phase: f64,
                    conf: f64,
                    leader_gap: f64,
                ) -> Vec<((usize, usize), f64)> {
                    let (w01, w04, w06, w18, w11) =
                        score_weights(game, uncertainty, phase, conf, leader_gap);
                    let mut votes = vec![
                        (
                            x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                                game, state, models,
                            ),
                            w01,
                        ),
                        (
                            x04_macro_route::choose_move_x04_macro_route(game, state, models),
                            w04,
                        ),
                        (
                            x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                game, state, models,
                            ),
                            w06,
                        ),
                    ];
                    if uncertainty >= 0.18 || conf > 1.0 {
                        votes.push((
                            x18_robust_minmax_guard::choose_move_x18_robust_minmax_guard(
                                game, state, models,
                            ),
                            w18,
                        ));
                    }
                    if phase > 0.52 && game.m >= 5 {
                        votes.push((
                            x19_frontier_recovery_sweep::choose_move_x19_frontier_recovery_sweep(
                                game, state, models,
                            ),
                            w11,
                        ));
                    }
                    if phase < 0.45 && uncertainty < 0.26 {
                        votes.push((
                            x11_contest_frontier_recovery::choose_move_x11_contest_frontier_recovery(
                                game, state, models,
                            ),
                            0.03,
                        ));
                    }
                    if uncertainty > 0.32 && game.m <= 5 {
                        votes.push((
                            x02_monte_carlo::choose_move_monte_carlo(game, state, models),
                            0.04,
                        ));
                    }
                    votes
                }

                fn conflict_signal(
                    conflict_map: &[Vec<f64>],
                    mv: (usize, usize),
                    game: &Game,
                ) -> f64 {
                    let (x, y) = mv;
                    let p = conflict_map[x][y];
                    let lv = game.u.max(1) as f64;
                    p + 0.000_000_5 * lv * game.v[x][y] as f64
                }

                fn two_step_probe(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    first_mv: (usize, usize),
                    top2: &[((usize, usize), (usize, usize), f64)],
                    keep_conflict: bool,
                ) -> f64 {
                    let mut cur = state.clone();
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let _s0 = scores[0] as f64;
                    let mut leaders = vec![false; game.m];
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
                            let cands = crate::__cargo_equip::crates::ahc061_solver::get_candidates(
                                game, &cur, 0,
                            );
                            if cands.is_empty() {
                                break;
                            }
                            let conflict =
                                crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                                    game, &cur, models,
                                );
                            let scores_now =
                                crate::__cargo_equip::crates::ahc061_solver::calc_scores(
                                    game, &cur,
                                );
                            let max_now =
                                scores_now.iter().skip(1).copied().max().unwrap_or(1).max(1);
                            let phase_now = cur.turn as f64 / game.t as f64;
                            let mut next_mv = cands[0];
                            let mut best_v = f64::NEG_INFINITY;
                            for &mv in &cands {
                                let mut v = crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                    game,
                                    &cur,
                                    mv,
                                    &scores_now,
                                    scores_now[0] as f64,
                                    max_now,
                                    phase_now,
                                    &conflict,
                                    cur.pos[0],
                                    &leaders,
                                );
                                if keep_conflict {
                                    v -= 0.35 * conflict[mv.0][mv.1];
                                }
                                if v > best_v {
                                    best_v = v;
                                    next_mv = mv;
                                }
                            }
                            moves.push(next_mv);
                            if crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(top2)
                                >= 0.20
                            {
                                moves.extend(crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(&scores_now, top2, 1));
                            } else {
                                moves.extend(top2.iter().map(|x| x.0));
                            }
                        }
                        cur = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, &cur, &moves,
                        );
                        total += (0.84_f64).powi(step as i32)
                            * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game, &cur,
                            );
                    }
                    total
                }

                pub(super) fn choose_move_x24_band_stage_adaptive_switch(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    if !(4..=6).contains(&game.m) {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }

                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2);
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let phase = state.turn as f64 / game.t as f64;
                    let leader_gap = ((max_ai as f64 - scores[0] as f64).abs()).min(100_000.0);
                    let conflict_map =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let conf = (conflict_map.iter().flatten().sum::<f64>()
                        / (game.n as f64 * game.n as f64))
                        .min(5.0);

                    let cands =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if cands.is_empty() {
                        return state.pos[0];
                    }
                    if cands.len() <= 3 {
                        return cands[0];
                    }

                    let mut vote_map: HashMap<(usize, usize), f64> = HashMap::new();
                    for (mv, w) in
                        weighted_votes(game, state, models, uncertainty, phase, conf, leader_gap)
                    {
                        *vote_map.entry(mv).or_insert(0.0) += w;
                    }

                    let mut leaders = vec![false; game.m];
                    for p in 1..game.m {
                        if scores[p] == max_ai {
                            leaders[p] = true;
                        }
                    }
                    let s0 = scores[0] as f64;
                    for &mv in cands.iter().take(20) {
                        let local =
                            crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                game,
                                state,
                                mv,
                                &scores,
                                s0,
                                max_ai,
                                phase,
                                &conflict_map,
                                state.pos[0],
                                &leaders,
                            );
                        let c = conflict_signal(&conflict_map, mv, game);
                        let entry = vote_map.entry(mv).or_insert(0.0);
                        *entry +=
                            0.020 * local + 0.006 * game.v[mv.0][mv.1] as f64 / 10000.0 - 0.09 * c;
                    }

                    let mut pool: Vec<_> = vote_map.into_iter().collect();
                    if pool.is_empty() {
                        return x04_macro_route::choose_move_x04_macro_route(game, state, models);
                    }
                    pool.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    pool.truncate(pool.len().max(7).min(16));

                    let mut best_mv = pool[0].0;
                    let mut best_score = f64::NEG_INFINITY;
                    for &(mv, w) in pool.iter().take(10) {
                        let mut p1 = Vec::with_capacity(game.m);
                        p1.push(mv);
                        p1.extend(top2.iter().map(|x| x.0).take(game.m - 1));
                        let mut p2 = Vec::with_capacity(game.m);
                        p2.push(mv);
                        p2.extend(
                            crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(
                                &scores, &top2, 1,
                            )
                            .into_iter()
                            .take(game.m - 1),
                        );

                        let s1 = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &p1,
                        );
                        let s2 = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &p2,
                        );
                        let rollout =
                            two_step_probe(game, state, models, mv, &top2, uncertainty > 0.30);
                        let score = 0.58
                            * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game, &s1,
                            )
                            + 0.26
                                * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                    game, &s2,
                                )
                            + 0.13 * rollout
                            + 0.12
                                * crate::__cargo_equip::crates::ahc061_solver::frontier_potential(
                                    game, &s1,
                                )
                            + 2.0 * w
                            - 1.2 * conflict_map[mv.0][mv.1];
                        if score > best_score {
                            best_score = score;
                            best_mv = mv;
                        }
                    }

                    if best_score.is_finite() {
                        best_mv
                    } else {
                        x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models)
                    }
                }
            }
            mod x25_race_adaptive_recovery {
                use std::collections::HashMap;

                use crate::__cargo_equip::crates::ahc061_solver::{
                    x01_beam_pessimistic, x02_monte_carlo, x04_macro_route,
                    x06_expert_switch_hybrid, x11_contest_frontier_recovery,
                    x13_frontier_consensus, x18_robust_minmax_guard, AiModel, Game, State,
                };

                fn recovery_weights(
                    phase: f64,
                    uncertainty: f64,
                    leader_gap: f64,
                    conf: f64,
                    m: usize,
                ) -> (f64, f64, f64, f64, f64) {
                    let w01 = 0.15 + 0.10 * (1.0 - phase);
                    let mut w04 = 0.33 + 0.22 * (1.0 - uncertainty);
                    let mut w06 = 0.28 + 0.16 * phase;
                    let mut w18 = 0.10 + 0.18 * uncertainty * conf;
                    let mut w13 = 0.14 + 0.06 * (1.0 - conf);
                    let mut w2 = if m <= 5 && phase < 0.45 {
                        (0.02 + 0.20 * (1.0 - uncertainty)).min(0.12)
                    } else {
                        0.02
                    };

                    if conf > 1.1 {
                        w18 += 0.14;
                        w04 -= 0.08;
                    }
                    if uncertainty > 0.30 {
                        w18 += 0.06;
                        w06 += 0.05;
                    }
                    if leader_gap > 2200.0 {
                        w13 += 0.03;
                        w06 -= 0.03;
                    }
                    if m <= 5 && phase < 0.45 {
                        w04 += 0.05;
                    }

                    if uncertainty < 0.30 && phase >= 0.40 {
                        w2 += 0.04;
                    }

                    let total = (w01 + w04 + w06 + w18 + w13 + w2).max(1e-12);
                    (
                        w01 / total,
                        w04 / total,
                        w06 / total,
                        w18 / total,
                        (w13 + w2) / total,
                    )
                }

                fn propose_votes(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    uncertainty: f64,
                    phase: f64,
                    leader_gap: f64,
                    conf: f64,
                ) -> Vec<((usize, usize), f64)> {
                    let (w01, w04, w06, w18, w13plus) =
                        recovery_weights(phase, uncertainty, leader_gap, conf, game.m);
                    let mut votes = vec![
                        (
                            x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                                game, state, models,
                            ),
                            w01,
                        ),
                        (
                            x04_macro_route::choose_move_x04_macro_route(game, state, models),
                            w04,
                        ),
                        (
                            x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                game, state, models,
                            ),
                            w06,
                        ),
                    ];
                    votes.push((
                        x18_robust_minmax_guard::choose_move_x18_robust_minmax_guard(
                            game, state, models,
                        ),
                        w18,
                    ));
                    votes.push((
                        x13_frontier_consensus::choose_move_x13_frontier_consensus(
                            game, state, models,
                        ),
                        w13plus,
                    ));
                    if uncertainty > 0.2 || conf > 1.0 {
                        votes.push((
                            x11_contest_frontier_recovery::choose_move_x11_contest_frontier_recovery(
                                game, state, models,
                            ),
                            0.03,
                        ));
                    }
                    if uncertainty < 0.28 && phase < 0.60 && game.m <= 5 {
                        votes.push((
                            x02_monte_carlo::choose_move_monte_carlo(game, state, models),
                            0.04,
                        ));
                    }
                    votes
                }

                fn pressure_score(game: &Game, state: &State, mv: (usize, usize)) -> f64 {
                    let (x, y) = mv;
                    let mut penalty = 0.0;
                    let dirs: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                    for (dx, dy) in &dirs {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if crate::__cargo_equip::crates::ahc061_solver::in_bounds(game.n, nx, ny) {
                            let ux = nx as usize;
                            let uy = ny as usize;
                            let owner = state.owner[ux][uy];
                            let level = state.level[ux][uy] as f64;
                            let v = game.v[ux][uy] as f64;
                            penalty += match owner {
                                -1 => -0.12 * v,
                                0 => {
                                    if level < game.u as f64 {
                                        -0.10 * v / 100.0 * level
                                    } else {
                                        0.06 * v
                                    }
                                }
                                _ => 0.03 * v / level.max(1.0),
                            };
                        }
                    }
                    penalty
                }

                fn build_rollout(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    first_mv: (usize, usize),
                    top2: &[((usize, usize), (usize, usize), f64)],
                    risk_mode: bool,
                ) -> f64 {
                    let mut cur = state.clone();
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
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
                            let cands = crate::__cargo_equip::crates::ahc061_solver::get_candidates(
                                game, &cur, 0,
                            );
                            if cands.is_empty() {
                                break;
                            }
                            let conflict =
                                crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                                    game, &cur, models,
                                );
                            let scores_now =
                                crate::__cargo_equip::crates::ahc061_solver::calc_scores(
                                    game, &cur,
                                );
                            let max_now =
                                scores_now.iter().skip(1).copied().max().unwrap_or(1).max(1);
                            let phase_now = cur.turn as f64 / game.t as f64;
                            let mut next_mv = cands[0];
                            let mut best = f64::NEG_INFINITY;
                            for &mv in &cands {
                                let mut v = crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
                                    game,
                                    &cur,
                                    mv,
                                    &scores_now,
                                    scores_now[0] as f64,
                                    max_now,
                                    phase_now,
                                    &conflict,
                                    cur.pos[0],
                                    &leaders,
                                );
                                v -= 0.12 * pressure_score(game, state, mv);
                                if risk_mode {
                                    v -= 0.25 * conflict[mv.0][mv.1];
                                }
                                if v > best {
                                    best = v;
                                    next_mv = mv;
                                }
                            }
                            moves.push(next_mv);
                            moves.extend(crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(&scores_now, top2, 1));
                        }
                        cur = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, &cur, &moves,
                        );
                        total += (0.88_f64).powi(step as i32)
                            * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game, &cur,
                            );
                    }
                    total
                }

                pub(super) fn choose_move_x25_race_adaptive_recovery(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    if !(4..=6).contains(&game.m) {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }

                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2);
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let phase = state.turn as f64 / game.t as f64;
                    let leader_gap = ((max_ai as f64 - scores[0] as f64).abs()).min(100_000.0);
                    let conflict =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let conf =
                        conflict.iter().flatten().sum::<f64>() / (game.n as f64 * game.n as f64);

                    let cands =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if cands.is_empty() {
                        return state.pos[0];
                    }
                    if cands.len() <= 3 {
                        return cands[0];
                    }

                    let mut votes: HashMap<(usize, usize), f64> = HashMap::new();
                    for (mv, w) in
                        propose_votes(game, state, models, uncertainty, phase, leader_gap, conf)
                    {
                        *votes.entry(mv).or_insert(0.0) += w;
                    }

                    let mut leaders = vec![false; game.m];
                    for p in 1..game.m {
                        if scores[p] == max_ai {
                            leaders[p] = true;
                        }
                    }
                    let s0 = scores[0] as f64;
                    for &mv in cands.iter().take(18) {
                        let local =
                            crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
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
                        let base = votes.entry(mv).or_insert(0.0);
                        *base += 0.015 * local - 0.07 * pressure_score(game, state, mv);
                    }

                    let mut pool: Vec<_> = votes.into_iter().collect();
                    if pool.is_empty() {
                        return x04_macro_route::choose_move_x04_macro_route(game, state, models);
                    }
                    pool.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    pool.truncate(pool.len().max(8).min(16));

                    let mut best_mv = pool[0].0;
                    let mut best = f64::NEG_INFINITY;
                    let risk_mode = uncertainty > 0.24 || conf > 1.05;
                    for &(mv, w) in pool.iter().take(12) {
                        let mut p1 = Vec::with_capacity(game.m);
                        p1.push(mv);
                        p1.extend(top2.iter().map(|x| x.0).take(game.m - 1));
                        let mut p2 = Vec::with_capacity(game.m);
                        p2.push(mv);
                        p2.extend(
                            crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(
                                &scores, &top2, 1,
                            )
                            .into_iter()
                            .take(game.m - 1),
                        );
                        let s1 = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &p1,
                        );
                        let s2 = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &p2,
                        );
                        let rollout = build_rollout(game, state, models, mv, &top2, risk_mode);
                        let front = 0.10
                            * crate::__cargo_equip::crates::ahc061_solver::frontier_potential(
                                game, &s1,
                            );
                        let score =
                            0.50 * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game, &s1,
                            ) + 0.25
                                * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                    game, &s2,
                                )
                                + 0.18 * rollout
                                + 0.12 * w
                                + front
                                - 0.02 * conflict[mv.0][mv.1];
                        if score > best {
                            best = score;
                            best_mv = mv;
                        }
                    }

                    if best.is_finite() {
                        best_mv
                    } else {
                        x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models)
                    }
                }
            }
            mod x26_reactive_frontier_pressure {
                use std::collections::HashMap;

                use crate::__cargo_equip::crates::ahc061_solver::{
                    x01_beam_pessimistic, x04_macro_route, x06_expert_switch_hybrid,
                    x08_pressure_frontier, x18_robust_minmax_guard, x19_frontier_recovery_sweep,
                    AiModel, Game, State,
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
                        if !crate::__cargo_equip::crates::ahc061_solver::in_bounds(game.n, nx, ny) {
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
                        (
                            x01_beam_pessimistic::choose_move_x01_beam_pessimistic(
                                game, state, models,
                            ),
                            w01 / total,
                        ),
                        (
                            x04_macro_route::choose_move_x04_macro_route(game, state, models),
                            w04 / total,
                        ),
                        (
                            x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                                game, state, models,
                            ),
                            w06 / total,
                        ),
                        (
                            x08_pressure_frontier::choose_move_x08_pressure_frontier(
                                game, state, models,
                            ),
                            w08 / total,
                        ),
                        (
                            x19_frontier_recovery_sweep::choose_move_x19_frontier_recovery_sweep(
                                game, state, models,
                            ),
                            w19 / total,
                        ),
                        (
                            x18_robust_minmax_guard::choose_move_x18_robust_minmax_guard(
                                game, state, models,
                            ),
                            w18 / total,
                        ),
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
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
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
                            let cands = crate::__cargo_equip::crates::ahc061_solver::get_candidates(
                                game, &cur, 0,
                            );
                            if cands.is_empty() {
                                break;
                            }
                            let conflict =
                                crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                                    game, &cur, models,
                                );
                            let s0 = scores[0] as f64;
                            let mut next_mv = cands[0];
                            let mut best = f64::NEG_INFINITY;
                            for &mv in &cands {
                                let mut v = crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
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
                        cur = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, &cur, &moves,
                        );
                        total += (0.90_f64).powi(step as i32)
                            * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game, &cur,
                            );
                    }
                    total
                }

                pub(super) fn choose_move_x26_reactive_frontier_pressure(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    if !(4..=6).contains(&game.m) {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }

                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2);
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let phase = state.turn as f64 / game.t as f64;
                    let gap = (max_ai as f64 - scores[0] as f64).abs().min(100_000.0);
                    let conflict =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );
                    let conf =
                        conflict.iter().flatten().sum::<f64>() / (game.n as f64 * game.n as f64);

                    let cands =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if cands.is_empty() {
                        return state.pos[0];
                    }
                    if cands.len() <= 3 {
                        return cands[0];
                    }

                    let mut votes: HashMap<(usize, usize), f64> = HashMap::new();
                    for (mv, w) in
                        weighted_votes(game, state, models, uncertainty, phase, conf, gap)
                    {
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
                        let local =
                            crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
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
                        secondary.extend(
                            crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(
                                &scores, &top2, 1,
                            )
                            .into_iter()
                            .take(game.m - 1),
                        );
                        let ns1 = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &primary,
                        );
                        let ns2 = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game, state, &secondary,
                        );
                        let core =
                            0.53 * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game, &ns1,
                            ) + 0.24
                                * crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                    game, &ns2,
                                );
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
                        x18_robust_minmax_guard::choose_move_x18_robust_minmax_guard(
                            game, state, models,
                        )
                    }
                }
            }
            mod x64_portfolio_mixer {
                use std::collections::HashMap;
                use std::env;

                use crate::__cargo_equip::crates::ahc061_solver::{
                    x01_beam_pessimistic, x04_macro_route, x06_expert_switch_hybrid,
                    x10_phase_adaptive_mix, x11_contest_frontier_recovery, x13_frontier_consensus,
                    x14_adaptive_risk_lane, x15_band_adaptive_route, x19_frontier_recovery_sweep,
                    x26_reactive_frontier_pressure, AiModel, Game, State,
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
                        if !crate::__cargo_equip::crates::ahc061_solver::in_bounds(game.n, nx, ny) {
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
                    let w_x04 =
                        env_f64("AHC_X64_W_X04", 1.10, 0.0, 3.0) + 0.15 * (1.0 - uncertainty);
                    let w_x06 = env_f64("AHC_X64_W_X06", 1.00, 0.0, 3.0) + 0.10 * phase;
                    let w_x10 = env_f64("AHC_X64_W_X10", 0.55, 0.0, 3.0) + 0.08 * uncertainty;
                    let w_x11 = env_f64("AHC_X64_W_X11", 0.70, 0.0, 3.0) + 0.10 * gap;
                    let w_x13 =
                        env_f64("AHC_X64_W_X13", 0.85, 0.0, 3.0) + 0.05 * (1.0 - uncertainty);
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
                            x15_band_adaptive_route::choose_move_x15_band_adaptive_route(
                                game, state, models,
                            ),
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
                    let candidates =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if candidates.is_empty() {
                        return state.pos[0];
                    }
                    if candidates.len() == 1 {
                        return candidates[0];
                    }

                    let phase = state.turn as f64 / game.t as f64;
                    let top2 =
                        crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(
                            game, state, models,
                        );
                    let uncertainty =
                        crate::__cargo_equip::crates::ahc061_solver::uncertainty_risk(&top2);
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let s0 = scores[0] as f64;
                    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let gap = ((max_ai as f64 - s0).max(0.0) / s0.max(1.0)).clamp(0.0, 1.5);
                    let conflict =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );

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
                        let local =
                            crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
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
                    local_rank
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

                    let mut pool = Vec::<(usize, usize)>::new();
                    for (mv, _) in &local_rank {
                        if !pool.contains(mv) {
                            pool.push(*mv);
                        }
                        if pool.len() >= 8 {
                            break;
                        }
                    }
                    let mut voted_moves: Vec<((usize, usize), f64)> =
                        vote_map.iter().map(|(k, v)| (*k, *v)).collect();
                    voted_moves
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    for (mv, _) in voted_moves.into_iter().take(8) {
                        if !pool.contains(&mv) {
                            pool.push(mv);
                        }
                    }
                    if pool.is_empty() {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }

                    let sec_cap = env_u64(
                        "AHC_X64_SECONDARY_CAP",
                        if game.m >= 6 { 2 } else { 1 },
                        1,
                        3,
                    );
                    let secondary =
                        crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(
                            &scores, &top2, sec_cap,
                        );
                    let primary: Vec<(usize, usize)> = top2.iter().map(|x| x.0).collect();

                    let aggro_gap = env_f64("AHC_X64_AGGRO_GAP", 0.20, 0.0, 1.0);
                    let aggro = if gap >= aggro_gap { 1.0 } else { 0.0 };
                    let w_primary = env_f64("AHC_X64_W_PRIMARY", 0.56, 0.20, 0.90) + 0.08 * aggro;
                    let w_secondary =
                        env_f64("AHC_X64_W_SECONDARY", 0.34, 0.05, 0.70) - 0.05 * aggro;
                    let w_local = env_f64("AHC_X64_W_LOCAL", 0.12, 0.00, 0.50);
                    let w_vote = env_f64("AHC_X64_W_VOTE", 22.0, 0.0, 60.0);
                    let w_frontier = env_f64("AHC_X64_W_FRONTIER", 0.0016, 0.0, 0.01);
                    let w_risk = env_f64("AHC_X64_W_RISK", 0.82, 0.0, 3.0) - 0.22 * aggro;
                    let w_gap_bonus = env_f64("AHC_X64_W_GAP_BONUS", 14.0, 0.0, 50.0);
                    let w_uncertainty = env_f64("AHC_X64_W_UNCERTAINTY", 0.06, 0.0, 0.40);

                    let mut best_mv = pool[0];
                    let mut best_val = f64::NEG_INFINITY;

                    for &mv in &pool {
                        let local =
                            crate::__cargo_equip::crates::ahc061_solver::evaluate_local_move(
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

                        let ns_primary = crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                            game,
                            state,
                            &moves_primary,
                        );
                        let ns_secondary =
                            crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                game,
                                state,
                                &moves_secondary,
                            );
                        let s_primary =
                            crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game,
                                &ns_primary,
                            );
                        let s_secondary =
                            crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                game,
                                &ns_secondary,
                            );

                        let vote = *vote_map.get(&mv).unwrap_or(&0.0);
                        let frontier = frontier_opportunity(game, state, mv);
                        let risk = conflict[mv.0][mv.1] * game.v[mv.0][mv.1] as f64;
                        let gap_bonus = if scores[0] < max_ai {
                            gap * w_gap_bonus
                        } else {
                            0.0
                        };

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
            }
            mod x67_gear_shift_hybrid {
                use std::collections::{HashMap, HashSet};
                use std::env;

                use crate::__cargo_equip::crates::ahc061_solver::{
                    x04_macro_route, x06_expert_switch_hybrid, x18_robust_minmax_guard,
                    x19_frontier_recovery_sweep, x20_band_stage_ensemble,
                    x21_band_stage_adaptive_guard, x22_band_stage_recovery_boost,
                    x25_race_adaptive_recovery, x26_reactive_frontier_pressure, AiModel, Game,
                    State,
                };

                fn env_f64(name: &str, default: f64, min: f64, max: f64) -> f64 {
                    env::var(name)
                        .ok()
                        .and_then(|s| s.parse::<f64>().ok())
                        .unwrap_or(default)
                        .clamp(min, max)
                }

                fn env_usize(name: &str, default: usize, min: usize, max: usize) -> usize {
                    env::var(name)
                        .ok()
                        .and_then(|s| s.parse::<usize>().ok())
                        .unwrap_or(default)
                        .clamp(min, max)
                }

                fn leader_flags(game: &Game, scores: &[i64]) -> Vec<bool> {
                    let mut flags = vec![false; game.m];
                    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    for p in 1..game.m {
                        if scores[p] == max_ai {
                            flags[p] = true;
                        }
                    }
                    flags
                }

                fn role_signal(
                    game: &Game,
                    state: &State,
                    mv: (usize, usize),
                    leaders: &[bool],
                    phase: f64,
                ) -> f64 {
                    let (x, y) = mv;
                    let owner = state.owner[x][y];
                    let level = state.level[x][y];
                    let v = game.v[x][y] as f64;

                    if owner == -1 {
                        (1.04 - 0.28 * phase).clamp(0.50, 1.10) * v
                    } else if owner == 0 {
                        if level < game.u {
                            (0.68 + 0.20 * (1.0 - phase)) * v * (game.u - level) as f64
                                / game.u as f64
                        } else {
                            -0.10 * v
                        }
                    } else {
                        let opp = owner as usize;
                        let lead_bonus = if leaders.get(opp).copied().unwrap_or(false) {
                            1.0
                        } else {
                            0.0
                        };
                        if level == 1 {
                            (1.15 + 0.55 * phase + 0.45 * lead_bonus) * v
                        } else {
                            (0.28 + 0.35 * phase + 0.20 * lead_bonus) * v / level as f64
                        }
                    }
                }

                fn neighborhood_signal(game: &Game, state: &State, mv: (usize, usize)) -> f64 {
                    const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                    let mut gain = 0.0_f64;
                    for (dx, dy) in DIRS {
                        let nx = mv.0 as isize + dx;
                        let ny = mv.1 as isize + dy;
                        if !crate::__cargo_equip::crates::ahc061_solver::in_bounds(game.n, nx, ny) {
                            continue;
                        }
                        let ux = nx as usize;
                        let uy = ny as usize;
                        let v = game.v[ux][uy] as f64;
                        match state.owner[ux][uy] {
                            -1 => gain += 0.07 * v,
                            0 => {
                                if state.level[ux][uy] < game.u {
                                    gain += 0.04 * v;
                                }
                            }
                            _ => {
                                if state.level[ux][uy] == 1 {
                                    gain += 0.09 * v;
                                } else {
                                    gain += 0.03 * v / state.level[ux][uy] as f64;
                                }
                            }
                        }
                    }
                    gain
                }

                fn rollout_move_value(
                    game: &Game,
                    state: &State,
                    mv: (usize, usize),
                    scores: &[i64],
                    leaders: &[bool],
                    phase: f64,
                ) -> f64 {
                    let role = role_signal(game, state, mv, leaders, phase);
                    let around = neighborhood_signal(game, state, mv);
                    let dist =
                        state.pos[0].0.abs_diff(mv.0) as f64 + state.pos[0].1.abs_diff(mv.1) as f64;

                    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1) as f64;
                    let s0 = scores[0] as f64;
                    let gap = ((max_ai - s0) / s0.max(1.0)).clamp(-1.0, 2.0);
                    let chase = if gap > 0.0 { 1.0 + 0.40 * gap } else { 0.92 };

                    let owner = state.owner[mv.0][mv.1];
                    let level = state.level[mv.0][mv.1];
                    let mut score = chase * role + 0.10 * around - (0.03 + 0.02 * phase) * dist;
                    if owner == 0 && level == game.u && mv == state.pos[0] {
                        score -= 0.12 * game.v[mv.0][mv.1] as f64;
                    }
                    if owner > 0 && level >= 2 {
                        score -= 0.06 * game.v[mv.0][mv.1] as f64;
                    }
                    score
                }

                fn pick_rollout_move(game: &Game, state: &State, phase: f64) -> (usize, usize) {
                    let cands =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if cands.is_empty() {
                        return state.pos[0];
                    }
                    if cands.len() == 1 {
                        return cands[0];
                    }
                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let leaders = leader_flags(game, &scores);
                    let mut best_mv = cands[0];
                    let mut best = f64::NEG_INFINITY;
                    for &mv in &cands {
                        let v = rollout_move_value(game, state, mv, &scores, &leaders, phase);
                        if v > best {
                            best = v;
                            best_mv = mv;
                        }
                    }
                    best_mv
                }

                fn scenario_ai_moves(
                    top2: &[((usize, usize), (usize, usize), f64)],
                    secondary: &[(usize, usize)],
                    leaders: &[bool],
                    scenario_id: usize,
                    step: usize,
                ) -> Vec<(usize, usize)> {
                    let mut out = Vec::with_capacity(top2.len());
                    for i in 0..top2.len() {
                        let player = i + 1;
                        let p1 = top2[i].0;
                        let p2 = top2[i].1;
                        let conf = top2[i].2;
                        let sec = secondary.get(i).copied().unwrap_or(p1);
                        let leader = leaders.get(player).copied().unwrap_or(false);

                        let mv = match scenario_id {
                            0 => p1,
                            1 => sec,
                            2 => {
                                if conf < 0.70 || ((step + player) % 2 == 1) {
                                    p2
                                } else {
                                    p1
                                }
                            }
                            _ => {
                                if leader {
                                    if conf < 0.90 {
                                        p2
                                    } else {
                                        p1
                                    }
                                } else if conf < 0.62 {
                                    sec
                                } else {
                                    p1
                                }
                            }
                        };
                        out.push(mv);
                    }
                    out
                }

                fn advisor_suggestions(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    gap_ratio: f64,
                    phase: f64,
                ) -> Vec<((usize, usize), f64)> {
                    let mut w_x04 = env_f64("AHC_X67_W_X04", 0.22, 0.0, 2.0);
                    let mut w_x06 = env_f64("AHC_X67_W_X06", 0.12, 0.0, 2.0);
                    let mut w_x18 = env_f64("AHC_X67_W_X18", 0.10, 0.0, 2.0);
                    let mut w_x19 = env_f64("AHC_X67_W_X19", 0.18, 0.0, 2.0);
                    let mut w_x20 = env_f64("AHC_X67_W_X20", 0.08, 0.0, 2.0);
                    let mut w_x21 = env_f64("AHC_X67_W_X21", 0.08, 0.0, 2.0);
                    let w_x22 = env_f64("AHC_X67_W_X22", 0.05, 0.0, 2.0);
                    let w_x25 = env_f64("AHC_X67_W_X25", 0.05, 0.0, 2.0);
                    let mut w_x26 = env_f64("AHC_X67_W_X26", 0.12, 0.0, 2.0);

                    if gap_ratio >= 0.20 {
                        w_x04 += 0.10;
                        w_x19 += 0.08;
                        w_x26 += 0.08;
                        w_x06 -= 0.03;
                    } else if phase >= 0.70 {
                        w_x18 += 0.06;
                        w_x20 += 0.05;
                        w_x21 += 0.05;
                    } else {
                        w_x06 += 0.03;
                    }

                    let total =
                        (w_x04 + w_x06 + w_x18 + w_x19 + w_x20 + w_x21 + w_x22 + w_x25 + w_x26)
                            .max(1e-9);
                    vec![
                        (
                            x04_macro_route::choose_move_x04_macro_route(game, state, models),
                            w_x04 / total,
                        ),
                        (
                            x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models),
                            w_x06 / total,
                        ),
                        (
                            x18_robust_minmax_guard::choose_move_x18_robust_minmax_guard(game, state, models),
                            w_x18 / total,
                        ),
                        (
                            x19_frontier_recovery_sweep::choose_move_x19_frontier_recovery_sweep(game, state, models),
                            w_x19 / total,
                        ),
                        (
                            x20_band_stage_ensemble::choose_move_x20_band_stage_ensemble(game, state, models),
                            w_x20 / total,
                        ),
                        (
                            x21_band_stage_adaptive_guard::choose_move_x21_band_stage_adaptive_guard(game, state, models),
                            w_x21 / total,
                        ),
                        (
                            x22_band_stage_recovery_boost::choose_move_x22_band_stage_recovery_boost(game, state, models),
                            w_x22 / total,
                        ),
                        (
                            x25_race_adaptive_recovery::choose_move_x25_race_adaptive_recovery(game, state, models),
                            w_x25 / total,
                        ),
                        (
                            x26_reactive_frontier_pressure::choose_move_x26_reactive_frontier_pressure(game, state, models),
                            w_x26 / total,
                        ),
                    ]
                }

                fn evaluate_candidate_rollout(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                    first_mv: (usize, usize),
                    phase0: f64,
                    gap_ratio: f64,
                ) -> f64 {
                    let horizon_base = env_usize("AHC_X67_HORIZON_BASE", 3, 2, 5);
                    let horizon_late = env_usize("AHC_X67_HORIZON_LATE", 2, 1, 4);
                    let horizon = if phase0 < 0.76 {
                        horizon_base
                    } else {
                        horizon_late
                    };
                    let scenario_env = env_usize("AHC_X67_SCENARIO_COUNT", 0, 0, 6);
                    let scenario_count = if scenario_env > 0 {
                        scenario_env
                    } else if game.m >= 6 || gap_ratio >= 0.20 {
                        4
                    } else {
                        3
                    };
                    let mut scenario_values = Vec::with_capacity(scenario_count);

                    for scenario_id in 0..scenario_count {
                        let mut cur = state.clone();
                        let mut total = 0.0_f64;
                        for step in 0..horizon {
                            let phase = (cur.turn as f64 / game.t as f64).clamp(0.0, 1.0);
                            let prev_scores =
                                crate::__cargo_equip::crates::ahc061_solver::calc_scores(
                                    game, &cur,
                                );
                            let leaders = leader_flags(game, &prev_scores);
                            let my_mv = if step == 0 {
                                first_mv
                            } else {
                                pick_rollout_move(game, &cur, phase)
                            };
                            let top2 = crate::__cargo_equip::crates::ahc061_solver::choose_predicted_ai_top2_moves(game, &cur, models);
                            let sec_cap = if game.m >= 7 {
                                3
                            } else if game.m >= 5 {
                                2
                            } else {
                                1
                            };
                            let secondary = crate::__cargo_equip::crates::ahc061_solver::build_secondary_ai_moves(&prev_scores, &top2, sec_cap);
                            let ai_moves =
                                scenario_ai_moves(&top2, &secondary, &leaders, scenario_id, step);

                            let mut all_moves = Vec::with_capacity(game.m);
                            all_moves.push(my_mv);
                            all_moves.extend(ai_moves);

                            let mut next =
                                crate::__cargo_equip::crates::ahc061_solver::simulate_turn(
                                    game, &cur, &all_moves,
                                );
                            next.turn = (cur.turn + 1).min(game.t);

                            let next_scores =
                                crate::__cargo_equip::crates::ahc061_solver::calc_scores(
                                    game, &next,
                                );
                            let my_prev = prev_scores[0] as f64;
                            let my_next = next_scores[0] as f64;
                            let lead_prev = prev_scores
                                .iter()
                                .skip(1)
                                .copied()
                                .max()
                                .unwrap_or(1)
                                .max(1) as f64;
                            let lead_next = next_scores
                                .iter()
                                .skip(1)
                                .copied()
                                .max()
                                .unwrap_or(1)
                                .max(1) as f64;

                            let ratio_prev = my_prev / lead_prev.max(1.0);
                            let ratio_next = my_next / lead_next.max(1.0);
                            let ratio_gain = (ratio_next - ratio_prev).clamp(-2.0, 2.0);
                            let sabotage =
                                ((lead_prev - lead_next) / lead_prev.max(1.0)).clamp(-1.0, 1.0);
                            let growth = ((my_next - my_prev) / my_prev.max(1.0)).clamp(-1.0, 1.0);

                            let strategic =
                                crate::__cargo_equip::crates::ahc061_solver::strategic_score(
                                    game, &next,
                                );
                            let role = role_signal(game, &cur, my_mv, &leaders, phase);
                            let step_value = 0.72 * strategic
                                + 20_000.0 * ratio_gain
                                + 9_000.0 * sabotage
                                + 5_500.0 * growth
                                + 0.05 * role;
                            total += (0.91_f64).powi(step as i32) * step_value;

                            cur = next;
                        }
                        scenario_values.push(total);
                    }

                    if scenario_values.is_empty() {
                        return f64::NEG_INFINITY;
                    }

                    let mean = scenario_values.iter().sum::<f64>() / scenario_values.len() as f64;
                    scenario_values
                        .sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                    let min = scenario_values[0];
                    let q25 = scenario_values[(scenario_values.len() - 1) / 4];
                    let risk = if gap_ratio >= 0.20 {
                        env_f64("AHC_X67_RISK_DEFICIT", 0.14, 0.0, 0.8)
                    } else if gap_ratio >= 0.05 {
                        env_f64("AHC_X67_RISK_NEUTRAL", 0.24, 0.0, 0.8)
                    } else {
                        env_f64("AHC_X67_RISK_LEAD", 0.36, 0.0, 0.8)
                    };
                    (1.0 - risk) * mean + risk * min + 0.08 * q25
                }

                pub(super) fn choose_move_x67_gear_shift_hybrid(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    let candidates =
                        crate::__cargo_equip::crates::ahc061_solver::get_candidates(game, state, 0);
                    if candidates.is_empty() {
                        return state.pos[0];
                    }
                    if candidates.len() == 1 {
                        return candidates[0];
                    }

                    let scores =
                        crate::__cargo_equip::crates::ahc061_solver::calc_scores(game, state);
                    let s0 = scores[0] as f64;
                    let max_ai_i64 = scores.iter().skip(1).copied().max().unwrap_or(1).max(1);
                    let phase = (state.turn as f64 / game.t as f64).clamp(0.0, 1.0);
                    let gap_ratio = ((max_ai_i64 as f64 - s0) / s0.max(1.0)).clamp(-1.0, 2.0);
                    let leaders = leader_flags(game, &scores);
                    let conflict =
                        crate::__cargo_equip::crates::ahc061_solver::estimate_conflict_map(
                            game, state, models,
                        );

                    let advisor_votes = advisor_suggestions(game, state, models, gap_ratio, phase);
                    let mut vote_map = HashMap::<(usize, usize), f64>::new();
                    for (mv, w) in advisor_votes {
                        *vote_map.entry(mv).or_insert(0.0) += w;
                    }

                    let mut local_map = HashMap::<(usize, usize), f64>::new();
                    let mut local_rank = Vec::<((usize, usize), f64)>::new();
                    let mut sabotage_rank = Vec::<((usize, usize), f64)>::new();
                    let mut stable_rank = Vec::<((usize, usize), f64)>::new();

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
                                &conflict,
                                state.pos[0],
                                &leaders,
                            );
                        local_map.insert(mv, local);

                        let v = game.v[mv.0][mv.1] as f64;
                        let risk = conflict[mv.0][mv.1] * v;
                        let role = role_signal(game, state, mv, &leaders, phase);
                        let around = neighborhood_signal(game, state, mv);

                        local_rank.push((mv, local));
                        sabotage_rank
                            .push((mv, role + 0.08 * around - (0.18 + 0.25 * phase) * risk));
                        stable_rank.push((mv, local + 0.05 * role - (0.58 + 0.15 * phase) * risk));
                    }

                    local_rank
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    sabotage_rank
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    stable_rank
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

                    let mut pool = Vec::<(usize, usize)>::new();
                    let mut seen = HashSet::<(usize, usize)>::new();
                    for list in [
                        local_rank.iter().take(6),
                        sabotage_rank.iter().take(5),
                        stable_rank.iter().take(5),
                    ] {
                        for (mv, _) in list {
                            if seen.insert(*mv) {
                                pool.push(*mv);
                            }
                        }
                    }

                    let mut voted: Vec<((usize, usize), f64)> =
                        vote_map.iter().map(|(k, v)| (*k, *v)).collect();
                    voted
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                    for (mv, _) in voted.into_iter().take(8) {
                        if seen.insert(mv) {
                            pool.push(mv);
                        }
                    }

                    if pool.is_empty() {
                        return x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        );
                    }

                    let cap = pool.len().min(14);
                    let local_weight = env_f64("AHC_X67_LOCAL_WEIGHT", 0.16, 0.0, 0.8);
                    let vote_weight = env_f64("AHC_X67_VOTE_WEIGHT", 15000.0, 0.0, 100000.0);
                    let mut best_mv = pool[0];
                    let mut best_score = f64::NEG_INFINITY;
                    for &mv in pool.iter().take(cap) {
                        let rollout =
                            evaluate_candidate_rollout(game, state, models, mv, phase, gap_ratio);
                        let local = *local_map.get(&mv).unwrap_or(&0.0);
                        let vote = *vote_map.get(&mv).unwrap_or(&0.0);
                        let total = rollout + local_weight * local + vote_weight * vote;
                        if total > best_score {
                            best_score = total;
                            best_mv = mv;
                        }
                    }

                    if best_score.is_finite() {
                        best_mv
                    } else {
                        local_rank[0].0
                    }
                }
            }
            mod x73_selective_unlocked_macro {
                use crate::__cargo_equip::crates::ahc061_solver::{
                    x04_macro_route, x06_expert_switch_hybrid, AiModel, Game, State,
                };

                pub(super) fn choose_move_x73_selective_unlocked_macro(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    if matches!(game.m, 3 | 4 | 6) {
                        x04_macro_route::choose_move_x04_macro_route(game, state, models)
                    } else {
                        x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models)
                    }
                }
            }
            mod x75_risk_gated_unlocked_macro {
                use std::env;

                use crate::__cargo_equip::crates::ahc061_solver::{
                    calc_scores, x04_macro_route, x06_expert_switch_hybrid, AiModel, Game, State,
                };

                unsafe fn set_x04_gate(allow_all_m: bool, disable_phase_cutoff: bool) {
                    env::set_var("AHC_X04_ALLOW_ALL_M", if allow_all_m { "1" } else { "0" });
                    env::set_var(
                        "AHC_X04_DISABLE_PHASE_CUTOFF",
                        if disable_phase_cutoff { "1" } else { "0" },
                    );
                }

                fn should_use_unlocked_for_m4(game: &Game, state: &State) -> bool {
                    if game.m != 4 {
                        return false;
                    }
                    let phase = state.turn as f64 / game.t as f64;
                    let scores = calc_scores(game, state);
                    let s0 = scores[0] as f64;
                    let max_ai = scores.iter().skip(1).copied().max().unwrap_or(1).max(1) as f64;
                    let gap_ratio = ((max_ai - s0) / s0.max(1.0)).clamp(-1.0, 2.0);
                    phase <= 0.72 || gap_ratio >= 0.08
                }

                pub(super) fn choose_move_x75_risk_gated_unlocked_macro(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    match game.m {
                        3 | 6 => x04_macro_route::choose_move_x04_macro_route(game, state, models),
                        4 => {
                            if should_use_unlocked_for_m4(game, state) {
                                x04_macro_route::choose_move_x04_macro_route(game, state, models)
                            } else {
                                unsafe {
                                    set_x04_gate(false, false);
                                }
                                let mv = x04_macro_route::choose_move_x04_macro_route(
                                    game, state, models,
                                );
                                unsafe {
                                    set_x04_gate(true, true);
                                }
                                mv
                            }
                        }
                        _ => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        ),
                    }
                }
            }
            mod x76_crossband_route_hybrid {
                use std::env;

                use crate::__cargo_equip::crates::ahc061_solver::{
                    x04_macro_route, x06_expert_switch_hybrid, AiModel, Game, State,
                };

                unsafe fn set_x04_gate(allow_all_m: bool, disable_phase_cutoff: bool) {
                    env::set_var("AHC_X04_ALLOW_ALL_M", if allow_all_m { "1" } else { "0" });
                    env::set_var(
                        "AHC_X04_DISABLE_PHASE_CUTOFF",
                        if disable_phase_cutoff { "1" } else { "0" },
                    );
                }

                pub(super) fn choose_move_x76_crossband_route_hybrid(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    match game.m {
                        3 | 6 => {
                            unsafe {
                                set_x04_gate(true, true);
                            }
                            x04_macro_route::choose_move_x04_macro_route(game, state, models)
                        }
                        4 => {
                            unsafe {
                                set_x04_gate(false, false);
                            }
                            let mv =
                                x04_macro_route::choose_move_x04_macro_route(game, state, models);
                            unsafe {
                                set_x04_gate(true, true);
                            }
                            mv
                        }
                        _ => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        ),
                    }
                }
            }
            mod x210_adaptive_beam_route {
                use crate::__cargo_equip::crates::ahc061_solver::{
                    x04_macro_route, AiModel, Game, State,
                };

                fn set_m_specific_params(m: usize, fast: bool) {
                    let (
                        phase_cutoff,
                        plan_slow,
                        plan_fast,
                        beam_slow,
                        beam_fast,
                        target_count,
                        target_eval,
                        candidate_cap,
                        route_coeff,
                        pressure_late,
                    ) = if fast {
                        // Online mode: aggressively reduced for 2s time limit (~17ms/turn)
                        match m {
                            2 => (0.90, 4, 3, 3, 2, 5, 3, 5, 30.0, 0.60),
                            3 => (0.85, 4, 3, 3, 2, 4, 3, 5, 38.0, 1.00),
                            5 => (0.70, 3, 2, 2, 2, 3, 2, 4, 35.0, 1.50),
                            6 => (0.60, 3, 2, 2, 2, 3, 2, 3, 25.0, 1.80),
                            _ => (0.50, 3, 2, 2, 2, 3, 2, 3, 20.0, 2.00),
                        }
                    } else {
                        // Local mode: full params
                        match m {
                            2 => (0.90, 10, 8, 8, 6, 12, 10, 12, 30.0, 0.60),
                            3 => (0.85, 8, 6, 7, 5, 9, 8, 10, 38.0, 1.00),
                            5 => (0.70, 5, 4, 4, 3, 6, 4, 6, 35.0, 1.50),
                            6 => (0.60, 4, 3, 3, 2, 4, 3, 5, 25.0, 1.80),
                            _ => (0.50, 4, 3, 3, 2, 4, 3, 5, 20.0, 2.00),
                        }
                    };
                    unsafe {
                        std::env::set_var("AHC_X04_PHASE_CUTOFF", phase_cutoff.to_string());
                        std::env::set_var("AHC_X04_PLAN_LEN_SLOW", plan_slow.to_string());
                        std::env::set_var("AHC_X04_PLAN_LEN_FAST", plan_fast.to_string());
                        std::env::set_var("AHC_X04_BEAM_WIDTH_SLOW", beam_slow.to_string());
                        std::env::set_var("AHC_X04_BEAM_WIDTH_FAST", beam_fast.to_string());
                        std::env::set_var("AHC_X04_TARGET_COUNT", target_count.to_string());
                        std::env::set_var("AHC_X04_TARGET_EVAL", target_eval.to_string());
                        std::env::set_var("AHC_X04_CANDIDATE_CAP", candidate_cap.to_string());
                        std::env::set_var("AHC_X04_ROUTE_COEFF", route_coeff.to_string());
                        std::env::set_var(
                            "AHC_X04_PRESSURE_WEIGHT_LATE",
                            pressure_late.to_string(),
                        );
                    }
                }

                pub(super) fn choose_move_x210_adaptive_beam_route(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    if game.m != 4 {
                        let time_limit_ms: u64 = std::env::var("AHC_TIME_LIMIT_MS")
                            .ok()
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(2000);
                        let per_turn = time_limit_ms / game.t as u64;
                        let fast = per_turn <= 25;
                        set_m_specific_params(game.m, fast);
                    }
                    x04_macro_route::choose_move_x04_macro_route(game, state, models)
                }
            }
            mod x211_deep_mc_expectimax {
                use std::time::{Duration, Instant};

                use crate::__cargo_equip::crates::ahc061_solver::{
                    build_ai_candidates_and_probs, calc_scores, estimate_conflict_map,
                    evaluate_local_move, get_candidates, sample_index, simulate_turn,
                    strategic_score, AiModel, FastRng, Game, State,
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
                    ranked
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
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
                            let val = expectimax_inner(
                                game,
                                &next,
                                ai_cp,
                                depth + 1,
                                max_depth,
                                params,
                                rng,
                                deadline,
                            );
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
                        ranked.sort_by(|a, b| {
                            b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
                        });
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
                    ranked
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
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
            }
            mod x212_maxn_mcts {
                use std::time::{Duration, Instant};

                use crate::__cargo_equip::crates::ahc061_solver::{
                    build_ai_candidates_and_probs, calc_scores, estimate_conflict_map,
                    evaluate_local_move, get_candidates, sample_index, simulate_turn,
                    strategic_score, AiModel, FastRng, Game, State,
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
                    ranked
                        .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
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
                                    a.total_score / a.visits as f64
                                        + c * (ln_total / a.visits as f64).sqrt()
                                };
                                let ub = if b.visits == 0 {
                                    f64::INFINITY
                                } else {
                                    b.total_score / b.visits as f64
                                        + c * (ln_total / b.visits as f64).sqrt()
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
                        let val =
                            greedy_rollout(game, &next, &ai_cp, &mut rng, params.rollout_depth);

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
            }
            mod x213_hybrid_portfolio {
                use crate::__cargo_equip::crates::ahc061_solver::{
                    x04_macro_route, x06_expert_switch_hybrid, x210_adaptive_beam_route,
                    x211_deep_mc_expectimax, AiModel, Game, State,
                };

                pub(super) fn choose_move_x213_hybrid_portfolio(
                    game: &Game,
                    state: &State,
                    models: &[AiModel],
                ) -> (usize, usize) {
                    match game.m {
                        3 => x210_adaptive_beam_route::choose_move_x210_adaptive_beam_route(
                            game, state, models,
                        ),
                        4 => x04_macro_route::choose_move_x04_macro_route(game, state, models),
                        5 | 6 => x211_deep_mc_expectimax::choose_move_x211_deep_mc_expectimax(
                            game, state, models,
                        ),
                        _ => x06_expert_switch_hybrid::choose_move_x06_expert_switch(
                            game, state, models,
                        ),
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

                // Keep neighbor traversal order aligned with the official tools implementation.
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
