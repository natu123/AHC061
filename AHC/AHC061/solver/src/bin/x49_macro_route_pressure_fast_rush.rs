use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    env::set_var("AHC_X04_PHASE_CUTOFF", "0.75");
    env::set_var("AHC_X04_PHASE_SPLIT", "0.50");
    env::set_var("AHC_X04_TARGET_COUNT", "9");
    env::set_var("AHC_X04_TARGET_EVAL", "8");
    env::set_var("AHC_X04_CANDIDATE_CAP", "10");
    env::set_var("AHC_X04_PLAN_LEN_FAST", "8");
    env::set_var("AHC_X04_PLAN_LEN_SLOW", "6");
    env::set_var("AHC_X04_BEAM_WIDTH_FAST", "3");
    env::set_var("AHC_X04_BEAM_WIDTH_SLOW", "7");
    env::set_var("AHC_X04_BRANCH_WIDTH", "3");
    env::set_var("AHC_X04_LOCAL_WEIGHT", "0.14");
    env::set_var("AHC_X04_LOCAL_COEFF", "0.11");
    env::set_var("AHC_X04_ROUTE_COEFF", "50.0");

    env::set_var("AHC_X04_TARGET_PRESSURE_WEIGHT", "0.95");
    env::set_var("AHC_X04_PRESSURE_WEIGHT", "0.88");
    env::set_var("AHC_X04_PRESSURE_WEIGHT_EARLY", "0.35");
    env::set_var("AHC_X04_PRESSURE_WEIGHT_LATE", "1.45");
    env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT", "0.70");
    env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_EARLY", "0.20");
    env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_LATE", "1.30");
    env::set_var("AHC_X04_PRESSURE_PHASE_SPLIT", "0.56");

    run_with_strategy(StrategyMode::MacroRoute);
}
