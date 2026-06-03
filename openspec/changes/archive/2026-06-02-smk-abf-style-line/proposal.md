## Why

目前 SMK 製程線是簡化的輸送帶模型（每台 AGV 在固定路段間穿梭、以 WIP 計數呈現），與 ABF 線那套精密的「slot/PORT + 分組 AGV + 取放料/預訂」引擎落差很大，無法真實呈現 SMK 的搬運節拍、各站入/出料 PORT 行為與堆積瓶頸。客戶與管理層看 ABF 與 SMK 兩條線時體感不一致，SMK 的說服力與分析價值不足。

## What Changes

- **以 ABF 引擎邏輯重建 SMK 線**（製程拓樸為 SMK）。把現有 ABF 引擎一般化為 line config 驅動，再以 SMK 設定實例化（ABF 維持行為不變）。
- **SMK 站別（11 站）依類型對應 ABF**：
  - `CLN` → refill 型（收空框 / 空框放置 / 出滿框）
  - `Annealing / Baking / SoftBaking / Postcure` → buffer 型（入料 PORT + 出料 PORT，含 holdTime / 容量）
  - `CZ / SMK壓機 / 曝光機 / 顯影機 / UVQ` → transfer 型（入料端：入滿框/拿料位/退空框；出料端：收空框/放置位/出滿框）
  - `換框機` → changer 型
- **6 組專用 AGV（A1–A11，共 11 台）固定分組**，線性主鏈 CLN→Annealing→CZ→Baking→SMK壓機→曝光機→顯影機→SoftBaking→UVQ→Postcure→換框機：
  - G1 CLN→Annealing→CZ（A1,A2）／G2 CZ→Baking→SMK壓機（A3,A4）／G3 SMK壓機→曝光機（A5）／G4 曝光機→顯影機（A6）／G5 顯影機→SoftBaking→UVQ（A7,A8）／G6 UVQ→Postcure→換框機（A9,A10,A11）
  - 新增「無中間 buffer 的 transfer→transfer 直送群組」型態（G3、G4），ABF 原本只有 3 站含 buffer 的群組。
- **SMK 參數全面可設定**（對齊 ABF 粒度）：各 transfer 站處理時間（CZ/壓機/曝光/顯影/UVQ）、各 buffer 容量與 holdTime、換框機拿料時間、CLN 生產時間、每框片數、11 站間路段行走時間、機台/緩衝站取放料時間。**BREAKING**：移除舊 SMK 參數 `SMK Cycle Time`、`每 Cycle 批量`、`SMK AGV 數量`、`WIP 警告/危險閾值`。
- **瓶頸呈現改為 PORT 堆積式**（沿用 ABF 的視覺語彙），取代原 SMK 的 WIP 閾值瓶頸面板；移除 SMK 即時加減 AGV 與 WIP 上色。
- **SMK 版面改為 ABF 式多 PORT 站卡**；11 站較 ABF 7 站密集，需要新的佈局策略（站卡縮窄／加寬畫布／蛇形換行，見 design）。

## Capabilities

### New Capabilities
- `line-config`: 以資料描述一條製程線（站別定義、群組與 AGV 分配、路段行走時間表、版面佈局），供一般化引擎實例化 ABF / SMK。

### Modified Capabilities
- `simulation-engine`: 由 ABF 專用一般化為 line-config 驅動；新增「2 站直送（無 buffer）群組」型態；ABF 行為不變。
- `factory-floor-view`: SMK 改用 ABF 式 slot/PORT 站卡與分組 zone 渲染；新增 11 站佈局策略。
- `parameter-panel`: 以 SMK 的 ABF 式參數取代舊 SMK 參數；路段示意圖支援 SMK 拓樸。
- `bottleneck-alert`: 由 WIP 閾值式改為 PORT 堆積式（與 ABF 一致）。

## Impact

- **檔案**：單一 `index.html`。大幅重構模擬核心：`ABF_DEFS`/`ABF_AGV_INITS`/`tABF`/`getGroupNTasks`/`updateABFMachines`/`stepABFAgv`/`buildABFSVG`/`updateUI` 一般化；新增 SMK line config；移除舊 SMK 輸送帶邏輯（`SMK_NAMES`/`SMK_BASE`/`stepSMKAgv`/`smkSeg` 等）。
- **與 `redesign-parameter-panel` 互動**：該 change 的 SMK 即時參數（`sa`/`ww`/`wd`）與熱加減 AGV 將被本案移除/取代；需協調。`tg`（目標產量）保留。
- **與進行中 `abf-smk-agv-simulation`**：本案修改其多個 capability（simulation-engine、factory-floor-view、parameter-panel、bottleneck-alert），需協調避免規格衝突。
- **無新依賴**：維持單檔、無建置、Chart.js CDN。
- **風險**：一般化引擎可能影響既有 ABF 行為（需回歸驗證）；11 站佈局密集需設計。
