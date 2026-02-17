# AHC061 Solver X26 Reactive Frontier Pressure

## 狙い
- `frontier` の圧力と衝突密度をリアルタイムに監視し、危険局面では安全寄り、余剰局面では侵攻寄りに切替える。
- 既存 `frontier` 系に対して、低レベル上の弱いセルを優先回収しにくい `frontier fatigue` を緩和。

## 独自性
- 候補ごとの局面圧力スコアを追加し、`x23` の純 frontier ガードではなく `攻守切替` を明示。
- 2-step予測中に衝突予測が高い候補は罰則を強めるが、`phase` 後半で `gap` が大きい場合は回復候補へ再点火。

## 期待効果
- quick で mean/median が baseline に接近する候補を優先し、過度な保守での劣後を抑制。
- min を崩しやすい局面を減らすことで tail-risk を低減。

## 実装スコープ
- `solver/src/x26_reactive_frontier_pressure.rs`
- `solver/src/bin/x26_reactive_frontier_pressure.rs`
- `solver/src/lib.rs`、`solver/src/strategy_mode.rs` の登録

## 検証計画
- baseline: `seed 0..99` の `x04` full
- quick: `seed 0..19` で 3 指標採取
- full: quick 成績が近接競争である候補を複数進める
