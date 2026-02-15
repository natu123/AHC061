use crate::{x01_beam_pessimistic, x02_monte_carlo, AiModel, Game, State};

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
