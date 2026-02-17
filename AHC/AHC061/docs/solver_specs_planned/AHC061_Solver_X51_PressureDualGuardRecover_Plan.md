# AHC061 Solver X51 Pressure Dual-Guard Recover Plan

## 目的
- `x47` の中盤回復性（早期圧力の失敗抑制）を上げ、分岐のロバスト性を高める。
- `QualityFirst` 運用下で `full` へ到達したとき `mean`・`min` の同時改善が生じるかを確認する。

## タグ
- ID: `x51`
- バイナリ: `x51_macro_route_pressure_dual_guard_recover`
- 主要変更ファイル:
  - `solver/src/bin/x51_macro_route_pressure_dual_guard_recover.rs`

## 仮説
- `early pressure` を上げつつ `late pressure` と `route pressure` を強めることで、回復局面の選択肢を広げる。
- 分岐の取りこぼしを減らし、`median` 低下の緩和を期待する。

## 検証計画
- quick: `seed 0..19`
- quick で競争条件を満たす候補のみ full: `seed 0..99`
- 判定:
  - quick: `mean / median / min` が baseline を上回るか
- full: `mean`, `median`, `min`, `elapsed` の改善方向性を比較

