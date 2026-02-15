# AHC061 Solver X06 Expert Switch Hybrid Plan

## 目的
- `x01`（高人数帯安定）と `x02`（中人数帯期待値）を局面ごとに切り替え、全体 `mean` を押し上げる。
- 新規探索器の導入コストを抑えながら、既存資産の強みを合成する。

## 対象
- 系統ID（予定）: `x06`
- 系統名（予定）: `x06_expert_switch_hybrid`
- 状態: 計画中（未実装）

## 仕様（計画）
- 狙い:
  - 既存の2 expert（`x01`, `x02`）を局面条件で使い分け、`M=3..5` の改善を取りつつ `M>=6` の安定性を維持する。
- 別系統性:
  - `x03` の確率推論（Particle + CVaR）でも、`x04` の中期経路探索でも、`x05` の逐次淘汰MCでもない。
  - 既存expertのメタ制御（policy orchestration）に特化する。
- 期待効果:
  - `seed 0..99` で `x01` 比 `mean +1.0%` 以上を目標とする。
  - `min` は `x01` 比で `-3%` 以内に抑える。

## 実装スコープ（初期）
- 実行モード:
  - `StrategyMode` に `ExpertSwitchHybrid` を追加する。
- 切替ロジック:
  - 初版は `M` 帯ベース（`M=3..5 -> x02`, それ以外 `x01`）。
  - 次段で `phase` や `uncertainty` を使った条件分岐へ拡張可能にする。
- 既存資産の再利用:
  - `choose_move_x01_beam_pessimistic`, `choose_move_monte_carlo` をそのまま利用。

## 実験計画（初期）
- Small batch:
  - seed `0..19` で A/B（現champion `x01` vs `x06` 初版）
- Full eval条件:
  - quickで `mean` 改善かつ `min` 悪化が軽微なら seed `0..99` へ拡張
- 記録必須:
  - mean/median/min/max, elapsed, `M` 帯別サマリ

## 注意点
- `x02` の下振れをそのまま持ち込むと `min` を悪化させる可能性がある。
- 条件分岐を増やしすぎると no-op か過学習になりやすい。

## 更新ルール
- 実装着手時に「着手日」「対象commit」「初期切替条件」を追記する。
- 廃案時も理由と再発防止タグを追記して残す。
