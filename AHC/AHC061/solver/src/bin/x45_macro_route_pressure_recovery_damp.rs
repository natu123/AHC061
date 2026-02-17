use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    env::set_var("AHC_X04_PHASE_CUTOFF", "0.82");
    env::set_var("AHC_X04_PHASE_SPLIT", "0.54");
    env::set_var("AHC_X04_TARGET_COUNT", "7");
    env::set_var("AHC_X04_TARGET_EVAL", "7");
    env::set_var("AHC_X04_CANDIDATE_CAP", "8");
    env::set_var("AHC_X04_PLAN_LEN_FAST", "6");
    env::set_var("AHC_X04_PLAN_LEN_SLOW", "6");
    env::set_var("AHC_X04_BEAM_WIDTH_FAST", "4");
    env::set_var("AHC_X04_BEAM_WIDTH_SLOW", "6");
    env::set_var("AHC_X04_BRANCH_WIDTH", "2");
    env::set_var("AHC_X04_LOCAL_WEIGHT", "0.11");
    env::set_var("AHC_X04_LOCAL_COEFF", "0.07");
    env::set_var("AHC_X04_ROUTE_COEFF", "47.0");

    env::set_var("AHC_X04_TARGET_PRESSURE_WEIGHT", "0.40");
    env::set_var("AHC_X04_PRESSURE_WEIGHT", "0.30");
    env::set_var("AHC_X04_PRESSURE_WEIGHT_EARLY", "-0.10");
    env::set_var("AHC_X04_PRESSURE_WEIGHT_LATE", "0.88");
    env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT", "0.02");
    env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_EARLY", "-0.42");
    env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_LATE", "0.65");
    env::set_var("AHC_X04_PRESSURE_PHASE_SPLIT", "0.58");
    run_with_strategy(StrategyMode::MacroRoute);
}
