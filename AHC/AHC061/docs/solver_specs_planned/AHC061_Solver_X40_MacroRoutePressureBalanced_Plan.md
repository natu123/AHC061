# AHC061 Solver X40 - Macro Route Pressure Balanced

## 狙い
- `x38` の回避寄りと `x39` の攻撃寄りを中間化し、phaseに応じて圧力係数を変える。
- 早期は安全寄り、終盤は機会主導へスイッチすることで安定性を優先。

## 別系統性
- `x38` / `x39` が圧力係数を固定で扱うのに対し、`x40` は early/late で係数を可変にし、時間分布に追従。

## 期待効果
- 安定性を保ちつつ、終盤の取り返しを強める。
- `full` での `mean/median` 破綻を避ける。

## 実装スコープ
- `solver/src/x04_macro_route.rs`: フェーズ依存の圧力係数を環境変数で扱えるように追加。
- `solver/src/bin/x40_macro_route_pressure_balanced.rs`: 早期は防御、後半は攻撃寄りに寄せる設定。

## 検証計画
- `seed 0..19` quick
- 条件を満たす場合に `seed 0..99` full

## 追記欄
- 実装日: 2026-02-16
- 期待順位: Loop #6  (x38~x40)
