# AHC061 Solver X19 Frontier Recovery Sweep

## 目的
- `M=5,6` の中盤以降で停滞しやすい局面を想定し、フロンティア回収と自陣保全の同時改善を狙う。
- frontier 候補を2層で再評価し、敵行動の局面分岐に対して復元力を上げる。

## 対象
- 系統ID: `x19`
- 系統名: `x19_frontier_recovery_sweep`
- 実装本体: `solver/src/x19_frontier_recovery_sweep.rs`
- 実行bin: `solver/src/bin/x19_frontier_recovery_sweep.rs`
- 状態: 実装済み（quick 実施、full 未実施）

## 仕様
- `frontier_recovery` 系の価値関数を追加し、隣接再生候補を加点。
- `x04`/`x01`/`x06` を参照した advisor 候補を束ね、上位候補を1st ステージで選抜。
- 1st 候補に対して `primary`・`secondary` の opponent シナリオを2層シミュレーション。
- 不確実性が高い時は防御側寄りの補正を強くする。

## 強み
- frontier 近傍の回収性を明示的に考慮しやすく、急落局面の代替安全策を持つ。

## 弱み
- quick で平均が baseline を下回り、`min` も悪化したため full 進行条件を満たしていない。

## 評価結果
- quick `0..19`:
  - baseline `x04`: `mean 147,335.4`, `median 119,019.5`, `min 70,744`, `max 388,857`, `elapsed 9,173ms`
  - `x19`: `mean 143,191.6`, `median 119,019.5`, `min 70,370`, `max 388,857`, `elapsed 11,518ms`
- full `0..99`:
  - quick 競合ゲート未達のため未実施
- 判定:
  - 不採用（quickで mean が悪化）
- 対応タグ:
  - `frontier-recovery`, `quick-regression`, `skip-full`

## 関連ログ
- 参考: `docs/solver_specs_planned/AHC061_Solver_X19_FrontierRecoverySweep_Plan.md`
- 実測: `docs/AHC061_Experiment_Failures_2026-02.md` の `T-080`
