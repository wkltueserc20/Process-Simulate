## Why

目前要新增一條製程線（如未來的第三條線）必須改動約 8 處寫死的程式：route 函式、procTime 函式、`DEF` 參數、`PARAM_META` 欄位、選擇畫面按鈕、`selectLine` 標題/UPH 列、`curBatch`、取放料 key——容易遺漏且只有工程師能做。使用者希望能在**頁面上以編輯精靈快速建立新製程線**，現場人員不必碰程式碼即可定義站別、群組、時間並立即模擬。

設備數量變更（上一個變更）已將站別/AGV/群組/佈局改為由 blueprint 動態產生，地基已具雛形。本變更把「一條線」完全收斂成單一 blueprint（資料），並在其上建立 JSON 格式與視覺化編輯精靈，使「新增製程線 = 填一份精靈表單」。

## What Changes

採**分 3 期**交付，每期可獨立上線：

### Phase 1 — Blueprint 單一真相（地基 / L1）
- 把目前每線寫死的 route、procTime、參數預設、面板欄位、選擇畫面、標題/UPH/batch/取放料，全部改為**從 blueprint 衍生**。
- 通用 `genRoute`（讀 `segTimes` 相鄰段 + 選填 `directRoutes` 捷徑）取代 `routeABF/routeSMK`。
- 通用 `genProcTime`（每機台 column 帶 `procKey`）取代 `procTimeABF/procTimeSMK`。
- `DEF` 與 `PARAM_META` 由掃描所有 blueprint **自動產生**；選擇畫面按鈕、標題、UPH 列、`curBatch`、取放料 key 改讀當前 `LC`。
- 成果：新增一條線 = 在 `BLUEPRINTS` 加一個物件，其餘自動衍生。

### Phase 2 — Blueprint JSON 格式與匯入/驗證（格式 / L2）
- 定義 line blueprint 的 JSON schema（含版本欄位）。
- 提供 blueprint 的**匯出/匯入**（沿用既有 scenario 匯入匯出習慣），自訂線存於 `localStorage`。
- 加入**驗證與乾跑（dry-run）**：群組鏈合法性（首組來源為補料、末組目的為計產站、組間銜接）、計產站指定、是否會出料；不合法給明確錯誤提示。
- 成果：不改程式碼即可用一份 JSON 新增/分享一條線。

### Phase 3 — 視覺化編輯精靈（UI / L3）
- 頁面上「新增製程線」精靈：①基本資料 ②站別流程（依序加站、選 4 種原型、設處理時間/緩衝容量）③搬運群組（框選站別成組、設每組 AGV、指定計產站）④路段時間（可自動預設）⑤即時預覽 → 命名儲存。
- 儲存後自動出現在選擇畫面，立即可模擬；可再編輯/刪除/匯出。
- 佈局**自動為主、可覆寫**：zones 由 groups 推、viewBox 寬由 `xiCount×gap` 算、預設單列（靠既有縮放/平移檢視），blueprint 可選填覆寫。

約束（明確界定，避免期待落差）：精靈是以引擎既有 **4 種站別原型**（refill／transfer／buffer／changer）組合，不可發明新的站別行為。

## Capabilities

### New Capabilities
- `line-blueprint-model`: 一條線完全由 blueprint 資料描述，route/procTime/參數/選單/標題等皆由其衍生（Phase 1）。
- `line-blueprint-io`: blueprint JSON schema、匯入/匯出、合法性驗證與乾跑（Phase 2）。
- `line-builder-wizard`: 頁面上的視覺化新增/編輯製程線精靈與即時預覽（Phase 3）。

### Modified Capabilities
<!-- 無需求層級變更。`equipment-count-config` 的設備數量參數/欄位改為「依當前 blueprint 動態產生」屬實作層變更，觀察行為等價，故不列為 Modified（見 Impact）。 -->


## Impact

- 單一檔案 `index.html`：
  - **Phase 1**：新增 `genRoute/genProcTime`；`BLUEPRINTS` 擴充 `segTimes/directRoutes/procKey/defaults/name`；`DEF`、`PARAM_META`、`SECTIONS`、`#sel-ov`、`selectLine`、`curBatch`、`getPickTime/getDropTime` 改為 blueprint 衍生；移除 `routeABF/routeSMK/procTimeABF/procTimeSMK/SMK_ROUTE_KEYS`。
  - **Phase 2**：新增 blueprint 序列化/反序列化、schema 驗證、dry-run 驗證器；`localStorage` 自訂線儲存；匯入/匯出 UI。
  - **Phase 3**：新增精靈 overlay（多步表單）＋即時預覽（重用既有 SVG 引擎）＋自訂線管理（編輯/刪除）。
- 參數面板需一般化為「依 blueprint 動態長欄位」（影響 `renderParamPanel` / `PARAM_META` 使用方式）。
- 無新增外部相依；純前端、純記憶體 + localStorage。
- 既有 abf/smk 線改以 blueprint 表示，須確保模擬結果與現行等價（parity）。

## 風險 / 注意

- **群組鏈合法性**：錯誤配置會卡死不出料 → Phase 2 的驗證/乾跑為必要安全網。
- **參數系統一般化**：靜態 `PARAM_META` → 動態，是 Phase 1 最大改動，須維持既有欄位行為。
- **佈局自動化邊界**：站數很多或非線性流程時，自動佈局可能不理想 → 提供覆寫；蛇形排版列為選配。
- **既有情境相容**：自訂線參數住 blueprint，舊 scenario JSON 需 fallback。
- **parity**：abf/smk 轉成 blueprint 後，預設值模擬結果須與現行一致。
