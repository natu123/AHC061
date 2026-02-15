# AHC061 Solver X03 Particle CVaR

## 目的
- 相手行動を単一分布で決め打ちせず、粒子化した相手モデルで複数シナリオを生成して tail-risk を制御する。
- `CVaR`（下位tail平均）を直接使い、下振れ耐性を持つ候補手を選びやすくする。

## 対象
- 系統ID: `x03`
- 系統名: `x03_particle_cvar`
- 実装本体: `solver/src/x03_particle_cvar.rs`
- 実行bin: `solver/src/bin/x03_particle_cvar.rs`
- 状態: 実装済み（非採用）

## 仕様
- 意思決定の中核:
  - `choose_move_x03_particle_cvar`
  - `M=3..5` で粒子 + `CVaR` 評価を適用し、それ以外は `x06` へフォールバック
- 粒子相手モデル:
  - AIごとに重みスケール・`eps`シフトの異なる `5` 粒子を生成
  - `seen/eps_est` から粒子重みを動的化
- 候補評価:
  - 候補手ごとにサンプリングして `strategic_score` を収集
  - `total = (1-cvar_w)*mean + cvar_w*CVaR + local_w*local`

## 強み
- quick（seed `0..19`）では `x06` 比で `mean/median` を改善する構成が確認できた。
- `M=5` 帯で上振れするseedがあり、既存MCでは拾いにくい分岐を取れる局面がある。

## 弱み
- full（seed `0..99`）では `M=3/4` が下振れし、全体 `mean/median/min` を悪化させた。
- サンプリング増により `elapsed` が増加し、計算効率でも不利だった。

## 評価結果（最終）
- quick（seed `0..19`, 対 `x06`）:
  - `x06`: mean `146,623.6`, median `119,019.5`, min `70,744`, max `388,857`, elapsed `1,920ms`
  - `x03`: mean `148,606.6`, median `123,776`, min `70,744`, max `388,857`, elapsed `2,142ms`
- full（seed `0..99`, 対 `x06`）:
  - `x06`: mean `155,863.2`, median `133,042.5`, min `51,023`, max `605,548`, elapsed `9,705ms`
  - `x03`: mean `154,876.7`, median `129,861`, min `44,717`, max `605,548`, elapsed `10,539ms`
- 判定:
  - 不採用（fullで `mean/median/min` 同時悪化）

## 関連ログ
- 不採用: `docs/AHC061_Experiment_Failures_2026-02.md` の `T-067`
- 計画: `docs/solver_specs_planned/AHC061_Solver_X03_Particle_CVaR_Plan.md`

## 更新ルール
- 粒子数・`CVaR alpha`・`M`適用帯を変更した場合、quick/fullの両方を追記する。
