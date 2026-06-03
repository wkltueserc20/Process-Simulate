## Why

現行參數面板是一道約 20 欄平鋪的「數字牆」：分組不足、主動作「套用」會被捲走、每次套用都強制完全重置且需手動重按播放、驗證用突兀的 `alert()`、9 個路段時間與平面圖對不起來，且改完參數沒有「變好還變壞」的回饋。對三大使用場景（客戶簡報 / 工程師調參 / iPad 現場）都造成摩擦——尤其它同時是工程師調參與簡報切情境的共同入口，體驗成本被放大。

## What Changes

- **即時生效（方案 B）**：不破壞模擬連續性的參數（AGV 數量、播放速度、WIP 警告/危險閾值、目標產量）改為 hot-apply，不重置；會改變製程結構/節拍的參數（cycle time、處理時間、路段行走時間、取放料時間、每框片數）仍走重新模擬。面板明確標示哪些是「即時」、哪些「需重跑」。**BREAKING**：取代原 `parameter-panel` 規格中「修改任何參數並套用即完全重置」的單一行為。
- **手風琴分區**：製程時間 / 取放料時間 / 路段行走時間 分區並可收合；目標產量與情境選擇器常駐於頂部。
- **固定底部套用列（sticky footer）**：套用鈕永遠可見，含「自動播放」選項與 dirty 狀態指示（參數已變更／畫面正在跑舊參數）。
- **inline 驗證**：錯誤欄位顯示紅框與欄位旁訊息，無效時 disable 套用鈕，取代 `alert()`。
- **觸控友善**：數字欄位加 −/＋ stepper、加大點擊區，適配 iPad。
- **路段時間迷你示意圖**：把 9 個路段數字對應到平面圖佈局；點線段 highlight 對應欄位、改欄位則線段變色。
- **套用前後對比卡**：記住上一輪結果，套用後顯示 UPH／預估完成時間的前後變化（▲▼）。
- **具名情境預設**：將現有單檔匯出/匯入升級為具名情境（理想節拍 / 瓶頸情境 / 客戶現況）下拉切換。

## Capabilities

### New Capabilities
- `scenario-presets`: 具名模擬情境的儲存、切換與匯出/匯入（取代並涵蓋原本零散的單檔參數匯入匯出）。

### Modified Capabilities
- `parameter-panel`: 變更套用行為（新增即時生效一類、保留重新模擬一類）、面板資訊架構（手風琴分區 + 常駐區 + sticky footer）、驗證方式（inline 取代 alert）、新增 dirty 狀態指示、觸控 stepper、路段示意圖與套用前後對比卡。

## Impact

- **檔案**：單一 `index.html`（內嵌 CSS/JS）。受影響區塊：`#sb`/`#sbi` 參數面板 DOM 與樣式、`applyP()`/`resetP()`/`applyParamsToInputs()`/`exportParams()`/`importParams()`，以及與 `P`、`initSim()`、`togglePlay()` 的互動。
- **即時生效**需在不呼叫 `initSim()` 全重置的前提下，將特定參數寫入執行中的 `P` 並調整既有 runtime 狀態（如增減 `abfAGVs`/`smkAGVs`、更新 `P.tg`/`P.ww`/`P.wd`），需釐清各參數的熱套用安全邊界。
- **無新依賴**：維持單檔、無建置步驟、Chart.js 經 CDN。localStorage 用量擴增（單一 `sim-params` → 具名情境集合）。
- **與進行中 change 的重疊**：`parameter-panel` capability 目前定義在尚未 archive 的 `abf-smk-agv-simulation` change（規格為「套用＝完全重置」）。本 change 修改其要求；實作時需與該 change 協調，避免規格衝突。
