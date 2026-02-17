# AHC061 Solver X27 - Macro Route Adaptive Cutoff

## 目的
- `x04` の `M=4` 専用経路探索を基軸に、`phase` 進行に応じた切替条件を最適化し、
  平均スコアと中央値を維持したまま安定性を上げる。

## 別系統性
- `x04` を母体とし、`AHC_X04_PHASE_CUTOFF` と `AHC_X04_PHASE_SPLIT` を
  非対称に変えることで、後半への移行点を再調整する。

## 期待効果
- `x04` の中盤〜後半で過剰に `x06` へ寄らない構成を作り、
  `median` と `min` を同時に伸ばす可能性を探る。

## 実装スコープ
- `solver/src/bin/x27_macro_route_adaptive_cutoff.rs`
  - 3点の環境変数を固定値で設定。
  - 参照戦略は `StrategyMode::MacroRoute`。

## 検証計画
- `seed 0..19` quick
- `seed 0..99` full（quick 上位条件クリア時）

## メモ
- 基準: x04 同等の `x04` 設定との比較。
