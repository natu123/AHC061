use std::env;

use crate::{
    x01_beam_pessimistic, x02_monte_carlo, x03_particle_cvar, x04_macro_route, x05_adaptive_racing_mc,
    x06_expert_switch_hybrid, x07_dual_horizon_route, x08_pressure_frontier, x09_regret_mix,
    x10_phase_adaptive_mix, x11_contest_frontier_recovery, x12_advisor_vote_ensemble, AiModel, Game, State,
    x13_frontier_consensus, x14_adaptive_risk_lane, x15_band_adaptive_route, x16_safe_recovery_route,
    x17_mid_band_dual_lane, x18_robust_minmax_guard, x19_frontier_recovery_sweep,
    x20_band_stage_ensemble, x21_band_stage_adaptive_guard, x22_band_stage_recovery_boost,
    x23_band_stage_frontier_guard, x24_band_stage_adaptive_switch, x25_race_adaptive_recovery,
    x26_reactive_frontier_pressure, x64_portfolio_mixer,
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
}

pub fn strategy_from_env() -> StrategyMode {
    match env::var("AHC_STRATEGY").ok().as_deref() {
        Some("champion") => StrategyMode::Champion,
        Some("mc") | Some("mc_sample") | Some("monte_carlo") => StrategyMode::MonteCarloExplore,
        Some("x03") | Some("particle_cvar") | Some("particle") => StrategyMode::ParticleCvar,
        Some("x04") | Some("macro_route") | Some("macro") => StrategyMode::MacroRoute,
        Some("hybrid_mid_mc") | Some("mid_mc") => StrategyMode::HybridMidMc,
        Some("x05") | Some("adaptive_racing_mc") | Some("armc") => StrategyMode::AdaptiveRacingMc,
        Some("x06") | Some("expert_switch_hybrid") | Some("expert_switch") => {
            StrategyMode::ExpertSwitchHybrid
        }
        Some("x07") | Some("dual_horizon_route") | Some("dual_horizon") => {
            StrategyMode::DualHorizonRoute
        }
        Some("x08") | Some("pressure_frontier") | Some("frontier") => {
            StrategyMode::PressureFrontier
        }
        Some("x09") | Some("regret_mix") | Some("regret") => StrategyMode::RegretMix,
        Some("x10") | Some("phase_adaptive_mix") | Some("phase_mix") => StrategyMode::PhaseAdaptiveMix,
        Some("x11") | Some("contest_frontier_recovery") | Some("frontier_recovery") => {
            StrategyMode::ContestFrontierRecovery
        }
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
        Some("x21") | Some("band_stage_adaptive_guard") | Some("adaptive_guard") => {
            StrategyMode::BandStageAdaptiveGuard
        }
        Some("x22") | Some("band_stage_recovery_boost") | Some("recovery_boost") => {
            StrategyMode::BandStageRecoveryBoost
        }
        Some("x23") | Some("band_stage_frontier_guard") | Some("frontier_guard") => {
            StrategyMode::BandStageFrontierGuard
        }
        Some("x24") | Some("band_stage_adaptive_switch") | Some("adaptive_switch") => {
            StrategyMode::BandStageAdaptiveSwitch
        }
        Some("x25") | Some("race_adaptive_recovery") | Some("adaptive_recovery") => {
            StrategyMode::RaceAdaptiveRecovery
        }
        Some("x26")
            | Some("reactive_frontier_pressure")
            | Some("frontier_pressure")
            | Some("reactive_pressure") => {
            StrategyMode::ReactiveFrontierPressure
        }
        Some("x64")
            | Some("portfolio_mixer")
            | Some("portfolio")
            | Some("meta_portfolio") => StrategyMode::PortfolioMixer,
        _ => StrategyMode::HybridMidMc,
    }
}

pub(crate) fn choose_move(
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
    }
}
