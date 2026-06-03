## ADDED Requirements

### Requirement: 即時 UPH 折線圖
系統 SHALL 在頁面下方顯示 UPH（Units Per Hour）折線圖，以 Chart.js 實作，每分鐘（模擬時間）更新一次。

#### Scenario: 雙線對比圖
- **WHEN** 模擬開始執行
- **THEN** 圖表顯示兩條折線：ABF UPH（藍色）與 SMK UPH（橘色），X 軸為模擬時間（分鐘），Y 軸為 UPH（片/小時）

#### Scenario: UPH 計算方式
- **WHEN** 每過 1 分鐘模擬時間
- **THEN** UPH = 過去 60 分鐘內從 PORT 輸出的片數（滾動視窗計算）

---

### Requirement: 統計摘要
圖表旁 SHALL 顯示即時統計數字。

#### Scenario: 統計數字顯示
- **WHEN** 模擬執行中
- **THEN** 顯示：ABF 當前 UPH、SMK 當前 UPH、已完成總片數、目標片數、預估完成時間

#### Scenario: 系統瓶頸標示
- **WHEN** SMK UPH < ABF UPH
- **THEN** SMK UPH 數字顯示紅色，並標示「⚠ 系統瓶頸」
