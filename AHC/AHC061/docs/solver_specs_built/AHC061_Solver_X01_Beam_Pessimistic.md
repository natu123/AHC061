# AHC061 Solver X01 Beam Pessimistic

## 目的
- 高人数帯での不確実性を抑えつつ、1手先+短期先読みで安定して`mean`を伸ばす。

## 対象
- 系統ID: `x01`
- 系統名: `x01_beam_pessimistic`
- 実装本体: `solver/src/x01_beam_pessimistic.rs`
- 実行bin: `solver/src/bin/x01_beam_pessimistic.rs`

## 仕様
- 意思決定の中核:
  - `choose_move_x01_beam_pessimistic`
  - 候補手を `evaluate_local_move` + 予測rolloutで採点
  - 上位候補に beam を適用し、`best_one_step_score` で1手先を評価
- 相手行動予測:
  - `choose_predicted_ai_top2_moves` で各AIの上位2手を推定
  - `build_secondary_ai_moves` で脅威上位AIに第2候補切替を適用
  - `pessimism_weight` で悲観重みを調整
- 主要な局面制御:
  - `M=5` かつ不確実性高めで full-beam
  - `M=6` で不確実性高めなら beam を拡張
- 共通ロジック依存:
  - `solver/src/lib.rs` の `simulate_turn`, `estimate_conflict_map`, `evaluate_local_move` などを利用

## 強み
- `M>=6`の下振れ抑制が比較的強い。
- top2予測+悲観重みの組合せでtail-riskが安定しやすい。

## 弱み
- 中人数帯（`M=3..5`）では、探索の多様性不足で伸びが頭打ちになりやすい。
- 予測誤差が大きい局面では、決め打ちrolloutの限界が出る。

## 関連ログ
- 採用改善系列: `docs/AHC061_Experiment_Log_2026-02.md` の `T-048`, `T-053`
- 基準化の系統運用: `AGENTS.md` の `Solver ID / Champion Policy`

## 注意点
- `x01`は作成順IDで固定し、採用状態は`champion`ラベルで管理する。

## 更新ルール
- `x01`内部のアルゴリズム変更時は、主関数・条件分岐・期待効果の3点を更新する。
