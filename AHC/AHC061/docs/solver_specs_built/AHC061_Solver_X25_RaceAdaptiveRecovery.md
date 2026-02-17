# AHC061 Solver X25 Race Adaptive Recovery

## Purpose
- Introduce two-step state-driven recovery switching for mid/late game pressure handling.
- Keep frontier exploration in stable phase and intensify recovery when advantage gap or conflict pressure suggests falling behind.
- Reduce long-tail degradation compared with static frontier-first policies.

## Result
- baseline quick 0..19 (`x04`): `mean 147,335.4`, `median 119,019.5`, `min 70,744`, `max 388,857`, `elapsed 9,173ms`
- `x25` quick 0..19: `mean 122,159.2`, `median 106,804`, `min 41,914`, `max 388,857`, `elapsed 46,568ms`
- full 0..99: not executed (quick gate not passed).

## Decision
- Rejected.
- Failure tags: `quick-regression`, `mean-drop`, `min-deterioration`, `full-skip`.

## References
- `docs/solver_specs_planned/AHC061_Solver_X25_RaceAdaptiveRecovery_Plan.md`
- `docs/AHC061_Experiment_Log_2026-02.md` (`T-083`)
- `docs/AHC061_Experiment_Failures_2026-02.md` (`F-66`)