# AHC061 - AGENTS Instructions

## Inheritance
- AtCoder共通ルールは `../../AGENTS.md` を正本として継承する
- AHC共通ルールは `../AGENTS.md` を正本として継承する
- このファイルは AHC061 固有事項のみを定義する
- 共通ルールと矛盾する場合は、AHC061 の実行要件に必要な範囲でこのファイルを優先する
- AtCoder LLM利用ルールの準拠は `../AGENTS.md` の `LLM Compliance` を適用する

## Source Of Truth
- AHC061 固有の運用方針はこのファイルを正本（single source of truth）とする
- 公式配布ツールの仕様は `N52XwIfp_windows/tools_x86_64-pc-windows-gnu/README.md` を補助正本とする
- ルール変更はまず該当スコープの `AGENTS.md` を更新し、必要な差分のみ他ファイルに反映する

## AHC061 Domain
- AHC061 問題特性に合わせ、制約順守とスコア最適化を同時に満たす解を設計する
- 単発seedではなく seed 群での安定性を重視して採用判断する

## Tech Stack
- Solver: C++ / Rust / Python（ユーザー採用言語を優先）
- Official Tools: `gen.exe`, `tester.exe`, `vis.exe`
- Input corpus: `N52XwIfp_windows/tools_x86_64-pc-windows-gnu/in`
- Seed list: `N52XwIfp_windows/tools_x86_64-pc-windows-gnu/seeds.txt`

## Key Files
- `N52XwIfp_windows/tools_x86_64-pc-windows-gnu/README.md` - 公式ローカル実行手順
- `N52XwIfp_windows/tools_x86_64-pc-windows-gnu/tester.exe` - ローカルスコア計測
- `N52XwIfp_windows/tools_x86_64-pc-windows-gnu/gen.exe` - 入力生成
- `N52XwIfp_windows/tools_x86_64-pc-windows-gnu/vis.exe` - 可視化HTML出力
- `N52XwIfp_windows/tools_x86_64-pc-windows-gnu/in/` - seed=0..99 の初期入力群

## AHC061 Protocol
- 評価は `seed=0..99` の既存入力を基本集合とし、必要時のみ `gen.exe` で追加生成する
- 検証では `平均・中央値・最小スコア` と `実行時間` を最低限併記する
- ローカル検証時は時間制限超過を避けるため、ソルバ内に時間ガードを実装する
- 乱数依存バグの切り分けのため、デバッグ時は固定seedモードを必須とする
- 実験スクリプト実行前にワークスペース健全性を検査し、`C:\Users\kenji\projects\AHC\...` のような誤ディレクトリ起点実行を拒否し、`...\\AtCoder\\AHC\\AHC061` での実行を前提とする
- quick/full 選抜は `quality_score/novelty_score` に時間効率補正を加えた `final_score` を採用する
  - `final_score = (1-efficiency_weight) * selection_score + efficiency_weight * efficiency_score`
  - `quality_score`/`novelty_score`/`selection_score` は `tmp_eval_loop10.ps1` と共通化する
- 10試行以上継続する大規模Loopでは `Exploit` と `Explore` の配分を監視する
  - 基本: `Exploit 5% / Explore 95%`
  - Explore 内訳: `new 30% / pair 50% / blend 20%`
- `docs/AHC061_Submission_Gap_Log_YYYY-MM.md` の最新 `ratio = my_score / top1_score` を、探索新規性の配分判断に使う
- `ratio >= 0.98` は微調整主軸（ただし time_budget と `min` 悪化抑制を維持）
- `0.90 <= ratio < 0.98` は混合（微調整 + 新規改良）
- `ratio < 0.90` は抜本改善中心
- `ratio < 0.95` の場合、Explore配分は `新規アーキテクチャ 80% / 既存派生 15% / Exploit監視 5%` を標準配分とする
- `ratio < 0.90`、`top20_ratio < 0.75`（`my_score / top20_score`）、または `10試行平均改善率 < +0.3%` の場合は Explore Gear-Shift を発動し、次の `3` ループで以下を固定運用する
  - 各ループで新規性の高い仮説を最低 `2` 件（推奨 `3` 件）実装して quick 評価（`seed 0..19`）まで実施する
  - quick 上位の競合性を使って full 進出候補を選別する。
    - quick は `mean` 降順で整列し、`seed 0..19` の比較 baseline は現 `champion` quick 指標（`mean / median / min`）を固定する
    - 競争候補条件をすべて満たす場合のみ full へ進める
      - `top1` と `topk` の `mean` 差が `2.5%` 以内（`top1` は quick 1位）
      - `mean` が `champion mean` の `98.5%` 以上
      - `median` が `champion median` の `98.5%` 以上、または `min` が `champion min` の `90%` 以上
    - 有望競争群は最大 `3` 件まで full（`seed 0..99`）へ進める
    - 競争群が空の場合は full を行わず、そのループは不採用候補を終了する
  - 既存方針の微調整のみの試行は全体の `20%` 以下に抑える

## Solver ID / Champion Policy
- `xNN`（例: `x01`, `x02`, `x10`）は「新規ソルバ作成順」の固定IDとして採番し、後から意味を変更しない
- `champion` は採用状態ラベルとしてのみ管理し、`xNN` の意味と分離する
- 提出時は `champion` が指す `xNN` から単一ファイル成果物（例: `submission_x10.rs`）を生成して提出する
- 提出成果物名には必ず `xNN` を含め、実験ログの採用記録と1対1で辿れる状態を維持する

## Local Execution (PowerShell)
- ツール配置ディレクトリ:
  - `cd "N52XwIfp_windows/tools_x86_64-pc-windows-gnu"`
- 既存入力でテスト:
  - `Get-Content in/0000.txt | ./tester.exe <solver command> > out.txt`
- 追加入力生成:
  - `./gen.exe seeds.txt`
- 可視化:
  - `./vis.exe in/0000.txt out.txt`
- `cargo` 版を使う場合:
  - `Get-Content in/0000.txt | cargo run -r --bin tester <solver command> > out.txt`

## Project Documentation
- `../AGENTS.md` - AHC共通ルール
- `AGENTS.md` - AHC061固有ルール
- `docs/AHC061_Codex_Guide.md` - AHC061向けCodex活用ガイド
- `docs/AHC061_Game_Rules_Strict.md` - AHC061ゲームルール厳密仕様（tester実装準拠）
- `docs/AHC061_Initial_Study_2026-02-15.md` - AHC061初期検討プロセスと比較結果
- `docs/AHC061_Experiment_Log_2026-02.md` - AHC061実験ログ（2026-02）
- `docs/AHC061_Experiment_Failures_2026-02.md` - AHC061不採用実験ログ（2026-02）
- `docs/AHC061_Status_2026-02-15-2113.md` - AHC061状況スナップショット（移行用）
- `docs/AHC061_Submission_Gap_Log_2026-02.md` - 提出ごとの時刻/スコア差ログ（2026-02）
- `docs/solver_specs_built/README.md` - 作成済みソルバ（xNN）の戦略詳細インデックス
- `docs/solver_specs_built/AHC061_Solver_X01_Beam_Pessimistic.md` - x01戦略詳細
- `docs/solver_specs_built/AHC061_Solver_X02_MonteCarloExplore.md` - x02戦略詳細
- `docs/solver_specs_built/AHC061_Solver_X03_Particle_CVaR.md` - x03戦略詳細
- `docs/solver_specs_built/AHC061_Solver_X04_MacroRoute.md` - x04戦略詳細
- `docs/solver_specs_built/AHC061_Solver_X05_AdaptiveRacingMC.md` - x05戦略詳細
- `docs/solver_specs_built/AHC061_Solver_X06_ExpertSwitchHybrid.md` - x06戦略詳細
- `docs/solver_specs_built/AHC061_Solver_X07_DualHorizonRoute.md` - x07戦略詳細
- `docs/solver_specs_built/AHC061_Solver_X08_PressureFrontier.md` - x08戦略詳細
- `docs/solver_specs_built/AHC061_Solver_X09_RegretMix.md` - x09戦略詳細
- `docs/solver_specs_planned/README.md` - 計画中ソルバ（xNN）の戦略詳細インデックス
- `docs/solver_specs_planned/AHC061_Solver_X03_Particle_CVaR_Plan.md` - x03計画（Particle + CVaR）
- `docs/solver_specs_planned/ARCHIVED_AHC061_Solver_X04_MacroRoute_Plan.md` - x04計画（移管済み）
- `docs/solver_specs_planned/ARCHIVED_AHC061_Solver_X05_AdaptiveRacingMC_Plan.md` - x05計画（Adaptive Racing MC）
- `docs/solver_specs_planned/AHC061_Solver_X06_ExpertSwitchHybrid_Plan.md` - x06計画（Expert Switch Hybrid）
- `docs/solver_specs_planned/AHC061_Solver_X07_DualHorizonRoute_Plan.md` - x07計画（Dual Horizon Route）
- `docs/solver_specs_planned/AHC061_Solver_X08_PressureFrontier_Plan.md` - x08計画（Pressure Frontier）
- `docs/solver_specs_planned/AHC061_Solver_X09_RegretMix_Plan.md` - x09計画（Regret Mix）
- `N52XwIfp_windows/tools_x86_64-pc-windows-gnu/README.md` - 公式ツール説明
- 必要になった時点で、提出コードや実験メモの該当ファイルを追加参照する

## Docs運用（AHC061）
- Docs運用は `../AGENTS.md` の `Docs Governance` に従う
- AHC061では、運用変更の反映先を必要ファイルに限定し、全md一括更新は行わない
- AHC061では次の責務分離を固定する
  - `docs/AHC061_Codex_Guide.md`: 実行手順・開発運用・Codex利用指針
  - `docs/AHC061_Game_Rules_Strict.md`: tester実装準拠の厳密ルール
  - `docs/AHC061_Initial_Study_2026-02-15.md`: 初期検討結果（固定ログ）
  - `docs/solver_specs_built/`: 作成済みソルバ（xNN）の戦略詳細
  - `docs/solver_specs_planned/`: 計画中ソルバ（xNN）の戦略詳細
  - `docs/solver_specs_built/README.md`: xNN別の「狙い/結果」を即参照できる索引（採否・失敗要因を必須掲載）
- 新しい検証フェーズを始める場合は `docs/AHC061_Experiment_Log_YYYY-MM.md` を新規作成する
- 検証結果を会話で報告した場合、採否判断まで含めて当日中にDocsへ反映する
- 仕様根拠が `README` と実装で食い違う場合、先に `docs/AHC061_Game_Rules_Strict.md` を更新してから他Docsへ反映する
- 文字コードは **UTF-8（BOMなし）** を原則とし、文字化けが見えた時点でファイル全体を再保存してから編集する
- 見出しと項目名は以下に統一する
  - 実験ログ: `背景 / 対象 / 変更 / 実験条件 / 結果 / A/B比較 / 考察 / 次アクション`
  - 仕様書・ガイド: `目的 / 対象 / 仕様（または手順） / 注意点 / 更新ルール`
- 実験の見出し番号は採用/不採用を通した試行通番 `T-xxx` で一元管理する
- 後追い転記が混在する場合も番号体系は分割せず、該当エントリに `retro` 注記を付けて追跡可能性を担保する
- Docs更新時は、保存直後に `Get-Content <file> -Encoding UTF8 -Head 20` で文字化け有無を確認する
- 用語表記は固定する（`seed`, `score`, `ratio`, `mean/median/min/max`, `tail-risk`）
- 実験サイクルの確認ロスを減らすため、次を標準運用にする
  - `checkpoint-before`: 新しい仮説実装に入る直前に、直前採用状態の Docs を1コミットして push する
  - `checkpoint-after`: 実験結果が出た時点で、採用/不採用を Docs に追記して1コミットする
  - 上記2つは原則「確認待ち」ではなく自律実行し、例外時のみユーザー確認を取る
- 不採用実験の記録を必須とする
  - 採用実験は `docs/AHC061_Experiment_Log_2026-02.md` に記録
  - 不採用実験は `docs/AHC061_Experiment_Failures_2026-02.md` に `仮説 / 変更 / 結果 / 比較 / 判定 / タグ` で記録
  - 実装を `restore` して終了した場合でも、同日中に不採用記録を残す
- retro の結果はメタ改善として扱い、AHC061ローカルで閉じずに `../AGENTS.md` と `../../AGENTS.md` へ必要差分を反映する
- 状況共有は `docs/AHC061_Status_YYYY-MM-DD-HHMM.md` のスナップショット方式で行う
- 提出ごとのスコア差共有は `docs/AHC061_Submission_Gap_Log_YYYY-MM.md` へ追記して履歴化する

## Living Document
このファイルはプロジェクトの成長に伴い継続的に更新する。新しい仕様・設計判断・重要な変更があれば、必要に応じて追記する。
