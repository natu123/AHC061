use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    unsafe {
        env::set_var("AHC_X67_HORIZON_BASE", "4");
        env::set_var("AHC_X67_HORIZON_LATE", "3");
        env::set_var("AHC_X67_SCENARIO_COUNT", "4");
        env::set_var("AHC_X67_RISK_DEFICIT", "0.08");
        env::set_var("AHC_X67_RISK_NEUTRAL", "0.16");
        env::set_var("AHC_X67_RISK_LEAD", "0.24");
        env::set_var("AHC_X67_LOCAL_WEIGHT", "0.14");
        env::set_var("AHC_X67_VOTE_WEIGHT", "22000");

        env::set_var("AHC_X67_W_X04", "0.30");
        env::set_var("AHC_X67_W_X19", "0.24");
        env::set_var("AHC_X67_W_X26", "0.18");
        env::set_var("AHC_X67_W_X06", "0.08");
        env::set_var("AHC_X67_W_X18", "0.06");
        env::set_var("AHC_X67_W_X20", "0.05");
        env::set_var("AHC_X67_W_X21", "0.05");
        env::set_var("AHC_X67_W_X22", "0.02");
        env::set_var("AHC_X67_W_X25", "0.02");
    }

    run_with_strategy(StrategyMode::GearShiftHybrid);
}
