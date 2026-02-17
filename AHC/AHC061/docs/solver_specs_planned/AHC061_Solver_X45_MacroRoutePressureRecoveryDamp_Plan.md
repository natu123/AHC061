# AHC061 Solver X45 - Macro Route Pressure Recovery Damp

## 背景
- 圧力指標が強すぎる初手で悪化しやすいケースを抑えるため、前半は抑圧し後半のみ回復寄せする形で比較する。

## 狙い
- 早期の過剰進行を避け、遅い盤面での回復性を優先。
- `pressure_weight` / `route_pressure_weight` を位相依存で制御し、`x42` より安定な下振れ抑制を狙う。

## 期待効果
- quick では `mean` 近傍を維持し、tail 部分の改善で full 比較に進みやすい構成を目指す。

## 検証計画
- quick: `seed 0..19`
- full: quickゲート通過時のみ `seed 0..99`
- `x04` と比較して `mean`, `median`, `min`, `elapsed_total` を確認。

## 実装スコープ
- `solver/src/bin/x45_macro_route_pressure_recovery_damp.rs`

## タグ
- `pressure-damp`, `safe-transition`, `x04-variant`

## 追記
- `2026-02-16`
- Loop: x45 新規投入
