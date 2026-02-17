# AHC061 Solver X46 - Macro Route Pressure Hybrid Length

## 背景
- x42 が full で優位を示す一方、plan 長のバランスが未最適の可能性が高い。

## 狙い
- `PLAN_LEN_FAST/SLOW` と `beam_width` を増減させて、局面の読みを深く保持しつつ終盤での回収率を改善。
- `candidate_cap` と `branch_width` を連動させて局所探索の回復性を高める。

## 期待効果
- quick/full の双方で `x42` の変動幅より高い再現性を狙う。
- 2-3候補同時 full 条件に収まりやすいよう、top2近傍の競争候補を想定。

## 検証計画
- quick: `seed 0..19`
- full: quick ゲート通過時のみ `seed 0..99`
- `AHC_X04_PLAN_LEN_*`, `AHC_X04_BEAM_WIDTH_*` の位相差分を比較。

## 実装スコープ
- `solver/src/bin/x46_macro_route_pressure_hybrid_len.rs`

## タグ
- `plan-length`, `beam-width`, `hybrid-depth`, `x04-variant`

## 追記
- `2026-02-16`
- Loop: x46 新規投入
