use crate::{x04_macro_route, x06_expert_switch_hybrid, AiModel, Game, State};

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
