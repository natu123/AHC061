# AHC061 Solver X50 Pressure Dual-Guard Focus Plan

## 目的
- `x47` の前線圧力探索を引き継ぎ、重点化フェーズ（`late`）を強めて `seed 0..99` の `mean` を上積みすることを狙う。
- `x47` の改善版として `品質重視（QualityFirst）` モードで、quick の競争優位性が維持できるか確認する。

## タグ
- ID: `x50`
- バイナリ: `x50_macro_route_pressure_dual_guard_focus`
- 主要変更ファイル:
  - `solver/src/bin/x50_macro_route_pressure_dual_guard_focus.rs`

## 仮説
- `x47` から以下を上方更新することで、終盤の圧力効率を高め、`mean` を維持しながら `median` の下落を抑える想定。
  - `phase_cutoff`, `phase_split`, `candidate_cap`, `beam_width_*`, `target_pressure_weight`, `pressure_weight_late` など。
- 早期段階の安全性は保ちながら、終盤の奪取効率を上げる。

## 検証計画
- quick: `seed 0..19`
- quick で上位 `2` 以上が `quality` 条件を満たす場合は full: `seed 0..99`
- 判定:
  - quick: `mean / median / min` が `baseline` の `98.5% / 98.5% / 90%` 近辺を確保
- full: `mean` および `median` の実質上昇を確認し、`min` 退化と `elapsed` を確認

