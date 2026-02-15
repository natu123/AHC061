use std::env;

use crate::{x01_beam_pessimistic, x02_monte_carlo, AiModel, Game, State};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrategyMode {
    Champion,
    MonteCarloExplore,
    HybridMidMc,
}

pub fn strategy_from_env() -> StrategyMode {
    match env::var("AHC_STRATEGY").ok().as_deref() {
        Some("champion") => StrategyMode::Champion,
        Some("mc") | Some("mc_sample") | Some("monte_carlo") => StrategyMode::MonteCarloExplore,
        Some("hybrid_mid_mc") | Some("mid_mc") => StrategyMode::HybridMidMc,
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
        StrategyMode::HybridMidMc => {
            if (3..=5).contains(&game.m) {
                x02_monte_carlo::choose_move_monte_carlo(game, state, models)
            } else {
                x01_beam_pessimistic::choose_move_x01_beam_pessimistic(game, state, models)
            }
        }
    }
}
