# AHC061 Solver X58 Late Recover Balance Plan

## 目的
- `x47` を起点に、終盤の圧力効率を上げつつ、早期過剰進行を抑えて `median` を維持する。
- `AHC_X04_PRESSURE_WEIGHT_LATE` を上方にし、`early` 側はわずかに正に寄せた運用を確認する。

## 仮説
- `x47` は `mean` 向上がある一方、`median` 停滞が見られるため、終盤の収束と回復のバランスを上げると
  `median` も改善される可能性が高い。
- 小幅の `route_pressure` 導入は、終盤の取り合いでの差し戻し率低下と中堅局面の安定化に寄与する想定。

## 仕様
- ID: `x58`
- 実装: `solver/src/bin/x58_macro_route_pressure_late_recover_balance.rs`
- 変更変数:
  - `AHC_X04_PHASE_CUTOFF`, `AHC_X04_PHASE_SPLIT`
  - `AHC_X04_TARGET_COUNT`, `AHC_X04_TARGET_EVAL`
  - `AHC_X04_CANDIDATE_CAP`, `AHC_X04_PLAN_LEN_*`, `AHC_X04_BEAM_WIDTH_*`, `AHC_X04_BRANCH_WIDTH`
  - `AHC_X04_LOCAL_WEIGHT`, `AHC_X04_LOCAL_COEFF`, `AHC_X04_ROUTE_COEFF`
  - `AHC_X04_TARGET_PRESSURE_WEIGHT`
  - `AHC_X04_PRESSURE_WEIGHT`, `AHC_X04_PRESSURE_WEIGHT_EARLY`, `AHC_X04_PRESSURE_WEIGHT_LATE`
  - `AHC_X04_ROUTE_PRESSURE_WEIGHT`, `AHC_X04_ROUTE_PRESSURE_WEIGHT_EARLY`, `AHC_X04_ROUTE_PRESSURE_WEIGHT_LATE`
  - `AHC_X04_PRESSURE_PHASE_SPLIT`

## 検証計画
- quick: `seed 0..19`
- full: `seed 0..99`
- 判定軸:
  - `mean/median/min` の `x47` 比較（`QualityFirst(0.85)`）
  - `median` と `min` の悪化があれば不採用（`tail-risk` 優先）
