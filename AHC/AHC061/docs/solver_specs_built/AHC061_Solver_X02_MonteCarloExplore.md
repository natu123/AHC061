# AHC061 Solver X02 MonteCarloExplore

## 目的
- 相手行動をサンプリングし、期待値と分散を使って中人数帯のスコアを底上げする。

## 対象
- 系統ID: `x02`
- 系統名: `x02_monte_carlo`
- 実装本体: `solver/src/x02_monte_carlo.rs`
- 実行bin: `solver/src/bin/x02_monte_carlo.rs`

## 仕様
- 意思決定の中核:
  - `choose_move_monte_carlo`
  - 候補手を局所評価でランキング
  - 上位候補のみ Monte Carlo サンプルで評価
- サンプリング評価:
  - `build_ai_candidates_and_probs` でAI候補手分布を構築
  - 各サンプルで `simulate_turn -> strategic_score` を評価
  - `mean - risk_w * std + 0.09 * local` で最終採点
- サンプル数制御:
  - `M`に応じて `6/8/10` の段階設定
- 共通ロジック依存:
  - `solver/src/lib.rs` の `sample_index`, `strategic_score`, `evaluate_local_move` などを利用

## 強み
- `M=3..5`で期待値主導の上積みが出やすい。
- 評価関数のノイズに対して、複数サンプルで平均化できる。

## 弱み
- サンプル数やリスク重みの設定が外れると急激に悪化する。
- `M=2`や高人数帯でchampion系より不安定になるケースがある。

## 関連ログ
- 採用の基点: `docs/AHC061_Experiment_Log_2026-02.md` の `T-060`
- 不採用派生: `docs/AHC061_Experiment_Failures_2026-02.md` の `T-061`, `T-062`

## 注意点
- `x02`は探索系ソルバとして固定し、採用判定は別ラベルで管理する。

## 更新ルール
- サンプル数・リスク重み・候補上限を変更したら、設定根拠とA/B結果を追記する。
