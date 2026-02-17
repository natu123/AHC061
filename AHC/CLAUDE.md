# AHC - Claude Code Instructions

## Scope
- このファイルは `AHC/` 配下の共通ルールを定義する
- `../CLAUDE.md`（AtCoder共通）を継承する
- 各コンテスト配下（例: `AHC061/CLAUDE.md`）は、この共通ルールを継承しつつ固有ルールで上書きできる
- `AGENTS.md` を正本とし、本ファイルはその Claude Code 向けサブセットである

## AHC Domain
- コンテスト問題に対するヒューリスティック最適化を対象とする
- 「制約を満たす出力」かつ「スコア最大化（または最小化）」が目的
- 単発のACではなく、seed群で安定して高スコアを出す実装を重視する

## Engineering Protocol（重要度順）
1. **Score-Driven Fix**: 変更の目的を「どの指標を何点改善したいか」で定義する
2. **Reproducibility First**: seed・コマンド・時間制限・commitIDをセットで記録する
3. **Root Cause Analysis**: スコア低下時は現象と原因を分離して分析する
4. **A/B Validation**: 同一seed集合で比較し、平均・中央値・最悪ケースを併記する
5. **Small Batch Experiments**: 少数seedで探索→有望案のみ全seed評価
6. **Time Budget Guard**: ローカルで上限時間を守るガードを実装する
7. **Constraint Safety**: 速度改善より制約違反検出を優先する
8. **Deterministic Mode**: 乱数seed固定モードでデバッグ時は検証する
9. **User-Led Final Check**: 最終提出判断はユーザー主導

## Experiment Management
- 比較は「baseline」と「candidate」の2系統以上で行う
- 記録項目: 実行日時, commit, コマンド, seed範囲, 平均/中央値/最小スコア, 実行時間
- 採用基準: 平均改善だけでなく、最悪ケース悪化と時間超過リスクを許容範囲内に収める
- 試行IDは `T-xxx (Xyy-zz) S/F-kk` 形式を推奨

## Plateau Pivot Policy
- 直近 `8` 試行で採用 `0` 件、または `10` 試行平均改善率 `+0.3%` 未満なら抜本探索へ移行
- 移行後: `Exploit 5% / Explore 95%`
- 最低 `2` 系統、推奨 `4` 系統以上の異種アプローチを並列評価する

## Solver Lineage
- `x01, x02, ...` の作成順で採番し、既存IDの意味は変更しない
- `champion` は「現在採用中の系統」を示すラベルとしてのみ扱う
- 提出物は採用時の系統IDを含む名前（例: `submission_x10.rs`）で生成する

## Docs分類（共通）
- `*_Codex_Guide.md`: 実行手順・運用ルール
- `*_Game_Rules_Strict.md`: 実装準拠の厳密仕様
- `*_Initial_Study_YYYY-MM-DD.md`: 初期検討プロセスと結果
- `*_Experiment_Log_YYYY-MM.md`: 継続検証ログ
- `*_Experiment_Failures_YYYY-MM.md`: 不採用実験ログ
- `solver_specs_built/`: 作成済みソルバの戦略詳細
- `solver_specs_planned/`: 計画中ソルバの戦略詳細
