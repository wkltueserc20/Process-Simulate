## Why

目前模擬器只提供「整線」UPH 與即時統計，無法看出個別站別的產能節奏，也看不出 AGV 到底花多少時間在動作 vs 閒置；而整條製程圖很寬卻只能塞進單一畫面，細節難以檢視。補上這三項能讓使用者更精準地觀察瓶頸與稼動率。

## What Changes

- 各站新增「每小時 UPH」統計：機台每完成一次 cycle 即計入該小時桶，滑鼠移到站上以 tooltip 顯示各小時 UPH（0~1hr、1~2hr…）、當前未滿小時顯示累計值並標註、以及平均值。
- 新增「AGV 稼動看板」全螢幕彈出視窗：累計每台 AGV 的移動 / 取放 / 閒置時間與稼動率，並顯示總計與平均；由 header 新增的按鈕開啟。
- 流程圖（`#fsvg`）新增滾輪縮放（以游標為中心）與拖曳平移，並提供「重置視角」；縮放狀態於切換製程線、切換精簡/詳細視圖時重置。

## Capabilities

### New Capabilities
- `station-uph-tooltip`: 各站每小時 UPH 的資料蒐集與 hover tooltip 顯示。
- `agv-utilization-board`: AGV 動作/閒置時間蒐集與全螢幕稼動看板。
- `flowchart-zoom-pan`: 流程圖 SVG 的滾輪縮放與拖曳平移（攝影機 viewBox 控制）。

### Modified Capabilities
<!-- 無：本次不改變既有 capability 的需求層級行為（皆為新增）。 -->

## Impact

- 單一檔案 `index.html`：
  - 模擬引擎：`updateABFMachines`（站別 cycle 完成處 `:676/:706/:730/:744`）累計站別產出；`stepABFAgv`（`:796`）累計 AGV phase 時間。
  - 渲染：`buildABFSVG`（`:1154`）為各站 `<g id="abf-stn-${i}">` 掛 hover；新增 tooltip 與看板 overlay DOM 與樣式。
  - 視圖：新增 `camera` 狀態與 `#fsvg` 的 wheel/drag 事件；`initSim`（`:641`）與 `setViewMode`（`:1081`）重置 camera。
  - Header（`:119`）新增「AGV 看板」「重置視角」按鈕。
- 無新增外部相依套件；純前端、純記憶體狀態，不影響既有模擬結果。
