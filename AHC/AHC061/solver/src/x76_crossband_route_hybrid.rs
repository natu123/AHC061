use std::env;

use crate::{x04_macro_route, x06_expert_switch_hybrid, AiModel, Game, State};

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
            let mv = x04_macro_route::choose_move_x04_macro_route(game, state, models);
            unsafe {
                set_x04_gate(true, true);
            }
            mv
        }
        _ => x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models),
    }
}
