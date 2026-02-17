# AHC061 Solver X31 - Macro Route Local Bias

## 狙い
- `x31` では局所評価の寄与を重くし、`AHC_X04_LOCAL_COEFF` と `AHC_X04_LOCAL_WEIGHT` の協調で短期安定性を上げる。
- `route_coeff` を抑え、暴走的な目標到達より安定候補選択を優先する。

## 別系統性
- `x27`/`x29` の `route coeff` 強化系に対し、局所スコア優先の対抗系として設計。

## 期待効果
- `median/min` の改善や分散縮小。
- `full` でも `elapsed` を抑えながら `mean` 低下を抑制。

## 実装スコープ
- 新規バイナリ: `solver/src/bin/x31_macro_route_local_bias.rs`
  - `StrategyMode::MacroRoute` 起動後、環境変数を固定セット

## 検証計画
- `seed 0..19` quick
- 条件を満たした場合に `seed 0..99` full

## 追記欄
- 実装日: 2026-02-16
- 最初のブランチ/コミット: 未定

