# AHC061 Solver X57 Focus Recover Blend

## 目的
- `x50`（終盤圧力寄せ）と `x52`（中盤回復改善）をブレンドして、`mean` 上積みと `median` 安定化を同時に狙う。

## 仕様
- ID: `x57`
- バイナリ: `x57_macro_route_pressure_dual_guard_focus_recover`
- 対応ファイル:
  - `solver/src/bin/x57_macro_route_pressure_dual_guard_focus_recover.rs`
  - `solver/src/x04_macro_route.rs`

## 仮説
- `x50` の探索量増加（`target_count/target_eval/beam_width`）を維持すると `mean` 追従性を持ちやすい。
- `x52` の `pressure_weight` から `median` を守る方向の成分（`pressure_weight_early`、`route_pressure` 系）を残すと、`x50` の悪化傾向を抑制できる。
- 早期圧力に小さな正値を入れ、末期圧力は上げることで、両者の分岐が滑らかに接続されるはず。

## 変更想定値（環境変数）
- `AHC_X04_PHASE_CUTOFF`: `0.80`
- `AHC_X04_PHASE_SPLIT`: `0.55`
- `AHC_X04_TARGET_COUNT`: `9`
- `AHC_X04_TARGET_EVAL`: `8`
- `AHC_X04_CANDIDATE_CAP`: `10`
- `AHC_X04_PLAN_LEN_FAST`: `7`
- `AHC_X04_PLAN_LEN_SLOW`: `8`
- `AHC_X04_BEAM_WIDTH_FAST`: `6`
- `AHC_X04_BEAM_WIDTH_SLOW`: `8`
- `AHC_X04_BRANCH_WIDTH`: `2`
- `AHC_X04_LOCAL_WEIGHT`: `0.11`
- `AHC_X04_LOCAL_COEFF`: `0.10`
- `AHC_X04_ROUTE_COEFF`: `46.0`
- `AHC_X04_TARGET_PRESSURE_WEIGHT`: `0.74`
- `AHC_X04_PRESSURE_WEIGHT`: `0.38`
- `AHC_X04_PRESSURE_WEIGHT_EARLY`: `0.05`
- `AHC_X04_PRESSURE_WEIGHT_LATE`: `1.24`
- `AHC_X04_ROUTE_PRESSURE_WEIGHT`: `0.05`
- `AHC_X04_ROUTE_PRESSURE_WEIGHT_EARLY`: `0.20`
- `AHC_X04_ROUTE_PRESSURE_WEIGHT_LATE`: `0.70`
- `AHC_X04_PRESSURE_PHASE_SPLIT`: `0.61`

## 検証計画
- quick: `seed 0..19`
- full: `seed 0..99`（quick `selection` 通過時）
- 判定指標: `mean / median / min / elapsed`
- 採否条件: `mean` は `x04` 比で `98.5%` 以上、`median` は `100%` 以上、`min` は `90%` 以上を基準。
- 目標: `x50` の `mean` 寄与を保ち、`x52` の `median` 追従を追加する。
