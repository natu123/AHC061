# AHC061 Solver X15 Band Adaptive Route Plan

## 逶ｮ逧・- `x04` 縺ｮ `M=4` 蛛城㍾繧呈弍豁｣縺励～M=5/6` 縺ｧ繧ゆｸｭ逶､邨瑚ｷｯ險育判繧呈ｴｻ縺九☆縺溘ａ縺ｮ譁ｰ隕剰ｷｯ邱壹ｒ螳溯｣・☆繧九・
## 蟇ｾ雎｡
- ID: `x15`
- 螳溯｣・錐: `x15_band_adaptive_route`
- 螳溯｣・ｺ亥ｮ壹ヱ繧ｹ:
  - `solver/src/x15_band_adaptive_route.rs`
  - `solver/src/bin/x15_band_adaptive_route.rs`

## 譁ｹ驥・- `M=4/5/6` 繧貞挨蟶ｯ縺ｨ縺励※謇ｱ縺・～phase` 縺ｨ `uncertainty` 縺ｫ蠢懊§縺ｦ `x04` 縺ｮ `plan_len`/`beam_width`/`candidate_cap` 繧貞・譖ｿ縺吶ｋ縲・- `M=5/6` 縺ｧ縺ｯ `x04` 繧剃ｽ弱Μ繧ｹ繧ｯ縺ｫ髯仙ｮ壹＠縲∬｡晉ｪ・ｫ倥さ繧ｹ繝亥ｱ髱｢縺ｧ縺ｯ菫晏ｮ育噪縺ｪ繝輔か繝ｼ繝ｫ繝舌ャ繧ｯ縺ｸ謌ｻ縺吶・- `x04` 縺ｨ `x06` 繧堤ｵ・∩蜷医ｏ縺帙～x04` 縺御ｸ榊茜縺ｪ螻髱｢縺ｧ縺ｯ逶ｴ縺｡縺ｫ `x06` 縺ｫ繝輔ぉ繧､繝ｫ繝舌ャ繧ｯ縺吶ｋ縲・
## 讀懆ｨｼ險育判
- `seed 0..19` quick
- `seed 0..19` quick の結果を `mean` 中心で整列し、`mean` が `98.5%` 以上かつ `median`/`min` が champion 基準（`98.5%` / `90%`）を満たす候補から、quick上位3件まで `seed 0..99` full
- 競争候補が0件なら full 不実施
- 判定: `mean/median/min/max, elapsed`
