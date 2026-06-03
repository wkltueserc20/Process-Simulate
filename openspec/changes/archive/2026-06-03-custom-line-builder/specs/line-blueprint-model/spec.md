## ADDED Requirements

### Requirement: 線由 blueprint 完整描述

系統 SHALL 以單一 blueprint 物件完整描述一條製程線（站別脊椎、群組、路段時間、處理時間對應、佈局、參數預設與計產站），且 route、procTime、參數預設、面板欄位、選擇畫面、標題、每框片數、取放料時間 MUST 由該 blueprint 衍生，不得另行寫死於各線專屬函式。

#### Scenario: 新增線僅需新增 blueprint
- **WHEN** 開發者在 `BLUEPRINTS` 加入一個合法的 line blueprint 物件
- **THEN** 該線自動出現於選擇畫面，且可正常模擬，無需另外修改 route／procTime／參數／面板等程式

#### Scenario: 內建線等價（parity）
- **WHEN** 既有 ABF／SMK 改以 blueprint 表示並以預設值模擬
- **THEN** 站別數、AGV 數、群組接線、產出結果與改版前等價

### Requirement: 通用路段與處理時間

系統 SHALL 提供通用的路段時間計算（相鄰段相加，支援選填直達捷徑）與處理時間查詢（依站別對應的參數鍵），取代各線專屬的 route／procTime 函式。

#### Scenario: 路段時間由 blueprint 段資料計算
- **WHEN** AGV 於兩站之間移動
- **THEN** 距離為兩站之間各相鄰段時間之和；若該對站點定義了直達捷徑則採用捷徑值；同站位（xi）距離為 0

#### Scenario: 處理時間由站別參數鍵查得
- **WHEN** 任一機台站開始加工
- **THEN** 其處理時間取自該站於 blueprint 指定的處理時間參數

### Requirement: 參數與選單由 blueprint 動態產生

系統 SHALL 由所有 blueprint 自動產生參數預設值與參數面板欄位，並由當前作用線的 blueprint 提供其面板欄位；選擇畫面按鈕、標題與每框片數 MUST 由 blueprint 衍生。

#### Scenario: 面板欄位隨當前線
- **WHEN** 使用者切換到某條線
- **THEN** 參數面板顯示該線 blueprint 對應的欄位（製程時間／路段／設備數量等），數值與單位正確

#### Scenario: 選擇畫面自動列出所有線
- **WHEN** 開啟選擇畫面
- **THEN** 每條 blueprint（含內建與自訂）各顯示一個可選按鈕，含名稱與流程摘要
