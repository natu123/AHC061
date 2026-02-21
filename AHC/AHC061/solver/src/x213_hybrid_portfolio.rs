use crate::{
    x04_macro_route, x06_expert_switch_hybrid, x210_adaptive_beam_route,
    x211_deep_mc_expectimax, AiModel, Game, State,
};

/// M帯別にベストソルバを切り替えるハイブリッド。
///
/// Quick eval (seed 0..19) の M帯別 mean に基づく割当:
///   M=2     → x06 expert switch (x47 同等)  mean 207k
///   M=3     → x210 adaptive beam             mean 208k (+62% vs x47)
///   M=4     → x04 beam (x47 champion params) mean 180k
///   M=5     → x211 deep mc expectimax        mean 101k (+4% vs x47)
///   M=6     → x211 deep mc expectimax        mean 117k (+4% vs x47)
///   M=7,8   → x06 expert switch (x47 同等)  mean 127k
pub(super) fn choose_move_x213_hybrid_portfolio(
    game: &Game,
    state: &State,
    models: &[AiModel],
) -> (usize, usize) {
    match game.m {
        3 => x210_adaptive_beam_route::choose_move_x210_adaptive_beam_route(game, state, models),
        4 => x04_macro_route::choose_move_x04_macro_route(game, state, models),
        5 | 6 => {
            x211_deep_mc_expectimax::choose_move_x211_deep_mc_expectimax(game, state, models)
        }
        _ => x06_expert_switch_hybrid::choose_move_x06_expert_switch(game, state, models),
    }
}
