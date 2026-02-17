use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    env::set_var("AHC_X04_PHASE_CUTOFF", "0.85");
    env::set_var("AHC_X04_PHASE_SPLIT", "0.66");
    env::set_var("AHC_X04_TARGET_COUNT", "5");
    env::set_var("AHC_X04_TARGET_EVAL", "6");
    env::set_var("AHC_X04_CANDIDATE_CAP", "8");
    env::set_var("AHC_X04_PLAN_LEN_FAST", "6");
    env::set_var("AHC_X04_PLAN_LEN_SLOW", "6");
    env::set_var("AHC_X04_BEAM_WIDTH_FAST", "4");
    env::set_var("AHC_X04_BEAM_WIDTH_SLOW", "4");
    env::set_var("AHC_X04_BRANCH_WIDTH", "1");
    env::set_var("AHC_X04_LOCAL_WEIGHT", "0.10");
    env::set_var("AHC_X04_LOCAL_COEFF", "0.03");
    env::set_var("AHC_X04_ROUTE_COEFF", "48.0");
    run_with_strategy(StrategyMode::MacroRoute);
}
