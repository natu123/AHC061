# AHC061 Solver X34 - Macro Route Early Converge

## 狙い
- 初期フェーズを短縮し、`phase_cutoff` を低く保って `x06` への切替を早める。
- `target_eval` と候補数を控えめにして過剰探索を抑え、速度を改善しつつ下振れを抑制。

## 別系統性
- `x29` の進行寄りより、収束速度・安全性を重視した低予算設計。

## 期待効果
- `elapsed` の大幅短縮。
- `min`/`median` の悪化抑制。

## 実装スコープ
- 新規バイナリ: `solver/src/bin/x34_macro_route_early_converge.rs`
  - `StrategyMode::MacroRoute` 起動後、環境変数を固定セット

## 検証計画
- `seed 0..19` quick
- 条件を満たした場合に `seed 0..99` full

## 追記欄
- 実装日: 2026-02-16
- 最初のブランチ/コミット: 未定
