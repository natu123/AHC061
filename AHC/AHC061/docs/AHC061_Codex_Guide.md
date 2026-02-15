# AHC061 Codex Guide

## 目的
- 本ドキュメントは、AHC061（THIRD Programming Contest 2026）向けに、Codex活用時の前提情報と運用指針をまとめた作業ガイドです。
- 公式仕様の最終確認は `N52XwIfp_windows/tools_x86_64-pc-windows-gnu/README.md` と問題文で実施します。

## コンテスト基本情報
- コンテスト名: THIRD Programming Contest 2026（AtCoder Heuristic Contest 061）
- 期間: 2026-02-13 19:00 〜 2026-02-23 19:00（JST）
- 問題数: 1問（A - Multi-Player Territory Game）
- 言語方針: Rustを第1候補、必要に応じてC++を選択
- 目的: プレイヤー0の比率 `S0 / SA` を最大化
- 最終スコア: `round(10^5 * log2(1 + S0 / SA))`

## 問題の固定制約（ガイド参照値）
- `N = 10`（10x10）
- `M = 2..8`（プレイヤー数）
- `T = 100`（ターン数）
- `U = 1..5`（レベル上限）
- `sum(V_i,j) = 100000`
- 初期領土: 各プレイヤー1マス（重複なし）

## ゲームルール要点（ガイド参照）
- 移動可能先: 自領土（連結）と隣接マス
- 競合時: 所有者優先、他プレイヤーの駒は回収
- 行動:
  - 未領土: 占領してレベル1
  - 自領土: レベル+1（上限U）
  - 他領土: レベル-1、0で奪取してレベル1
- 回収された駒: 元位置に復帰
- 最終スコア: 自領土の `V * L` 合計
- 厳密仕様: `docs/AHC061_Game_Rules_Strict.md` を参照

## AI（プレイヤー1..M-1）モデル（ガイド参照）
- 評価値 `A_p,i,j` は状態別の重み `wa, wb, wc, wd` で決まる
- 重み: `rand(0.3..1.0)`
- ランダム行動率: `eps_p = rand(0.1..0.5)`
- 行動:
  - `eps` でランダム（一様）
  - `1-eps` で最大評価値を貪欲選択（同値ランダム）
- `r1,t,p / r2,t,p` はローカルテスタで与えられる前提

## LLM利用ルール（必読）
- 公式: `https://info.atcoder.jp/entry/ahc-llm-rules-ja`
- 禁止:
  - APIやスクリプトで生成AIに多数出力を自動生成させ、評価・選別する行為
  - 自動コード生成を大量実行し、最良のみ採用する行為
- 許可:
  - 問題文翻訳
  - アルゴリズム情報整理
  - 戦略提案
  - 補完、バグ修正、リファクタリング
  - 自然言語指示による手動イテレーション
- 補足: 提出コードの作業ログを必要に応じて保存する

## 配布ツール実装・挙動メモ
- 対象ディレクトリ: `N52XwIfp_windows/tools_x86_64-pc-windows-gnu`
- 配布形態: `gen.exe` `tester.exe` `vis.exe` は実行バイナリ配布で、内部実装コードは同梱されていない
- 仕様の一次情報: `README.md`（必要に応じて `README.html` も参照）
- Rustローカル版: `N52XwIfp_local/tools`（`Cargo.toml`, `src/bin/gen.rs`, `src/bin/tester.rs`, `src/bin/vis.rs` を確認済み）

### `gen.exe`（入力生成）
- ヘルプ: `./gen.exe --help`
- 基本: `./gen.exe seeds.txt`
- 主要オプション:
  - `--dir <DIR>` 出力先を `in` 以外へ変更
  - `--M <M>` 生成入力の `M` を固定
  - `--U <U>` 生成入力の `U` を固定
  - `--verbose` 生成詳細をCSV出力
- 備考: 既定の `seeds.txt` は `0..99` の100件

### `tester.exe`（ローカルテスタ）
- ヘルプ相当: `./tester.exe`（引数なしでUsage表示、`--help` は未サポート）
- 基本: `Get-Content in/0000.txt | ./tester.exe <solver command> > out.txt`
- 仕様メモ:
  - スコアと検証メッセージは標準エラー出力（stderr）側に出る
  - 解が不正な場合は不正理由を表示し、`Score = 0` になる
  - プログラム異常終了時は `Your program has terminated unexpectedly` を表示
  - 出力ファイル `out.txt` には、実際に読み取れた行動列のみ書き出される（途中で不正なら途中まで）

### `vis.exe`（可視化）
- ヘルプ相当: `./vis.exe`（引数不足でUsage表示）
- 基本: `./vis.exe in/0000.txt out.txt`
- 仕様メモ:
  - 実行後に `vis.html` を生成
  - スコアや不正理由を標準出力に表示
  - 入力/出力ファイルが不正または欠落時はエラー終了

### 運用上の注意
- PowerShellでは `<` リダイレクトが使えないため、`Get-Content ... | tester.exe ...` 形式を使う
- solverコマンド解決に失敗する場合はフルパス指定する（`gcm <command>` で確認）
- 改善検証時は、`stderr` の不正メッセージ有無を先に確認してからスコア比較する
- 推奨運用: 通常評価はWindows版、仕様調査と実装読解はRustローカル版を使い分ける

### Rustローカル版（MSVC）実行手順
- 前提:
  - Rust: `rustc` / `cargo` / `rustup` 導入済み
  - Build Tools: `Visual Studio Build Tools 2022`（`Microsoft.VisualStudio.Workload.VCTools`）導入済み
- `link.exe` を解決するため、`VsDevCmd.bat` 経由で `cargo` を実行する
- 例（PowerShellから実行）:
  - `cmd /c "\"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat\" -no_logo && %USERPROFILE%\.cargo\bin\cargo.exe run -r --bin gen -- --help"`
- ローカルテスタ実行例:
  - `cmd /c "\"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat\" -no_logo && cd /d C:\Users\kenji\projects\AtCoder\AHC\AHC061\N52XwIfp_local\tools && type in\\0000.txt | %USERPROFILE%\.cargo\bin\cargo.exe run -r --bin tester cmd /c type out.txt > out_run.txt"`
- 確認コマンド:
  - `winget list --id Microsoft.VisualStudio.2022.BuildTools`
  - `%USERPROFILE%\\.cargo\\bin\\rustc.exe --version`
  - `%USERPROFILE%\\.cargo\\bin\\cargo.exe --version`

## Codexに期待する役割
- Rustテンプレート生成（入力、BFS到達判定、状態管理）
- 評価関数の提案（greedy、妨害重視、簡易先読み）
- 近傍操作設計（候補生成と選択ロジック）
- デバッグ支援（実行時エラー、所有権エラー修正）
- ローカルテスト結果を踏まえた改善提案

## 依頼テンプレート例
- 「このゲームでプレイヤー0の移動を決めるRust関数を書いて。評価関数は高V未領土優先 + 他者レベル1攻撃」
- 「seed 0..9 の結果を比較して、平均と最小ケース重視で次の改良案を3つ提案して」
- 「制約違反検知を先に入れて、違反時フォールバックを実装して」

## 運用メモ
- 本ファイルはガイド文書です。最終仕様は公式資料を優先します。
- ルール変更時は `AGENTS.md` を先に更新し、本ファイルへ必要差分を反映します。
- 検討結果・実験ログは `docs/AHC061_Initial_Study_2026-02-15.md` を参照します。
- 採用実験は `docs/AHC061_Experiment_Log_2026-02.md`、不採用実験は `docs/AHC061_Experiment_Failures_2026-02.md` を参照します。

## ソルバIDと提出運用
- ソルバIDは `x01, x02, ...` の作成順で採番し、既存IDの意味を変更しません。
- `champion` は「現在採用中のID」を指すラベルであり、IDそのものではありません。
- 開発中は複数bin/複数ファイル構成を許容し、共通ロジックは重複させない方針を優先します。
- 提出時は `champion` が指すソルバIDから単一ファイル成果物を生成します（例: `submission_x10.rs`）。
- 実験ログには「採用ID（xNN）」と「提出成果物名（submission_xNN.rs）」を対応付けて記録します。
- ソルバ詳細Docsは `docs/solver_specs_built`（作成済み）と `docs/solver_specs_planned`（作成予定）に分離し、将来コンテストでも同じ英語フォルダ名を使います。

## Docs統一ルール（retro）
- 文字コードは UTF-8（BOMなし）に統一します。
- 実験ログの項目名は `背景 / 対象 / 変更 / 実験条件 / 結果 / A/B比較 / 考察 / 次アクション` に固定します。
- 仕様系ドキュメントは `目的 / 対象 / 仕様 / 注意点 / 更新ルール` を基本構成にします。
- 保存後は `Get-Content <file> -Encoding UTF8 -Head 20` で文字化け確認を実施します。
- 試行番号は採用/不採用を通した `T-xxx` の通番を使用します（採用ログと失敗ログで共有）。

## チェックポイント運用
- 目的: 「コミットしますか？」の確認往復を減らし、検証サイクルを止めないことです。
- ルール:
  - `checkpoint-before`: 新しい仮説実装の直前に、直前採用状態の Docs を `commit & push` します。
  - `checkpoint-after`: 実験結果（採用/不採用）が確定した時点で、実験ログを `commit & push` します。
  - 既定は確認省略で自律実行し、競合や例外がある場合のみユーザー確認を行います。
  - `checkpoint-after` では、不採用時に `AHC061_Experiment_Failures_2026-02.md` への追記を必須とします。

## メタ改善の考え方
- スコア改善だけでなく、運用改善（メタ改善）を同等に重視します。
- 失敗を記録しないこと自体を失敗として扱い、retro 対象に含めます。
- retro の最小テンプレート:
1. 事実: 何を記録し損ねたか
2. 影響: 何が再発リスクになったか
3. 改善: どのルール/チェックを追加したか
4. 展開: どの上位 `AGENTS.md` に反映したか
