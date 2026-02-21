# AHC061 Solver Specs Index (built)

## 目的
- 作成済み系統 `xNN` を、戦略立案の起点となる「狙い」と「結果」で即時検索できる形に統一します。
- 以降のループでは、この index を最初に確認してから仮説着想を始め、重複実装と失敗繰返しを抑えます。

## 使い方
- 各 `xNN` の見出しで次を確認します。
  - 狙い（どのボトルネックを狙ったか）
  - 結果（`seed` 範囲、`mean/median/min/max/elapsed`）
  - 判定（採用 / 不採用 / 保留）
  - 参照先（spec / experiment log / failure log）
- `status` が採用なら、同系統の派生設計に再利用しやすい順に上位へ。
- `status` が不採用なら、失敗要因と関連タグを優先して回避条件として反映します。

## x01〜x14

### x01 - Beam Pessimistic
- 狙い
  - `M>=6` の高不確実帯で下振れを抑える。
  - 1手先+短期先読み + 悲観的評価で `mean` を上げる。
- 結果
  - `seed 0..99`（再計測）: `mean 151,174.2`, `median 127,334`, `min 50,617`, `max 605,548`, `elapsed 14,505ms`（`T-063`）
- 状態
  - 参照値として継続利用。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X01_Beam_Pessimistic.md`
  - `docs/AHC061_Experiment_Log_2026-02.md` の `T-063`

### x02 - Monte Carlo Explore
- 狙い
  - `M=3..5` 帯で AI 行動サンプルを使い、期待値と分散を安定化。
- 結果
  - `seed 0..99`（探索版）: `mean 137,293.5`, `median 128,650`, `min 46,987`, `max 311,449`, `elapsed 8,033ms`（`T-063`）
  - `seed 0..99`（リスク重み調整版）: `mean 152,209.1` → `147,982.3`（不採用）
- 状態
  - 採用基準を維持する比較器としては有効、派生運用は不採用。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X02_MonteCarloExplore.md`
  - `docs/AHC061_Experiment_Log_2026-02.md` の `T-063`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `T-061`, `T-062`

### x03 - Particle + CVaR
- 狙い
  - `CVaR` と粒子サンプリングで tail-risk を抑制し、帯別の下振れを抑える。
- 結果
  - quick `0..19`: `mean 148,606.6`, `median 123,776`, `min 70,744`, `max 388,857`, `elapsed 2,142ms`
  - full `0..99`: `mean 154,876.7`, `median 129,861`, `min 44,717`, `max 605,548`, `elapsed 10,539ms`
  - `x06`（現champion）比較: `mean 155,863.2 -> 154,876.7`、`median 133,042.5 -> 129,861`
- 状態
  - 不採用。
  - 失敗要因: `mean` と `median` の悪化、`elapsed` 増加。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X03_Particle_CVaR.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `T-067`

### x04 - Macro Route
- 狙い
  - `M=4` 帯の中期経路最適化を強化し、平均値を押し上げる。
- 結果
  - quick `0..19`（phase切り替えスイープ）:
    - `cutoff=0.65`: `mean 150,024.2`, `median 120,483.5`, `min 70,744`
    - `cutoff=0.75`: `mean 149,810.2`, `median 120,062.5`, `min 70,744`
  - full `0..99`:
    - `cutoff=0.75`: `mean 158,549.1`, `median 138,335.5`, `min 52,543`, `max 605,548`, `elapsed 40,144ms`
    - 軽量化: `mean 158,923.8`, `median 138,335.5`, `min 52,543`, `max 605,548`, `elapsed 31,035ms`
- 状態
  - 採用。
  - 採用要因: `x06 -> x04` へ改善。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X04_MacroRoute.md`
  - `docs/AHC061_Experiment_Log_2026-02.md` の `T-069`, `T-070`

### x05 - Adaptive Racing MC
- 狙い
  - 固定サンプルMCを淘汰付きで制御し、ノイズを抑えて速度制約を満たす。
- 結果
  - quick `0..19`: `mean 138,876.4`（初版） / `134,894.6`（改良版）
  - full `0..99`: `mean 147,673.9`（初版） / `148,149.3`（改良版）
- 状態
  - 不採用。
  - 失敗要因: `adaptive-racing-mc`, `risk-under-tuned`, `tail-risk-regression`。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X05_AdaptiveRacingMC.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `T-064`, `T-065`

### x06 - Expert Switch Hybrid
- 狙い
  - `x02`（低人数帯）と `x01`（高人数帯）を phase/uncertainty で切替し、帯別の底上げを狙う。
- 結果
  - quick `0..19`: `mean 146,623.6`, `median 119,019.5`, `min 70,744`, `max 388,857`, `elapsed 2,027ms`
  - full `0..99`: `mean 155,863.2`, `median 133,042.5`, `min 51,023`, `max 605,548`, `elapsed 10,158ms`
- 状態
  - 採用（現champion）。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X06_ExpertSwitchHybrid.md`
  - `docs/AHC061_Experiment_Log_2026-02.md` の `T-066`

### x07 - Dual Horizon Route
- 狙い
  - 1 horizon 制約を崩し、短期と中期候補を併用して局面分岐を補う。
- 結果
  - quick `0..19`: `mean 137,783.7`, `median 114,450`, `min 70,744`, `max 388,857`, `elapsed 2,198ms`
  - full `0..99`: `mean 155,182.1`, `median 130,877`, `min 52,543`, `max 605,548`, `elapsed 10,834ms`
- 状態
  - 不採用。
  - 失敗要因: `mean/median/min` が baseline を下回る。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X07_DualHorizonRoute.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `T-071`

### x08 - Pressure Frontier
- 狙い
  - `frontier_pressure` を強めて収束速度を優先し、軽量化を狙う。
- 結果
  - quick `0..19`: `mean 125,655.7`, `median 113,953.5`, `min 48,933`, `max 388,857`, `elapsed 1,701ms`
- 状態
  - 不採用（quick で悪化）。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X08_PressureFrontier.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `T-072`

### x09 - Regret Mix
- 狙い
  - `x04`/`x06`/`x02` の提案手を反実仮想で混合し、過激偏向を抑制。
- 結果
  - quick `0..19`: `mean 125,930.3`, `median 107,568.5`, `min 51,051`, `max 456,591`, `elapsed 8,975ms`
- 状態
  - 不採用（quickで平均・中央値・maxの悪化）。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X09_RegretMix.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `T-073`

### x10 - Phase Adaptive Mix
- 狙い
  - phase / uncertainty / 競合率を使って `x04` の代替を検討し、帯別の期待値を改善する。
- 結果
  - quick `0..19`: `mean 130,815`, `median 117,134.5`, `min 58,992`, `max 362,287`, `elapsed 9,299ms`
  - full `0..99`: `mean 137,863.4`, `median 124,831`, `min 44,177`, `max 409,094`, `elapsed 35,931ms`
- 状態
  - 不採用（fullで平均・中央値・minが低下）。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X10_PhaseAdaptiveMix.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `T-074`

### x11 - Contest Frontier Recovery
- 狙い
  - 中盤以降の前線回収を強め、停滞帯の回復率を上げる。
- 結果
  - quick `0..19`: `mean 120,218.6`, `median 117,153`, `min 62,164`, `max 195,538`, `elapsed 18,103ms`
- 状態
  - 不採用。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X11_ContestFrontierRecovery.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `T-075`

### x12 - Advisor Vote Ensemble
- 狙い
  - advisor の提案票を phase/uncertainty で重み付けし、決定方針を安定化する。
- 結果
  - quick `0..19`: `mean 125,138.6`, `median 120,293.5`, `min 30,819`, `max 314,524`, `elapsed 50,041ms`
- 状態
  - 不採用（平均劣化が大きい）。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X12_AdvisorVoteEnsemble.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `T-076`

### x13 - Frontier Consensus
- 狙い
  - frontier 候補の整合性を上げ、極端な候補選択を抑える。
- 結果
  - quick `0..19`: `mean 122,625.4`, `median 111,184.5`, `min 55,341`, `max 346,844`, `elapsed 11,362ms`
- 状態
  - 不採用（quickで平均・中央値低下）。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X13_FrontierConsensus.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `T-077`

### x14 - Adaptive Risk Lane
- 狙い
  - レーンを動的切替し、`mean/median/min` を同時改善。
- 結果
  - quick `0..19`: `mean 133,809.5`, `median 120,847.5`, `min 41,882`, `max 279,763`, `elapsed 17,445ms`
  - full `0..99`: `mean 128,359.5`, `median 113,709.5`, `min 39,592`, `max 311,620`, `elapsed 58,889ms`
- 状態
  - 不採用（fullで平均・中央値低下）。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X14_AdaptiveRiskLane.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `T-078`

## x15〜x17（現ループで実施済み・未採用）
- これらは `x15~x17` として実装済みだが、未だ `built` 詳細Docsへ移管していない。
- `x15~x17` は再実装を含む比較対象であり、現時点では `採用保留`。

### x15 - Band Adaptive Route
- 狙い
  - `M=4/5` 帯の中間復元経路を強化し、局面の再利用可能性を上げる。
- 結果（試行メモベース）
  - quick `0..19`: `mean 131,231.6`, `median 109,662`, `min 62,037`, `max 388,857`, `elapsed 20,347ms`
  - full `0..99`: `mean 148,589.2`, `median 127,141`, `min 55,364`, `max 605,548`, `elapsed 101,736ms`
- 状態
  - 不採用。
  - 主な失敗要因: `mean`・`median` の優位が small、`elapsed` が長い。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X15_BandAdaptiveRoute_Plan.md`

### x16 - Safe Recovery Route
- 狙い
  - 回収失敗時の回復局面を安全側に寄せ、再発を抑制する。
- 結果（試行メモベース）
  - quick `0..19`: `mean 130,752.75`, `median 114,972`, `min 73,333`, `max 388,857`, `elapsed 23,310ms`
- 状態
  - 不採用。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X16_SafeRecoveryRoute_Plan.md`

### x17 - Mid-Band Dual Lane
- 狙い
  - `M=4..6` を対象に2レーン構造で局面多様性を確保する。
- 結果（試行メモベース）
  - quick `0..19`: `mean 117,849.9`, `median 102,649`, `min 53,705`, `max 388,857`, `elapsed 24,264ms`
- 状態
  - 不採用。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X17_MidBandDualLane_Plan.md`

### x18 - Robust Minmax Guard
- 狙い
  - `M=4..6` 帯で相手行動の不確実性に対して最悪寄りの制御を置く。
  - `x01` と `x06` の決定の安定性を保ちつつ、tail-risk の悪化を抑える。
- 結果（seed 0..19 quick）
  - `x04` 比較: `mean 147,335.4`, `median 119,019.5`, `min 70,744`, `max 388,857`, `elapsed 9,173ms`
  - `x18`: `mean 120,893.8`, `median 103,413`, `min 64,002`, `max 388,857`, `elapsed 1,606ms`
- 状態
  - 不採用（quickで劣後し、full未実施）。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X18_RobustMinmaxGuard.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `T-079`

### x19 - Frontier Recovery Sweep
- 狙い
  - frontier 回収候補を2層再評価し、局面変動が大きい時の回復率を上げる。
  - `frontier` 近傍での選択を補完し、停滞状態からの立て直しを狙う。
- 結果（seed 0..19 quick）
  - `x04` 比較: `mean 147,335.4`, `median 119,019.5`, `min 70,744`, `max 388,857`, `elapsed 9,173ms`
  - `x19`: `mean 143,191.6`, `median 119,019.5`, `min 70,370`, `max 388,857`, `elapsed 11,518ms`
- 状態
  - 不採用（quick平均は現状に対して不利、full未実施）。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X19_FrontierRecoverySweep.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `T-080`

### x20 - Band Stage Ensemble
- 狙い
  - `phase/uncertainty/conflict` を軸に `x01/x04/x06` 系を条件付き混合する。
  - `M=4..6` を中心に、局面ごとに重みを切り替えて過激化を抑える。
- 結果（seed 0..19 quick）
  - `x04` 比較: `mean 147,335.4`, `median 119,019.5`, `min 70,744`, `max 388,857`, `elapsed 9,173ms`
  - `x20`: `mean 125,026.7`, `median 117,420`, `min 49,314`, `max 388,857`, `elapsed 9,802ms`
- 状態
  - 不採用（quickで低下し、full未実施）。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X20_BandStageEnsemble.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `T-081`

### x21 - Band Stage Adaptive Guard
- 狙い
  - 高不確実かつ衝突率の高い局面で `x20` と `x18` の保守比率を可変化し、tail-risk を抑えつつ早期退却を防ぐ。
- 結果（seed 0..19 quick）
  - `x04`: `mean 147,335.4`, `median 119,019.5`, `min 70,744`, `max 388,857`, `elapsed 9,173ms`
  - `x21`: `mean 125,280.8`, `median 112,317`, `min 56,641`, `max 388,857`, `elapsed 17,057ms`
- 状態
  - 不採用（quick 指標が現champion `x04` より大幅低下）。
  - full 進行条件を満たさず `full` 未実施。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X21_BandStageAdaptiveGuard.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `F-62`

### x22 - Band Stage Recovery Boost
- 狙い
  - 回復圧力が高い帯で recovery boost を適用し、停滞時の復帰率を上げる。
- 結果（seed 0..19 quick）
  - `x04`: `mean 147,335.4`, `median 119,019.5`, `min 70,744`, `max 388,857`, `elapsed 9,173ms`
  - `x22`: `mean 121,732.5`, `median 111,857.5`, `min 41,914`, `max 388,857`, `elapsed 18,785ms`
- 状態
  - 不採用（quick 指標が大幅低下）。
  - full 進行条件を満たさず `full` 未実施。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X22_BandStageRecoveryBoost.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `F-63`

### x23 - Band Stage Frontier Guard
- 狙い
  - frontier 移動時の過剰分岐を抑え、後半の崩れを防ぐ保守ガードを追加。
- 結果（seed 0..19 quick）
  - `x04`: `mean 147,335.4`, `median 119,019.5`, `min 70,744`, `max 388,857`, `elapsed 9,173ms`
  - `x23`: `mean 89,220.6`, `median 100,309`, `min 2,029`, `max 388,857`, `elapsed 8,742ms`
- 状態
  - 不採用（quick 指標が大幅低下）。
  - full 進行条件を満たさず `full` 未実施。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X23_BandStageFrontierGuard.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `F-64`

### x24 - Band Stage Adaptive Switch
- 狙い
  - `M=4..6` の band-stage で `x04`/`x06`/`x18` を状況に応じて切替し、過剰進行と過剰保守の両方を抑える。
- 結果
  - `x04` quick `0..19`: `mean 147,335.4`, `median 119,019.5`, `min 70,744`, `max 388,857`, `elapsed 9,173ms`
  - `x24` quick `0..19`: `mean 120,302.4`, `median 109,302`, `min 35,245`, `max 388,857`, `elapsed 14,489ms`
- 状態
  - 不採用。
  - quick 上位競争条件を満たさず、full 進行しませんでした。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X24_BandStageAdaptiveSwitch.md`
  - `docs/AHC061_Experiment_Log_2026-02.md` の `T-083`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `F-65`

### x25 - Race Adaptive Recovery
- 狙い
  - 競合圧力に応じて回復重視/進攻重視を2ステップで切替え、落差拡大局面の立て直しを速くする。
- 結果
  - `x04` quick `0..19`: `mean 147,335.4`, `median 119,019.5`, `min 70,744`, `max 388,857`, `elapsed 9,173ms`
  - `x25` quick `0..19`: `mean 122,159.2`, `median 106,804`, `min 41,914`, `max 388,857`, `elapsed 46,568ms`
- 状態
  - 不採用。
  - quick 上位競争条件を満たさず、full 進行しませんでした。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X25_RaceAdaptiveRecovery.md`
  - `docs/AHC061_Experiment_Log_2026-02.md` の `T-083`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `F-66`

### x26 - Reactive Frontier Pressure
- 狙い
  - frontier と衝突密度を用いたリアルタイム圧力評価で、危険局面は保守寄り、優位局面は回復・侵攻へ再配分する。
- 結果
  - `x04` quick `0..19`: `mean 147,335.4`, `median 119,019.5`, `min 70,744`, `max 388,857`, `elapsed 9,173ms`
  - `x26` quick `0..19`: `mean 125,585.2`, `median 116,424.5`, `min 37,224`, `max 388,857`, `elapsed 34,466ms`
- 状態
  - 不採用。
  - quick 上位競争条件を満たさず、full 進行しませんでした。
- 参照
  - `docs/solver_specs_built/AHC061_Solver_X26_ReactiveFrontierPressure.md`
  - `docs/AHC061_Experiment_Log_2026-02.md` の `T-083`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `F-67`

### x27 - Macro Route Adaptive Cutoff
- 狙い
  - `M=4` 中核の `x04` を維持しつつ、`phase` 遷移点を再調整して中盤〜後半の追従力を上げる。
- 結果
  - `x04` quick `0..19`: `mean 148,911`, `median 126,813`, `min 70,744`, `max 388,857`
  - `x27` quick `0..19`: `mean 149,217.6`, `median 118,060`, `min 70,744`, `max 388,857`, `elapsed 34,500ms`
  - `x27` full `0..99`: `mean 157,322.6`, `median 135,590`, `min 52,543`, `max 605,548`, `elapsed 139,336ms`
- 状態
  - 再検証中（quick/fullともに baseline 超過だが、再現性確認が必要）。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X27_MacroRouteAdaptiveCutoff_Plan.md`
  - `docs/AHC061_Experiment_Log_2026-02.md` の `T-086`

### x29 - Macro Route Aggressive Frontier
- 狙い
  - 前線探索時の aggressive frontier 条件を強め、収束速度と末端得点を改善する。
- 結果
  - `x04` quick `0..19`: `mean 148,911`, `median 126,813`, `min 70,744`, `max 388,857`
  - `x29` quick `0..19`: `mean 150,490.4`, `median 124,919.5`, `min 70,744`, `max 388,857`, `elapsed 51,634ms`
  - `x29` full `0..99`: `mean 157,418.4`, `median 138,098`, `min 52,543`, `max 605,548`, `elapsed 194,369ms`
- 状態
  - 再検証中（baseline 上回りだが、`x42` と比較して優位条件を詰める必要あり）。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X29_MacroRouteAggressiveFrontier_Plan.md`
  - `docs/AHC061_Experiment_Log_2026-02.md` の `T-086`

### x30 - Macro Route Path Focus
- 狙い
  - path 長に対する重み配分を再設計し、frontier 追従と安定性のバランスを改善する。
- 結果
  - `x04` quick `0..19`: `mean 148,911`, `median 126,813`, `min 70,744`, `max 388,857`
  - `x30` quick `0..19`: `mean 141,281.2`, `median 114,450`, `min 70,744`, `max 388,857`, `elapsed 55,634ms`
- 状態
  - 不採用（quick 指標で baseline 下回り）。
  - `full` 未実施。
- 参照
  - `docs/solver_specs_built/`（未移管）
  - `docs/solver_specs_planned/AHC061_Solver_X30_MacroRoutePathFocus_Plan.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `F-75`

### x31 - Macro Route Local Bias
- 狙い
  - 局所偏りを強めることで、短期最適な圧力回避と復旧を改善する。
- 結果
  - `x04` quick `0..19`: `mean 148,911`, `median 126,813`, `min 70,744`, `max 388,857`
  - `x31` quick `0..19`: `mean 138,610`, `median 115,075`, `min 70,744`, `max 388,857`, `elapsed 21,380ms`
- 状態
  - 不採用（quick 指標で baseline 下回り）。
  - `full` 未実施。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X31_MacroRouteLocalBias_Plan.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `F-76`

### x32 - Macro Route Wide Frontier
- 狙い
  - `frontier` 幅を広げることで局所障害回避と回収率の底上げを狙う。
- 結果
  - `x04` quick `0..19`: `mean 148,911`, `median 126,813`, `min 70,744`, `max 388,857`
  - `x32` quick `0..19`: `mean 144,994.8`, `median 119,019.5`, `min 70,744`, `max 388,857`, `elapsed 45,605ms`
- 状態
  - 不採用（quick 指標で baseline 下回り）。
  - `full` 未実施。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X32_MacroRouteWideFrontier_Plan.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `F-77`

### x33 - Macro Route Balanced Focus
- 狙い
  - 攻守配分を同時に最適化して、frontier と中期回収の緩衝特性を調整する。
- 結果
  - `x04` quick `0..19`: `mean 148,911`, `median 126,813`, `min 70,744`, `max 388,857`
  - `x33` quick `0..19`: `mean 142,361.6`, `median 114,673`, `min 70,744`, `max 388,857`, `elapsed 47,570ms`
- 状態
  - 不採用（quick 指標で baseline 下回り）。
  - `full` 未実施。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X33_MacroRouteBalancedFocus_Plan.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `F-78`

### x38 - Macro Route Pressure Protect
- 狙い
  - Macro Route の先読みを圧力重み付きで強化し、過密セルの衝突リスクを抑制する。
- 結果
  - `x04` quick `0..19`: `mean 148,911`, `median 126,813`, `min 70,744`
  - `x38` quick `0..19`: `mean 141306.4`, `median 119019.5`, `min 70744`, `elapsed 43618ms`
- 状態
  - 不採用（quickで現状劣後）。  
  - `x42` が同系の圧力軸を回復したため、再設計を保留。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X38_MacroRoutePressureProtect_Plan.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `F-68`

### x39 - Macro Route Pressure Attack
- 狙い
  - 圧力を攻撃寄りに解釈し、前半で積極的な境界回復を狙う方向へ寄与度を調整。
- 結果
  - `x04` quick `0..19`: `mean 148,911`, `median 126,813`, `min 70,744`
  - `x39` quick `0..19`: `mean 143108.1`, `median 117781`, `min 70744`, `elapsed 47629ms`
- 状態
  - 不採用（quickで現状劣後）。  
  - `full` 未実施。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X39_MacroRoutePressureAttack_Plan.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `F-69`

### x40 - Macro Route Pressure Balanced
- 狙い
  - 中盤以降の圧力と先行評価のバランスを取り、進退の安定化を検証する。
- 結果
  - `x04` quick `0..19`: `mean 148,911`, `median 126,813`, `min 70,744`
  - `x40` quick `0..19`: `mean 138915.8`, `median 114450`, `min 70744`, `elapsed 46571ms`
- 状態
  - 不採用（quickで現状劣後）。  
  - `full` 未実施。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X40_MacroRoutePressureBalanced_Plan.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `F-70`

### x41 - Macro Route Pressure Aggressive Target
- 狙い
  - ターゲット選定段階で衝突圧力を高く課し、侵攻候補を厳選する。
- 結果
  - `x04` quick `0..19`: `mean 148,911`, `median 126,813`, `min 70,744`
  - `x41` quick `0..19`: `mean 147106`, `median 114727.5`, `min 70744`, `elapsed 51711ms`
  - `x41` full `0..99`: `mean 156054.6`, `median 133042.5`, `min 52543`, `elapsed 187199ms`
- 状態
  - 検証中（quickは近接、fullはbaselineを+0.65%上回り）。  
  - 追加再検証（安定性/テールリスク）を保留。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X41_MacroRoutePressureAggressiveTarget_Plan.md`
  - `docs/AHC061_Experiment_Log_2026-02.md` の `T-084`

### x42 - Macro Route Pressure Frontier Shield
- 狙い
  - フロンティア制御を圧力シグナルで安定化し、前半・後半の選択強度を分離する。
- 結果
  - `x04` quick `0..19`: `mean 148,911`, `median 126,813`, `min 70,744`
  - `x42` quick `0..19`: `mean 149359.2`, `median 119201.5`, `min 70744`, `elapsed 41535ms`
  - `x42` full `0..99`: `mean 157756.1`, `median 135590`, `min 52543`, `elapsed 176347ms`
- 状態
  - 検証中（quick・fullともに既存を上回る方向の改善）。  
  - 次ループで再現seed/時間安定性を再確認し、採用可否を判断。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X42_MacroRoutePressureFrontierShield_Plan.md`
  - `docs/AHC061_Experiment_Log_2026-02.md` の `T-084`

### x43 - Macro Route Pressure Adaptive Lane
- 狙い
  - phase依存の圧力重みでレーン別に攻撃・防御の配分を変える。
- 結果
  - `x04` quick `0..19`: `mean 147335.4`, `median 119019.5`, `min 70744`
  - `x43` quick `0..19`: `mean 144122`, `median 115690.5`, `min 70744`, `elapsed 53506ms`
- 状態
  - 不採用（quickでbaselineを下回る）。  
  - `full` 未実施。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X43_MacroRoutePressureAdaptiveLane_Plan.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `F-71`

### x44 - Macro Route Pressure Dual Window
- 狙い
  - 前半/後半の探索バランスを分離しつつ、candidate数とroute長を増加して追い上げを狙う。
- 結果
  - `x04` quick `0..19`: `mean 148,911`, `median 126,813`, `min 70,744`
  - `x44` quick `0..19`: `mean 137051.9`, `median 114,450`, `min 70,744`, `elapsed 57,615ms`
- 状態
  - 不採用（quickでbaselineを大きく下回る）。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X44_MacroRoutePressureDualWindow_Plan.md`
  - `docs/AHC061_Experiment_Log_2026-02.md` の `T-085`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `F-72`

### x45 - Macro Route Pressure Recovery Damp
- 狙い
  - 初動は保守寄り、後半に回復寄せする位相分割で安全域を維持しつつ得点を狙う。
- 結果
  - `x04` quick `0..19`: `mean 148,911`, `median 126,813`, `min 70,744`
  - `x45` quick `0..19`: `mean 141,662.9`, `median 114,450`, `min 70,744`, `elapsed 39,588ms`
- 状態
  - 不採用（quickでbaselineを下回る）。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X45_MacroRoutePressureRecoveryDamp_Plan.md`
  - `docs/AHC061_Experiment_Log_2026-02.md` の `T-085`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `F-73`

### x46 - Macro Route Pressure Hybrid Length
- 狙い
  - 深掘り長さとbeam幅を拡張し、終盤の局面復元性を上げる。
- 結果
  - `x04` quick `0..19`: `mean 148,911`, `median 126,813`, `min 70,744`
  - `x46` quick `0..19`: `mean 138,591.8`, `median 114,450`, `min 70,744`, `elapsed 59,615ms`
- 状態
  - 不採用（quickでbaselineを下回る）。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X46_MacroRoutePressureHybridLength_Plan.md`
  - `docs/AHC061_Experiment_Log_2026-02.md` の `T-085`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `F-74`

### x47 - Macro Route Pressure Dual Guard
- 狙い
  - 2系統 pressure を同時に扱い、末端での回収力を上げる `x04` 進化系。
- 結果
  - quick `0..19`: `mean 152,109.8`, `median 119,020`, `min 70,744`, `max 388,857`, `elapsed 58,669ms`
  - full `0..99`: `mean 158,655.7`, `median 133,042`, `min 52,543`, `max 605,548`, `elapsed 220,241ms`（`T-087`）
- 状態
  - 不採用保留（`median` が改善しないため）。
  - 現在のチャンピオン候補として監視対象。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X47_MacroRoutePressureDualGuard_Plan.md`
  - `docs/AHC061_Experiment_Log_2026-02.md` の `T-087`

### x50 - Macro Route Pressure Dual Guard Focus
- 狙い
  - 終盤寄与を強めるため `late phase` の圧力配分と候補探索量を増やす。
- 結果
  - quick `0..19`: `mean 145,860`, `median 114,450`, `min 63,227`, `max 388,857`, `elapsed 76,922ms`
  - full `0..99`: `mean 158,958.4`, `median 130,877`, `min 52,543`, `max 605,548`, `elapsed 294,761ms`
- 状態
  - 不採用（`median` が `x04`/`x47` を下回る）。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X50_PressureDualGuardFocus_Plan.md`
  - `docs/AHC061_Experiment_Log_2026-02.md`（新規T記録）

### x51 - Macro Route Pressure Dual Guard Recover
- 狙い
  - 回復局面の分岐を増やし、早期過剰圧力の失速を抑える。
- 結果
  - quick `0..19`: `mean 145,748.3`, `median 119,020`, `min 70,744`, `max 388,857`, `elapsed 117,422ms`
  - full `0..99`: `mean 157,298.9`, `median 133,042`, `min 52,543`, `max 605,548`, `elapsed 369,526ms`
- 状態
  - 不採用（`median` は改善せず、`mean` 伸びも限定的）。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X51_PressureDualGuardRecover_Plan.md`
  - `docs/AHC061_Experiment_Log_2026-02.md`（新規T記録）

### x52 - Macro Route Pressure Dual Guard Mid Recover
- 狙い
  - 中盤回復を補強し、`median` 低下を抑えながら `mean` を維持。
- 結果
  - quick `0..19`: `mean 151,762.2`, `median 119,526`, `min 70,744`, `max 388,857`, `elapsed 29,664ms`
  - full `0..99`: `mean 158,578.9`, `median 138,336`, `min 52,543`, `max 605,548`, `elapsed 268,418ms`
- 状態
  - 不採用（全指標で目標差に届かず、最終採用判定まで継続保留）。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X52_MidRecoverPressure_Plan.md`
  - `docs/AHC061_Experiment_Log_2026-02.md`（新規T記録）

### x53 - Macro Route Pressure Dual Guard Mid Boost
- 狙い
  - x52 を起点に `route_pressure` 強化を寄せ、`median` と mean の両立を再試行。
- 結果
  - quick `0..19`: `mean 149,277.8`, `median 119,020`, `min 70,744`, `max 388,857`, `elapsed 71,788ms`
  - full `0..99`: `mean 157,278.8`, `median 135,590`, `min 52,543`, `max 605,548`, `elapsed 327,241ms`
- 状態
  - 不採用（`mean` 改善はあるが `x47/x52` に及ばない）。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X53_MidBoostPressure_Plan.md`
  - `docs/AHC061_Experiment_Log_2026-02.md`（新規T記録）

### x54 - Macro Route Pressure Late Boost
- 狙い
  - `pressure_weight_late` を増加し終盤巻き返し性能を重点的に再検証。
- 結果
  - quick `0..19`: `mean 140,447`, `median 119,020`, `min 70,744`, `max 388,857`, `elapsed 76,881ms`
  - full `0..99`: なし（`full` 選抜基準未達）
- 状態
  - 不採用（quick で大きく劣後）。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X54_LatePressureBoost_Plan.md`
  - `docs/AHC061_Experiment_Failures_2026-02.md`（新規F記録）

### x55 - Macro Route Pressure Recovery Branch Boost
- 狙い
  - 回復局面の分岐幅を広げ、`median` 安定化を狙う。
- 結果
  - quick `0..19`: `mean 146,700.8`, `median 125,460`, `min 70,744`, `max 388,857`, `elapsed 154,943ms`
  - full `0..99`（fallback で進出）: `mean 155,710.3`, `median 131,014`, `min 52,543`, `max 605,548`, `elapsed 502,069ms`
- 状態
  - 不採用（`mean` が `x47` を下回る、`median` も悪化）。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X55_RecoveryBranchBoost_Plan.md`
  - `docs/AHC061_Experiment_Log_2026-02.md`（新規T記録）

### x56 - Macro Route Pressure Mid Recover Stabilized
- 狙い
  - `x52` の `median` 改善を維持しつつ、`late` 圧力の過剰化を抑える。
- 結果
  - quick `0..19`: `mean 151,762.2`, `median 119,526`, `min 70,744`, `max 388,857`, `elapsed 29,664ms`
  - full `0..99`: `mean 158,578.9`, `median 138,336`, `min 52,543`, `max 605,548`, `elapsed 263,581ms`
- 状態
  - 不採用（`x52` と結果が同等で増分が薄く、重複候補寄り）。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X56_MidRecoverStabilized_Plan.md`
  - `docs/AHC061_Experiment_Log_2026-02.md` の `T-089`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `F-83`

### x57 - Macro Route Pressure Focus Recover Blend
- 狙い
  - `x50` と `x52` の中間戦略を狙い、`focus` と `回復` の混合効果を検証。
- 結果
  - quick `0..19`: `mean 145,672.7`, `median 116,886`, `min 70,744`, `max 388,857`, `elapsed 89,711ms`
  - full `0..99`: `mean 155,947.6`, `median 129,487`, `min 52,543`, `max 605,548`, `elapsed 283,250ms`
- 状態
  - 不採用（`mean/median` が baseline 及び `x52` より低い）。
- 参照
  - `docs/solver_specs_planned/AHC061_Solver_X57_FocusRecoverBlend_Plan.md`
  - `docs/AHC061_Experiment_Log_2026-02.md` の `T-089`
  - `docs/AHC061_Experiment_Failures_2026-02.md` の `F-83`

### x63 - Macro Route Structural Reset
- 狙い
  - `x56~x62` の同系統微調整から離れ、候補多様性・経路重み・圧力位相を同時に再設計して別系統の改善余地を作る。
- 結果
  - quick `0..19`: 未実施
  - full `0..99`: 未実施
- 状態
  - 実装済み・評価待ち（新規設計優先の監視対象）。
- 参照
  - `solver/src/bin/x63_macro_route_structural_reset.rs`
  - `docs/solver_specs_planned/AHC061_Solver_X63_StructuralReset_Plan.md`
  - `docs/AHC061_Experiment_Log_2026-02.md` の `T-090`

### x64 - Portfolio Mixer Base
- 狙い
  - 複数戦略（`x01/x04/x06/x10/x11/x13/x14/x19/x26`）を重み付きで統合し、単一系統依存を解消する。
- 結果
  - quick `0..19`: 未実施
  - full `0..99`: 未実施
- 状態
  - 実装済み・評価待ち。
- 参照
  - `solver/src/x64_portfolio_mixer.rs`
  - `solver/src/bin/x64_portfolio_mixer.rs`
  - `docs/solver_specs_planned/AHC061_Solver_X64_PortfolioMixerBase_Plan.md`

### x65 - Portfolio Mixer Aggro
- 狙い
  - `x64` の攻撃型派生として `gap` 追従時の上振れ期待を高める。
- 結果
  - quick `0..19`: 未実施
  - full `0..99`: 未実施
- 状態
  - 実装済み・評価待ち。
- 参照
  - `solver/src/bin/x65_portfolio_mixer_aggro.rs`
  - `docs/solver_specs_planned/AHC061_Solver_X65_PortfolioMixerAggro_Plan.md`

### x66 - Portfolio Mixer Guard
- 狙い
  - `x64` の防御型派生として `median/min` の保全を重視する。
- 結果
  - quick `0..19`: 未実施
  - full `0..99`: 未実施
- 状態
  - 実装済み・評価待ち。
- 参照
  - `solver/src/bin/x66_portfolio_mixer_guard.rs`
  - `docs/solver_specs_planned/AHC061_Solver_X66_PortfolioMixerGuard_Plan.md`

### x67 - Gear Shift Hybrid
- 狙い
  - 非Macro系 advisor と複数シナリオ先読みを合成し、Gear-Shift用の新規アーキテクチャを検証する。
- 結果
  - quick `0..9`: `mean 112,200.8`, `median 102,260`, `min 51,816`, `max 235,957`
- 状態
  - 不採用（quick で baseline を大きく下回る）。
- 参照
  - `solver/src/x67_gear_shift_hybrid.rs`
  - `solver/src/bin/x67_gear_shift_hybrid.rs`

### x58 - Macro Route Pressure Late Recover Balance
- 狙い
  - `x56` 系の median 改善を維持しつつ、late recover の平均値伸長を狙う。
- 結果
  - quick `0..19`: `mean 151,294.4`, `median 119,020`, `min 70,744`, `max 388,857`
  - full `0..99`: `mean 158,960.4`, `median 135,590`, `min 52,543`, `max 605,548`, `elapsed 263,257ms`
  - baseline `x04_full` 比: `mean +2.52%`, `median +1.92%`, `min +0.00%`
- 状態
  - 現在の平均最良候補（提出候補）。
- 参照
  - `solver/src/bin/x58_macro_route_pressure_late_recover_balance.rs`
  - `submissions/submission_x58.rs`

### x75 - Risk Gated Unlocked Macro
- 狙い
  - 帯域と局面で unlocked macro の発火を制御し、`min` 保全と `mean` 改善の両立を狙う。
- 結果
  - quick `0..19`: `mean 155,464.9`, `median 123,984`, `min 92,456`, `max 388,857`
  - full `0..99`: `mean 157,951.5`, `median 134,732`, `min 56,283`, `max 605,548`, `elapsed 602,232ms`
  - baseline `x04_full` 比: `mean +1.87%`, `median +1.27%`, `min +7.12%`
- 状態
  - 不採用（`x58` の mean を上回れない）。
- 参照
  - `solver/src/x75_risk_gated_unlocked_macro.rs`
  - `solver/src/bin/x75_risk_gated_unlocked_macro.rs`

### x76 - Crossband Route Hybrid
- 狙い
  - `M` 帯別に unlocked/locked macro を切り替え、`x75` の上振れのみを抽出する。
- 結果
  - quick `0..19`: `mean 155,269.6`, `median 123,984`, `min 88,551`, `max 388,857`
  - full `0..99`: `mean 158,022.9`, `median 134,732`, `min 56,283`, `max 605,548`, `elapsed 595,368ms`
  - baseline `x04_full` 比: `mean +1.92%`, `median +1.27%`, `min +7.12%`
- 状態
  - 不採用（`x58` 比で `mean -0.59%`）。
- 参照
  - `solver/src/x76_crossband_route_hybrid.rs`
  - `solver/src/bin/x76_crossband_route_hybrid.rs`

## メタ運用（再利用指針）
- 狙い:
  - 同じ失敗を繰り返さないため、xNNの設計意図を固定形式で保持する。
- 運用ルール:
  - 新規xNNの実装前に、本indexで狙い語（`minmax`, `frontier`, `ensemble` 等）を検索し、既存軸の重複を避ける。
  - quick で有望でも、`quick` 進出条件を満たす候補だけを full 対象にする（`mean/median/min` は品質条件、`novelty_score` は新規性条件）。
  - 競争上位が複数有望な場合は、同一ループで複数候補を full へ進める。
  - Depth（Quality）/Breadth（Novelty）の二軸を分離し、`selection_score = mode_weight * quality_score + (1-mode_weight) * novelty_score` で順序を決める。
  - 現在は品質寄りの実験比率を優先し、`mode_weight` を `0.85`（QualityFirst）から開始する。
  - 不採用時は `docs/AHC061_Experiment_Failures_2026-02.md` に `背景/変更/結果/比較/判定/タグ` を即時記録する。
- 反映先:
  - `docs/solver_specs_planned` へ新規仮説を先行登録し、実装後に `built` へ移管する。
  - `docs/AHC061_Codex_Guide.md` の quick/full ルールと整合を保つ。

## 次アクション
- 戦略立案前に本 index を確認し、`狙いが近いxNN` と比較しない試行を避ける。
- 未採用系統の `失敗要因` は `docs/AHC061_Experiment_Failures_2026-02.md` に再現性を付与して残す。

