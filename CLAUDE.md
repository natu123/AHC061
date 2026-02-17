# AtCoder - Claude Code Instructions

## Scope
- このファイルは `AtCoder/` 配下の共通ルールを定義する
- 各カテゴリ配下（例: `AHC/CLAUDE.md`）は、この共通ルールを継承しつつ固有ルールで上書きできる
- `AGENTS.md` を正本とし、本ファイルはその Claude Code 向けサブセットである

## Communication
- 一人称は「私」を使用する
- 返答は敬語（です・ます調）で統一する
- ユーザー向け返答は日本語（技術識別子は英語可）
- 受動的な「問い待ち」ではなく、自律的に提案・計画を行う
- タスクやフェーズの節目に進捗を要約（Recap）する
- 「終了の提案」や「時刻の通知」は行わず、ユーザー主導の区切りを尊重する

## Smart-Commit Workflow
**原則: 1変更 = 1コミット**

- コミット前に `git diff --name-only` と `git diff` を確認し、`Feat / Fix / Refactor / Perf / Style / Docs / Chore` 単位で分割する
- `git add .` の一括ステージは原則禁止（意図しない混在防止）
- 形式: `<Type>: <対象>を<目的>（必要なら手段）`
- 「調整」「改善」など抽象語だけの件名を禁止する
- 本文テンプレート: `背景 / 変更 / 影響`
- push前に `git log --oneline -n <分割予定数>` で意図した並びを確認する

## Docs Governance
- 1ファイル1責務を原則とする
- 運用改善の反映は「必要なファイルに必要な分だけ」を原則とし、全md一括更新を禁止する
- 文字コードは UTF-8（BOMなし）を標準とする
- 用語はプロジェクト内で統一する（`seed`, `score`, `ratio`, `mean/median/min/max`）
- 不採用実験は必ず残し、再発防止タグを付与する
- 系統ID（`x01`, `x02`, ...）は作成順の固定IDとして扱い、採用状態と混在させない

## Checkpoint Workflow
- `checkpoint-before`: 新仮説実装前に Docs をコミット＆push
- `checkpoint-after`: 実験結果確定後に Docs を追記してコミット＆push
- 原則自律実行し、例外時のみユーザー確認を取る

## LLM Compliance
- AtCoderのLLM利用ルール準拠
- APIやスクリプト等による生成AIへの多数出力の自動生成依頼は行わない
- 生成AIの利用は人手主導の対話・検討に限定する

## Conventions
- 既存ファイル編集優先、新規ファイル追加は必要最小限
- PowerShell実行を前提とし、コマンド連結は `;` を使う
