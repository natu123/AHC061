# AHC061 Solver X24 Band Stage Adaptive Switch

## Purpose
- Switch strategies by band-stage signals in `M=4..6` to avoid both over-aggressive and over-conservative behavior.
- Preserve `x04`’s route planning strength while reducing local crash risk during high conflict phases.
- Re-use `x20` / `x18` style mixtures without merging both as a single static heuristic.

## Result
- baseline quick 0..19 (`x04`): `mean 147,335.4`, `median 119,019.5`, `min 70,744`, `max 388,857`, `elapsed 9,173ms`
- `x24` quick 0..19: `mean 120,302.4`, `median 109,302`, `min 35,245`, `max 388,857`, `elapsed 14,489ms`
- full 0..99: not executed (quick gate not passed; weak in mean/median).

## Decision
- Rejected.
- Failure tags: `quick-regression`, `mean-drop`, `min-deterioration`, `full-skip`.

## References
- `docs/solver_specs_planned/AHC061_Solver_X24_BandStageAdaptiveSwitch_Plan.md`
- `docs/AHC061_Experiment_Log_2026-02.md` (`T-083`)
- `docs/AHC061_Experiment_Failures_2026-02.md` (`F-65`)