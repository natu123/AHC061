# AHC061 Solver X17 Mid-Band Dual-Lane Plan

## 逶ｮ逧・- `M=4/5/6` 縺ｧ遏ｭ譛滓姶逡･縺ｨ邨瑚ｷｯ謌ｦ逡･繧貞酔譎ょｮ溯｡後＠縲∝ｱ髱｢霆｢謠帶凾縺ｮ蟠ｩ繧後ｒ貂帙ｉ縺吶・
## 蟇ｾ雎｡
- ID: `x17`
- 螳溯｣・錐: `x17_mid_band_dual_lane`
- 螳溯｣・ｺ亥ｮ壹ヱ繧ｹ:
  - `solver/src/x17_mid_band_dual_lane.rs`
  - `solver/src/bin/x17_mid_band_dual_lane.rs`

## 譁ｹ驥・- `phase` 縺ｧ縲碁ｫ倬滓焔・・ocal・峨阪檎ｵ瑚ｷｯ謇具ｼ・oute・峨阪・2繝ｬ繝ｼ繝ｳ繧貞・譖ｿ縺励∵ｷｷ蜷域ｯ皮紫繧・`uncertainty` 縺ｨ `conflict` 縺ｧ閾ｪ蜍戊ｪｿ謨ｴ縺吶ｋ縲・- `M=6` 縺ｧ縺ｯ蛟呵｣應ｸ企剞繧呈椛縺医∬｡晉ｪ∝屓驕ｿ繧貞━蜈医☆繧九・- `M=4/5` 縺ｮ蛻晄悄逶､縺ｧ縺ｯ `x04` 縺ｮ驕縺・ｾ｡蛟､迯ｲ蠕励ｒ險ｱ螳ｹ縺励∝ｾ悟濠縺ｯ `x06` 縺ｧ螳牙ｮ壼喧縺吶ｋ縲・
## 讀懆ｨｼ險育判
- `seed 0..19` quick
- `seed 0..19` quick の結果を `mean` 中心で整列し、`mean` が `98.5%` 以上かつ `median`/`min` が champion 基準（`98.5%` / `90%`）を満たす候補から、quick上位3件まで `seed 0..99` full
- 競争候補が0件なら full 不実施
- 判定: `mean/median/min/max, elapsed`
