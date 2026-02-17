# AHC061 Solver X30 - Macro Route Path Focus

## 狙い
- `x27` / `x28` / `x29` の挙動を踏まえ、`MacroRoute` の短期〜中期経路上を重点的に改善する。
- `plan_len` / `beam_width` / `candidate_cap` を拡張し、良好ターゲットへの到達期待値を強める。

## 別系統性
- 既存 `x27`/`x28`/`x29` 系の派生ではあるが、`phase_split` と `target_eval` に重みを寄せた別レバー構成。

## 期待効果
- `mean/median` の同時改善。
- `elapsed` は `x29` 程度に抑えつつ、`min` の悪化を防ぐ。

## 実装スコープ
- 新規バイナリ: `solver/src/bin/x30_macro_route_path_focus.rs`
  - `StrategyMode::MacroRoute` 起動後、環境変数を固定セット

## 検証計画
- `seed 0..19` quick
- 条件を満たした場合に `seed 0..99` full

## 追記欄
- 実装日: 2026-02-16
- 最初のブランチ/コミット: 未定

