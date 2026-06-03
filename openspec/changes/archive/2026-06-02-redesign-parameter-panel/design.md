## Context

`index.html` 是單檔、無框架、無後端的瀏覽器模擬器。參數面板（`#sb`/`#sbi`）目前把約 20 個輸入欄位平鋪，套用流程是 `applyP()` → 寫入全域 `P` + localStorage → `resetSim()` → `initSim()` 全重置，使用者須再手動 `togglePlay()` 才會動。驗證用 `alert()`。參數匯出/匯入是單檔 JSON。

runtime 狀態關鍵物件：`P`（參數）、`abfAGVs`/`smkAGVs`（AGV 陣列，含位置/相位/載貨）、`abfSt`/`smkSt`（站點狀態）、`abfOut`/`smkOut`/`abfPcs`/`smkPcs`（產出累計）、`simT`/`running`/`spd`。動畫由 `tick()` 的 rAF 迴圈驅動。

使用者已明確選擇**方案 B（部分參數即時生效）**，並要求三大場景兼顧（客戶簡報 / 工程師調參 / iPad 現場）。

## Goals / Non-Goals

**Goals:**
- 把「數字牆」重整為可掃描的分區面板，主動作永遠可達。
- 讓不破壞模擬連續性的參數即時生效，免去「重置→重按播放」的摩擦。
- 提供改參數的即時回饋（前後對比）與安全網（inline 驗證、dirty 指示）。
- 觸控友善，適配 iPad 現場操作。
- 具名情境讓簡報能一鍵切換敘事。

**Non-Goals:**
- 不改動模擬引擎的物理/排程邏輯（AGV 狀態機、slot 邏輯維持原樣）。
- 不引入框架、建置步驟或後端；維持單檔 + Chart.js CDN。
- 不新增 ABF 可變 AGV 數量（目前 ABF AGV 由 `ABF_AGV_INITS` 固定為 7 台；列為未來工作）。
- 不做完整 timeline 回放/scrubbing（另案）。

## Decisions

### D1. 熱套用 vs 重新模擬的參數分類

依「是否破壞模擬連續性」二分，面板上以標籤明示（⚡即時 / ↻需重跑）：

| 類別 | 參數 | 套用方式 |
|------|------|----------|
| ⚡ 即時（hot-apply） | 目標產量 `tg`、SMK WIP 警告 `ww`、SMK WIP 危險 `wd`、SMK AGV 數量 `sa` | 寫入執行中 `P` + 調整 runtime，不重置、不中斷播放 |
| ↻ 需重跑（re-sim） | 每框片數 `abfBt`、`clnTime`、`czTime`、`pressTime`、`changerTime`、所有路段 `t01..t46`、取放料 `pick/drop/bufpick/bufdrop`、SMK `sc`、`bt` | 維持原 `applyP` → 全重置流程 |

理由：`tg`/`ww`/`wd` 僅影響顯示/判定（進度分母、WIP 上色與瓶頸偵測），改值零風險。`sa` 影響 `smkAGVs` 陣列，但 SMK AGV 彼此獨立、無跨車狀態，可安全增減（見 D2）。其餘參數會改變站點處理節拍或路段拓樸，中途變更會使 in-flight 的計時器/距離語意不一致，故仍須重跑。

替代方案：全部即時（方案 A 的反面）——否決，因節拍類參數中途變更讓數據無法解讀。全部重跑（現狀）——否決，即為本案要解決的摩擦。

### D2. SMK AGV 數量熱增減策略

`smkAGVs` 由 `P.sa` 與 `smkSeg` 推導，每台僅持有 `{fi,ti,seg,pos,phase,waitStagger}`，無跨車耦合。
- **增加**：append 新 AGV，`phase:'waiting'`、`waitStagger` 給一個錯開值，立即進入排程。
- **減少**：優先移除 `phase==='waiting'` 且未載貨者；若不足，標記「收車中」待其完成當前 leg（送達 sink 或入庫）後移除，避免吞掉在途批量造成產出帳不平。
- 同步 rebuild SMK AGV 的 SVG（`buildSMKAgvSVG()`）。

### D2 風險邊界：ABF 不在此列。ABF AGV 為固定編組（A1–A7 綁 grp），動態增減牽涉任務分配與編組顏色，超出本案範圍。

### D3. 資訊架構：常駐區 + 手風琴 + sticky footer

```
[情境選擇器 ▾]        ← 常駐頂部
[目標產量]            ← 常駐
▼ ⏱ 製程時間  (↻)
▶ 🤖 取放料時間 (↻)   ← 預設收合
▶ 🛣 路段行走時間 (↻) [看示意圖]
▼ ⚡ 即時參數          ← WIP 閾值 / SMK AGV 數量（標 ⚡）
──────── sticky footer ────────
● dirty 指示    [套用] ☑自動播放  恢復預設  ⬇匯出 ⬆匯入
```
分區用純 CSS（`max-height` transition + class toggle），無需 JS 框架。footer `position:sticky; bottom:0`。

### D4. Dirty 狀態模型

維護 `paramsDirty`（輸入值 ≠ 已套用 `P`）。任一輸入 `input` 事件比對當前 `P` 推導 dirty。footer 顯示三態：
- 乾淨：無提示。
- 已變更（含需重跑項）：「● 參數已變更，畫面正在跑舊參數」+ 套用鈕高亮。
- 僅即時項變更：可選擇即時套用而不顯示「舊參數」警示。

### D5. Inline 驗證

以欄位 metadata（min/max/型別）逐欄驗證，取代 `applyP()` 內單一大 OR + `alert()`。無效欄位加 `.invalid`（紅框）+ 欄位旁 `<span>` 訊息；存在任一無效欄位時 disable 套用鈕。SMK cycle time 仍經 `parseMMSS` 驗證 `mm:ss`。

### D6. 套用前後對比卡

套用（不論即時或重跑）前 snapshot `{uph, eta, done, simT}`；套用後比對顯示 UPH 與預估完成的 ▲▼ 差值，3–5 秒後淡出（可釘選）。重跑情境因從 0 起算，對比卡顯示「上一輪最終 UPH vs 本輪預估」；即時情境顯示「變更前 vs 變更後」即時值。

### D7. 具名情境（scenario-presets）

localStorage schema 由單一 `sim-params` 遷移為：
```json
{ "version": 2,
  "active": "理想節拍",
  "presets": { "理想節拍": {line, params}, "瓶頸情境": {...}, "客戶現況": {...} } }
```
載入時若偵測舊 `sim-params`（無 `version`），自動包成 `presets["匯入設定"]` 並保留。內建 3 個唯讀範本（理想/瓶頸/現況）可另存為自訂。匯出/匯入沿用現有 Blob 下載與 FileReader，但以「情境」為單位。

### D8. 路段時間迷你示意圖

在「路段行走時間」區內嵌一個小 SVG，重用 `ABF_DEFS` 的 xi 佈局畫出 7 站 + 直達線，每段標數字；段 ↔ 欄位雙向 highlight（hover/focus 同步加亮）。純讀取既有常數，不影響主 SVG。

## Risks / Trade-offs

- **即時套用破壞可重現性** → 簡報重播時中途改值會讓該輪不可重現。Mitigation：在 UPH 圖上對即時變更打註記點（時間 + 變更摘要），讓「改了什麼、何時改」可追溯；對比卡也留痕。
- **SMK AGV 中途減車的產出帳** → 在途批量若被硬移除會少計或多計。Mitigation：D2 的「收車中」延後移除策略；只在 `waiting` 安全點即時加減，其餘排隊。
- **與進行中 change 規格衝突** → `abf-smk-agv-simulation` 的 `parameter-panel` 仍定義「套用＝完全重置」。Mitigation：本案以 MODIFIED 明確覆寫該行為；實作前確認前案此 capability 的落地狀態，必要時兩案協調或先 archive 前案。
- **localStorage 遷移失誤** → 結構改動可能讓舊使用者設定遺失。Mitigation：偵測舊格式自動包裝、保留原值；遷移以 try/catch 包覆，失敗則回退預設且不丟例外。
- **面板資訊密度仍高（工程師場景）** → 手風琴預設收合可能讓進階使用者多點一下。Mitigation：記住各區展開狀態於 localStorage。

## Open Questions

- 內建情境的 3 組具體數值由誰定義（需欣興提供「瓶頸情境 / 客戶現況」的代表性參數）？
- 即時減少 SMK AGV 時，是否需要在畫面上明確標示「收車中」的車？（影響 SVG 狀態樣式）
- 是否要在本案一併把「自動播放」設為預設開啟，或保留使用者上次選擇？
