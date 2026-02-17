# AHC061 Solver X18 Robust Minmax Guard

## 目的
- `M=4..6` 帯で、`seed 0..99` で悪化しやすい `x04` の局所方針を補うため、敵行動の不確実性に対する最悪寄りの評価で一手を選抜する。
- `main` 経路ではなく、対手の反応を複数シナリオで比較して、期待値だけでなくリスク抑制を効かせた選択を行う。

## 期待効果
- `mean/median` の小さな悪化を抑え、`max`（最悪ケース）を下げる。
- `x07/x08/x17` と同時期の戦略と異なり、敵行動の「最悪接触」を明示的に評価する点が新規。

## 仕様
- 対象: `x18_robust_minmax_guard`
- 適用帯: `game.m == 4..6` を優先対象。
- 候補評価:
  - 自分候補を `get_candidates` から収集し、`local` 上位を再評価。
  - `primary` / `secondary` の双方（AI予測）で 1手先展開を比較。
  - 各相手プレイヤーを1体ずつ `primary` から `secondary` へ切替えた最悪シナリオ（1体ずつの対抗最悪）を別評価。
  - 2ステップの quick rollout で戦略スコアに先行加点。
- 判定式: `local + 1-step + worst_penalty + conflict_guard + quick_rollout` の加重和を最大化。

## 実装スコープ
- `solver/src/x18_robust_minmax_guard.rs`
- `solver/src/bin/x18_robust_minmax_guard.rs`
- `solver/src/lib.rs` と `solver/src/strategy_mode.rs` に `x18` を追加

## 検証計画
- `x04` をベースラインに
  - quick: `seed 0..19`
  - full: quick 競争ゲートで有望候補を抽出し、`seed 0..99` は上位最大3件を実施（0件は full 不実施）
- `result` が baseline 改善しない場合は即不採用として `docs/AHC061_Experiment_Failures_2026-02.md` へ記録。
