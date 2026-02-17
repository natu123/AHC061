# AHC061 Solver X65 Portfolio Mixer Aggro Plan

## 目的
- `x64` の派生として、遅れ局面での上振れ狙いを強化し、短期的な伸び幅を最大化する。

## 狙い / 別系統性
- `x64` と同じ統合器を使い、`risk penalty` を弱めて `gap bonus/frontier` を強める。
- `x11/x19/x26` の重みを増やし、追い上げ局面で攻撃的遷移を優先する。

## 期待効果
- 既存系統より大きい `mean` 改善のヒット確率を狙う。

## 実装スコープ
- 対象ID: `x65`
- 実装日: `2026-02-17`
- 実装ファイル: `solver/src/bin/x65_portfolio_mixer_aggro.rs`

## 検証計画
- quick `seed 0..19` を優先し、`mean/gain/sec` が `x64` を上回る場合のみ full 候補とする。
