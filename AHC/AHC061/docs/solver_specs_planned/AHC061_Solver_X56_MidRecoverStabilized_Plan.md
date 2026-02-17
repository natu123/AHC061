# AHC061 Solver X56 Mid Recover Stabilized Plan

## 目的
- x52 の median 改善を維持しつつ、late pressure の過剰化を抑えて分散悪化を防ぐ。
- `x47` と x52 の特性をブレンドし、mean/median/min の同時改善を目指す。

## 目的と関連性
- ID: `x56`
- 種別: `Macro Route Pressure Mid-Recover Stabilized`
- 関連系統: `x52`（起点） / `x47`（監視対象）

## 仮説
- x52 は late pressure を伸ばすことで median を改善できるが、mean の上積みが限定的。
- `pressure_weight_late` を 1.22 -> 1.18 に下げ、`pressure_weight_early` を 0.03 に寄せることで、過剰圧力依存の悪化を抑えつつ回復性を保つ。
- route 側は `route_pressure_late` を 0.74 -> 0.64 に抑え、過学習的な巻き返しを避ける。

## 変更ファイル（想定）
- `solver/src/bin/x56_macro_route_pressure_midrecover_stable.rs`

## 検証計画
- quick: `seed 0..19`
- quick で gate 通過した場合のみ full: `seed 0..99`
- 判定指標:
  - `mean` / `median` / `min` / `elapsed`
  - x47 と対比し、`median` の改善が維持できるか確認
