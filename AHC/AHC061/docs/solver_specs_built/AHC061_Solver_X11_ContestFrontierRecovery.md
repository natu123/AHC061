# AHC061 Solver X11 Contest Frontier Recovery

## 目的
- 前線争奪の停滞を解消し、`M=3..5` 帯を中心に後半での回復力を上げる探索器。

## 対象
- 系統ID: `x11`
- 系統名: `x11_contest_frontier_recovery`
- 実装本体: `solver/src/x11_contest_frontier_recovery.rs`
- 実行bin: `solver/src/bin/x11_contest_frontier_recovery.rs`
- 状態: 実装済み（非採用）

## 仕様
- `get_candidates` で局所候補を列挙し、`frontier_recovery_pressure` を併用して候補を再スコア。
- `x04` / `x02` / `x06` / `x07` / `x08` / `x09` の提案手を advisory として活用。
- `x11` は `leader_gap` と `frontier` を重み付けし、競合率と局所評価を最終スコアへ統合。

## 強み
- quick では `M=3` 前後の攻守切替性能に有利な局面を狙える設計。

## 弱み
- `x04` 基準と比べて全体平均と中央値で継続悪化。

## 評価結果（Loop #2）
- quick（seed `0..19`, 対 `x04`）:
  - `x04`: mean `147,335.4`, median `119,019.5`, min `70,744`, max `388,857`, elapsed `9,173ms`
  - `x11`: mean `120,218.6`, median `117,153`, min `62,164`, max `195,538`, elapsed `18,103ms`
- 判定:
  - 不採用（quick 段階での `mean/median/min` 悪化）

## 関連ログ
- 不採用: `docs/AHC061_Experiment_Failures_2026-02.md` の `T-075`
- 計画: `docs/solver_specs_planned/AHC061_Solver_X11_ContestFrontierRecovery_Plan.md`

## 更新ルール
- 候補集合と重み戦略を更新した場合、失敗理由と比較値を更新する。
