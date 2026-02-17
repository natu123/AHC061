# AHC061 Solver X43 - Macro Route Pressure Adaptive Lane

## 狙い
- 早期は回避的、中後期は攻撃的に圧力バランスを反転する 2 フェーズ戦略。

## 別系統性
- phase 分割を明示的に使い、x40 を圧力遷移をより鋭くした派生にする。

## 期待効果
- 速度・安定性を保ったまま後半の取り返し上振れを狙う。

## 実装スコープ
- `solver/src/bin/x43_macro_route_pressure_adaptive_lane.rs`: phase別圧力重みを強めた設定。

## ?証計画
- `seed 0..19` quick
- `full` は quick上位競合が champion 近傍の場合

## 追記欄
- 実装日: 2026-02-16
- 期待順位: Loop #7 (x41~x43)
