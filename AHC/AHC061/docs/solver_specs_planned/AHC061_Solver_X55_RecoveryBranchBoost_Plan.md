# AHC061 Solver X55 Recovery Branch Boost Plan

## 目的
- x51 の分岐拡張意図と x52 の中盤安定化を組み合わせ、`median` 側の下振れをさらに抑える。
- `x47` 系の失敗収束局面で、回復候補を増やして再発・崩れを減らす。

## タグ
- ID: `x55`
- バイナリ: `x55_macro_route_pressure_recovery_branch_boost`
- 主要変更ファイル:
  - `solver/src/bin/x55_macro_route_pressure_recovery_branch_boost.rs`

## 仮説
- `branch_width` と `candidate_cap` を拡張すると、回復シーケンスの取りこぼしを抑えられ `median` が上がるはず。
- `pressure_weight_early` を中程度に上げて前半の失速を抑え、`pressure_weight_late` を過剰にしないことで `tail-risk` を改善する。
- `route_pressure` を適度に上げることで、失敗後の戻しを増やしつつ平均の大幅崩れを回避する。

## 検証計画
- quick: `seed 0..19`
- quick で門番条件を満たした場合のみ full: `seed 0..99`
- 判定:
  - quick/full: `mean` / `median` / `min` / `elapsed` を同時比較
