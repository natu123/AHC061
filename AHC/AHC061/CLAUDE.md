# AHC061 - Claude Code Instructions

## Inheritance
- AtCoder共通ルールは `../../CLAUDE.md` を継承する
- AHC共通ルールは `../CLAUDE.md` を継承する
- このファイルは AHC061 固有事項のみを定義する
- `AGENTS.md` を正本とし、本ファイルはその Claude Code 向けサブセットである

## Source Of Truth
- AHC061 固有の運用方針は `AGENTS.md` を正本とする
- 公式配布ツール仕様は `N52XwIfp_windows/tools_x86_64-pc-windows-gnu/README.md` を補助正本とする

## Contest Info
- コンテスト: THIRD Programming Contest 2026 (AHC061)
- 期間: 2026-02-13 19:00 〜 2026-02-23 19:00 (JST)
- 問題: A - Multi-Player Territory Game
- 言語: Rust（`solver/` ディレクトリ、`edition = "2021"`）

## Problem Summary
- 10x10 盤面、M=2..8 プレイヤー、T=100 ターン、U=1..5 レベル上限
- player0 を操作し、`S0/SA` の比率を最大化する
- スコア: `round(10^5 * log2(1 + S0 / SA))`
- 移動可能先は自領土BFS＋隣接マス、競合時は所有者優先
- 厳密仕様: `docs/AHC061_Game_Rules_Strict.md`

## Current State
- **Champion**: `x47`（Macro Route Pressure Dual Guard）
- **Ratio**: ~0.60（`my_score / top1_score`、SG-003時点）
- **Phase**: `ratio < 0.90` → 抜本改善中心、Explore Gear-Shift 該当

## Tech Stack
- Solver: Rust (`solver/Cargo.toml`, `solver/src/bin/x*.rs`)
- Official Tools: `gen.exe`, `tester.exe`, `vis.exe`
- Input corpus: `N52XwIfp_windows/tools_x86_64-pc-windows-gnu/in/` (seed 0..99)
- Evaluation scripts: `tmp_eval_loop10.ps1` 等

## Key Files
| Path | 説明 |
|------|------|
| `AGENTS.md` | AHC061固有ルール（正本） |
| `solver/src/bin/` | ソルバ実装（x01〜x81） |
| `solver/src/lib.rs` | 共通ライブラリ |
| `submissions/` | 提出用ビルド成果物 |
| `docs/AHC061_Game_Rules_Strict.md` | ゲームルール厳密仕様 |
| `docs/AHC061_Codex_Guide.md` | 実行手順・運用ガイド |
| `docs/AHC061_Experiment_Log_2026-02.md` | 採用実験ログ |
| `docs/AHC061_Experiment_Failures_2026-02.md` | 不採用実験ログ |
| `docs/AHC061_Submission_Gap_Log_2026-02.md` | 提出スコア差ログ |
| `docs/solver_specs_built/README.md` | 作成済みソルバ索引 |
| `docs/solver_specs_planned/README.md` | 計画中ソルバ索引 |
| `N52XwIfp_windows/tools_x86_64-pc-windows-gnu/README.md` | 公式ツール説明 |

## Local Execution (PowerShell)
```powershell
# ツール配置ディレクトリへ移動
cd "N52XwIfp_windows/tools_x86_64-pc-windows-gnu"

# 既存入力でテスト
Get-Content in/0000.txt | ./tester.exe <solver command> > out.txt

# 追加入力生成
./gen.exe seeds.txt

# 可視化
./vis.exe in/0000.txt out.txt
```

## AHC061 Protocol
- 評価は `seed=0..99` の既存入力を基本集合とする
- 検証では `平均・中央値・最小スコア` と `実行時間` を最低限併記する
- 乱数依存バグの切り分けのため、デバッグ時は固定seedモード必須
- 実験スクリプトの起点は `AtCoder\AHC\AHC061` を前提とする

## Quick/Full Selection
- **quick**: seed 0..19 で候補を絞り込む
- **full**: seed 0..99 で最終評価
- 即却下ルール:
  - `mean < 1.01 * champion_mean` → 即却下
  - `improvement_per_sec <= 0` → 即却下
- full へ進めるのは quick 上位 `2` 件まで

## Exploit/Explore Balance
- 基本: `Exploit 5% / Explore 95%`
- Explore 内訳: `new 30% / pair 50% / blend 20%`
- `ratio < 0.90` では既存 Macro 派生は探索全体の 10% 以下に制限
- `ratio < 0.95` では新規アーキテクチャ 80% / 既存派生 15% / Exploit 5%

## Explore Gear-Shift（ratio < 0.90 時）
- 各ループで新規性の高い仮説を最低 2 件実装して quick 評価まで実施
- 競争候補条件をすべて満たす場合のみ full へ進める
- 既存方針の微調整は全体の 20% 以下に抑える

## Solver ID / Champion Policy
- `xNN` は作成順の固定IDで後から意味を変更しない
- `champion` は採用状態ラベルとしてのみ管理する
- 提出時は `submission_xNN.rs` を生成して実験ログと1対1で辿れる状態を維持する

## Docs運用
- 見出しと項目名統一:
  - 実験ログ: `背景 / 対象 / 変更 / 実験条件 / 結果 / A/B比較 / 考察 / 次アクション`
  - 仕様書・ガイド: `目的 / 対象 / 仕様（または手順） / 注意点 / 更新ルール`
- 実験の見出し番号は `T-xxx` で一元管理する
- Docs更新時は `Get-Content <file> -Encoding UTF8 -Head 20` で文字化け確認する
- 文字コードは UTF-8（BOMなし）
