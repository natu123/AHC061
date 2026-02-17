# AHC061 Solver X36 - Macro Route Candidate Rich

## 狙い
- 候補数・ビーム幅を広げることで、良好な中盤局面を取りこぼさない探索を狙う。

## 別系統性
- `x31` の局所寄与と `x29` の経路寄与を両立する中間構成。

## 期待効果
- `mean` と `min` の改善。
- `median` の悪化を最小化。

## 実装スコープ
- 新規バイナリ: `solver/src/bin/x36_macro_route_candidate_rich.rs`
  - `StrategyMode::MacroRoute` 起動後、環境変数を固定セット

## 検証計画
- `seed 0..19` quick
- 条件を満たした場合に `seed 0..99` full

## 追記欄
- 実装日: 2026-02-16
- 最初のブランチ/コミット: 未定
