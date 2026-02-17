# AHC061 Solver X54 Late Pressure Boost Plan

## 目的
- x50/x53 系で確認できた終盤寄与をさらに強め、`mean`/`median` の同時改善余地を探索する。
- `x04` 直系を保ちつつ、後半圧力を局所化せずに強化して巻き返し性能を上げる。

## タグ
- ID: `x54`
- バイナリ: `x54_macro_route_pressure_late_boost`
- 主要変更ファイル:
  - `solver/src/bin/x54_macro_route_pressure_late_boost.rs`

## 仮説
- `phase` が進むにつれ `pressure_weight_late` を大きく取り、late phase の圧力追従を強めると `mean` が伸びる一方、`median` が崩れにくいはず。
- `plan_len_slow` / `beam_width_slow` / `candidate_cap` を少し上げることで、後半の経路選択が局所に寄りすぎる問題を軽減できる。
- `route_pressure_late` を中〜高域にすることで、取り返しの精度を高める。

## 検証計画
- quick: `seed 0..19`
- quick で門番条件を満たした場合のみ full: `seed 0..99`
- 判定:
  - quick: `mean` と `median` の改善確認
  - full: `mean/median/min` と `elapsed` を比較
