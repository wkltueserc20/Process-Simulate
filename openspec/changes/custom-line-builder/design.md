## Context

`index.html` 目前以 `BLUEPRINTS.{abf,smk}`（spine／groups／layout）＋產生器（`genDefs/genGroups/genAgvs/genLayout/buildLineConfig`）動態產生站別/AGV/群組/佈局，`initSim()` 全量重建。但「一條線」仍有約 8 處寫死：`routeABF/routeSMK`、`procTimeABF/procTimeSMK`、`DEF`、`PARAM_META`、`#sel-ov` 按鈕、`selectLine` 標題/UPH 列、`curBatch`、`getPickTime/getDropTime`。引擎站別行為固定為 4 種原型：refill／transfer／buffer／changer。

目標：讓「新增製程線 = 頁面精靈填表」。本設計分 3 期，地基（L1）先把 8 處收斂為單一 blueprint，格式（L2）加 JSON/驗證，UI（L3）加精靈。

## Goals / Non-Goals

**Goals:**
- 一條線完全由 blueprint 資料描述；route/procTime/參數/面板/選單/標題/batch/取放料全部衍生。
- 提供 blueprint JSON schema、匯入/匯出、合法性驗證與乾跑。
- 頁面精靈可新增/編輯/刪除自訂線，存 localStorage，立即模擬。
- 佈局自動為主（zones 由 groups 推、viewBox 由 xiCount 算）可覆寫。
- abf/smk 改以 blueprint 表示且與現行模擬等價（parity）。

**Non-Goals:**
- 不新增站別原型（只組合既有 4 種）。
- 不改既有 slot/PORT 引擎、任務分配、瓶頸、UPH、縮放等核心邏輯。
- 不做雲端同步/多人協作（自訂線僅 localStorage + 檔案匯出）。
- 不做任意非線性流程拓樸（以線性鏈為主；分歧/合流非本次目標）。

## Decisions

### D1（Phase 1）blueprint 擴充為「線的唯一真相」
blueprint 增補欄位：`name`、`flowSummary`（選單摘要）、每機台 column 的 `procKey`、`segTimes`（相鄰段 xi k↔k+1 的 param key 或預設值）、`directRoutes`（選填捷徑 `{a_b: key/值}`）、`batchKey`、`pick/drop key`、`sinkRole`（計產站角色）、選填 `layout` 覆寫。內建 abf/smk 改寫成此格式。

### D2（Phase 1）通用 route / procTime
- `genRoute(bp)` 回傳 `(i,j)=>` 距離：同 xi=0；有 `directRoutes` 命中用之；否則相鄰段相加（讀 `P[segKey]`）。取代 `routeABF/routeSMK`。
- `genProcTime(bp)` 回傳 `(d)=>` 依該站 column 的 `procKey` 查 `P`。取代 `procTimeABF/procTimeSMK`。
- 兩者掛到 `LC.route/LC.procTime`，引擎呼叫點不變。

### D3（Phase 1）參數系統一般化 —— 採「混合」（已定案）
- **機械性欄位自動產生**（proc time／count／buffer cap／AGV／pick-drop／batch）：由 spine／groups 程式化推出 descriptor。標籤用規則自動產生（proc：refill→「生產時間」、changer→「拿料時間」、transfer→「處理時間」；count→「{名} 機台數」；buffer→「{名} 容量」）。
- **需人讀標籤的欄位由 blueprint 宣告**：路段（segTimes／directRoutes）改帶 `label`；少量機械欄位可選 `*Label`/`*Unit` 覆寫以對齊現行文字。
- **預設值（DEF）策略**：內建線的 `DEF` **維持為扁平預設物件不動**（降低 parity 風險）；自訂線的預設值住其 blueprint，於載入時 merge 進 `DEF`（`DEF={...BASE_DEF, ...collectBlueprintDefaults()}`）。
- `PARAM_META = buildParamMeta()`（掃描 blueprints 產生，仍帶 `line` tag，沿用既有 `data-line` 篩選與所有消費函式，介面不變）。
- **parity 安全網**：以 harness 比對「產生的 descriptors」與現行手寫 `PARAM_META` 的 **key／default／section／apply／min／max**（標籤文字差異可接受，屬 cosmetic）。
- 連帶：`buildRouteMap`（路段示意圖）亦需改讀 blueprint 的路段宣告 → 與本步一起處理。

> 註：此步（1.5/1.6）影響參數面板渲染，建議於有瀏覽器可目視的情況下進行與驗證；本次 session 先交付並驗證了 route/procTime/選單/標題/batch/取放料 的一般化（1.1–1.4＋部分 1.7/1.8）。

### D4（Phase 1）選單/標題/batch/取放料衍生
- `#sel-ov` 按鈕由迭代 blueprints 動態產生（name + flowSummary）。
- `selectLine(id)` 通用：標題取 `bp.name`；UPH 列改為單一通用列（不再 abf/smk 兩列硬切）。
- `curBatch` 讀 `P[LC.batchKey]`；`getPickTime/getDropTime` 讀 `LC` 上的 pick/drop key（依站別 buffer/machine 分流）。

### D5（Phase 2）JSON schema + 持久化
- 定義 blueprint JSON（含 `schemaVersion`）。內建線只讀；自訂線存 `localStorage`（與 scenario 分開命名空間）。
- 匯出＝blueprint JSON 檔；匯入＝驗證後加入自訂線清單。

### D6（Phase 2）驗證 + 乾跑
- 靜態驗證：站別至少 1 個 refill 起點與 1 個 sink（changer）；groups 串成鏈（首組 src=refill、組間 dst→下一組 src 相符、末組 dst=sink）；每組有 AGV≥1；xi 連續。
- 乾跑（dry-run）：以該 blueprint 建一個暫時模擬、快轉有限步數，確認有產出且無立即死鎖；失敗回報定位（哪一組/哪一站）。

### D7（Phase 3）精靈 = 產生 blueprint 的 UI
- 多步 overlay：①基本（id/name/batch）②站別流程（依序加站、選原型、設 proc/buf）③群組（框選站別成組、設 AGV、指定 sink）④路段時間（預設＝合理值，可改）⑤預覽（用既有 SVG 引擎即時 build）＋儲存。
- 精靈輸出即 D5 的 blueprint JSON；存檔→驗證/乾跑→加入選單。
- 編輯＝載入既有 blueprint 回填精靈；刪除＝移除自訂線。

### D8（Phase 3）佈局自動為主可覆寫
- `zones` 由 groups 的 xi 範圍推得；`viewBox` 寬＝`SX + (xiCount-1)*gap + 邊距`、高沿用既有自動加高；預設單列。
- blueprint 可選填 `layout` 覆寫（zones/gap/viewBox/serpentine/simple）。蛇形排版列為選配，預設單列＋既有縮放/平移。

## Risks / Trade-offs

- [參數系統一般化破壞既有面板] → 先做 abf/smk descriptors 與現行 `PARAM_META` 的等價測試，再切換。
- [自訂線配置無效→卡死] → Phase 2 驗證 + 乾跑為硬性前置；精靈存檔必過。
- [自動佈局在複雜流程不佳] → 提供覆寫；蛇形選配；靠既有縮放/平移兜底。
- [localStorage 容量/遺失] → 提供匯出 JSON 備份/分享；schemaVersion 應對格式演進。
- [parity 回歸] → 內建 abf/smk 轉 blueprint 後，需與現行預設值模擬結果比對。
- [一次做太多] → 嚴格分期，Phase 1 完成即可「加物件建線」、Phase 2 可「匯入 JSON 建線」、Phase 3 才上精靈；逐期 apply/archive。

## Migration Plan

1. Phase 1：擴充 blueprint + 通用 route/procTime + 參數一般化 + 選單/標題衍生；abf/smk 轉為 blueprint，跑 parity 驗證後移除舊函式。
2. Phase 2：加 schema/匯入匯出/驗證/乾跑（不影響內建線）。
3. Phase 3：加精靈 UI 與自訂線管理。
每期獨立 apply 與 archive；任一期可暫停而不破壞既有功能。

## 實作備註（Phase 1–3 完成）

- **Phase 1**：route/procTime 通用化（`genRoute`/`genProcTime` 由 blueprint `routeParams`/`procKey`）、`PARAM_META` 改 `buildParamMeta()` 產生（DEF 維持預設來源）、選單/標題/UPH 圖表與統計/路段示意圖全部 blueprint 衍生。已用 harness 驗 route/procTime parity 與 PARAM_META 69-key functional parity。
- **Phase 2**：自訂線存 `localStorage`（`sim-custom-lines`）、`registerCustomLine` 合併 defaults 進 DEF/P、`validateBlueprint`（起點/計產站/群組鏈/xi 連續，已測）、`dryRunBlueprint`（暫存全域→快轉→還原，headless `buildSimState`）、`exportBlueprint`/`importLineFile`、`addCustomLine`（驗證+乾跑）。
- **Phase 3**：以**單一表單式精靈**（非分步 stepper，較易維護且無瀏覽器亦低風險）。**群組自動推導**：機台站(refill/transfer/changer)為 stop、兩 stop 間的 buffer 為 mid——已驗證可重現 ABF/SMK 既有分組。`wzCompile` 產生完整 blueprint（key 以 line id 命名空間、defaults map）。

### 與原設計的差異（deviations）
- 精靈採單表單而非 ①~⑤ 分步畫面（功能等價）。
- 「即時預覽」以**結構摘要 + 乾跑結果**呈現（`驗證／乾跑` 按鈕），未即時重繪 SVG；實際視覺於儲存後從選擇畫面進入該線檢視。
- 群組由站別順序自動推導（使用者不需手動框選），降低誤設與 UI 複雜度；代價是「兩 stop 間最多一個 buffer」的限制。
- 計產站(sink)自動取最後一個 changer。

## Open Questions

- UPH 列由「abf/smk 雙列」改為「當前線單列」是否可接受（簡化但改變現有呈現）？
- 自訂線是否需要可調色（zone 配色）或沿用內建配色輪替即可？
- 乾跑的判定門檻（模擬多少步、最低產出數）取多少合適？
