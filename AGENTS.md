# AtCoder - AGENTS Instructions

## Scope
- このファイルは `AtCoder/` 配下の共通ルールを定義します。
- 各カテゴリ配下（例: `AHC/AGENTS.md`）は、この共通ルールを継承しつつ固有ルールで上書きできます。

## Source Of Truth
- AtCoder共通の運用方針はこのファイルを正本（single source of truth）とします。
- カテゴリ固有の運用は各カテゴリの `AGENTS.md` を正本とします。
- ルール変更はまず該当スコープの `AGENTS.md` を更新し、必要差分だけ下位へ反映します。

## Communication
- 一人称は「私」を使用します。
- 返答は敬語（です・ます調）で統一します。
- ユーザー向け返答は日本語を基本とします（技術識別子は英語可）。
- 節目では進捗要約（Recap）を行います。

## Smart-Commit
- 原則: 1変更 = 1コミット
- コミット前に `git diff --name-only` / `git diff --cached --name-only` を確認します。
- push 前に `git log --oneline -n <件数>` で意図した分割順を確認します。

## Docs Governance
- Docs は 1ファイル1責務を原則とします。
- 文字コードは UTF-8（BOMなし）を標準とします。
- 用語はプロジェクト内で統一します（例: `seed`, `score`, `ratio`, `mean/median/min/max`）。
- 保存後は `Get-Content <file> -Head 20` で文字化けを確認します。

## Checkpoint Workflow
- 目的: 実験サイクル中の「コミット確認往復」による停止を減らすことです。
- `checkpoint-before`: 新仮説実装前に、直前採用状態の Docs をコミットして push します。
- `checkpoint-after`: 実験結果確定後、採用/不採用を Docs に追記してコミットします。
- 原則は自律実行し、競合・例外時のみユーザー確認を行います。

## Living Document
- 新しい運用知見が出た場合は、このファイルへ先に反映します。
