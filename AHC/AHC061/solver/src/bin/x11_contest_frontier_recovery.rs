use ahc061_solver::{run_with_strategy, StrategyMode};

fn main() {
    run_with_strategy(StrategyMode::ContestFrontierRecovery);
}
