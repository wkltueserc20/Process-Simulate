## 1. 參數模型與分類

- [x] 1.1 建立參數 metadata 表（id、label、單位、min/max、型別、套用類別 ⚡即時/↻需重跑、步進量），集中管理取代散落的 `applyP`/`resetP`/`applyParamsToInputs`
- [x] 1.2 依 metadata 重構 `applyP()`：分流「即時參數」與「需重跑參數」，需重跑者維持 `resetSim()` 流程
- [x] 1.3 實作即時套用 `tg`/`ww`/`wd`：直接寫入執行中 `P`，下個 `updateUI()` 反映，不重置

## 2. SMK AGV 熱增減

- [x] 2.1 實作即時增加 SMK AGV：append 至 `smkAGVs`（`phase:'waiting'` + 錯開 stagger）並 rebuild SVG
- [x] 2.2 實作即時減少 SMK AGV：優先移除 waiting 且未載貨者；不足時標記「收車中」待完成當前 leg 後移除
- [x] 2.3 驗證熱增減後產出統計（`smkPcs`/`smkOut`）不失準、無重複或漏計

## 3. 面板資訊架構

- [x] 3.1 將參數欄位重組為手風琴分區（製程時間 / 取放料時間 / 路段行走時間 / 即時參數），純 CSS `max-height` 過渡
- [x] 3.2 情境選擇器與目標產量常駐頂部
- [x] 3.3 套用列改為 sticky footer（套用、自動播放勾選、恢復預設、匯出/匯入恆可見）
- [x] 3.4 每欄標示 ⚡即時 / ↻需重跑 視覺標籤
- [x] 3.5 記住各分區展開狀態於 localStorage

## 4. Dirty 狀態與自動播放

- [x] 4.1 實作 `paramsDirty` 偵測（輸入值 vs 已套用 `P`），footer 顯示三態
- [x] 4.2 「自動播放」選項：套用後自動 `togglePlay()` 進入播放
- [x] 4.3 套用後清除 dirty 指示

## 5. Inline 驗證

- [x] 5.1 依 metadata 逐欄驗證，無效欄位加 `.invalid`（紅框）+ 欄位旁訊息
- [x] 5.2 存在任一無效欄位時 disable 套用鈕；全部修正後恢復
- [x] 5.3 移除 `alert()`；Cycle Time 經 `parseMMSS` 走相同 inline 機制

## 6. 觸控友善

- [x] 6.1 數字欄位加 −/＋ stepper，依 metadata 步進量增減
- [x] 6.2 加大點擊區與間距，適配 iPad；stepper 觸發與直接輸入共用驗證/ dirty 流程

## 7. 路段時間示意圖

- [x] 7.1 在「路段行走時間」分區內嵌迷你 SVG，重用 `ABF_DEFS` xi 佈局畫 7 站 + 直達線並標數字
- [x] 7.2 段 ↔ 欄位 hover/focus 雙向 highlight

## 8. 套用前後對比卡

- [x] 8.1 套用前 snapshot `{uph, eta, done, simT}`
- [x] 8.2 套用後顯示對比卡（UPH 與預估完成的 ▲▼），數秒後淡出且可釘選
- [x] 8.3 在 UPH 圖上對即時變更打註記點（時間 + 變更摘要）以保留可追溯性

## 9. 具名情境（scenario-presets）

- [x] 9.1 設計 localStorage v2 schema（`{version, active, presets}`）
- [x] 9.2 載入時偵測舊 `sim-params` 自動包裝遷移；解析失敗安全回退預設
- [x] 9.3 內建 3 個唯讀範本（理想節拍 / 瓶頸情境 / 客戶現況）—— 數值待欣興提供，先用合理佔位
- [x] 9.4 情境選擇器切換 + 「另存為自訂情境」
- [x] 9.5 匯出/匯入改以情境為單位，匯入錯誤改 inline 提示

## 10. 驗證與收尾

- [ ] 10.1 三場景手動驗證：客戶簡報（情境切換 + 對比卡）、工程師調參（即時生效 + dirty）、iPad（stepper + sticky footer）
- [ ] 10.2 回歸測試：需重跑參數仍正確全重置；既有 ABF/SMK 模擬行為不變
- [x] 10.3 與進行中 `abf-smk-agv-simulation` 的 `parameter-panel` capability 協調，避免規格衝突 —— 決策：依使用者指示獨立處理本案，不執行/不阻塞前案
