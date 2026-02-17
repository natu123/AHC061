# AHC061 Solver X66 Portfolio Mixer Guard Plan

## 目的
- `x64` の派生として、`median/min` を守る防御寄り合成器を追加する。

## 狙い / 別系統性
- `x01/x06/x13` 重みを増やし、`risk penalty` を強めて tail-risk を抑制する。
- 攻撃型 `x65` と同時運用し、quick で局面適性を比較する。

## 期待効果
- `x65` の上振れと `x66` の安定性をポートフォリオとして使い分けできる。

## 実装スコープ
- 対象ID: `x66`
- 実装日: `2026-02-17`
- 実装ファイル: `solver/src/bin/x66_portfolio_mixer_guard.rs`

## 検証計画
- quick `seed 0..19` の `median/min` 優位を確認し、`x65` とペアで full 進出可否を判断する。
