# AHC061 Submission Gap Log 2026-02

## 目的
- 提出後に共有された「時刻」と「スコア差」を、時系列で追跡可能に記録するためのログです。

## 記録ルール
- 時刻は `YYYY-MM-DD HH:mm (JST)` で記録します。
- `top1_score` は同時点の1位スコアを記録します。
- `my_rank` と `my_score` は同時点の自分の順位/獲得スコアを記録します。
- `score_gap` は `top1_score - my_score` で記録します（正の値が大きいほど差が大きい）。
- `ratio` は `my_score / top1_score` で記録します。
- ユーザー共有値を正本として転記し、推定値では上書きしません。

## Entries

| ID | Reported At (JST) | top1_score | my_rank | my_score | score_gap | ratio | Memo |
| --- | --- | --- | --- | --- | --- | --- | --- |
| SG-001 | 2026-02-15 22:57 | 74,963,811,852 | 284 | 46,295,251,986 | 28,668,559,866 | 0.617568 | user reported |
