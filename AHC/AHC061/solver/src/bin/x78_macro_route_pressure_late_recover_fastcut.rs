use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    unsafe {
        env::set_var("AHC_X04_PHASE_CUTOFF", "0.62");
        env::set_var("AHC_X04_PHASE_SPLIT", "0.55");
        env::set_var("AHC_X04_TARGET_COUNT", "7");
        env::set_var("AHC_X04_TARGET_EVAL", "5");
        env::set_var("AHC_X04_CANDIDATE_CAP", "8");
        env::set_var("AHC_X04_PLAN_LEN_FAST", "5");
        env::set_var("AHC_X04_PLAN_LEN_SLOW", "6");
        env::set_var("AHC_X04_BEAM_WIDTH_FAST", "4");
        env::set_var("AHC_X04_BEAM_WIDTH_SLOW", "5");
        env::set_var("AHC_X04_BRANCH_WIDTH", "2");
        env::set_var("AHC_X04_LOCAL_WEIGHT", "0.11");
        env::set_var("AHC_X04_LOCAL_COEFF", "0.09");
        env::set_var("AHC_X04_ROUTE_COEFF", "46.2");

        env::set_var("AHC_X04_TARGET_PRESSURE_WEIGHT", "0.74");
        env::set_var("AHC_X04_PRESSURE_WEIGHT", "0.35");
        env::set_var("AHC_X04_PRESSURE_WEIGHT_EARLY", "0.01");
        env::set_var("AHC_X04_PRESSURE_WEIGHT_LATE", "1.10");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT", "0.06");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_EARLY", "0.20");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_LATE", "0.50");
        env::set_var("AHC_X04_PRESSURE_PHASE_SPLIT", "0.60");
    }

    run_with_strategy(StrategyMode::MacroRoute);
}
