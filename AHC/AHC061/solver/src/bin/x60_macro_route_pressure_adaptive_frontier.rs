use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    unsafe {
        env::set_var("AHC_X04_PHASE_CUTOFF", "0.80");
        env::set_var("AHC_X04_PHASE_SPLIT", "0.50");
        env::set_var("AHC_X04_TARGET_COUNT", "8");
        env::set_var("AHC_X04_TARGET_EVAL", "7");
        env::set_var("AHC_X04_CANDIDATE_CAP", "10");
        env::set_var("AHC_X04_PLAN_LEN_FAST", "6");
        env::set_var("AHC_X04_PLAN_LEN_SLOW", "8");
        env::set_var("AHC_X04_BEAM_WIDTH_FAST", "6");
        env::set_var("AHC_X04_BEAM_WIDTH_SLOW", "8");
        env::set_var("AHC_X04_BRANCH_WIDTH", "2");
        env::set_var("AHC_X04_LOCAL_WEIGHT", "0.10");
        env::set_var("AHC_X04_LOCAL_COEFF", "0.09");
        env::set_var("AHC_X04_ROUTE_COEFF", "46.0");

        env::set_var("AHC_X04_TARGET_PRESSURE_WEIGHT", "0.67");
        env::set_var("AHC_X04_PRESSURE_WEIGHT", "0.32");
        env::set_var("AHC_X04_PRESSURE_WEIGHT_EARLY", "0.00");
        env::set_var("AHC_X04_PRESSURE_WEIGHT_LATE", "1.14");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT", "-0.06");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_EARLY", "0.14");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_LATE", "0.45");
        env::set_var("AHC_X04_PRESSURE_PHASE_SPLIT", "0.62");
    }

    run_with_strategy(StrategyMode::MacroRoute);
}
