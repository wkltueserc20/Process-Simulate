## Why

目前每條製程線的「機台數量」（如 ABF壓機 4 台、CZ 2 台、換框機 2 台）是寫死在 `ABF_DEFS / SMK_DEFS` 等常數陣列裡，要改數量必須改原始碼。使用者希望能**直接在頁面上增減各站的機台數量**，以便快速試算「多／少幾台設備對產能與瓶頸的影響」，這正是模擬器最核心的價值。

由於每種機台型別都固定對應某一個搬運 group（例：壓機固定屬 group 3 的目的端、CZ 固定屬 group 1 的目的／group 2 的來源），增減機台只發生在「同一個 group 內」，不需要重新連接跨 group 的任務接線——這讓本變更的範圍可控。

## What Changes

可在頁面調整的有三類（皆為 resim 重模擬類參數）：

- **並聯機台數量**（transfer 與 changer 型別）：每種型別一個數量設定，**範圍 1–9**。
  - ABF：CZ、壓機、換框機
  - SMK：CZ、壓機、曝光、顯影、UVQ、換框機
  - 機制：在同一 `xi` 增／減 `sub` 站；型別固定屬同一 group，僅在組內增減。
- **緩衝站容量**（buffer 型別的 `buf` 上限）：Annealing／Baking／Postcure／SoftBaking 各自的容量可調，**最少 1**。
  - 機制：改寫該緩衝站的 `buf` 值，不改變站別數量。
- **每組 AGV 數量**：每個搬運 group 的 AGV 台數可調，**範圍 1–9**。
  - 機制：依數量產生 `agvInits`（自動分配起始站與錯峰 `stag`）。

其餘配套：

- 將原本寫死的設定改為**依數量產生**：站別清單（含 `xi/sub` 與 `buf`）、各 group 的 `src/dst` 索引、`layout` 分區與標籤、`viewBox` 高度，皆由數量參數動態生成。
- 在參數面板新增「設備數量」分區，使用既有 stepper（＋／−）控制，套用方式為 **resim**；按「套用」後透過既有 `initSim()` 全量重建站別／AGV／流程圖。
- 機台堆疊超出畫面時，自動加高 `viewBox` 以容納（並可搭配既有的流程圖縮放／平移檢視）。
- 移除已不再使用的死碼 `getGroup1/2/3Tasks` 與 `CZ_INDICES / PRESS_INDICES / CHANGER_INDICES`（現行引擎走 `genGroupTasks(LC.groups)`，這些常數已無作用且會與動態數量衝突）。

CLN（refill）維持固定 1 台（不在本次可調範圍）。

## Capabilities

### New Capabilities
- `equipment-count-config`: 各站機台數量與每組 AGV 數量的設定、由數量動態產生站別／群組／佈局、以及對應的參數面板 UI 與 resim 套用流程。

### Modified Capabilities
<!-- 既有能力的「需求層級」行為不變（站別引擎、任務分配、瓶頸、UPH 等邏輯不改），僅資料來源由寫死改為動態產生，屬實作層變更，故不列為 Modified。 -->

## Impact

- 單一檔案 `index.html`：
  - **設定來源**：`ABF_DEFS`/`SMK_DEFS`（`:289`/`:415`）、`ABF_AGV_INITS`/`SMK_AGV_INITS`（`:312`/`:437`）、`LINES[].groups`（`:356`）、`LINES[].layout`（`:361`）改為由產生器函式依數量輸出。
  - **重建流程**：`initSim()`（`:577`）已全量重建，預期僅需在重建前重新產生 `LC.defs/agvInits/groups/layout`。
  - **路段／處理時間**：`routeABF/routeSMK`（依 `xi`）與 `procTime*`（依站別型別）**不需更動**——同一站多台機台距離為 0。
  - **參數面板**：`FIELDS` 參數中繼資料（`:1950` 起）新增數量欄位與「設備數量」section；沿用 `apply:'resim'`、stepper、scenario 匯入／匯出。
  - **死碼移除**：`getGroup1/2/3Tasks`（`:506–561`）與三個 `*_INDICES`（`:303–305`）。
- 無新增外部相依；純前端、純記憶體狀態。
- 既有 scenario JSON 無數量欄位 → 需以預設值（現行台數）做 fallback，確保相容。

## 已確認決策（Resolved）

1. **可調站別範圍**：transfer（CZ/壓機/曝光/顯影/UVQ）與 changer（換框機）→ 並聯機台數可調；緩衝站（Annealing/Baking/Postcure/SoftBaking）→ 容量（`buf`）可調，最少 1。CLN 維持固定 1 台。
2. **AGV 數量**：本次一併納入——每組 AGV 數量可調。
3. **數量上下限**：並聯機台數與每組 AGV 數量皆為 **1–9**；緩衝站容量最少 1（上限從寬，沿用合理預設）。
