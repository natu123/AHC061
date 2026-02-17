use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    env::set_var("AHC_X04_PHASE_CUTOFF", "0.78");
    env::set_var("AHC_X04_PHASE_SPLIT", "0.46");
    env::set_var("AHC_X04_TARGET_COUNT", "6");
    env::set_var("AHC_X04_TARGET_EVAL", "4");
    env::set_var("AHC_X04_CANDIDATE_CAP", "9");
    env::set_var("AHC_X04_PLAN_LEN_FAST", "6");
    env::set_var("AHC_X04_PLAN_LEN_SLOW", "7");
    env::set_var("AHC_X04_BEAM_WIDTH_FAST", "4");
    env::set_var("AHC_X04_BEAM_WIDTH_SLOW", "5");
    env::set_var("AHC_X04_BRANCH_WIDTH", "2");
    env::set_var("AHC_X04_LOCAL_WEIGHT", "0.12");
    env::set_var("AHC_X04_LOCAL_COEFF", "0.07");
    env::set_var("AHC_X04_ROUTE_COEFF", "42.0");
    run_with_strategy(StrategyMode::MacroRoute);
}
