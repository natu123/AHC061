# AHC061 Solver X07 Dual Horizon Route Plan

## 目的
- `x04` の中期経路探索を維持しつつ、短期安全性を同時に評価して tail-risk を抑える。
- `mean` 改善だけでなく、`min` の悪化を防ぎながら `M=4` 帯の上積みを狙う。

## 対象
- 系統ID（予定）: `x07`
- 系統名（予定）: `x07_dual_horizon_route`
- 状態: 初版実装済み（2026-02-16, 非採用）

## 仕様（計画）
- 狙い:
  - 1つの固定 horizon に依存せず、短期 horizon と中期 horizon を同時評価して局面適応する。
- 別系統性:
  - `x03` の Particle + CVaR でも `x06` の expert固定切替でもなく、同一候補手に対する多地平線評価を主軸とする。
- 期待効果:
  - `seed 0..19` で `x04` 比 `mean +1.0%` 以上、`min` 同等以上を目標とする。

## 実装スコープ（初期）
- `x04` の route 評価を流用しつつ、`short_horizon` と `long_horizon` の2系列を算出。
- 不確実性（`uncertainty`）に応じて短期/中期の混合比を可変化。
- `M=4`・序中盤限定で適用し、非適用帯は `x06` フォールバックを維持。

## 実験計画（初期）
- quick:
  - seed `0..19` で `x04` と A/B。
- full移行条件:
  - Loop内 quick 上位1件になった場合のみ seed `0..99` 実施。
- 記録必須:
  - mean/median/min/max, elapsed, `M=4` 帯の差分観測。

## 注意点
- horizon二重化で計算量が増えやすいため、branch幅と候補上限の制御を必須とする。

## 更新ルール
- 実装着手時に「着手日」「対象commit」「初期パラメータ」を追記する。
- 実装済みになったら `docs/solver_specs_built` に移管または追記する。

## 実装着手メモ
- 着手日: 2026-02-16
- 対象commit: `8f4cff5` 以降
- 初回結果:
  - `T-071` で不採用（fullで score 指標が `x04` を下回る）
