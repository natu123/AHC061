# AHC061 Solver X11 Contest Frontier Recovery Plan

## 目的
- 前線の対立セル（敵1階・高価値セル）への回収を優先しつつ、過剰な衝突リスクを抑える経路評価を検討する。
- `x07` や `x08` が苦手な局面補完を狙う。

## 対象
- ID: `x11`
- 実装名: `x11_contest_frontier_recovery`
- 実装パス（予定）:
  - `solver/src/x11_contest_frontier_recovery.rs`
  - `solver/src/bin/x11_contest_frontier_recovery.rs`
- 着手日: `2026-02-16`

## 方針
- `frontier` 近傍の衝突確率が高いセルをスコアリングし、  
  `x04/x07` の候補に対して「回収価値」「競合ペナルティ」「保険評価」を同時加点。
- `M=4` で早期から中盤は攻め寄り、終盤は保守寄りへ比率移行。
- `leader` との開きが小さい場合は `x09` のように複数 expert の候補を加重混合。

## 検証計画
- `seed 0..19` で quick 比較。
- quick 競争ゲートで有望候補を抽出し、`seed 0..99` full を上位最大3件まで実施（0件は full 不実施）。
- 評価: `mean/median/min/max, elapsed`。
