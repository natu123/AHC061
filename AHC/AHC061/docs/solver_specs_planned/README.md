# 作成予定ソルバ詳細

## 目的
- 未実装の有力戦略を事前仕様として明文化し、設計のブレと重複検討を防ぐ。
- 新規セッション開始時のコンテキスト欠落を防ぎ、探索精度を上げる。

## 対象
- `x03`: Particle相手モデル + CVaR最適化（実装済み・非採用、builtへ移管）
- `x04`: マクロ行動計画（複数ターン経路最適化）（実装済み・採用、builtへ移管）
- `x05`: Adaptive Racing Monte Carlo（逐次サンプル配分 + 候補淘汰）（計画）
- `x06`: Expert Switch Hybrid（x01/x02切替メタ方策）（計画）
- `x07`: Dual Horizon Route Blend（短期/中期の二重地平線探索）
- `x08`: Pressure Frontier Control（前線圧力主導の局所制御）
- `x09`: Regret Mix Policy（複数expertの反実仮想混合）
- `x10`: Phase Adaptive Mix（フェーズ別に expert を切替）
- `x11`: Contest Frontier Recovery（対立前線回収重視）
- `x12`: Advisor Vote Ensemble（advisor投票重み付き統合）

## 仕様
- 各計画は `狙い / 別系統性 / 期待効果 / 実装スコープ / 検証計画` を必須記載とする。
- 実装に着手した時点で、計画ファイルへ `着手日` と `最初のブランチ/コミット` を追記する。

## 一覧
- `AHC061_Solver_X03_Particle_CVaR_Plan.md`
- `ARCHIVED_AHC061_Solver_X04_MacroRoute_Plan.md`
- `ARCHIVED_AHC061_Solver_X05_AdaptiveRacingMC_Plan.md`
- `AHC061_Solver_X06_ExpertSwitchHybrid_Plan.md`
- `AHC061_Solver_X07_DualHorizonRoute_Plan.md`
- `AHC061_Solver_X08_PressureFrontier_Plan.md`
- `AHC061_Solver_X09_RegretMix_Plan.md`
- `AHC061_Solver_X10_PhaseAdaptiveMix_Plan.md`
- `AHC061_Solver_X11_ContestFrontierRecovery_Plan.md`
- `AHC061_Solver_X12_AdvisorVoteEnsemble_Plan.md`

## 注意点
- 計画段階のため、採用主張は書かない。
- 実験後は採用/不採用の判断を実験ログへ反映し、必要なら本計画を更新する。

## 更新ルール
- 新規案を追加する場合は `xNN` を採番して同フォーマットで追加する。
- 廃案化した計画も削除せず、廃案理由を追記して残す。
