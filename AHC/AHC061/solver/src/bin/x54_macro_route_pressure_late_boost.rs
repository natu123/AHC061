use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    env::set_var("AHC_X04_PHASE_CUTOFF", "0.82");
    env::set_var("AHC_X04_PHASE_SPLIT", "0.60");
    env::set_var("AHC_X04_TARGET_COUNT", "10");
    env::set_var("AHC_X04_TARGET_EVAL", "8");
    env::set_var("AHC_X04_CANDIDATE_CAP", "11");
    env::set_var("AHC_X04_PLAN_LEN_FAST", "7");
    env::set_var("AHC_X04_PLAN_LEN_SLOW", "10");
    env::set_var("AHC_X04_BEAM_WIDTH_FAST", "6");
    env::set_var("AHC_X04_BEAM_WIDTH_SLOW", "9");
    env::set_var("AHC_X04_BRANCH_WIDTH", "3");
    env::set_var("AHC_X04_LOCAL_WEIGHT", "0.12");
    env::set_var("AHC_X04_LOCAL_COEFF", "0.10");
    env::set_var("AHC_X04_ROUTE_COEFF", "47.0");

    env::set_var("AHC_X04_TARGET_PRESSURE_WEIGHT", "0.75");
    env::set_var("AHC_X04_PRESSURE_WEIGHT", "0.42");
    env::set_var("AHC_X04_PRESSURE_WEIGHT_EARLY", "0.06");
    env::set_var("AHC_X04_PRESSURE_WEIGHT_LATE", "1.55");
    env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT", "0.28");
    env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_EARLY", "0.16");
    env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_LATE", "0.88");
    env::set_var("AHC_X04_PRESSURE_PHASE_SPLIT", "0.63");

    run_with_strategy(StrategyMode::MacroRoute);
}
