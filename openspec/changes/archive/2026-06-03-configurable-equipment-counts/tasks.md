## 1. 參數與預設值

- [x] 1.1 在 `DEF` 新增 ABF 數量參數：`abfCzN:2, abfPressN:4, abfChangerN:2`；緩衝容量 `abfAnnBuf:16, abfBakBuf:6, abfPostBuf:6`；每組 AGV `abfG1Agv:2, abfG2Agv:2, abfG3Agv:3`
- [x] 1.2 在 `DEF` 新增 SMK 數量參數：`smkCzN:2, smkPressN:4, smkExpN:1, smkDevN:2, smkUvqN:2, smkChangerN:2`；緩衝容量 `smkAnnBuf:16, smkBakBuf:6, smkSoftBuf:6, smkPostBuf:6`；每組 AGV `smkG1Agv:2, smkG2Agv:2, smkG3Agv:1, smkG4Agv:1, smkG5Agv:2, smkG6Agv:3`
- [x] 1.3 確認預設值等同現行台數／容量／AGV 配置（parity 基準）

## 2. 線路藍圖與產生器

- [x] 2.1 為 ABF、SMK 各定義 blueprint：站別脊椎（`xi`、型別、名稱前綴、`countKey`/`bufKey`、所屬 group 角色）＋ group 型別接線（src 型別→mid 緩衝→dst 型別＋priA/priB/midSpace）＋ layout 靜態部分（type/gap/zones/header/simple）
- [x] 2.2 `genDefs(bp, P)`：依脊椎展開——可數型別產 `count` 份（`sub 0..n-1`、名稱 `前綴_{n}`）、緩衝站套 `buf=P[bufKey]`、CLN 固定 1；回傳 defs 與 `byType`（型別→索引）與緩衝角色→索引
- [x] 2.3 `genGroups(bp, byType)`：將 group 的型別角色解析為實際 `src/dst/mid` 索引，保留 priA/priB/midSpace
- [x] 2.4 `genAgvs(bp, P)`：每組依 AGV 數量產生 `agvInits`（id 全線連號、起始 pos 在組內站別輪流、`stag` 遞增 30s、grp）
- [x] 2.5 `genLayout(bp, defs, P)`：產生 `agvLbls`、動態 `viewBox` 高度（= `ABF_SY+(maxSub-1)*SUB_SPACING+底margin`，不小於原高）、其餘沿用 blueprint
- [x] 2.6 `buildLineConfig(lineId, P)`：組裝 `{id, defs, agvInits, route, procTime, groups, layout}` 回傳

## 3. 接線到引擎

- [x] 3.1 `initSim()` 改用 `LC = buildLineConfig(selectedLine, P)` 取代靜態 `LINES[id]`
- [x] 3.2 確認 `computeGeom`／`buildSVG`／`genGroupTasks`／`route*`／`procTime*` 皆能吃動態 LC（不需改邏輯）
- [x] 3.3 確認 `syncCamera()` 以新 viewBox 為基準（縮放/平移重置正確）

## 4. 參數面板 UI

- [x] 4.1 在 `PARAM_META` 新增「設備數量」欄位：機台數（`max:9,min:1`）、AGV 數（`max:9,min:1`）、緩衝容量（`min:1,max:99`），皆 `type:'int', apply:'resim', section:'equip'`，分 line 標註
- [x] 4.2 在 `SECTIONS` 新增 `{id:'equip', title:'🏭 設備數量', tag:'resim'}`
- [x] 4.3 驗證 stepper、min/max 邊界、`applyLineFilter` 依當前線顯示對應欄位

## 5. 死碼清除

- [x] 5.1 移除 `getGroup1Tasks/getGroup2Tasks/getGroup3Tasks`（`:506–561`）
- [x] 5.2 移除 `CZ_INDICES/PRESS_INDICES/CHANGER_INDICES`（`:303–305`）
- [x] 5.3 確認移除後無引用殘留、語法通過

## 6. 驗證

- [x] 6.1 預設值下產生的 defs/groups/agvInits/viewBox 與現行等價（parity）
- [ ] 6.2 ABF：增/減 CZ、壓機、換框機數量套用後，流程圖與搬運正確
- [ ] 6.3 SMK：增/減各機台型別數量套用後正確
- [ ] 6.4 緩衝容量調整生效（容量顯示/滿載判定）
- [ ] 6.5 每組 AGV 數量調整後 AGV 數量與搬運正確
- [ ] 6.6 機台堆疊超出時 viewBox 自動加高、可縮放/平移
- [ ] 6.7 載入無數量欄位的舊情境以預設值 fallback 正常
- [ ] 6.8 邊界：各型別/AGV 設為 1 與 9、緩衝設為 1 皆正常
