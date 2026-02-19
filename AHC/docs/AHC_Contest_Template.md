# AHC コンテストテンプレート

新規 AHC コンテスト開始時にコピーして使うチェックリスト。
`Meta_AHC_Learnings.md` の教訓を実運用に落とし込んだもの。

---

## Day 1: 初日セットアップ

- [ ] プロジェクトディレクトリ作成（`AHC/AHC<NNN>/`）
- [ ] `CLAUDE.md`, `AGENTS.md` を AHC 共通テンプレートからコピー
- [ ] 問題文を精読し、`docs/Game_Rules.md` に厳密仕様を記述
- [ ] 公式ツール（tester, gen, vis）を配置・動作確認
- [ ] 入力コーパス生成（seed 0..99）
- [ ] ベースラインソルバ（x01: 貪欲 or 1手先読み）を実装・評価
- [ ] 評価スクリプト（eval_quick.ps1）を用意
- [ ] ローカル/ジャッジ速度比を測定（→ 時間制限の校正に使用）
- [ ] 問題のパラメータ空間を把握（帯域別のゲーム性差異を確認）

## 実験サイクル

```
仮説 → 実装 → quick評価(seed 0..19) → 即却下判定 → [通過] → full評価(seed 0..99) → 記録
```

### 即却下基準テンプレート
```
即却下基準: quick mean < champion_mean × 1.10
full 進出: quick 上位 2 件まで
```

- 基準は champion 更新のたびに再計算
- 候補が基準未達なら即打ち切り、理由を記録して次の仮説へ

### 記録テンプレート（Experiment_Log）
```markdown
## T-<NNN> (x<NNN>) <名称>
- **背景**: なぜこの仮説を試すか
- **対象**: x<NNN>_<name>
- **変更**: 具体的な実装変更
- **結果**: quick mean <N> (<±X%>), median <N>, min <N>, max <N>, <T>s/seed
- **考察**: なぜこの結果になったか
- **判定**: ACCEPT / REJECT / 検証継続
```

## 飽和検知ルール

以下のいずれかに該当したら、現在の方向性を打ち切り新パラダイムに移行:

- [ ] 直近 8 試行で ACCEPT 0 件
- [ ] 直近 10 試行の平均改善率が +0.3% 未満
- [ ] 同一アルゴリズム枠組み内の全コンポーネントをテストして全て悪化

移行後は **Explore 95% / Exploit 5%** で、最低 2 系統の異種アプローチを並列評価する。

## ドキュメント構成

```
AHC<NNN>/
├── AGENTS.md                      # 運用ルール正本
├── CLAUDE.md                      # Claude Code 向けルール
├── README.md                      # プロジェクトエントリポイント
├── docs/
│   ├── Game_Rules.md              # ゲームルール厳密仕様
│   ├── Codex_Guide.md             # 実行手順・運用ガイド
│   ├── Solver_Registry.md         # 全ソルバ台帳
│   ├── Key_Learnings.md           # 教訓まとめ
│   ├── Next_Actions.md            # 次の改善方針
│   └── Experiment_Log_YYYY-MM.md  # 月別実験ログ
├── solver/                        # ソルバ実装
├── scripts/                       # 評価スクリプト
└── submissions/                   # 提出用成果物
```

### 運用ルール
- 1ファイル1責務
- UTF-8（BOM なし）
- 100 行超のファイルには TOC 追加
- 不採用実験も必ず記録
- 即却下基準を明文化して AGENTS.md に記載

## Solver ID Policy

- `x01, x02, ...` の作成順で固定 ID を採番
- 既存 ID の意味は変更しない
- `champion` は採用状態ラベルとしてのみ管理
- 提出物は `submission_x<NN>.rs` 形式で実験ログと紐づけ

## コミット規約

- 1 コミット = 1 つの論理的変更
- Conventional Commits 形式（`feat:`, `fix:`, `docs:`, `chore:`）
- ソルバ追加は個別コミット（revert/cherry-pick を可能にする）
