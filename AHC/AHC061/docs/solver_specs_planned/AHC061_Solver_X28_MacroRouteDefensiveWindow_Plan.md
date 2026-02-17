# AHC061 Solver X28 - Macro Route Defensive Window

## 目的
- `x04` の `M=4` 経路計画をより防御寄りに振り、
  乱択ノイズが高い場面で過剰な長期寄与を抑える。

## 別系統性
- `x04` を母体に、`phase` 判定の早期移行と探索幅を絞る調整で
  計算量と暴れを下げる。

## 期待効果
- 低スコア事例の頻度を下げ、`min` を改善する。

## 実装スコープ
- `solver/src/bin/x28_macro_route_defensive_window.rs`
  - `phase_cutoff` を高め、`target_count`/`beam_width`/`plan_len` を抑制。

## 検証計画
- `seed 0..19` quick
- `seed 0..99` full（quick 条件を満たした場合）

## メモ
- 速度重視の安全側チューニングとして扱う。
