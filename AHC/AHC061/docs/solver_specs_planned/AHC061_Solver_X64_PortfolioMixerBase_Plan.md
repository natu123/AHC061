# AHC061 Solver X64 Portfolio Mixer Base Plan

## 目的
- `x56~x63` の Macro 調整系から離れ、複数ソルバの提案を重み付きで統合する新規メタ戦略を導入する。

## 狙い / 別系統性
- `x01/x04/x06/x10/x11/x13/x14/x19/x26` を同時に候補化し、1手評価で最終選択する。
- `gap/phase/uncertainty` に応じて重みを変え、単一系統依存を避ける。

## 期待効果
- `mean` だけでなく `median/min` の同時改善余地を作り、`gain/sec` の上位候補として機能させる。

## 実装スコープ
- 対象ID: `x64`
- 実装日: `2026-02-17`
- 実装ファイル: `solver/src/x64_portfolio_mixer.rs`, `solver/src/bin/x64_portfolio_mixer.rs`

## 検証計画
- quick `seed 0..19` で `x63` と同時比較し、`final_score` と `gain/sec` 上位のみ full 候補化する。
