# AHC061 Solver X10 Phase Adaptive Mix

## 目的
- x04 チャネルをベースに、`phase` と `uncertainty` で評価ロジックを切替し、`x04 / x06 / x02 / x01` の強みを局面別に混合する戦略。

## 観測系統
- ID: `x10`
- ファイル:
  - `solver/src/x10_phase_adaptive_mix.rs`
  - `solver/src/bin/x10_phase_adaptive_mix.rs`

## 実装
- 候補: `get_candidates` から上位局所候補 + `x04 / x02 / x01 / x06 / x07` の混合。
- 評価: primary(全AI予測) / secondary(一部 top2 切替) 2系統の `simulate_turn` を比較。
- `uncertainty` が高い場合に secondary 切替を強め、`phase` が進むほどリスク重視を増加。

## 実験
- `seed 0..19` quick:
  - `x10`: mean `130,815`, median `113,294`, min `58,992`, max `362,287`
  - `elapsed`: `9,557ms`
- `seed 0..99` full:
  - `x10`: mean `137,863.4`, median `124,831`, min `44,177`, max `409,094`, elapsed `35,931ms`

## 結果
- 採否: 不採用（`x04` 比で大幅劣化）
- 不採用タグ: `tail-risk-worsen`, `mean-dropped`

## ログ対応
- 不採用ログ: `docs/AHC061_Experiment_Failures_2026-02.md` の `T-074`
- 実験ログ: `docs/AHC061_Experiment_Log_2026-02.md`
