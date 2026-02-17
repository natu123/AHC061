# AHC061 Solver X60 Adaptive Frontier Pressure Plan

## 目的
1位との巨大ギャップ（約40%）を前提に、前線圧力の時間分解能を上げて、急落シードの `median/min` を回復する新規アーキテクチャを狙う。

## 狙い
- `pressure_phase_split` と `route_pressure` の重みを、局面深度に応じて動的に切替える。
- 早期の過激な圧力集中を抑えつつ、終盤の奪還機会を増やす。
- 1位との差が大きいため、速度優先の評価枠で `x50~x59` から一段上を狙う。

## 期待効果
- `median` の崩れを軽減しつつ `mean` の下支え。
- `min` の最悪ケース悪化を抑制。

## 実装スコープ
- `solver/src/x60_macro_route_pressure_adaptive_frontier.rs`（新規作成）
- パラメータ群: `AHC_X04_PRESSURE_PHASE_SPLIT`, `AHC_X04_TARGET_PRESSURE_WEIGHT`, `AHC_X04_PRESSURE_WEIGHT_EARLY`, `AHC_X04_PRESSURE_WEIGHT_LATE`, `AHC_X04_PRESSURE_PHASE_SPLIT`
- 併せて候補生成経路の `candidate_cap` と `target_count` を簡易的に再調整

## 検証計画
- quick: `seed 0..9`
- full: `seed 0..49`（top1）
- 判定軸: `mean/median/min` を `x04` と比較し、`median/min` を優先採点
- 速度指標: `quick ms/seed` と `full elapsed` を都度記録
