# AHC061 Solver X53 Mid-Boost Pressure Plan

## 目的
- `x52` の `median` 改善を保ちながら、`mean` を上積みしやすい圧力重みを追加探索する。
- `x47` 系統の平均性能を下回らない圏で `median` を維持する設計点を探る。

## タグ
- ID: `x53`
- バイナリ: `x53_macro_route_pressure_dual_guard_midboost`
- 主要変更ファイル:
  - `solver/src/bin/x53_macro_route_pressure_dual_guard_midboost.rs`

## 仮説
-  `phase` と `pressure` を中盤寄りに早めに切り替え、後半の収束を強める。
- `BEAM/PLAN` を拡張して局所化を避け、再び `mean` を取り戻す。

## 検証計画
- quick: `seed 0..19`
- 条件を満たす場合のみ full: `seed 0..99`
- 判定:
  - `mean` / `median` の両方が baseline を上回るなら採用候補として full 比較

