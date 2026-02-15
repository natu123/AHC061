# AHC061 Solver X04 Macro Route

## 目的
- 1手の局所評価だけでなく、中期の行動連鎖（育成 -> 侵攻）を明示的に評価して `M=4` 帯の伸びを狙う。
- `x06` をベースにしつつ、改善余地が大きい局面だけ route planning を適用する。

## 対象
- 系統ID: `x04`
- 系統名: `x04_macro_route`
- 実装本体: `solver/src/x04_macro_route.rs`
- 実行bin: `solver/src/bin/x04_macro_route.rs`
- 状態: 実装済み（採用中 / champion）

## 仕様
- 意思決定の中核:
  - `choose_move_x04_macro_route`
  - `M=4` かつ `phase <= phase_cutoff` のときのみ Macro Route を適用し、それ以外は `x06` フォールバック
- 目標セル抽出:
  - `未領土 / 敵Lv1 / 自領土育成余地` を重み付けして上位セルを target として選択
- ルート評価:
  - 先頭手ごとに `target` 別 beam 探索を実行し、`strategic_score` 差分 + 目標接近ボーナスで評価
  - 予測AI行動は `x06` の予測系（primary/secondary切替）を再利用
- 主要パラメータ（最終）:
  - `phase_cutoff = 0.65`（`AHC_X04_PHASE_CUTOFF` で上書き可）
  - `plan_len = 7 / 6(phase>0.50)`, `beam_width = 5 / 4(phase>0.50)`, `branch_width = 2`
  - `target cap = 5`, `candidate cap = 5..7`

## 強み
- `seed 0..99` で `x06` 比 `mean/median/min` を同時改善した。
- 改善は主に `M=4` 帯で発生し、他帯は `x06` と同値を維持できる。

## 弱み
- `x06` 比で計算時間が増える（約3.05倍）。
- `M=4` 内では改善/悪化seedの分散が大きく、境界値運用を誤ると tail-risk が悪化しやすい。

## 評価結果（最終）
- full（`seed 0..99`, 対 `x06`）:
  - `x06`: mean `155,863.2`, median `133,042.5`, min `51,023`, max `605,548`, elapsed `10,174ms`
  - `x04`（cutoff `0.65`, 軽量化後）: mean `158,923.8`, median `138,335.5`, min `52,543`, max `605,548`, elapsed `31,035ms`
- 差分（`x04 - x06`）:
  - mean `+3,060.6`
  - median `+5,293.0`
  - min `+1,520`
  - max `±0`
  - elapsed `+20,861ms`
- 実行時間分布（`x04`, cutoff `0.65`, seed `0..99`）:
  - overall: mean `309ms`, p95 `1,170ms`, max `1,315ms`

## 関連ログ
- 採用: `docs/AHC061_Experiment_Log_2026-02.md` の `T-069`, `T-070`
- 不採用試行: `docs/AHC061_Experiment_Failures_2026-02.md` の `T-068`
- 計画: `docs/solver_specs_planned/ARCHIVED_AHC061_Solver_X04_MacroRoute_Plan.md`

## 更新ルール
- `phase_cutoff` / `plan_len` / beam幅を変更した場合は、quickとfull両方の比較結果を追記する。
- `M` 適用帯を変更した場合は、帯別差分（`M`別meanとmin）を必ず更新する。
