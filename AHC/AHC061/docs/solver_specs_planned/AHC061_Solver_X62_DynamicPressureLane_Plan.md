# AHC061 Solver X62 Dynamic Pressure Lane Plan

## 目的
攻撃・防御の固定係数依存をやめ、進行段階に合わせて経路候補の選好を動的に変更する。

## 狙い
- `x47` 系の `pressure guard` に、`lane` 切替のヒューリスティックを追加。
- 前線差分が小さいときは深掘り、逆転困難時は防御復元型へ寄せる。

## 期待効果
- 速度低下が少ない範囲で `median` を上げる。
- `x53` 的な一時的改善より継続可能な改善率に寄与。

## 実装スコープ
- `solver/src/x62_macro_route_dynamic_pressure_lane.rs`（新規作成）
- パラメータ: `phase_cutoff`, `phase_split`, `target_count`, `candidate_cap`, `branch_width`
- ガード側重みを `lane_pressure` の値で時間遷移

## 検証計画
- quick: `seed 0..9`
- full: `seed 0..49`（top2）
- 判定軸: `median/min`, `mean` 併用、`full runtime` を厳密上限内に維持
