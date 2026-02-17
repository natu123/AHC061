# AHC061 Solver X44 - Macro Route Pressure Dual Window

## 背景
- `x41~x43` 系の圧力パラメータチューニングで差が出る局面に対し、前半/後半で候補分割を分離した場合の改善を狙う。

## 狙い
- ターン初期は短期集中で候補幅を広げ、終盤は経路重視で収束を強める。
- `AHC_X04_TARGET_COUNT` と `AHC_X04_PLAN_LEN_*` を増やして、`x42` 周辺の遅れを補正する。

## 期待効果
- quick で `mean` と `median` が `x42` 付近まで追従し、full 対象入りを狙う。
- `top2` 同時full条件で、x42と並走する局面を増やせること。

## 検証計画
- quick: `seed 0..19`
- full: quickゲート通過時のみ `seed 0..99`
- ベース比較は `x04`。判定基準は `x04` 近傍 98.5%, top gap 3%。

## 実装スコープ
- `solver/src/bin/x44_macro_route_pressure_dual_window.rs`

## タグ
- `pressure-dual-window`, `x04-parameter-sweep`, `x04-variant`

## 追記
- `2026-02-16`
- Loop: x44 新規投入
