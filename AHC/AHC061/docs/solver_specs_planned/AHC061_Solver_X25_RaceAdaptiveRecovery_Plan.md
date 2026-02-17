# AHC061 Solver X25 Race Adaptive Recovery

## 狙い
- `contest frontier recovery` と `x20系` の長所を残しつつ、対戦相手圧力に応じて回復手段を速い/丁寧に切替。
- 落差が拡大した局面では `回復志向` を強め、安定領域では `frontier` 開拓を維持。

## 独自性
- 局面を `phase`, `max gap`, `衝突総量`, `自陣有利度` で分類し、候補の二段階スコアを切替。
- `simulate_turn` ベースの 2step ルックで、回復フェーズか攻勢フェーズかを可変化。

## 期待効果
- 進行中の劣勢からの立て直しを速くし、`min` 下振れの連鎖を減らす。
- quick が良ければ full へ進める前提で `x04` との差分優位性を検証。

## 実装スコープ
- `solver/src/x25_race_adaptive_recovery.rs`
- `solver/src/bin/x25_race_adaptive_recovery.rs`
- `solver/src/lib.rs`、`solver/src/strategy_mode.rs` の登録

## 検証計画
- baseline: `seed 0..99` の `x04` full
- quick: `seed 0..19` の 3 指標採取
- quick 上位競合なら full を複数進める（2候補目まで）
