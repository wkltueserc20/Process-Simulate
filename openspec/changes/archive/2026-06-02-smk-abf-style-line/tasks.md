## 1. 引擎一般化（ABF 零回歸）

- [x] 1.0 修正既有 changer 雙重處理 bug：`updateABFMachines` loop1 由 `else` 改 `else if(mtype==='transfer')`，避免換框機被 transfer 邏輯重複處理而吞掉產出（換框機原本幾乎不計產）。＊前置必修：SMK 換框機會繼承同 bug
- [x] 1.5a 擷取 ABF 行為基準（headless dt=1, 20000 步）：17 件、checksum=191246，存於 __abf_baseline.json，供一般化後逐點比對
- [x] 1.1 採「config 間接化」最小風險路線：`abfSt/abfAGVs/abfOut/abfPcs` 保留為當前作用線狀態，引擎透過 `LC`(當前 line config) 取得分派/路段/資料來源（非重命名 state 物件，行為等價、回歸已驗）
- [x] 1.2 定義 `LINES` line-config schema，現有 ABF 抽成 `LINES.abf`（defs/agvInits/route/groupTasks）
- [x] 1.3 `tABF`→`LC.route` 分派；群組分派改 `LC.groupTasks[grp]()`。＊通用 `genGroupTasks`（含 2 站直送）併入 task 2/3 撰寫
- [ ] 1.4 `buildABFSVG`/`updateUI`/AGV 渲染一般化（不影響物理 checksum，與 task 4 蛇形佈局一起做）
- [x] 1.5 ABF 回歸：一般化後產出逐點等價基準（17 件 / checksum 191246，REGRESSION_MATCH:true）

## 2. 2 站直送群組型態

- [x] 2.1 `genGroupTasks` 通用版（含 `mid==null` 2 站直送分支）；ABF 改用後回歸 17/191246 逐點等價，證明與 getGroup1/2/3Tasks 完全相同
- [x] 2.2 直送分支沿用 srcOutFull/dstInFree/dstRetAvail（含 out_full_hold/in_ret_hold 保護期）
- [x] 2.3 驗證直送群組不卡死、空框正確回流（SMK G3/G4 料流通過、45 件產出）

## 3. SMK line config

- [x] 3.1 `SMK_DEFS`：11 站型態正確（refill/buffer/transfer/changer）
- [x] 3.2 `LINES.smk.groups`：G1[0,1,2] G2[2,3,4] G3[4,5] G4[5,6] G5[6,7,8] G6[8,9,10]，A1–A11 分配
- [x] 3.3 `routeSMK` + `SMK_SEG_T`：10 段相鄰行走時間（佔位，task5 接參數）
- [x] 3.4 `SMK_AGV_INITS`：11 台起始站與 stagger。＊headless 驗證：11 站全有料、首件@4221s、45 件

## 4. 佈局（11 站）

- [x] 4.0 第一版：單列加寬（layout 驅動 xiCount/gap/zones/agvLbls/viewBox）；`buildABFSVG`/`segGrp`/`stGrp` 一般化；ABF 重用垂直結構、`updateABFAgvPos` 不動。headless 兩線無拋錯
- [x] 4.1 蛇形佈局（上 6 下 5）：`computeGeom` 依 layout.type 計算 x,y + 軌道錨點 tfy/tyr；track 用錨點連接（含轉列）；`updateABFAgvPos` 跨列插值 y；zones 改 bounding box。ABF 分支不變、回歸通過
- [x] 4.2 SMK viewBox 改 `-30 0 1520 730` + legendY 690（蛇形兩列、字級放大）

## 5. SMK 參數（取代舊 SMK 參數）

- [x] 5.1 新增 SMK 製程時間參數（smkCln/smkCz/smkPress/smkExp/smkDev/smkUvq/smkChanger/smkBt）至 metadata
- [x] 5.2 取放料 smkPick/smkDrop/smkBufPick/smkBufDrop + 路段 **10 段獨立** smkR0..smkR9（緩衝容量/holdTime 同 ABF 為 defs 內定）
- [x] 5.3 移除舊 SMK 參數 sc/bt/sa/ww/wd（DEF + metadata）；保留 tg
- [x] 5.4 SMK 路段示意圖：`buildSMKRouteMap` 線性 11 站 10 段（s0..s9 ↔ smkR0..9）雙向 highlight；`selectLine` 依作用線重建。順修 BUILTIN_SCENARIOS 殘留 sa/sc 引用
- [x] 5.5 移除 `hotSetSMKAgv`/`pruneRetiringSMK` 與 applyLiveParam sa/ww/wd 即時邏輯
- [x] 5.6 引擎接參數：`LC.procTime`/`curBatch`/`routeSMK`/line-aware 取放料；checkDone/chart/對比卡改用 abfPcs/abfOut

## 6. PORT 堆積瓶頸

- [x] 6.1 移除 SMK WIP 閾值上色與舊瓶頸 WIP 邏輯（隨輸送帶清除）
- [x] 6.2 `stPiled`/`pileT` 累計 + `refreshBottleneck` 更新 `#bnklist`（出滿框/退空框/緩衝滿載滯留 >150s）
- [x] 6.3 ABF/SMK 瓶頸語彙一致（面板對兩線皆顯示）。＊headless 驗證 SMK 偵到 CLN/顯影機 堆積

## 7. 移除舊 SMK 輸送帶程式

- [x] 7.2 `buildSVG`/`tick`/`updateUI`/`initSim` 改走一般化引擎分派；舊 smk-* 群組隱藏
- [x] 7.1 死碼已清：`SMK_NAMES`/`SMK_BASE`/`stepSMKAgv`/`buildSMKSVG`/`buildSMKAgvSVG`/`updateSMKAgvPos`/`updateSMKAndStats`/舊 smkSt 建構/smkSt/smkAGVs/smkSeg/smkOut/smkPcs/SMK_SY 等

## 8. 驗證與收尾

- [x] 8.1 ABF 全回歸：headless 逐點等價（17/191246）跨所有改動
- [x] 8.2 SMK 功能驗證（headless）：11 站 PORT 流動、6 組 AGV、換框機計產 45 件、PORT 瓶頸偵測、參數驅動
- [ ] 8.3 三場景視覺驗證（簡報/調參/iPad）—— 需使用者在瀏覽器確認蛇形視覺
- [x] 8.4 與 `redesign-parameter-panel` 協調：移除其 SMK 即時 sa/ww/wd 與熱加減 AGV，tg 保留
- [ ] 8.5 內建情境/預設值：SMK 各站時間/緩衝/路段預設待欣興提供（目前合理佔位 360s/200s）
