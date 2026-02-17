# AHC061 Solver X26 Reactive Frontier Pressure

## Purpose
- Monitor frontier pressure and collision density per turn, then switch between conservative and recovery-oriented actions.
- Reduce repeated tail losses where static frontier-gate policies over-prioritize safe moves.
- Keep attack cadence recoverable under high gap pressure without exploding worst-case failures.

## Result
- baseline quick 0..19 (`x04`): `mean 147,335.4`, `median 119,019.5`, `min 70,744`, `max 388,857`, `elapsed 9,173ms`
- `x26` quick 0..19: `mean 125,585.2`, `median 116,424.5`, `min 37,224`, `max 388,857`, `elapsed 34,466ms`
- full 0..99: not executed (quick gate not passed; mean/median did not satisfy close-competition conditions).

## Decision
- Rejected.
- Failure tags: `quick-regression`, `mean-drop`, `min-deterioration`, `median-shortfall`, `full-skip`.

## References
- `docs/solver_specs_planned/AHC061_Solver_X26_ReactiveFrontierPressure_Plan.md`
- `docs/AHC061_Experiment_Log_2026-02.md` (`T-083`)
- `docs/AHC061_Experiment_Failures_2026-02.md` (`F-67`)