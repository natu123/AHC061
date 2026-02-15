# AHC061 Solver X09 Regret Mix

## 目的
- `x04/x06/x02` 由来の候補手を反実仮想評価し、固定分岐より柔軟な expert 混合で改善を狙う。

## 対象
- 系統ID: `x09`
- 系統名: `x09_regret_mix`
- 実装本体: `solver/src/x09_regret_mix.rs`
- 実行bin: `solver/src/bin/x09_regret_mix.rs`
- 状態: 実装済み（非採用）

## 仕様
- 意思決定の中核:
  - `choose_move_x09_regret_mix`
  - `x04/x06`（必要時 `x02`）の提案手 + 局所上位手を候補集合化
- 候補評価:
  - 共通サンプル列でAI行動を生成し、候補手ごとの `strategic_score` 分布を計測
  - `mean - risk_w * std + local_bonus` で採択
- 安全策:
  - 候補不足時は `x06` へ即時フォールバック

## 強み
- quickで `max` は `x04` を上回るseedを確認（`+67,734`）。

## 弱み
- `mean/median/min` が大幅悪化し、分散増加に対して安定性確保が不十分だった。

## 評価結果（Loop #1）
- quick（seed `0..19`, 対 `x04`）:
  - `x04`: mean `147,335.4`, median `119,019.5`, min `70,744`, max `388,857`, elapsed `9,173ms`
  - `x09`: mean `125,930.3`, median `107,568.5`, min `51,051`, max `456,591`, elapsed `8,975ms`
- 判定:
  - 不採用（quick段階で大幅退行のため full 未実施）

## 関連ログ
- 不採用: `docs/AHC061_Experiment_Failures_2026-02.md` の `T-073`
- 計画: `docs/solver_specs_planned/AHC061_Solver_X09_RegretMix_Plan.md`

## 更新ルール
- expert候補の構成とリスク重みを変更した場合、quick比較と採択比率を更新する。
