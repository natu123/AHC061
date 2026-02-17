# AHC061 Solver X24 Band Stage Adaptive Switch

## 狙い
- `M=4..6` を対象に、`x04` / `x06` / `x18` の局面別得点密度を可変切替する。
- 高不確実領域での暴走を防ぎつつ、遅延期に回復余地がある帯で攻勢に転ずる。
- 既存の `x20~x23` の重み付き混合と差別化し、局所的な `frontier` 判断と `gap` 管理を分離する。

## 独自性
- 重み付けは `uncertainty`, `phase`, `global_conflict`, `leader_gap` の4軸で制御。
- `frontier recovery` と `robust guard` の導線を同一フレーム内で切り替え、同一候補に複数信号を合算。

## 期待効果
- quick では `x04` との `mean/median` 近接を優先し、`min` 低下は抑制。
- full での tail-risk を避けるため、full は quick 上位かつ `median/min` が一定以上のときのみ進める。

## 実装スコープ
- `solver/src/x24_band_stage_adaptive_switch.rs`
- `solver/src/bin/x24_band_stage_adaptive_switch.rs`
- `solver/src/lib.rs`、`solver/src/strategy_mode.rs` の登録

## 検証計画
- baseline: `seed 0..99` の `x04` full
- quick: `seed 0..19` で `mean / median / min` を採取
- quick優位・競争ケースで full を実施
