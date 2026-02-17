# AHC061 Solver X33 - Macro Route Balanced Focus

## 狙い
- `x27` / `x29` の延長として、`target_eval` と `candidate_cap` を拡張しつつ、`route_coeff` を中庸化して偏りを抑える。
- `x04` の既定挙動との差を保持しつつ、quick での平均差を縮める。

## 別系統性
- `x29` の経路主導寄りとは別に、局所スコアと経路候補を並行評価するためのバランス型。

## 期待効果
- `mean` と `median` の同時改善。
- full の `elapsed` を 2s/seed 以内に抑制。

## 実装スコープ
- 新規バイナリ: `solver/src/bin/x33_macro_route_balanced_focus.rs`
  - `StrategyMode::MacroRoute` 起動後、環境変数を固定セット

## 検証計画
- `seed 0..19` quick
- 条件を満たした場合に `seed 0..99` full

## 追記欄
- 実装日: 2026-02-16
- 最初のブランチ/コミット: 未定
