# AHC061 Solver X59 Recover Guard Blend Plan

## 目的
- `x47` 系の過剰攻勢を抑制し、回復局面での選択幅を上げて `median` と `min` を安定化する。
- `branch_width` と `route_pressure` の同時調整で、中盤〜終盤の取りこぼしを減らす。

## 仮説
- 早期圧力を正側寄りにすることで短期爆発を抑え、`median` の悪化を防げる可能性がある。
- 回復候補を増やすため `branch_width` を拡張すると、分岐品質の分散が上がり `min` の回復に効く可能性がある。

## 仕様
- ID: `x59`
- 実装: `solver/src/bin/x59_macro_route_pressure_recover_guard_blend.rs`
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
  - `mean/median/min` を `x47`・`x04_full` と比較
  - `median` と `min` 悪化を最優先で不採用
