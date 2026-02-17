# AHC061 Solver X20 Band Stage Ensemble

## 目的
- 同一ゲーム内で `phase`/`uncertainty`/`m` を横断して、`x01/x02/x04/x06` の採用比率を状態変動に応じて自動変更する高速 ensemble を実装する。

## 期待効果
- 既存の2系統（固定切替 or 2手読み）と異なり、帯・局面ごとの状態推定で戦略を再重み付けし、`M=4..6` の安定性を狙う。
- quick での `mean` 向上と late phase での `min` 改善を両立する。

## 仕様
- 対象: `x20_band_stage_ensemble`
- 入力: `phase`, `uncertainty`, `phase_score_gap`, `conflict`。
- スコア生成:
  - 候補集合は `x01` / `x04` / `x06` の3提案をベースに、`M=6` でのみ `x11` 系を追加。
  - 各候補に対し `1-step local`、`2-step rollout`、`conflict`、`frontier` を加重合算。
  - `uncertainty` 高時は保守側（`x06`）を重くし、低時は探索側（`x04`）を重くする。
- 適用帯: `game.m == 4..6`。

## 実装スコープ
- `solver/src/x20_band_stage_ensemble.rs`
- `solver/src/bin/x20_band_stage_ensemble.rs`
- `solver/src/lib.rs` と `solver/src/strategy_mode.rs` に `x20` を追加

## 検証計画
- `x04` をベースラインに quick `0..19` を実施し、まず quick mean 上位を作り、`top1` と `topk` の mean 差が `2.5%` 以内かつ、`mean` が champion の `98.5%` を満たす候補を競争ゲート候補とする。
- 併せて `median` が `champion median` の `98.5%` 以上、または `min` が `champion min` の `90%` 以上を満たす候補のみ full 判定に進める。
- 競争候補が複数ある場合は `seed 0..99` の full を上位から最大 `3` 件まで実施（0件は full 不実施）。
- 失敗時は `tail-risk` と `no-op` を明記して不採用ログへ反映。




