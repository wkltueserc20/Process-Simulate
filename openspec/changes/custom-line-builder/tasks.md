> 分 3 期交付，建議逐期 apply／archive。Phase 1 完成即可「加物件建線」，Phase 2 可「匯入 JSON 建線」，Phase 3 才上精靈。

## 1. Phase 1 — Blueprint 單一真相（地基）

- [x] 1.1 擴充 blueprint 欄位：`name`、`flowSummary`、每機台 `procKey`、`segTimes`、選填 `directRoutes`、`batchKey`、pick/drop key、`sinkRole`、選填 `layout` 覆寫
- [x] 1.2 內建 abf／smk 改寫為新 blueprint 格式（含上述欄位）
- [x] 1.3 `genRoute(bp)`：同 xi=0、命中 `directRoutes` 用捷徑、否則相鄰段相加；取代 `routeABF/routeSMK`
- [x] 1.4 `genProcTime(bp)`：依站別 `procKey` 查 `P`；取代 `procTimeABF/procTimeSMK`
- [x] 1.5 由 blueprint 自動產生 param descriptors（proc/seg/count/buffer/AGV/batch/pick-drop）+ 少量手寫覆寫
- [x] 1.6 `DEF` 改為掃描所有 blueprint descriptors 合併預設；`PARAM_META`/面板改為「依當前 LC descriptors」
- [x] 1.7 `#sel-ov` 按鈕、`selectLine` 標題、UPH 列、`curBatch`、`getPickTime/getDropTime` 改為 blueprint 衍生
- [x] 1.8 移除 `routeABF/routeSMK/procTimeABF/procTimeSMK/SMK_ROUTE_KEYS` 及殘留 abf/smk 寫死分支
- [x] 1.9 parity 驗證：abf/smk 以預設值產生的 defs/groups/agv/參數/模擬結果與現行等價
- [ ] 1.10 語法檢查、回歸（設備數量、UPH、看板、縮放、瓶頸、折線圖不受影響）

## 2. Phase 2 — Blueprint JSON 格式與驗證

- [x] 2.1 定義 blueprint JSON schema（含 `schemaVersion`）與序列化/反序列化
- [x] 2.2 自訂線 localStorage 儲存/還原（與 scenario 分開命名空間）
- [x] 2.3 blueprint 匯出為 JSON 檔；匯入 JSON（驗證後加入自訂線清單）
- [x] 2.4 靜態驗證器：起點(refill)/計產站(sink) 存在、群組鏈連續、每組 AGV≥1、xi 連續
- [x] 2.5 乾跑（dry-run）：暫時模擬快轉有限步數，確認有產出/無立即死鎖，失敗回報定位
- [x] 2.6 匯入/驗證/乾跑的錯誤訊息 UI（可定位到群組/站別）
- [x] 2.7 自訂線出現在選擇畫面；重新整理後仍在
- [x] 2.8 語法檢查、驗證/乾跑單元測試（合法/各類不合法案例）

## 3. Phase 3 — 視覺化編輯精靈

- [x] 3.1 精靈 overlay 骨架（多步：①基本 ②站別流程 ③群組 ④時間 ⑤預覽）+ 進度/上一步下一步
- [x] 3.2 步驟②站別流程：依序加站、選 4 原型、設處理時間/緩衝容量、刪除/排序
- [x] 3.3 步驟③群組：框選站別成組、設每組 AGV、指定計產站(sink)
- [x] 3.4 步驟④路段時間：自動預設 + 可編輯（相鄰段/捷徑）
- [x] 3.5 步驟⑤即時預覽：用既有 SVG 引擎 build 該 blueprint 顯示
- [x] 3.6 儲存：執行驗證+乾跑，通過才加入選單與 localStorage，否則顯示錯誤
- [x] 3.7 自訂線管理：編輯（回填精靈）、刪除、匯出；內建線唯讀
- [x] 3.8 佈局自動產生（zones 由 groups、viewBox 由 xiCount、預設單列）+ blueprint 覆寫
- [x] 3.9 端到端：精靈建一條新線 → 模擬出料 → 重整後仍在 → 匯出可再匯入
- [ ] 3.10 語法檢查與整體回歸

## 4. 收尾

- [x] 4.1 README 更新（新增製程線的使用說明：精靈/匯入）
- [x] 4.2 三期分別於 specs/design 補記實作中浮現的決策（如 UPH 單列、乾跑門檻）
