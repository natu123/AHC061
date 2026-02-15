# AHC061 Solver X08 Pressure Frontier

## 目的
- 前線圧力ベースの局所評価で `elapsed` を抑えつつ、`x04` の代替となる高速探索軸を検証する。

## 対象
- 系統ID: `x08`
- 系統名: `x08_pressure_frontier`
- 実装本体: `solver/src/x08_pressure_frontier.rs`
- 実行bin: `solver/src/bin/x08_pressure_frontier.rs`
- 状態: 実装済み（非採用）

## 仕様
- 意思決定の中核:
  - `choose_move_x08_pressure_frontier`
  - `M=3..6` のみ適用し、他帯は `x06` フォールバック
- 候補評価:
  - `evaluate_local_move` に加えて `frontier_pressure` を算出
  - primary/secondary 2シナリオの1手先 `strategic_score` 増分を混合
  - リスクマップ由来のペナルティを差し引いて採択

## 強み
- quick では最速（`x04` 比 `-7,472ms`）で、計算時間は大きく改善。

## 弱み
- quick の `mean/median/min` が大幅悪化し、探索品質が不足した。

## 評価結果（Loop #1）
- quick（seed `0..19`, 対 `x04`）:
  - `x04`: mean `147,335.4`, median `119,019.5`, min `70,744`, max `388,857`, elapsed `9,173ms`
  - `x08`: mean `125,655.7`, median `113,953.5`, min `48,933`, max `388,857`, elapsed `1,701ms`
- 判定:
  - 不採用（quick段階で大幅退行のため full 未実施）

## 関連ログ
- 不採用: `docs/AHC061_Experiment_Failures_2026-02.md` の `T-072`
- 計画: `docs/solver_specs_planned/AHC061_Solver_X08_PressureFrontier_Plan.md`

## 更新ルール
- 前線圧力重みとリスクペナルティを変更した場合、quickのA/B差分を更新する。
