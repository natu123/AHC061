use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    unsafe {
        // Structural reset: widen candidate diversity and rebalance route/pressure phases.
        env::set_var("AHC_X04_PHASE_CUTOFF", "0.72");
        env::set_var("AHC_X04_PHASE_SPLIT", "0.46");
        env::set_var("AHC_X04_TARGET_COUNT", "11");
        env::set_var("AHC_X04_TARGET_EVAL", "9");
        env::set_var("AHC_X04_CANDIDATE_CAP", "12");
        env::set_var("AHC_X04_PLAN_LEN_FAST", "5");
        env::set_var("AHC_X04_PLAN_LEN_SLOW", "10");
        env::set_var("AHC_X04_BEAM_WIDTH_FAST", "4");
        env::set_var("AHC_X04_BEAM_WIDTH_SLOW", "9");
        env::set_var("AHC_X04_BRANCH_WIDTH", "3");
        env::set_var("AHC_X04_LOCAL_WEIGHT", "0.16");
        env::set_var("AHC_X04_LOCAL_COEFF", "0.14");
        env::set_var("AHC_X04_ROUTE_COEFF", "39.0");

        env::set_var("AHC_X04_TARGET_PRESSURE_WEIGHT", "0.55");
        env::set_var("AHC_X04_PRESSURE_WEIGHT", "0.22");
        env::set_var("AHC_X04_PRESSURE_WEIGHT_EARLY", "-0.08");
        env::set_var("AHC_X04_PRESSURE_WEIGHT_LATE", "1.48");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT", "-0.22");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_EARLY", "0.30");
        env::set_var("AHC_X04_ROUTE_PRESSURE_WEIGHT_LATE", "0.88");
        env::set_var("AHC_X04_PRESSURE_PHASE_SPLIT", "0.58");
    }

    run_with_strategy(StrategyMode::MacroRoute);
}
