# AHC061 Solver X13 Frontier Consensus Plan

## 目的
- `x13` は複数 expert の示唆を「前線回復と一致度」で統合し、局所衝突を避けつつリカバリ手を優先する。

## 対象
- ID: `x13`
- 実装名: `x13_frontier_consensus`
- 実装予定パス:
  - `solver/src/x13_frontier_consensus.rs`
  - `solver/src/bin/x13_frontier_consensus.rs`

## 方針
- `x06` / `x01` / `x02` / `x04` / `x07` / `x08` / `x09` の提案手を候補として収集。
- `x04` 系候補は `uncertainty` と phase 条件で重み付け。
- 候補スコアは `local+1手先+secondary` を統合。
- 競合率に応じてリスク罰則を強くし、`M>=6` では expert 混合を抑制。

## 検証計画
- `seed 0..19` で quick。
- quick 競争ゲートで有望候補を抽出し、`seed 0..99` full を上位最大3件まで実施（0件は full 不実施）。
- 指標: `mean/median/min/max, elapsed`。
