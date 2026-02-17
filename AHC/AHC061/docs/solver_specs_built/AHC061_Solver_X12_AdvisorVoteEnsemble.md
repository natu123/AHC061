# AHC061 Solver X12 Advisor Vote Ensemble

## 目的
- 複数方策の提案手を advisor 投票として統合し、`phase` と `uncertainty` で重みを切替。

## 対象
- 系統ID: `x12`
- 系統名: `x12_advisor_vote_ensemble`
- 実装本体: `solver/src/x12_advisor_vote_ensemble.rs`
- 実行bin: `solver/src/bin/x12_advisor_vote_ensemble.rs`
- 状態: 実装済み（非採用）

## 仕様
- `x04` / `x06` / `x01` / `x02` / `x07` / `x08` / `x09` を advisor pool にして重み付き票を作成。
- 上位投票を候補候補集合に反映し、`local` と `primary/secondary` シミュレーションを合わせて評価。
- `leader_gap` が大きい場合に `recover_boost` を追加する fallback を導入。

## 強み
- `phase<=0.7` など早期局面での高速再計算が可能。

## 弱み
- 全体平均の改善に至らず、最悪ケース悪化傾向。

## 評価結果（Loop #2）
- quick（seed `0..19`, 対 `x04`）:
  - `x04`: mean `147,335.4`, median `119,019.5`, min `70,744`, max `388,857`, elapsed `9,173ms`
  - `x12`: mean `125,138.6`, median `120,293.5`, min `30,819`, max `314,524`, elapsed `50,041ms`
- 判定:
  - 不採用（quickで `mean` 低下。`tail-risk` は改善不十分）

## 関連ログ
- 不採用: `docs/AHC061_Experiment_Failures_2026-02.md` の `T-076`
- 計画: `docs/solver_specs_planned/AHC061_Solver_X12_AdvisorVoteEnsemble_Plan.md`

## 更新ルール
- 重み設計を変える際は `x11`/`x13`/`x14` との比較を再実行する。
