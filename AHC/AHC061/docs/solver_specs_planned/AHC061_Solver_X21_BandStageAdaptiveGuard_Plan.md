# AHC061 Solver X21 Band Stage Adaptive Guard

## 目的
- `x20` の過学習気味な混合を抑え、`high-uncertainty`/`high-conflict` 局面で `x18` を含む安全寄りの救済を追加して
  tail-risk を縮める。

## 狙い
- `M=4..6` に限定し、`x01/x04/x06`/`x18` の切替を phase・不確実性・衝突強度で行う。
- `x20` の vote 重みを再設計し、`x18` が選ばれるケースでは攻勢を抑えつつ復旧性能を上げる。

## 期待効果
- `mean` だけでなく `min` の悪化を抑える。
- quick で `median` を取り戻し、競争ゲート条件を増やす。

## 実装スコープ
- `solver/src/x21_band_stage_adaptive_guard.rs`
- `solver/src/bin/x21_band_stage_adaptive_guard.rs`
- `solver/src/lib.rs` / `solver/src/strategy_mode.rs` へ `x21` 追加

## 検証計画
- champion `x04` を baseline として quick `0..19` を実施。
- quick 上位条件:
  - `mean` が champion の `98.5%` 以上
  - かつ (`median` が `98.5%` 以上、または `min` が `90%` 以上)
  - top1 との差を 2.5% 以内なら競争候補へ採用。
- 条件を満たす複数候補があれば full `seed 0..99` を最大3件まで実施。
- 不採用は `docs/AHC061_Experiment_Failures_2026-02.md` に `tail-risk` と `no-op` を明記して記録。
