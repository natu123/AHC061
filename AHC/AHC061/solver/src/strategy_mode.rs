use std::env;

use crate::{
    x01_beam_pessimistic, x02_monte_carlo, x03_particle_cvar, x05_adaptive_racing_mc,
    x04_macro_route, x06_expert_switch_hybrid, x07_dual_horizon_route, x08_pressure_frontier,
    x09_regret_mix, AiModel, Game, State,
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
    }
}
