# AHC061 Solver X05 Adaptive Racing MC

## 目的
- `M=3..5` で固定サンプルMCのノイズを下げ、`mean/median` 改善を狙う。
- `M` 帯ごとに探索方式を切り替え、全体 `min` 悪化を抑える。

## 対象
- 系統ID: `x05`
- 系統名: `x05_adaptive_racing_mc`
- 実装本体: `solver/src/x05_adaptive_racing_mc.rs`
- 実行bin: `solver/src/bin/x05_adaptive_racing_mc.rs`
- 状態: 実装済み（現時点では未採用）

## 仕様
- 意思決定の中核:
  - `choose_move_x05_adaptive_racing`
  - `M=3..5` では逐次サンプリング + 候補淘汰（racing）
  - それ以外の `M` では `x01` をフォールバック利用
- racing評価:
  - 各roundで同一サンプル（common random numbers）を全候補へ適用
  - `LCB/UCB` で劣後候補を除外し、有望候補へ計算資源を集中
  - 最終的に `mean - risk_w * std - downside_w * downside_prob + local_w * local` で採択
- 共通ロジック依存:
  - `build_ai_candidates_and_probs`, `sample_index`, `simulate_turn`, `strategic_score`

## 強み（確認できた点）
- `seed 0..19` の quick 比較では `mean` を押し上げるケースを確認。
- `M=2,6,7,8` は `x01` フォールバックのため退行しにくい。

## 弱み（確認できた点）
- `seed 0..99` では `mean` が `x01` を上回らず、採用基準未達。
- `M=3..5` の分布安定化が不十分で、特に `min` の悪化が出やすい。

## 関連ログ
- 不採用: `docs/AHC061_Experiment_Failures_2026-02.md` の `T-064`, `T-065`

## 注意点
- `x05` は作成順IDで固定し、採用状態は `champion` ラベルで管理する。

## 更新ルール
- racingパラメータ（候補上限・round数・淘汰条件・risk/downside重み）を変更したら、A/B結果を追記する。
