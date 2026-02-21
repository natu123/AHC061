use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    unsafe {
        env::set_var("AHC_X04_ALLOW_ALL_M", "1");
        env::set_var("AHC_X04_DISABLE_PHASE_CUTOFF", "1");

        env::set_var("AHC_X04_PHASE_CUTOFF", "0.80");
        env::set_var("AHC_X04_PHASE_SPLIT", "0.56");
        env::set_var("AHC_X04_TARGET_COUNT", "9");
        env::set_var("AHC_X04_TARGET_EVAL", "8");
        env::set_var("AHC_X04_CANDIDATE_CAP", "10");
        env::set_var("AHC_X04_PLAN_LEN_FAST", "6");
        env::set_var("AHC_X04_PLAN_LEN_SLOW", "8");
        env::set_var("AHC_X04_BEAM_WIDTH_FAST", "6");
        env::set_var("AHC_X04_BEAM_WIDTH_SLOW", "8");
        env::set_var("AHC_X04_BRANCH_WIDTH", "2");
        env::set_var("AHC_X04_LOCAL_WEIGHT", "0.11");
        env::set_var("AHC_X04_LOCAL_COEFF", "0.09");
        env::set_var("AHC_X04_ROUTE_COEFF", "46.8");

        env::set_var("AHC_X04_TARGET_PRESSURE_WEIGHT", "0.76");
        env::set_var("AHC_X04_PRESSURE_WEIGHT", "0.36");
        env::set_var("AHC_X04_PRESSURE_WEIGHT_EARLY", "0.02");
        env::set_var("AHC_X04_PRESSURE_WEIGHT_LATE", "1.36");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT", "0.08");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_EARLY", "0.24");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_LATE", "0.72");
        env::set_var("AHC_X04_PRESSURE_PHASE_SPLIT", "0.62");
    }

    run_with_strategy(StrategyMode::MacroRoute);
}
