# AHC061 Solver X20 Band Stage Ensemble

## 目的
- `x01`/`x04`/`x06` を段階条件（`phase` / `uncertainty` / `conflict`）で切替し、
  `M=4..6` 帯の安定性を高める fast ensemble を実装する。

## 対象
- 系統ID: `x20`
- 系統名: `x20_band_stage_ensemble`
- 実装本体: `solver/src/x20_band_stage_ensemble.rs`
- 実行bin: `solver/src/bin/x20_band_stage_ensemble.rs`
- 状態: 実装済み（quick 実施、full 未実施）

## 仕様
- `frontier` を直接用いたスコアより、`phase` ごとの重み付き投票を生成。
- `x01` / `x04` / `x06`（必要時 `x11`, `x02`）の提案を候補として集約。
- `primary`/`secondary` の2シナリオを小規模シミュレーションで比較し、`confidence` を加点。
- 不確実性高時は保守寄り（`x06`）へ重み寄せ。

## 強み
- phase 切替に応じて評価関数を再配分するため、局面分岐が大きい時に追従しやすい。

## 弱み
- quick で平均と最小値の両方が現状を下回り、full 進行条件を満たしていない。

## 評価結果
- quick `0..19`:
  - baseline `x04`: `mean 147,335.4`, `median 119,019.5`, `min 70,744`, `max 388,857`, `elapsed 9,173ms`
  - `x20`: `mean 125,026.7`, `median 117,420`, `min 49,314`, `max 388,857`, `elapsed 9,802ms`
- full `0..99`:
  - quick 競合ゲート未達のため未実施
- 判定:
  - 不採用（quickで mean/median の改善がなく full 進行不能）
- 対応タグ:
  - `band-stage-ensemble`, `quick-regression`, `skip-full`

## 関連ログ
- 参考: `docs/solver_specs_planned/AHC061_Solver_X20_BandStageEnsemble_Plan.md`
- 実測: `docs/AHC061_Experiment_Failures_2026-02.md` の `T-081`
