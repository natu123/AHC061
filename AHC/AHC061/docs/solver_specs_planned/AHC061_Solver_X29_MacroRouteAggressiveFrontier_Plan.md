# AHC061 Solver X29 - Macro Route Aggressive Frontier

## 目的
- `x04` の frontier 重視動作を強め、
  収束前半の方向決定を積極化して `mean` を取り返す。

## 別系統性
- `x04` を母体に、`target` 展開数と `candidate cap`、beam幅を拡大し
  局所最適化の追従力を上げる。

## 期待効果
- `seed 0..99` 全体平均の回復と `median` 改善。

## 実装スコープ
- `solver/src/bin/x29_macro_route_aggressive_frontier.rs`
  - `target_count`/`target_eval` を増やし、`beam_width` を拡大。

## 検証計画
- `seed 0..19` quick
- `seed 0..99` full（quick 条件を満たした場合）

## メモ
- 探索量増大に伴う elapsed の劣化を併記して判定する。
