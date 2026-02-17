# AHC061 Solver X48 - Macro Route Pressure Conservative Shield

## 狙い
- `x42` と `x47` を補完し、後半の過剰プレッシャーを抑える方向で衝突リスク・テール悪化の縮小を狙う。

## 別系統性
- 低分枝（branch=1）と短め計画長で保守寄りに寄せた独自設定。

## 期待効果
- `min` と `tail-risk` 改善を優先し、`median` を維持する。

## 実装スコープ
- `solver/src/bin/x48_macro_route_pressure_conservative_shield.rs`: `AHC_X04_*` の再設定。

## 検証計画
- `seed 0..19` quick
- 条件満足時のみ `seed 0..99` full

## 追記欄
- 実装日: 2026-02-16
- 期待順位: Loop #9 (x47~x49 quick triage)
