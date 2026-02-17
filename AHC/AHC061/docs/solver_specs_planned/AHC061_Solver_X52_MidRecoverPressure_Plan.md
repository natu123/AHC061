# AHC061 Solver X52 Mid-Recover Pressure Plan

## 目的
- `x50` の `mean` 改善を維持しつつ、`median` を回復方向に寄せる補助設計を検証する。
- 早期圧力の暴走を抑え、中盤以降の安定性を取り戻す。

## タグ
- ID: `x52`
- バイナリ: `x52_macro_route_pressure_dual_guard_midrecover`
- 主要変更ファイル:
  - `solver/src/bin/x52_macro_route_pressure_dual_guard_midrecover.rs`

## 仮説
- `AHC_X04_PRESSURE_WEIGHT_EARLY` をわずかに正にして、早期の過剰攻撃を抑える。
- `ROUTE_PRESSURE` 系を中程度に保ち、終盤の取り戻しを維持しながら `median` の崩れを抑える。

## 検証計画
- quick: `seed 0..19`
- quick で競争条件を満たした場合のみ full: `seed 0..99`
- 判定:
  - quick: `mean / median / min` の改善を確認
  - full: `mean` と `median` 両立を確認し、`min` と `elapsed` も併記

