## 1. 站別每小時 UPH 統計（資料層）

- [x] 1.1 `initSim()` 站別初始化時為每站加入 `prodHist: []`（refill/transfer/changer/buffer 皆有）
- [x] 1.2 新增小工具 `recordStnProd(s)`：`s.prodHist[Math.floor(simT/3600)] = (…||0) + curBatch()`
- [x] 1.3 在 refill 完成處（`index.html:676` `s_out=1`）呼叫 `recordStnProd(s)`
- [x] 1.4 在 transfer 完成處（`:706` timer 歸零移框）呼叫 `recordStnProd(s)`
- [x] 1.5 在 changer 計產處（`:730`，與既有 `abfOut.push` 同處）呼叫 `recordStnProd(s)`
- [x] 1.6 在 buffer 框 ready 處（`:744` `fullAvail+=ready`）依 ready 數呼叫累計
- [x] 1.7 驗證快轉（`fastForward` `:913`）路徑也正確累計（共用引擎函式，毋須額外改）

## 2. 站別 UPH tooltip（顯示層）

- [x] 2.1 新增 `#stn-tip` HTML div 與其 CSS（絕對定位、隱藏、可多列）
- [x] 2.2 新增 `stnTipHTML(i)`：依 `prodHist` 組出各小時 UPH 列、當前小時標「← 現在」、平均列；無資料時顯示「尚無產出」
- [x] 2.3 `buildABFSVG()` 建立每個 `abf-stn-${i}` 後掛 `mouseenter/mousemove/mouseleave`，更新並定位 tooltip
- [x] 2.4 驗證滑入/移動/滑出顯示與隱藏正確，且座標跟隨游標

## 3. AGV 時間累計（資料層）

- [x] 3.1 `initSim()` AGV 初始化時加入 `tMove/tHandle/tIdle/tStag`（皆 0）
- [x] 3.2 `stepABFAgv(agv,dt)` 入口依當前狀態累加：`stag>0`→tStag、`picking|dropping`→tHandle、`idle`→tIdle、其餘→tMove
- [x] 3.3 驗證快轉路徑同樣累計

## 4. AGV 稼動看板（顯示層）

- [x] 4.1 新增 `#agv-board` overlay DOM（全螢幕底 + 置中卡片 + 關閉鈕）與 CSS
- [x] 4.2 header 新增「📊 AGV 看板」按鈕，`toggleAgvBoard()` 切換 `agvBoardOpen` 旗標與顯示
- [x] 4.3 `renderAgvBoard()`：表格每車一列（移動/取放/閒置/稼動率 + 長條），加總計列與平均稼動率
- [x] 4.4 `updateUI()` 末端 `if(agvBoardOpen) renderAgvBoard()` 即時刷新
- [x] 4.5 驗證稼動率 = (tMove+tHandle)/(tMove+tHandle+tIdle)，總計/平均正確

## 5. 流程圖縮放 / 平移（攝影機）

- [x] 5.1 新增 `camera` 狀態與 `applyCamera()` / `resetCamera()`；`initSim()`、`setViewMode()` 設定 viewBox 後同步 `camera`
- [x] 5.2 `#fsvg` 加 `wheel`（`{passive:false}`）：以游標 SVG 座標為中心縮放，clamp 比例，`preventDefault`
- [x] 5.3 `#fsvg` 加 `mousedown/mousemove/mouseup` 拖曳平移，用 `dragMoved` 門檻區分點擊
- [x] 5.4 座標換算統一用 `createSVGPoint()` + `getScreenCTM().inverse()`
- [x] 5.5 header 新增「⤢ 重置視角」按鈕呼叫 `resetCamera()`
- [x] 5.6 驗證拖曳不誤觸 hover tooltip；換線/換視圖視角自動重置

## 6. 整合驗證

- [ ] 6.1 ABF 線完整跑一遍：站別 tooltip、AGV 看板、縮放平移皆正常
- [ ] 6.2 SMK 線重複上述驗證
- [x] 6.3 重置 / 切換製程線後三項統計與視角皆歸零（initSim 重置 prodHist/AGV 計時、syncCamera 重置攝影機）
- [x] 6.4 確認既有模擬數值（產出、UPH、瓶頸、折線圖）不受影響（變更皆為附加，未改既有流程）
