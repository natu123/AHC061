use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    env::set_var("AHC_X04_PHASE_CUTOFF", "0.86");
    env::set_var("AHC_X04_PHASE_SPLIT", "0.60");
    env::set_var("AHC_X04_TARGET_COUNT", "6");
    env::set_var("AHC_X04_TARGET_EVAL", "5");
    env::set_var("AHC_X04_CANDIDATE_CAP", "7");
    env::set_var("AHC_X04_PLAN_LEN_FAST", "4");
    env::set_var("AHC_X04_PLAN_LEN_SLOW", "6");
    env::set_var("AHC_X04_BEAM_WIDTH_FAST", "4");
    env::set_var("AHC_X04_BEAM_WIDTH_SLOW", "6");
    env::set_var("AHC_X04_BRANCH_WIDTH", "1");
    env::set_var("AHC_X04_LOCAL_WEIGHT", "0.13");
    env::set_var("AHC_X04_LOCAL_COEFF", "0.07");
    env::set_var("AHC_X04_ROUTE_COEFF", "45.0");

    env::set_var("AHC_X04_TARGET_PRESSURE_WEIGHT", "1.05");
    env::set_var("AHC_X04_PRESSURE_WEIGHT", "0.20");
    env::set_var("AHC_X04_PRESSURE_WEIGHT_EARLY", "-0.22");
    env::set_var("AHC_X04_PRESSURE_WEIGHT_LATE", "0.92");
    env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT", "1.05");
    env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_EARLY", "0.55");
    env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_LATE", "1.02");
    env::set_var("AHC_X04_PRESSURE_PHASE_SPLIT", "0.74");

    run_with_strategy(StrategyMode::MacroRoute);
}
