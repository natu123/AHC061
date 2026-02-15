# 作成済みソルバ詳細

## 目的
- AHC061で既に実装済みのソルバ系統（`xNN`）について、戦略・実装位置・強み弱みを参照可能にする。
- 新規戦略立案時に「何を既に試し、何が未実施か」を高精度で確認できる状態を作る。

## 対象
- `x01`: `x01_beam_pessimistic`
- `x02`: `x02_monte_carlo`
- `x05`: `x05_adaptive_racing_mc`
- `x06`: `x06_expert_switch_hybrid`

## 仕様
- `xNN`は作成順の固定IDとして扱う。
- `champion` は採用状態ラベルであり、`xNN` の意味を上書きしない。
- 実装ファイル/エントリポイント/主パラメータ/既知の効果を系統ごとに記載する。

## 一覧
- `AHC061_Solver_X01_Beam_Pessimistic.md`
- `AHC061_Solver_X02_MonteCarloExplore.md`
- `AHC061_Solver_X05_AdaptiveRacingMC.md`
- `AHC061_Solver_X06_ExpertSwitchHybrid.md`

## 注意点
- 実装更新で戦略仕様が変わった場合、当日中に該当ファイルを更新する。
- 実験ログ（`docs/AHC061_Experiment_Log_2026-02.md`）の採否結果と矛盾しないことを維持する。

## 更新ルール
- 新規ソルバを作成したら、`xNN`を採番して本フォルダへ追加する。
- 既存ソルバの挙動が実質的に変化した場合は、同一`xNN`ファイルを更新し、変更日を追記する。
