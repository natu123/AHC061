# AHC061 Solver X21 Band Stage Adaptive Guard

## Purpose
- Increase robustness in high-uncertainty and high-conflict bands by adapting the mixing weight between base and conservative guards.
- Keep `M=4..6` behavior stable while avoiding overconfident aggressive steps from `x20` style ensemble.

## Result
- baseline quick 0..19: `x04` `mean 147,335.4`, `median 119,019.5`, `min 70,744`, `max 388,857`, `elapsed 9,173ms`
- `x21` quick 0..19: `mean 125,280.8`, `median 112,317`, `min 56,641`, `max 388,857`, `elapsed 17,057ms`
- full 0..99: not executed (quick gate not passed; candidate was not within 1.5% of loop top and not within baseline 98% line).

## Decision
- Rejected.
- Failure tags: `quick-regression`, `mean-drop`, `full-skip`.

## References
- `docs/solver_specs_planned/AHC061_Solver_X21_BandStageAdaptiveGuard_Plan.md`
- `docs/AHC061_Experiment_Log_2026-02.md` (`T-082`)
- `docs/AHC061_Experiment_Failures_2026-02.md` (`F-62`)
