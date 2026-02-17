# AHC061 Solver X23 Band Stage Frontier Guard

## 目的
- frontier 付近の高価値候補を維持しつつ、競合と崩壊リスクが高い手を抑える局面ガードを追加する。

## 狙い
- x20 系譜の提案重みは維持し、候補生成段階で frontier への寄与と衝突リスクの比率を再配分。
- `M=4..6` の中盤以降で `unrest`（不確実性＋衝突率）を考慮した「守りを優先する回避手」を選びやすくする。

## 期待効果
- `max` 側の悪化を抑え、`min` が突出して落ちる種子群を回避。
- quickで `median` と `min` が上がる傾向が見られるなら、full へ拡張。

## 実装スコープ
- `solver/src/x23_band_stage_frontier_guard.rs`
- `solver/src/bin/x23_band_stage_frontier_guard.rs`
- `solver/src/lib.rs` / `solver/src/strategy_mode.rs` へ `x23` 追加

## 検証計画
- `x04` baseline に対して quick `0..19` を実施。
- 候補ゲートは `mean` と `median/min` の3条件を厳守。
- 競争候補が複数時、quick 上位3件を full `seed 0..99` へ同時展開。
- 不採用は `frontier-worsen` / `recovery-miss` を付与して失敗ログへ記載。
