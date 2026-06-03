## Context

整個模擬器是單一檔案 `index.html`（約 2146 行），純前端、狀態全在記憶體：`simT`（模擬秒數）、`abfSt[]`（各站）、`abfAGVs[]`（各車）、`abfOut[]`（整線產出時間戳）。動畫迴圈 `tick(rt)` 以 `dt` 推進 `updateABFMachines(dt)` 與 `stepABFAgv(a,dt)`，再 `updateUI()`。流程圖是一個 `<svg id="fsvg">`，站別為 `<g id="abf-stn-${i}">`、AGV 為 `<g id="agv-${id}">`，viewBox 於 `initSim()` 依 `LC.layout` 設定。本設計在不改變既有模擬數值行為的前提下，新增三項觀測功能。

## Goals / Non-Goals

**Goals:**
- 各站每小時 UPH 蒐集 + hover tooltip，當前小時顯示累計並標註。
- 全螢幕 AGV 稼動看板：移動 / 取放 / 閒置 / 稼動率，即時更新。
- 流程圖滾輪縮放（游標為中心）+ 拖曳平移 + 重置；切換線/視圖時重置。

**Non-Goals:**
- 不改動模擬引擎的派工、時序與既有產出計算邏輯。
- 不引入任何外部相依（除既有 Chart.js）。
- 不持久化（不寫 localStorage）這些統計；重置即清空。
- 精簡（simple）視圖不一定要支援站別 hover tooltip（以詳細視圖為主）。

## Decisions

### D1：站別產出用「每小時桶陣列」而非時間戳陣列
每站新增 `s.prodHist = []`，於機台 cycle 完成處執行 `s.prodHist[Math.floor(simT/3600)] = (…||0) + curBatch()`。
- 完成點：refill `index.html:676`（`s_out=1`）、transfer `:706`（timer 歸零移框）、changer `:730`（已有 `abfOut.push`，同處加記）、buffer `:744`（框 ready）。
- 為什麼選桶不選時間戳：桶天然對應「0~1hr / 1~2hr」顯示，記憶體 O(小時數) 而非 O(完成次數)，且 tooltip 不必每次重算窗口。
- 已完成整點小時的 UPH = 該桶片數；當前小時（`floor(simT/3600)` 那桶）顯示累計值並標「← 現在」。平均 = 全部桶總和 ÷ 已涵蓋小時（當前未滿小時按比例 `simT%3600/3600` 計，避免被未滿小時拉低）。

### D2：tooltip 用單一 HTML div，事件掛在站 `<g>`
在 `buildABFSVG()` 建立每個 `abf-stn-${i}` 後，`addEventListener('mouseenter'/'mousemove'/'mouseleave')`；mousemove 更新一顆固定的 `#stn-tip` div 內容與位置（跟游標）。
- 為什麼用 HTML div 而非 SVG `<title>`：可即時、可排版多列、樣式可控；`<title>` 原生提示延遲且樣式不可控。
- 內容由站索引動態組出，重用 `s.prodHist`，不需新資料結構。

### D3：AGV 時間累計放在 `stepABFAgv` 入口
每台車新增 `tMove / tHandle / tIdle / tStag`（初始 0）。在 `stepABFAgv(agv,dt)` 一進入即依「當前 phase / stag」把 `dt` 加到對應欄位（在 phase 可能轉換前先計），分類：
- `stag>0` → `tStag`；`picking|dropping` → `tHandle`；`idle` → `tIdle`；其餘（`toPickup|delivering`）→ `tMove`。
- 動作時間 = `tMove + tHandle`；稼動率 = 動作 ÷（動作 + `tIdle`）。`tStag` 為啟動錯峰，列為獨立欄位不計入稼動率分母（避免汙染）。

### D4：看板用 overlay，仿既有 `#ov` 模式
新增 `#agv-board`（fixed 全螢幕半透明底 + 置中卡片），header 加「📊 AGV 看板」鈕 `toggleAgvBoard()`。開啟時設旗標 `agvBoardOpen=true`，在 `updateUI()` 末端 `if(agvBoardOpen) renderAgvBoard()` 即時刷新表格與稼動率長條。關閉清旗標。
- 為什麼即時刷新綁在 `updateUI`：沿用既有每幀刷新管線，無需另開 timer。

### D5：縮放/平移用 viewBox 攝影機，獨立 `camera` 狀態
新增 `let camera=null`（`{x,y,w,h}`）。`initSim()`/`setViewMode()` 設定 viewBox 後，同步 `camera` = 解析後的預設 viewBox。
- `wheel`：以游標的 SVG 座標為中心，`w,h *= factor`（factor≈1.1/0.9），調整 `x,y` 使游標點不動；clamp 縮放比例（如 0.3x~5x，相對預設）。`preventDefault` 防頁面捲動。
- 拖曳：`mousedown` 記起點與 `camera`，`mousemove` 以「Δ螢幕像素 × (camera.w/svgClientW)」換算成使用者座標位移更新 `x,y`；`mouseup` 結束。設 `dragMoved` 門檻區分點擊/拖曳，避免吃掉 hover。
- 套用：`applyCamera()` 寫 `fsvg.setAttribute('viewBox', …)`。「重置視角」鈕呼叫 `resetCamera()` 重設回預設並 `applyCamera()`。

### D6：座標換算統一用 `getScreenCTM().inverse()`
滾輪縮放中心與拖曳位移皆需螢幕↔SVG 座標換算，統一用 SVG `createSVGPoint()` + `getScreenCTM().inverse()`，避免手算 client 邊界誤差。

## Risks / Trade-offs

- [拖曳與 hover tooltip 互搶事件] → 用 `dragMoved` 像素門檻；拖曳中暫時隱藏 tooltip，放開後恢復。
- [滾輪 `preventDefault` 影響頁面] → 只在 `#fsvg` 上監聽，且事件需 `{passive:false}` 才能 preventDefault。
- [每小時桶在超長模擬下成長] → 桶數 = 小時數，數十筆級別，可忽略；tooltip 只列有資料的桶。
- [切換精簡↔詳細時 hover 目標不同] → 站 hover 以詳細視圖為主；精簡視圖站元素 id 不同，tooltip 綁定需判斷 `viewMode`，或精簡視圖暫不提供（Non-Goal 已聲明）。
- [`fastForward`/`立即結果` 不走逐幀 tick] → 站別桶與 AGV 時間在快轉路徑也需累計（快轉 `step()` 迴圈內同樣呼叫引擎函式即可涵蓋；需確認 `fastForward` 走的是同一組引擎函式）。
