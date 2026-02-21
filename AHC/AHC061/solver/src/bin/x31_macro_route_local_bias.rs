use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    env::set_var("AHC_X04_PHASE_CUTOFF", "0.75");
    env::set_var("AHC_X04_PHASE_SPLIT", "0.58");
    env::set_var("AHC_X04_TARGET_COUNT", "4");
    env::set_var("AHC_X04_TARGET_EVAL", "3");
    env::set_var("AHC_X04_CANDIDATE_CAP", "5");
    env::set_var("AHC_X04_PLAN_LEN_FAST", "4");
    env::set_var("AHC_X04_PLAN_LEN_SLOW", "7");
    env::set_var("AHC_X04_BEAM_WIDTH_FAST", "4");
    env::set_var("AHC_X04_BEAM_WIDTH_SLOW", "6");
    env::set_var("AHC_X04_BRANCH_WIDTH", "2");
    env::set_var("AHC_X04_LOCAL_WEIGHT", "0.12");
    env::set_var("AHC_X04_LOCAL_COEFF", "0.30");
    env::set_var("AHC_X04_ROUTE_COEFF", "35.0");
    run_with_strategy(StrategyMode::MacroRoute);
}
