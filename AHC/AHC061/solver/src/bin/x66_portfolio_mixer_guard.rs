use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    unsafe {
        env::set_var("AHC_X64_AGGRO_GAP", "0.30");
        env::set_var("AHC_X64_SECONDARY_CAP", "2");
        env::set_var("AHC_X64_W_PRIMARY", "0.50");
        env::set_var("AHC_X64_W_SECONDARY", "0.40");
        env::set_var("AHC_X64_W_LOCAL", "0.16");
        env::set_var("AHC_X64_W_VOTE", "20.0");
        env::set_var("AHC_X64_W_FRONTIER", "0.0012");
        env::set_var("AHC_X64_W_RISK", "1.15");
        env::set_var("AHC_X64_W_GAP_BONUS", "8.0");
        env::set_var("AHC_X64_W_UNCERTAINTY", "0.04");

        env::set_var("AHC_X64_W_X01", "0.95");
        env::set_var("AHC_X64_W_X06", "1.20");
        env::set_var("AHC_X64_W_X13", "1.00");
        env::set_var("AHC_X64_W_X26", "0.70");
    }

    run_with_strategy(StrategyMode::PortfolioMixer);
}
