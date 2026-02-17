# AHC061 Solver X12 Advisor Vote Ensemble Plan

## 目的
- `x01/x04/x06/x07/x09` の各 expert の長所を単純重ねではなく重み付き投票で統合し、候補抽出を拡張する。
- `seed` 横断での頑健性を上げつつ、短期・中期のトレードオフを吸収する。

## 対象
- ID: `x12`
- 実装名: `x12_advisor_vote_ensemble`
- 実装パス（予定）:
  - `solver/src/x12_advisor_vote_ensemble.rs`
  - `solver/src/bin/x12_advisor_vote_ensemble.rs`
- 着手日: `2026-02-16`

## 方針
- `x01/x04/x06/x07/x09` から候補移動を収集し、重複を集約。
- 各候補は
  - 1手先 `strategic_score`（primary/secondary）
  - `vote`（何れの expert が同じ候補を選んだか）
  - `local_score`（保守評価）
  - `conflict`（衝突リスク）
  で再スコアリング。
- `vote` が少ない候補は危険時に抑制し、後半は `x01/x06` 側へ寄る。

## 検証計画
- `seed 0..19` で quick 比較。
- quick 競争ゲートで有望候補を抽出し、`seed 0..99` full を上位最大3件まで実施（0件は full 不実施）。
- 評価: `mean/median/min/max, elapsed`。
