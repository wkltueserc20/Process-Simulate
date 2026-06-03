## Why

欣興電子需要一個視覺化工具，讓管理層與客戶能直觀理解 ABF 和 SMK 兩條製程線的 AGV 搬運節拍、WIP 堆積行為與瓶頸位置，取代目前只有靜態數字報表的溝通方式。

## What Changes

- 新增瀏覽器動畫模擬系統（單一 `index.html`，無需安裝）
- ABF 製程線視覺化：6 個站點、5 台 AGV、即時 WIP 顯示
- SMK 製程線視覺化：8 個站點、6 台 AGV、瓶頸自動標紅
- 播放控制：播放 / 暫停 / 快轉(2x, 5x) / 重置
- 可調參數面板：cycle time、AGV 數量、批量大小、目標產量
- 即時 UPH 折線圖（ABF vs SMK 對比）
- 瓶頸警示面板（顯示瓶頸站點、等待時間）

## Capabilities

### New Capabilities

- `simulation-engine`: 離散事件模擬核心，驅動 AGV 狀態機與 WIP 計算
- `factory-floor-view`: SVG 工廠平面圖，包含站點、軌道、AGV 動畫
- `playback-controls`: 播放控制列（時鐘、速度、重置）
- `parameter-panel`: 可收折參數設定側邊欄
- `uph-chart`: 即時 UPH 折線圖（Chart.js）
- `bottleneck-alert`: 瓶頸偵測與視覺警示邏輯

### Modified Capabilities

（無現有 spec）

## Impact

- 新增單一前端專案（`index.html` + 內嵌 CSS/JS）
- 依賴 Chart.js（CDN 引入，不需本地安裝）
- 無後端、無資料庫、無 API
- 產出物可直接用 USB 或 Email 分發給客戶
