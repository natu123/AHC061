use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    unsafe {
        env::set_var("AHC_X04_PHASE_CUTOFF", "0.78");
        env::set_var("AHC_X04_PHASE_SPLIT", "0.56");
        env::set_var("AHC_X04_TARGET_COUNT", "9");
        env::set_var("AHC_X04_TARGET_EVAL", "8");
        env::set_var("AHC_X04_CANDIDATE_CAP", "9");
        env::set_var("AHC_X04_PLAN_LEN_FAST", "7");
        env::set_var("AHC_X04_PLAN_LEN_SLOW", "8");
        env::set_var("AHC_X04_BEAM_WIDTH_FAST", "5");
        env::set_var("AHC_X04_BEAM_WIDTH_SLOW", "7");
        env::set_var("AHC_X04_BRANCH_WIDTH", "3");
        env::set_var("AHC_X04_LOCAL_WEIGHT", "0.12");
        env::set_var("AHC_X04_LOCAL_COEFF", "0.11");
        env::set_var("AHC_X04_ROUTE_COEFF", "46.0");

        env::set_var("AHC_X04_TARGET_PRESSURE_WEIGHT", "0.64");
        env::set_var("AHC_X04_PRESSURE_WEIGHT", "0.28");
        env::set_var("AHC_X04_PRESSURE_WEIGHT_EARLY", "0.06");
        env::set_var("AHC_X04_PRESSURE_WEIGHT_LATE", "0.98");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT", "-0.08");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_EARLY", "0.22");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_LATE", "0.32");
        env::set_var("AHC_X04_PRESSURE_PHASE_SPLIT", "0.68");
    }

    run_with_strategy(StrategyMode::MacroRoute);
}
