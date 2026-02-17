# AHC061 Solver X14 Adaptive Risk Lane Plan

## 目的
- `x14` は局面の不確実性とプレイヤー勢力差を基準に、候補レーンを適応切替し tail-risk を抑える。

## 対象
- ID: `x14`
- 実装名: `x14_adaptive_risk_lane`
- 実装予定パス:
  - `solver/src/x14_adaptive_risk_lane.rs`
  - `solver/src/bin/x14_adaptive_risk_lane.rs`

## 方針
- `x06` / `x01` / `x02` / `x04` / `x07` / `x08` / `x09` の候補を条件付きで注入。
- `phase` と `uncertainty` で `primary/secondary` の重みを調整。
- 再現衝突を避けるため `conflict_map` を局所評価に組み込む。

## 検証計画
- `seed 0..19` で quick。
- quick 競争ゲートで有望候補を抽出し、`seed 0..99` full を上位最大3件まで実施（0件は full 不実施）。
- 指標: `mean/median/min/max, elapsed`。
