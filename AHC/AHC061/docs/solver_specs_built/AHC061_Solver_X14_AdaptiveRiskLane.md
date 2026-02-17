# AHC061 Solver X14 Adaptive Risk Lane

## 目的
- 競合率・リスク指標に応じて候補レーンを動的に切替し、`mean/median/min` 同時改善を狙う。

## 対象
- 系統ID: `x14`
- 系統名: `x14_adaptive_risk_lane`
- 実装本体: `solver/src/x14_adaptive_risk_lane.rs`
- 実行bin: `solver/src/bin/x14_adaptive_risk_lane.rs`
- 状態: 実装済み（非採用）

## 仕様
- `phase` と `uncertainty` に応じて候補集合を最適化。
- `x06` / `x01` / `x02` / `x04` / `x07` / `x08` / `x09` を段階的に候補へ追加。
- `primary` と `secondary` の配分を `uncertainty` / `game.m` / `phase` で切替。
- conflict map 由来のリスクペナルティをスコアに明示的反映。

## 強み
- 一部 seed で `x11/x13` より安定した高値域を抑えられる構造。

## 弱み
- 全体 `mean/median` は baseline へ到達せず、tail-risk 指標も改善せず。

## 評価結果（Loop #3）
- quick（seed `0..19`, 対 `x04`）:
  - `x04`: mean `147,335.4`, median `119,019.5`, min `70,744`, max `388,857`, elapsed `9,173ms`
  - `x14`: mean `133,809.5`, median `120,847.5`, min `41,882`, max `279,763`, elapsed `17,445ms`
- full（seed `0..99`）:
  - `x04`: mean `158,923.8`, median `138,335.5`, min `52,543`, max `605,548`, elapsed `30,000ms`
  - `x14`: mean `128,359.5`, median `113,709.5`, min `39,592`, max `311,620`, elapsed `58,889ms`
- 判定:
  - 不採用（quick・fullとも baseline より平均低下）

## 関連ログ
- 不採用: `docs/AHC061_Experiment_Failures_2026-02.md` の `T-078`
- 計画: `docs/solver_specs_planned/AHC061_Solver_X14_AdaptiveRiskLane_Plan.md`

## 更新ルール
- full実行で full 走行時間が baseline を上回る場合、候補集合と候補数制限を再検討する。
