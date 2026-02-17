# AHC061 Solver X18 Robust Minmax Guard

## 目的
- `M=4..6` 帯で、前提対戦手の不確実性を前提に「最悪ケース抑制」を強める。
- `x01` の堅牢性と `x06` の回収性を統合し、リスク局面での崩れを抑える。

## 対象
- 系統ID: `x18`
- 系統名: `x18_robust_minmax_guard`
- 実装本体: `solver/src/x18_robust_minmax_guard.rs`
- 実行bin: `solver/src/bin/x18_robust_minmax_guard.rs`
- 状態: 実装済み（quick 実施、full 未実施）

## 仕様
- `top2` 予測を基に opponent の主力/代替手を生成。
- `x04` / `x06` / `x18` の候補を比較し、`conflict` と `phase` を使って重み付け。
- 主要候補を少数ピックし、`primary`/`secondary` 推定を併用してロバスト評価。

## 強み
- 不確実性が高い時に `x06` 側寄りへ寄せ、過激な攻め手を抑制する方針を持つ。

## 弱み
- quick 時点で `mean/median/min` が基準を下回った。
- 現時点では full へ進めるだけの根拠が不足。

## 評価結果
- quick `0..19`:
  - baseline `x04`: `mean 147,335.4`, `median 119,019.5`, `min 70,744`, `max 388,857`, `elapsed 9,173ms`
  - `x18`: `mean 120,893.8`, `median 103,413`, `min 64,002`, `max 388,857`, `elapsed 1,606ms`
- full `0..99`:
  - quick 競合ゲート未達のため未実施
- 判定:
  - 不採用（quickで平均・中央値・最小値が悪化）
- 対応タグ:
  - `new-architecture`, `robust-minmax`, `quick-regression`, `skip-full`

## 関連ログ
- 参考: `docs/solver_specs_planned/AHC061_Solver_X18_RobustMinmaxGuard_Plan.md`
- 実測: `docs/AHC061_Experiment_Failures_2026-02.md` の `T-079`
