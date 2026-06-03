## ADDED Requirements

### Requirement: 視覺化新增製程線精靈

系統 SHALL 提供頁面上的多步精靈，讓使用者不必修改程式即可新增製程線。精靈 MUST 僅以引擎既有的 4 種站別原型（refill／transfer／buffer／changer）組合，並輸出一份 blueprint。

#### Scenario: 逐步建立站別流程
- **WHEN** 使用者於精靈依序新增站別，為每站選擇原型並設定處理時間或緩衝容量
- **THEN** 系統據此建立站別脊椎（站序即 xi）

#### Scenario: 定義搬運群組與計產站
- **WHEN** 使用者把站別框選為搬運群組、設定每組 AGV 數量並指定計產站
- **THEN** 系統據此建立群組接線

#### Scenario: 即時預覽
- **WHEN** 使用者進入預覽步驟
- **THEN** 以既有 SVG 引擎即時繪出該線的流程圖供確認

#### Scenario: 儲存前驗證
- **WHEN** 使用者儲存精靈結果
- **THEN** 系統執行合法性驗證與乾跑，通過才加入選擇畫面；失敗則顯示可定位的錯誤且不加入

### Requirement: 自訂線管理

系統 SHALL 允許對自訂線進行編輯、刪除與匯出；內建線（ABF／SMK）為唯讀。

#### Scenario: 編輯自訂線
- **WHEN** 使用者選擇編輯某條自訂線
- **THEN** 精靈以該線現有設定回填，修改後可重新驗證並儲存

#### Scenario: 刪除自訂線
- **WHEN** 使用者刪除某條自訂線
- **THEN** 該線自選擇畫面與 localStorage 移除，內建線不受影響

### Requirement: 佈局自動產生並可覆寫

系統 SHALL 為自訂線自動產生視覺佈局（分區由群組推得、viewBox 寬由站位數計算、預設單列），且 blueprint 可選填覆寫。

#### Scenario: 自動佈局
- **WHEN** 使用者未指定佈局
- **THEN** 系統自動產生分區與 viewBox，流程圖可完整檢視（可搭配縮放／平移）

#### Scenario: 覆寫佈局
- **WHEN** blueprint 提供佈局覆寫（如自訂分區或蛇形）
- **THEN** 系統採用覆寫設定繪製
