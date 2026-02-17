# AHC061 Solver X19 Frontier Recovery Sweep

## 目的
- `M=5,6` で頻発する中盤以降の失速を抑えるため、周辺フロンティアの回収効率と自陣保全を両立した 2レイヤー候補選抜を実装する。

## 期待効果
- 単純な到達価値偏重を避け、回収可能性が高い frontier を優先しつつ、既存占有地の崩壊リスクを抑える。
- 既存の `x04`/`x06` とは異なる切替軸（frontier 構造 + 安全回復）を組み込む。

## 仕様
- 対象: `x19_frontier_recovery_sweep`
- 候補生成:
  - 自分の `local` 候補上位 + 近傍 frontier 候補 + `x04`/`x01` の中継候補 + `x06` の回収候補を結合。
- 評価:
  - `frontier_potential` への寄与（`crate::frontier_potential`）を明示的加点。
  - 自陣 level0/1 レベルでの脆弱隣接を減点する防御項を追加。
  - `uncertainty` が高いほど `x04` と `x01` の比重を高める。
- 適用帯: `game.m >= 5` 優先、`game.m == 4` は保守的に `x04` にフォールバック。

## 実装スコープ
- `solver/src/x19_frontier_recovery_sweep.rs`
- `solver/src/bin/x19_frontier_recovery_sweep.rs`
- `solver/src/lib.rs` と `solver/src/strategy_mode.rs` に `x19` を追加

## 検証計画
- `x04` をベースラインに quick/full を実施。
- 不採用なら同一 `x19` と `x04` の失敗差分を `docs/AHC061_Experiment_Failures_2026-02.md` へ記載。
