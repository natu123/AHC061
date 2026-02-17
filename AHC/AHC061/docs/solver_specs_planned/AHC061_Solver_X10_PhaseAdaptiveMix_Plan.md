# AHC061 Solver X10 Phase Adaptive Mix Plan

## 目的
- `x04` と他方策を `phase` / `uncertainty` / `競合率` で動的切替する新規探索方針を検討する。
- `mean` と `tail-risk` の同時改善を狙う。

## 対象
- ID: `x10`
- 実装名: `x10_phase_adaptive_mix`
- 実装パス（予定）:
  - `solver/src/x10_phase_adaptive_mix.rs`
  - `solver/src/bin/x10_phase_adaptive_mix.rs`
- 着手日: `2026-02-16`

## 方針
- `M=4` を軸に、`x04` の強みが出る `phase` 帯を優先し、
  - 不確実性が高い場合は `x04` を追加利用
  - 安定性優先領域では `x06` を優先
  - 終盤のリスク帯では保守側（`x06` / `x01`）に寄せる
- 候補は `x04/x06/x01` で列挙し、共通の1手先評価で再スコアリングして決定。

## 検証計画
- `seed 0..19` で quick 比較。
- quick で有望候補を競争ゲートで抽出し、`seed 0..99` の full を最大3件まで実施（0件は full 不実施）。
- 評価: `mean/median/min/max, elapsed`。
