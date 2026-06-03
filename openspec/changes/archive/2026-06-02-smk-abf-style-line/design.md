## Context

ABF 引擎已是 slot/PORT 模型，但**站別類型邏輯是泛用的、站別清單與分組是硬編碼的**：

- 泛用（可直接重用）：`updateABFMachines`（依 `mtype` refill/transfer/changer/buffer 驅動）、`stepABFAgv`（依 group 分派 task）、`makeSlot`/`makeArrow`、`applySlotColor`、`updateUI` 的站別渲染、預訂/取放料機制。
- 硬編碼 ABF：`ABF_DEFS`、`CZ_INDICES/PRESS_INDICES/CHANGER_INDICES`、`ABF_AGV_INITS`、`tABF(i,j)`（路段表）、`getGroup1/2/3Tasks`（站索引、cargo、優先度）、`buildABFSVG`（7 個 xi、3 個 zone）。

SMK 現為輸送帶模型（`SMK_NAMES`/`SMK_BASE`/`smkSeg`/`stepSMKAgv`/WIP），與 ABF 完全不同，將被移除。

使用者已確認 SMK 線性拓樸與 6 組 AGV 分配，並要求參數可設定如 ABF。

## Goals / Non-Goals

**Goals:**
- 把 ABF 引擎一般化為 **line config 驅動**，ABF/SMK 各為一份 config，行為由資料決定。
- SMK 完整呈現 11 站的 slot/PORT 行為、6 組 AGV 搬運、PORT 堆積瓶頸。
- SMK 參數對齊 ABF 粒度且可設定。
- **ABF 行為零回歸**。

**Non-Goals:**
- 不改站別類型的物理邏輯（refill/transfer/changer/buffer 規則不變）。
- 不引入框架/建置/後端。
- 不做兩線同畫面並列（仍一次看一條）。
- 不在本案處理 SMK 站別的多機台（每站暫定 1 台；未來可加 sub）。

## Decisions

### D1. 一般化（config-driven）而非複製
把引擎狀態與邏輯一般化，避免 ~600 行重複、並讓未來新增線變成「加一份 config」。
- 以 `LINES = { abf: <config>, smk: <config> }` 描述每條線；執行期 `L = LINES[selectedLine]`。
- 把全域 `abfSt/abfAGVs/abfOut/abfPcs` 改為每條線的 runtime 物件 `state = {st, agvs, out, pcs}`，引擎函式接收 `state` 與 `config`。
- ABF 的 `ABF_DEFS/AGV_INITS/route/groupTasks` 改寫成 abf config；行為須逐位元等價（回歸測試保護）。

替代方案：**複製**一份 SMK 引擎（不動 ABF）。否決理由：長期維護成本高、雙線易發散；但列為 fallback——若一般化導致 ABF 回歸難解，退回平行複製。

### D2. line-config schema
```js
const LINE_ABF = {
  id:'abf',
  defs:[ {name,mtype|buf,holdTime?,xi,sub} ... ],          // 站別定義（沿用現格式）
  agvInits:[ {id,pos,stag,grp} ... ],                       // AGV 起始與分組
  groups:[ {grp:1, stations:[srcIdx, midIdx?, dstIdx], agvLabel:'A1,A2'} ... ],
  routes:{ '0_1':P=>P.t01, ... },                           // xi 對的行走時間（讀參數）
  layout:{ xiCount, zones:[[x0,x1]...], serpentine:false }, // 版面
  paramPrefix:'abf'                                          // 參數命名前綴
};
```
群組任務不再寫死成 `getGroup1Tasks`，改由 `genGroupTasks(state, group)` 依 `group.stations` 的型態組合產生（見 D3）。

### D3. 群組任務一般化（含 2 站直送）
每組 `stations` 描述滿框前進路徑（empties 反向回流）：
- **3 站含 buffer**（如 G1 `[CLN, Annealing, CZ]`）：full = src.out→buffer→dst.in；empty = dst.in_ret→src.out_recv（refill 用 s_recv）。等同現 ABF group 邏輯。
- **2 站直送**（SMK G3/G4 `[壓機, 曝光]`）：**無 buffer**，full = src.out→dst.in（直接），empty = dst.in_ret→src.out_recv。需在 `genGroupTasks` 新增此分支。
`genGroupTasks` 依 `stations.length` 與各站 `mtype` 推導 task 清單（cargo、優先度沿用 ABF 既有級距）。

### D4. SMK config（站別 / 分組）
```
defs(xi 0..10, 各1台):
  0 CLN(refill) 1 Annealing(buf) 2 CZ(transfer) 3 Baking(buf) 4 SMK壓機(transfer)
  5 曝光機(transfer) 6 顯影機(transfer) 7 SoftBaking(buf) 8 UVQ(transfer)
  9 Postcure(buf) 10 換框機(changer)
groups:
  G1 [0,1,2]  A1,A2     G2 [2,3,4]  A3,A4     G3 [4,5]   A5
  G4 [5,6]    A6        G5 [6,7,8]  A7,A8     G6 [8,9,10] A9,A10,A11
```
共享站：CZ(2) 屬 G1/G2；壓機(4) 屬 G2/G3；曝光(5) G3/G4；顯影(6) G4/G5；UVQ(8) G5/G6——與 ABF 的相鄰組共享站點一致。

### D5. 11 站佈局
ABF 7 站、transfer 卡寬 192px、間距 ~205px。SMK 11 站同寬畫布間距僅 ~123px，卡片會嚴重重疊。採以下其一（預設 a）：
- **(a) 蛇形換行 serpentine**：11 站排成上下兩列（如上 6 下 5），主軌道蛇形連接。卡片維持可讀寬度。viewBox 高度加大。
- (b) 縮窄站卡 + 加高 viewBox：transfer 卡縮至 ~120px、字級維持。資訊較擠。
- (c) 寬畫布水平捲動：破壞「一眼看全線」的簡報價值，否決。
design 採 (a)，buildSVG 依 `layout.serpentine` 計算 x,y 與軌道路徑；zone/group label 隨列繪製。

### D6. SMK 參數（對齊 ABF 粒度）
新增 `line:'smk'` 參數（沿用 redesign-parameter-panel 的 metadata 機制與分區）：
- 製程時間：CLN 生產、CZ、SMK壓機、曝光、顯影、UVQ 處理時間、換框機拿料時間、每框片數。
- 緩衝：Annealing/Baking/SoftBaking/Postcure 各 容量 + holdTime。
- 路段：10 段相鄰行走時間（直達捷徑暫不設，未來可加）。
- 取放料：機台 pick/drop、緩衝站 pick/drop。
移除舊 SMK 參數（sc/bt/sa/ww/wd）。`tg` 目標產量保留（兩線共用）。路段示意圖依 SMK 拓樸重繪。

### D7. 瓶頸改 PORT 堆積式
移除 WIP 閾值上色與 SMK 瓶頸面板的 WIP 邏輯。改為偵測「出滿框/退空框 PORT 長時間有料未被取走」或 buffer 接近滿載，列出堆積站點（與 ABF 視覺語彙一致）。沿用既有 `#bnklist` 區塊但資料來源改 PORT 狀態。

### D8. 與 redesign-parameter-panel 協調
- 移除其 SMK 即時參數 `sa/ww/wd` 與 `hotSetSMKAgv`/`pruneRetiringSMK`（SMK 改固定分組，無可調 AGV 數）。
- 即時生效類別在 SMK 僅剩 `tg`；其餘 SMK 參數皆 `resim`。
- 對比卡、情境、dirty、stepper、驗證機制全數沿用。

## Risks / Trade-offs

- **一般化破壞 ABF 行為** → 最大風險。Mitigation：先一般化並讓 ABF config 跑出與現況逐位元等價（保留現有 28 項邏輯測試 + 新增 ABF 快照回歸），確認後才接 SMK。必要時退 D1 fallback（複製）。
- **2 站直送群組語意** → G3/G4 無 buffer，empties 直接回流可能與保護期（hold）時序衝突。Mitigation：沿用 transfer 的 in_ret_hold/out_full_hold，在 genGroupTasks 直送分支套用相同保護期。
- **11 站蛇形佈局複雜** → 軌道轉折、AGV 沿蛇形移動的座標插值較繁。Mitigation：佈局以 config 計算每站 x,y 與每段路徑點；AGV 仍以 leg（站到站）插值，蛇形轉角落在站點即可。
- **參數爆量** → SMK 約 20+ 參數。Mitigation：沿用手風琴分區；以 metadata 自動生成，無額外面板工。
- **與兩個進行中 change 規格重疊** → Mitigation：明確以 MODIFIED 覆寫；實作前對齊落地狀態。

## Migration Plan

1. 一般化引擎 + 抽出 ABF config，回歸驗證 ABF 等價。
2. 加入 SMK config（defs/groups/routes/layout）。
3. genGroupTasks 支援 2 站直送。
4. SVG 蛇形佈局。
5. SMK 參數 metadata + 移除舊 SMK 參數/瓶頸 WIP。
6. PORT 堆積瓶頸。
7. 全線回歸 + 三場景驗證。
回退：保留舊 SMK 程式於 git 歷史；若一般化失敗，採 D1 fallback 平行複製，不動 ABF。

## Open Questions

- SMK 各站處理時間、buffer 容量/holdTime、路段時間的**預設值**由欣興提供；先用合理佔位。
- 蛇形佈局上下列分配（6/5 或 5/6 或其他）與視覺方向，待初版出來再微調。
- 是否需要 SMK 站別多機台（如多台壓機）？目前假設各 1 台。
