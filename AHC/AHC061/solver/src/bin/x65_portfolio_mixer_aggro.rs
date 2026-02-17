use std::env;

use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    unsafe {
        env::set_var("AHC_X64_AGGRO_GAP", "0.10");
        env::set_var("AHC_X64_SECONDARY_CAP", "3");
        env::set_var("AHC_X64_W_PRIMARY", "0.62");
        env::set_var("AHC_X64_W_SECONDARY", "0.26");
        env::set_var("AHC_X64_W_LOCAL", "0.14");
        env::set_var("AHC_X64_W_VOTE", "25.0");
        env::set_var("AHC_X64_W_FRONTIER", "0.0022");
        env::set_var("AHC_X64_W_RISK", "0.60");
        env::set_var("AHC_X64_W_GAP_BONUS", "22.0");
        env::set_var("AHC_X64_W_UNCERTAINTY", "0.10");

        env::set_var("AHC_X64_W_X11", "0.95");
        env::set_var("AHC_X64_W_X19", "0.95");
        env::set_var("AHC_X64_W_X26", "1.05");
        env::set_var("AHC_X64_W_X14", "0.90");
    }

    run_with_strategy(StrategyMode::PortfolioMixer);
}
