# AHC061 Solver X06 Expert Switch Hybrid

## 目的
- `x01` と `x02` の強みを局面で切替え、全体 `mean/median` を改善する。
- 新規探索器を追加せず、既存資産の再利用で低コストに改善を狙う。

## 対象
- 系統ID: `x06`
- 系統名: `x06_expert_switch_hybrid`
- 実装本体: `solver/src/x06_expert_switch_hybrid.rs`
- 実行bin: `solver/src/bin/x06_expert_switch_hybrid.rs`
- 状態: 実装済み（現採用）

## 仕様
- 意思決定の中核:
  - `choose_move_x06_expert_switch`
  - `M=3..5` は `x02_monte_carlo` を採用
  - それ以外は `x01_beam_pessimistic` を採用
- 共通ロジック依存:
  - `x01` と `x02` の既存実装を直接再利用するため、追加の評価関数や状態は持たない。

## 強み
- `M=3/4/5` 帯で `x02` の優位性を取り込み、`x01` 比で `mean/median` を押し上げる。
- `M>=6` は `x01` を維持するため、高人数帯の安定性を崩しにくい。

## 弱み
- 切替条件が `M` のみで、`phase` や `uncertainty` には未対応。
- `M=3..5` の下振れ局面を個別制御する機構は未実装。

## 関連ログ
- 採用: `docs/AHC061_Experiment_Log_2026-02.md` の `T-066`
- 参照元:
  - `x01`: `docs/solver_specs_built/AHC061_Solver_X01_Beam_Pessimistic.md`
  - `x02`: `docs/solver_specs_built/AHC061_Solver_X02_MonteCarloExplore.md`

## 注意点
- `x06` は作成順IDで固定し、採用状態は `champion` ラベルで管理する。

## 更新ルール
- 切替条件（適用 `M` 帯、将来の `phase/uncertainty` 条件）を変更した場合、A/B結果とセットで更新する。
