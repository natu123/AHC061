# AHC061 Solver X32 - Macro Route Wide Frontier

## 狙い
- `frontier` 候補を広く評価し、`plan_len` と `target_eval` を増やして再探索余地を増加。
- `branch/beam` の組み合わせを見直し、長期到達の利益機会を拾える設計を目指す。

## 別系統性
- `x27` のアグレッシブ寄り、`x28` の防御寄りとは別に、`wide` 化した経路評価を中心に試す。

## 期待効果
- `max` / `min` の崩れを抑えつつ `median` を改善。
- 一部 seed で高スコアの上振れ改善を狙う。

## 実装スコープ
- 新規バイナリ: `solver/src/bin/x32_macro_route_wide_frontier.rs`
  - `StrategyMode::MacroRoute` 起動後、環境変数を固定セット

## 検証計画
- `seed 0..19` quick
- 条件を満たした場合に `seed 0..99` full

## 追記欄
- 実装日: 2026-02-16
- 最初のブランチ/コミット: 未定

