# AHC061 Solver X23 Band Stage Frontier Guard

## Purpose
- Prevent over-aggressive frontier moves by introducing a frontier-specific conservative guard.
- Keep recovery smooth in `M=3..6` while suppressing risky late-game branch shifts.

## Result
- baseline quick 0..19: `x04` `mean 147,335.4`, `median 119,019.5`, `min 70,744`, `max 388,857`, `elapsed 9,173ms`
- `x23` quick 0..19: `mean 89,220.6`, `median 100,309`, `min 2,029`, `max 388,857`, `elapsed 8,742ms`
- full 0..99: not executed (quick gate not passed; candidate was not within 1.5% of loop top and not within baseline 98% line).

## Decision
- Rejected.
- Failure tags: `quick-regression`, `mean-drop`, `full-skip`.

## References
- `docs/solver_specs_planned/AHC061_Solver_X23_BandStageFrontierGuard_Plan.md`
- `docs/AHC061_Experiment_Log_2026-02.md` (`T-082`)
- `docs/AHC061_Experiment_Failures_2026-02.md` (`F-64`)
