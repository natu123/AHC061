# AHC061 Solver X61 Recovery Ensemble Switch Plan

## 目的
既存の 1位差が大きい局面で、単一ルールの欠点を避けるため、回復寄りと攻撃寄りの2系統を状態依存で切替する。

## 狙い
- `x53`（中盤加速）系と `x56`（安定化）系の中核ロジックを切り替えます。
- 盤面特徴量（敵残数/残コマ数/距離分散）に基づき、ブレンド比率を可変化。

## 期待効果
- 低中央値域でのリスク行動を抑え、全体のロバスト性を改善。
- 速く改善が出る組み合わせを `x60~x62` の第一世代で特定。

## 実装スコープ
- `solver/src/x61_macro_route_recovery_ensemble_switch.rs`（新規作成）
- 切替条件: `phase`, `frontier_gap`, `blocked_frontier_count`
- `route_pressure`, `branch_width`, `candidate_cap` を状態別に最適化

## 検証計画
- quick: `seed 0..19`
- full: `seed 0..99`（top1）
- 判定軸: `mean/median/min`, `EGR`（gain/time）
- `fallback` 条件: `median` が既存比で悪化し続ける場合は即時中止
