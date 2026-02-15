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
- `N52XwIfp_windows/tools_x86_64-pc-windows-gnu/README.md` - 公式ツール説明
- 必要になった時点で、提出コードや実験メモの該当ファイルを追加参照する

## Docs運用（AHC061）
- Docs運用は `../AGENTS.md` の `Docs Governance` に従う
- AHC061では次の責務分離を固定する
  - `docs/AHC061_Codex_Guide.md`: 実行手順・開発運用・Codex利用指針
  - `docs/AHC061_Game_Rules_Strict.md`: tester実装準拠の厳密ルール
  - `docs/AHC061_Initial_Study_2026-02-15.md`: 初期検討結果（固定ログ）
- 新しい検証フェーズを始める場合は `docs/AHC061_Experiment_Log_YYYY-MM.md` を新規作成する
- 検証結果を会話で報告した場合、採否判断まで含めて当日中にDocsへ反映する
- 仕様根拠が `README` と実装で食い違う場合、先に `docs/AHC061_Game_Rules_Strict.md` を更新してから他Docsへ反映する
- 文字コードは **UTF-8（BOMなし）** を原則とし、文字化けが見えた時点でファイル全体を再保存してから編集する
- 見出しと項目名は以下に統一する
  - 実験ログ: `背景 / 対象 / 変更 / 実験条件 / 結果 / A/B比較 / 考察 / 次アクション`
  - 仕様書・ガイド: `目的 / 対象 / 仕様（または手順） / 注意点 / 更新ルール`
- 実験の見出し番号は採用/不採用を通した試行通番 `T-xxx` を使用する（例: `[T-019]`）
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

## Living Document
このファイルはプロジェクトの成長に伴い継続的に更新する。新しい仕様・設計判断・重要な変更があれば、必要に応じて追記する。
