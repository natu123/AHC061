use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    unsafe {
        env::set_var("AHC_X67_HORIZON_BASE", "3");
        env::set_var("AHC_X67_HORIZON_LATE", "2");
        env::set_var("AHC_X67_SCENARIO_COUNT", "5");
        env::set_var("AHC_X67_RISK_DEFICIT", "0.22");
        env::set_var("AHC_X67_RISK_NEUTRAL", "0.34");
        env::set_var("AHC_X67_RISK_LEAD", "0.48");
        env::set_var("AHC_X67_LOCAL_WEIGHT", "0.20");
        env::set_var("AHC_X67_VOTE_WEIGHT", "12000");

        env::set_var("AHC_X67_W_X04", "0.20");
        env::set_var("AHC_X67_W_X19", "0.14");
        env::set_var("AHC_X67_W_X26", "0.12");
        env::set_var("AHC_X67_W_X06", "0.14");
        env::set_var("AHC_X67_W_X18", "0.16");
        env::set_var("AHC_X67_W_X20", "0.09");
        env::set_var("AHC_X67_W_X21", "0.09");
        env::set_var("AHC_X67_W_X22", "0.04");
        env::set_var("AHC_X67_W_X25", "0.02");
    }

    run_with_strategy(StrategyMode::GearShiftHybrid);
}
