# AHC061 Solver X39 - Macro Route Pressure Attack

## 狙い
- `x38` と対になる設計として、敵圧力の高い候補を“逆張り”で攻略し得点争い領域へ積極進入する。
- `phase` 中盤以降に一時的に競合セルへ接近し、リーチ獲得速度を高める。

## 別系統性
- `x38` が保守的な競合回避を主眼とするのに対し、`x39` は競合局面を作戦行動として使う仮説。

## 期待効果
- 勝ちに近い局面での局地点差の立て直し。
- `mean` や `elapsed` を悪化させずに攻撃寄りの改善を確認。

## 実装スコープ
- `solver/src/x04_macro_route.rs`: 相手圧力項をマイナス係数（追従しない）で加算し、争奪局面を選好。
- `solver/src/bin/x39_macro_route_pressure_attack.rs`: 攻撃寄り係数を設定。

## 検証計画
- `seed 0..19` quick
- quickがchampion近傍なら `seed 0..99` full

## 追記欄
- 実装日: 2026-02-16
- 期待順位: Loop #6  (x38~x40)
