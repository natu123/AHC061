# AHC061 Solver X38 - Macro Route Pressure Protect

## 狙い
- 相手の行動確率に基づいて、衝突リスクが高いセルへの移動を抑制し、事故的な失点を減らす。
- `M=4` 限定の `x04` フレーム上で、リスク抑制型の中盤経路計画を検証する。

## 別系統性
- `x27~x37` 系は主に速度/重み調整の系統だったのに対し、`x38` は競合確率を直接スコアに織り込む新規性を追加する。

## 期待効果
- `median` と `min` を下げない範囲で `mean` を改善。
- 衝突過多局面で tail-risk を抑える。

## 実装スコープ
- `solver/src/x04_macro_route.rs`: 移動候補の選定・経路評価時に相手圧力コストを追加。
- `solver/src/bin/x38_macro_route_pressure_protect.rs`: `AHC_X04_PRESSURE_WEIGHT` を高める設定。

## 検証計画
- `seed 0..19` quick
- quickがchampion近傍を満たす場合に `seed 0..99` full

## 追記欄
- 実装日: 2026-02-16
- 期待順位: Loop #6  (x38~x40)
