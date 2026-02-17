use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    env::set_var("AHC_X04_PHASE_CUTOFF", "0.78");
    env::set_var("AHC_X04_PHASE_SPLIT", "0.50");
    env::set_var("AHC_X04_TARGET_COUNT", "6");
    env::set_var("AHC_X04_TARGET_EVAL", "5");
    env::set_var("AHC_X04_CANDIDATE_CAP", "8");
    env::set_var("AHC_X04_PLAN_LEN_FAST", "6");
    env::set_var("AHC_X04_PLAN_LEN_SLOW", "8");
    env::set_var("AHC_X04_BEAM_WIDTH_FAST", "5");
    env::set_var("AHC_X04_BEAM_WIDTH_SLOW", "7");
    env::set_var("AHC_X04_BRANCH_WIDTH", "3");
    env::set_var("AHC_X04_LOCAL_WEIGHT", "0.10");
    env::set_var("AHC_X04_LOCAL_COEFF", "0.08");
    env::set_var("AHC_X04_ROUTE_COEFF", "45.0");
    env::set_var("AHC_X04_TARGET_PRESSURE_WEIGHT", "-0.22");
    env::set_var("AHC_X04_PRESSURE_WEIGHT", "-0.18");
    env::set_var("AHC_X04_PRESSURE_WEIGHT_LATE", "-0.35");
    env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT", "-0.30");
    env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_LATE", "-0.48");
    env::set_var("AHC_X04_PRESSURE_PHASE_SPLIT", "0.55");
    run_with_strategy(StrategyMode::MacroRoute);
}
