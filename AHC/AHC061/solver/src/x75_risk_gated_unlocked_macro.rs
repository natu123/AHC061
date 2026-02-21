use std::env;

use crate::{calc_scores, x04_macro_route, x06_expert_switch_hybrid, AiModel, Game, State};

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
                let mv = x04_macro_route::choose_move_x04_macro_route(game, state, models);
                unsafe {
                    set_x04_gate(true, true);
                }
                mv
            }
        }
        _ => x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models),
    }
}
