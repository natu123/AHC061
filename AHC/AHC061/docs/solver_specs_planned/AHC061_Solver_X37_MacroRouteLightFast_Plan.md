# AHC061 Solver X37 - Macro Route Light Fast

## 狙い
- 計算予算を抑える設定を試し、速度優先での安定性改善を狙う。

## 別系統性
- 深い探索の代わりに早期切替を入れた高効率寄り設計。

## 期待効果
- `elapsed` の大幅短縮。
- `median` の悪化が小さい領域を狙う。

## 実装スコープ
- 新規バイナリ: `solver/src/bin/x37_macro_route_light_fast.rs`
  - `StrategyMode::MacroRoute` 起動後、環境変数を固定セット

## 検証計画
- `seed 0..19` quick
- 条件を満たした場合に `seed 0..99` full

## 追記欄
- 実装日: 2026-02-16
- 最初のブランチ/コミット: 未定
