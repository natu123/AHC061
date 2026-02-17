use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    unsafe {
        env::set_var("AHC_X04_PHASE_CUTOFF", "0.77");
        env::set_var("AHC_X04_PHASE_SPLIT", "0.60");
        env::set_var("AHC_X04_TARGET_COUNT", "8");
        env::set_var("AHC_X04_TARGET_EVAL", "7");
        env::set_var("AHC_X04_CANDIDATE_CAP", "8");
        env::set_var("AHC_X04_PLAN_LEN_FAST", "7");
        env::set_var("AHC_X04_PLAN_LEN_SLOW", "8");
        env::set_var("AHC_X04_BEAM_WIDTH_FAST", "5");
        env::set_var("AHC_X04_BEAM_WIDTH_SLOW", "8");
        env::set_var("AHC_X04_BRANCH_WIDTH", "2");
        env::set_var("AHC_X04_LOCAL_WEIGHT", "0.10");
        env::set_var("AHC_X04_LOCAL_COEFF", "0.09");
        env::set_var("AHC_X04_ROUTE_COEFF", "47.0");

        env::set_var("AHC_X04_TARGET_PRESSURE_WEIGHT", "0.66");
        env::set_var("AHC_X04_PRESSURE_WEIGHT", "0.30");
        env::set_var("AHC_X04_PRESSURE_WEIGHT_EARLY", "-0.02");
        env::set_var("AHC_X04_PRESSURE_WEIGHT_LATE", "1.08");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT", "-0.07");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_EARLY", "0.12");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_LATE", "0.40");
        env::set_var("AHC_X04_PRESSURE_PHASE_SPLIT", "0.63");
    }

    run_with_strategy(StrategyMode::MacroRoute);
}
