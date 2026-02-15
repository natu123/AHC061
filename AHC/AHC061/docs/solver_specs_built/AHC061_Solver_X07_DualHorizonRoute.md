# AHC061 Solver X07 Dual Horizon Route

## 目的
- `x04` の中期経路探索に短期安全性評価を重ね、tail-risk を抑えながら `mean` 改善を狙う。

## 対象
- 系統ID: `x07`
- 系統名: `x07_dual_horizon_route`
- 実装本体: `solver/src/x07_dual_horizon_route.rs`
- 実行bin: `solver/src/bin/x07_dual_horizon_route.rs`
- 状態: 実装済み（非採用）

## 仕様
- 意思決定の中核:
  - `choose_move_x07_dual_horizon_route`
  - `M=4`・`phase<=0.72` でのみ適用し、他帯は `x06` へフォールバック
- 候補評価:
  - 上位候補手に対して短期 rollout（3step）と中期 rollout（6-7step）を別々に算出
  - 不確実性に応じた重みで `short/long` を混合して採択
- 相手行動予測:
  - `x04` 同等の top2 + secondary 切替を再利用

## 強み
- `elapsed` は `x04` より大幅に短縮（quick/full とも約3分の1以下）。
- full では `max/min` を `x04` と同値で維持した。

## 弱み
- quick/full とも `mean/median` が `x04` を下回り、スコア改善に届かなかった。

## 評価結果（Loop #1）
- quick（seed `0..19`, 対 `x04`）:
  - `x04`: mean `147,335.4`, median `119,019.5`, min `70,744`, max `388,857`, elapsed `9,173ms`
  - `x07`: mean `137,783.7`, median `114,450`, min `70,744`, max `388,857`, elapsed `2,198ms`
- full（seed `0..99`, 対 `x04`）:
  - `x04`: mean `158,923.8`, median `138,335.5`, min `52,543`, max `605,548`, elapsed `32,128ms`
  - `x07`: mean `155,182.1`, median `130,877`, min `52,543`, max `605,548`, elapsed `10,834ms`
- 判定:
  - 不採用（速度改善のみで、score指標は採用基準未達）

## 関連ログ
- 不採用: `docs/AHC061_Experiment_Failures_2026-02.md` の `T-071`
- 計画: `docs/solver_specs_planned/AHC061_Solver_X07_DualHorizonRoute_Plan.md`

## 更新ルール
- horizon長と混合重みを変更した場合、quick/fullの両比較を更新する。
