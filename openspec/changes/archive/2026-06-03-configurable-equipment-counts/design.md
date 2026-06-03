## Context

`index.html` 目前每條線的設定是寫死常數：`ABF_DEFS/SMK_DEFS`（站別含 `xi/sub/buf`）、`ABF_AGV_INITS/SMK_AGV_INITS`（AGV）、`LINES[].groups`（任務群組的 `src/dst/mid` 絕對索引）、`LINES[].layout`（分區/標籤/viewBox）。`initSim()` 會依當前 `LC`（line config）**全量重建** `abfSt`、`abfAGVs`、SVG；`computeGeom()` 由 `xi/sub` 自動算座標；`route*` 依 `xi` 查表（同 `xi` 距離 0）；任務分配走資料驅動的 `genGroupTasks(LC.groups)`。`getGroup1/2/3Tasks` 與 `*_INDICES` 為死碼。

關鍵前提（使用者確認）：**每種機台型別固定屬於同一個 group**，增減機台只在組內發生，不需重連跨組接線。

## Goals / Non-Goals

**Goals:**
- 頁面上可調：並聯機台數（CZ/壓機/曝光/顯影/UVQ/換框機，1–9）、緩衝站容量（buf，最少 1）、每組 AGV 數量（1–9）。
- 將寫死的 defs/agvInits/groups/layout 改為由數量參數**動態產生**；套用走既有 resim → `initSim` 重建。
- 機台堆疊超出畫面時自動加高 viewBox。
- 移除死碼 `getGroup1/2/3Tasks`、`*_INDICES`。

**Non-Goals:**
- 不新增/刪除「製程站別本身」（新 `xi`、新路段）——僅調整既有站別的並聯數/容量。
- 不改既有引擎邏輯（站別 slot 機制、任務分配、瓶頸、UPH、縮放）。
- CLN（refill）維持固定 1 台。
- 不改 `route*` 與 `procTime*`（同 `xi` 多台距離 0、處理時間依型別）。

## Decisions

### D1：以「藍圖 + 產生器」取代寫死陣列
為每條線定義一份**藍圖（blueprint）**：站別脊椎（每個 `xi` 一個站的型別、名稱前綴、所屬 group 角色、是否可數、buf 參數鍵）＋ group 的型別接線（src 型別 → mid 緩衝 → dst 型別）。新增 `buildLineConfig(lineId, P)` 依藍圖 + `P` 的數量參數產出完整 `LC`（defs/agvInits/route/procTime/groups/layout）。`initSim()` 改呼叫 `buildLineConfig(selectedLine, P)` 取代靜態 `LINES[id]`。
- 為何：數量一變索引就位移，唯有「集中產生 + 由型別推導索引」才能保持 `groups.src/dst` 正確。

### D2：defs 產生規則
依藍圖脊椎（`xi` 遞增）逐站展開：
- 可數機台型別（transfer/changer）：產生 `count` 份，`sub = 0..count-1`，名稱 `前綴_{n}`（單台時可省略後綴或維持 `_1`，與現況一致用 `_1` 起算）。
- 緩衝站：1 份，`buf = P[bufKey]`（clamp ≥1）。
- refill（CLN）：固定 1 份。
- 產出後建立 `byType`（型別→defs 索引陣列）與 `bufIndexOf(role)`，供 groups 使用。

### D3：groups 由型別接線解析為索引
藍圖中每個 group 以「型別角色」描述（例 ABF：G1 src=refill, mid=Annealing, dst=CZ；G2 src=CZ, mid=Baking, dst=Press；G3 src=Press, mid=Postcure, dst=Changer）。產生器把角色轉成實際索引集（`byType`），其餘欄位（priA/priB/midSpace）沿用現值。
- 為何：機台數變動時 `src/dst` 自動跟著對。

### D4：agvInits 由每組數量產生
每個 group 一個 AGV 數量參數。產生器為每組產出 N 台：`id` 沿用 `A1,A2,…` 全線連號；起始 `pos` 在該組相關站別間輪流分配；`stag` 以固定間隔（如 30s）遞增；`grp` 為組號。
- 為何：增配 AGV 才能餵飽新增機台，否則 AGV 成為瓶頸（雖真實但需可調）。

### D5：layout 動態化
- `zones`（依 `xi` 範圍）與分組結構**不變**（型別所屬 group 固定）。
- `agvLbls` 由各組 AGV id 產生。
- `viewBox` 高度 = `ABF_SY + (maxSubAcrossColumns-1)*SUB_SPACING + 底部邊距`，自動加高以容納最多並聯機台；寬度不變。
- camera（縮放/平移）於 `initSim` 既有 `syncCamera()` 會以新 viewBox 為基準，無需額外處理。

### D6：參數面板新增「設備數量」section
於 `FIELDS` 中繼資料新增數量欄位（`type:'int', apply:'resim', section:'equip', min:1`，機台/AGV `max:9`，buffer 容量 `max` 從寬如 99）。沿用既有 stepper、validate、scenario 匯入/匯出。`DEF` 補上所有新鍵的預設值（= 現行台數/容量），確保**舊 scenario JSON 無此鍵時 fallback 正確**（`P={...DEF,...loaded}`）。

### D7：移除死碼
刪 `getGroup1/2/3Tasks`（`:506–561`）與 `CZ_INDICES/PRESS_INDICES/CHANGER_INDICES`（`:303–305`）。確認無其他引用（已查證僅死碼互相引用）。

## Risks / Trade-offs

- [索引位移導致 groups 錯接] → 一律由 `byType` 推導，禁止寫死索引；加開發期 assert（數量=預設時，產生的 defs/groups 應與現行等價）。
- [AGV 起始站分配不當造成初期卡頓] → 沿用「在組內站別輪流 + 遞增 stag」策略，與現值對齊；數量=預設時應重現現有 AGV 配置。
- [viewBox 過高導致站別變小] → 自動高度僅在超過原高(870)時才加高；平時維持原比例，並有縮放/平移可用。
- [參數數量暴增、面板變長] → 集中在「設備數量」可收合 section；標示 resim。
- [scenario 相容] → `DEF` 提供所有新鍵預設值；`applyParamsToInputs` 對缺鍵用預設。
- [效能] → 機台×AGV 上限 9，站數與 AGV 數仍在數十量級，逐幀更新可負荷。
