# AHC061 Solver X13 Frontier Consensus

## 目的
- 前線候補と local 安定性の整合性で意思決定し、競合局面での過激な偏りを抑制する。

## 対象
- 系統ID: `x13`
- 系統名: `x13_frontier_consensus`
- 実装本体: `solver/src/x13_frontier_consensus.rs`
- 実行bin: `solver/src/bin/x13_frontier_consensus.rs`
- 状態: 実装済み（非採用）

## 仕様
- `frontier_pressure` で候補の地形価値を再評価。
- advisory pool として `x06/x01/x04/x07/x08/x09` を重複排除後に収集。
- `primary/secondary` を用いた2条件シミュレーションを `leaders` 情報と合算。
- 衝突高濃度セルには負の係数を付与。

## 強み
- quick の上位候補集合が多様化し、同質化を抑える設計。

## 弱み
- 全体平均は baseline より一貫して下振れ。

## 評価結果（Loop #3）
- quick（seed `0..19`, 対 `x04`）:
  - `x04`: mean `147,335.4`, median `119,019.5`, min `70,744`, max `388,857`, elapsed `9,173ms`
  - `x13`: mean `122,625.4`, median `111,184.5`, min `55,341`, max `346,844`, elapsed `11,362ms`
- 判定:
  - 不採用（quickで `mean/median/min` 悪化）

## 関連ログ
- 不採用: `docs/AHC061_Experiment_Failures_2026-02.md` の `T-077`
- 計画: `docs/solver_specs_planned/AHC061_Solver_X13_FrontierConsensus_Plan.md`

## 更新ルール
- quick退行要因（`top2` 形成重みや `secondary_cap`）を変更した場合、再評価と失敗タグ更新を実施。
