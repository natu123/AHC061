use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    env::set_var("AHC_X04_PHASE_CUTOFF", "0.75");
    env::set_var("AHC_X04_PHASE_SPLIT", "0.54");
    env::set_var("AHC_X04_TARGET_COUNT", "7");
    env::set_var("AHC_X04_TARGET_EVAL", "8");
    env::set_var("AHC_X04_CANDIDATE_CAP", "9");
    env::set_var("AHC_X04_PLAN_LEN_FAST", "7");
    env::set_var("AHC_X04_PLAN_LEN_SLOW", "8");
    env::set_var("AHC_X04_BEAM_WIDTH_FAST", "6");
    env::set_var("AHC_X04_BEAM_WIDTH_SLOW", "8");
    env::set_var("AHC_X04_BRANCH_WIDTH", "2");
    env::set_var("AHC_X04_LOCAL_WEIGHT", "0.08");
    env::set_var("AHC_X04_LOCAL_COEFF", "0.10");
    env::set_var("AHC_X04_ROUTE_COEFF", "44.0");

    env::set_var("AHC_X04_TARGET_PRESSURE_WEIGHT", "0.20");
    env::set_var("AHC_X04_PRESSURE_WEIGHT", "0.20");
    env::set_var("AHC_X04_PRESSURE_WEIGHT_EARLY", "-0.12");
    env::set_var("AHC_X04_PRESSURE_WEIGHT_LATE", "0.85");
    env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT", "-0.18");
    env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_EARLY", "-0.25");
    env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_LATE", "0.40");
    env::set_var("AHC_X04_PRESSURE_PHASE_SPLIT", "0.62");
    run_with_strategy(StrategyMode::MacroRoute);
}
