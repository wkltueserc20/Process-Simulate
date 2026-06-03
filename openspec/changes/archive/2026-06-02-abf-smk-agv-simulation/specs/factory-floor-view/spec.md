## ADDED Requirements

### Requirement: 工廠平面圖 SVG 佈局
系統 SHALL 以 SVG 呈現工廠平面圖，ABF 線在上方、SMK 線在下方，兩線水平排列。

#### Scenario: 站點方塊顯示
- **WHEN** 頁面載入
- **THEN** 每個站點顯示為圓角矩形，內含站點名稱與 WIP 數字

#### Scenario: 站點間軌道線顯示
- **WHEN** 頁面載入
- **THEN** 相鄰站點之間顯示一條水平連接線作為 AGV 軌道

---

### Requirement: AGV 動畫移動
每台 AGV SHALL 以 SVG circle 表示，在來源站和目標站之間沿軌道平滑移動。

#### Scenario: AGV 從左站移動到右站
- **WHEN** AGV 狀態變為 MOVING（向右）
- **THEN** AGV 圖示以 CSS transition 在指定 duration（依 cycle time 與播放速度計算）內從來源站座標移動到目標站座標

#### Scenario: AGV 從右站返回左站
- **WHEN** AGV 完成 UNLOADING 後返回
- **THEN** AGV 圖示以較快速度（空車返回，時間=裝載時間×0.5）移回來源站

#### Scenario: 快轉時跳過動畫
- **WHEN** 播放速度 ≥ 5x
- **THEN** AGV 不使用 CSS transition，直接跳到目標位置（避免卡頓）

---

### Requirement: WIP 顏色警示
站點方塊 SHALL 依 WIP 數量改變背景顏色。

#### Scenario: WIP 正常（< 閾值）
- **WHEN** 站點 WIP < 5
- **THEN** 站點背景顯示為正常色（藍色/綠色）

#### Scenario: WIP 警告
- **WHEN** 站點 WIP >= 5 且 < 10
- **THEN** 站點背景變為橘色，並顯示警告圖示

#### Scenario: WIP 危險（瓶頸爆發）
- **WHEN** 站點 WIP >= 10
- **THEN** 站點背景變為紅色，WIP 數字以粗體顯示，並有閃爍動畫

---

### Requirement: AGV 狀態顏色
AGV 圖示 SHALL 以顏色區分當前狀態。

#### Scenario: AGV 狀態對應顏色
- **WHEN** AGV 狀態改變
- **THEN** IDLE=灰色、LOADING=黃色、MOVING（有料）=綠色、MOVING（空車）=淺藍、UNLOADING=黃色
