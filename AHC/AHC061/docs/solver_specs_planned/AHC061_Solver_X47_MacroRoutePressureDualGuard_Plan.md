# AHC061 Solver X47 - Macro Route Pressure Dual Guard

## 狙い
- `x42` の frontier shield を前提に、候補選別を濃密化しつつ前半をやや探索寄り、後半は制約を抑えるハイブリッド化を試す。

## 別系統性
- `target/candidate` と `plan/beam` 設定を分離し、`pressure` 系をフェーズで反転させる。

## 期待効果
- quick で `mean/median` の下振れ抑制（`x42` を上回る余地）と、遅延時の安定性改善。

## 実装スコープ
- `solver/src/bin/x47_macro_route_pressure_dual_guard.rs`: `AHC_X04_*` の再設定。

## 検証計画
- `seed 0..19` quick
- 条件（mean/median/min）満たす場合 `seed 0..99` full

## 追記欄
- 実装日: 2026-02-16
- 期待順位: Loop #9 (x47~x49 quick triage)
