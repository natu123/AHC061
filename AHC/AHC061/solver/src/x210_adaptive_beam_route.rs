use crate::{x04_macro_route, AiModel, Game, State};

/// M値に応じた x04 beam search パラメータをセットし、x04 を呼び出す。
/// M=4 は bin エントリで設定済みの x47 champion params をそのまま使用。
/// fast=true の場合はオンライン向けに縮小パラメータを使用。
fn set_m_specific_params(m: usize, fast: bool) {
    let (phase_cutoff, plan_slow, plan_fast, beam_slow, beam_fast,
         target_count, target_eval, candidate_cap, route_coeff, pressure_late) = if fast {
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
        std::env::set_var("AHC_X04_PRESSURE_WEIGHT_LATE", pressure_late.to_string());
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
