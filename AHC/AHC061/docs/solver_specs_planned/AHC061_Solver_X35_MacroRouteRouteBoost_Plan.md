# AHC061 Solver X35 - Macro Route Route Boost

## 狙い
- `route_coeff` を大きくして経路到達価値を強め、x29系の探索をさらに強化する。
- `target_eval` と `candidate_cap` を維持しつつ、beam を広める。

## 別系統性
- `x29` / `x30` から `route系` を重点化した派生。

## 期待効果
- `mean` を上方向に寄せ、`median` も維持。
- fullの速度は許容範囲内に保つ。

## 実装スコープ
- 新規バイナリ: `solver/src/bin/x35_macro_route_route_boost.rs`
  - `StrategyMode::MacroRoute` 起動後、環境変数を固定セット

## 検証計画
- `seed 0..19` quick
- 条件を満たした場合に `seed 0..99` full

## 追記欄
- 実装日: 2026-02-16
- 最初のブランチ/コミット: 未定
