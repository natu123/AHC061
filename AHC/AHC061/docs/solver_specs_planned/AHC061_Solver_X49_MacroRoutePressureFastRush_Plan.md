# AHC061 Solver X49 - Macro Route Pressure Fast Rush

## 狙い
- `x42` の遅め設定を対比し、前半の広範選択・後半の高圧力制御で一気に上振れしやすい経路を狙う。

## 別系統性
- `candidate/beam` 構成を拡張し、`route_coeff` と `route_pressure` の高め設定で先読みを強化。

## 期待効果
- `mean` と `median` の両方改善が見込める場合は `full` へ進み、上位競合候補として扱う。

## 実装スコープ
- `solver/src/bin/x49_macro_route_pressure_fast_rush.rs`: `AHC_X04_*` の再設定。

## 検証計画
- `seed 0..19` quick
- 条件満足時および競合上位の場合、同時に `seed 0..99` full

## 追記欄
- 実装日: 2026-02-16
- 期待順位: Loop #9 (x47~x49 quick triage)
