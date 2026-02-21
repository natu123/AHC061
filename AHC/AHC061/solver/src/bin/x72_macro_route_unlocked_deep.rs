use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    unsafe {
        env::set_var("AHC_X04_ALLOW_ALL_M", "1");
        env::set_var("AHC_X04_DISABLE_PHASE_CUTOFF", "1");

        env::set_var("AHC_X04_PHASE_CUTOFF", "0.85");
        env::set_var("AHC_X04_PHASE_SPLIT", "0.50");
        env::set_var("AHC_X04_TARGET_COUNT", "11");
        env::set_var("AHC_X04_TARGET_EVAL", "9");
        env::set_var("AHC_X04_CANDIDATE_CAP", "12");
        env::set_var("AHC_X04_PLAN_LEN_FAST", "7");
        env::set_var("AHC_X04_PLAN_LEN_SLOW", "10");
        env::set_var("AHC_X04_BEAM_WIDTH_FAST", "6");
        env::set_var("AHC_X04_BEAM_WIDTH_SLOW", "9");
        env::set_var("AHC_X04_BRANCH_WIDTH", "3");
        env::set_var("AHC_X04_LOCAL_WEIGHT", "0.13");
        env::set_var("AHC_X04_LOCAL_COEFF", "0.12");
        env::set_var("AHC_X04_ROUTE_COEFF", "44.0");

        env::set_var("AHC_X04_TARGET_PRESSURE_WEIGHT", "0.58");
        env::set_var("AHC_X04_PRESSURE_WEIGHT", "0.24");
        env::set_var("AHC_X04_PRESSURE_WEIGHT_EARLY", "-0.06");
        env::set_var("AHC_X04_PRESSURE_WEIGHT_LATE", "1.45");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT", "-0.10");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_EARLY", "0.26");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_LATE", "0.92");
        env::set_var("AHC_X04_PRESSURE_PHASE_SPLIT", "0.58");
    }

    run_with_strategy(StrategyMode::MacroRoute);
}
