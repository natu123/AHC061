# AHC061 Solver X22 Band Stage Recovery Boost

## 目的
- x06 の回復能力と x11/x19 の後半回収を前倒しで使えるようにし、失速帯での `min` 折れを抑制する。

## 狙い
- `phase > 0.45` で回復寄り方針を上げ、`M=5,6` に対して `frontier recovery` 成分を増強。
- `x20` より強い recovery ブレンドを入れ、同時に初手の過激化を抑える。

## 期待効果
- late phase の `min` と `tail` を改善し、quick の下位 `25%` での悪化を抑制する。
- 競争ゲートを通った場合、full で `x04` より `mean/median/min` の再現性を上げる。

## 実装スコープ
- `solver/src/x22_band_stage_recovery_boost.rs`
- `solver/src/bin/x22_band_stage_recovery_boost.rs`
- `solver/src/lib.rs` / `solver/src/strategy_mode.rs` へ `x22` 追加

## 検証計画
- `x04` baseline quick `0..19` を実施し、勝ち筋候補を抽出。
- 条件は `x21` と同一：
  - `mean >= champion mean * 0.985`
  - `median >= champion median * 0.985` または `min >= champion min * 0.90`
  - top1 との差が `2.5%` 以内なら競争候補へ加点。
- 有望候補が複数なら full へ同時実施（上位3件）。  
- 不採用要因は `tail-risk`, `mean-drop`, `median-drop` を付けて失敗ログへ追記。
