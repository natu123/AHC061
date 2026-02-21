use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    unsafe {
        // x47 champion params (for M=4 via x04, and baseline for M=3 via x210)
        env::set_var("AHC_X04_PHASE_CUTOFF", "0.79");
        env::set_var("AHC_X04_PHASE_SPLIT", "0.54");
        env::set_var("AHC_X04_TARGET_COUNT", "8");
        env::set_var("AHC_X04_TARGET_EVAL", "7");
        env::set_var("AHC_X04_CANDIDATE_CAP", "9");
        env::set_var("AHC_X04_PLAN_LEN_FAST", "6");
        env::set_var("AHC_X04_PLAN_LEN_SLOW", "8");
        env::set_var("AHC_X04_BEAM_WIDTH_FAST", "5");
        env::set_var("AHC_X04_BEAM_WIDTH_SLOW", "7");
        env::set_var("AHC_X04_BRANCH_WIDTH", "2");
        env::set_var("AHC_X04_LOCAL_WEIGHT", "0.10");
        env::set_var("AHC_X04_LOCAL_COEFF", "0.09");
        env::set_var("AHC_X04_ROUTE_COEFF", "46.0");
        env::set_var("AHC_X04_TARGET_PRESSURE_WEIGHT", "0.65");
        env::set_var("AHC_X04_PRESSURE_WEIGHT", "0.30");
        env::set_var("AHC_X04_PRESSURE_WEIGHT_EARLY", "-0.05");
        env::set_var("AHC_X04_PRESSURE_WEIGHT_LATE", "1.10");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT", "-0.10");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_EARLY", "0.18");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_LATE", "0.38");
        env::set_var("AHC_X04_PRESSURE_PHASE_SPLIT", "0.64");
        // Allow x04 beam for M=3 (via x210) and M=4
        env::set_var("AHC_X04_ALLOWED_M", "3,4");
    }
    run_with_strategy(StrategyMode::HybridPortfolio);
}
