use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    env::set_var("AHC_X04_PHASE_CUTOFF", "0.88");
    env::set_var("AHC_X04_PHASE_SPLIT", "0.58");
    env::set_var("AHC_X04_TARGET_COUNT", "4");
    env::set_var("AHC_X04_TARGET_EVAL", "3");
    env::set_var("AHC_X04_CANDIDATE_CAP", "6");
    env::set_var("AHC_X04_PLAN_LEN_FAST", "5");
    env::set_var("AHC_X04_PLAN_LEN_SLOW", "6");
    env::set_var("AHC_X04_BEAM_WIDTH_FAST", "3");
    env::set_var("AHC_X04_BEAM_WIDTH_SLOW", "4");
    env::set_var("AHC_X04_BRANCH_WIDTH", "2");
    env::set_var("AHC_X04_LOCAL_WEIGHT", "0.08");
    env::set_var("AHC_X04_LOCAL_COEFF", "0.08");
    env::set_var("AHC_X04_ROUTE_COEFF", "46.0");
    run_with_strategy(StrategyMode::MacroRoute);
}
