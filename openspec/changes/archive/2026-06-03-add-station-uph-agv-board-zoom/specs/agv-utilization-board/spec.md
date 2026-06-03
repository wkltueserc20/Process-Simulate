## ADDED Requirements

### Requirement: AGV 時間累計

系統 SHALL 為每台 AGV 累計其處於不同狀態的時間。每個模擬步進 `dt`，系統 MUST 依該車當前 phase 將 `dt` 加到對應分類：移動（toPickup / delivering）、取放（picking / dropping）、閒置（idle）、啟動等待（stag）。

#### Scenario: 移動時間累計
- **WHEN** AGV 處於 toPickup 或 delivering 狀態並前進
- **THEN** 該車的「移動」累計時間增加該步進的 `dt`

#### Scenario: 取放時間累計
- **WHEN** AGV 處於 picking 或 dropping 狀態
- **THEN** 該車的「取放」累計時間增加該步進的 `dt`

#### Scenario: 閒置時間累計
- **WHEN** AGV 處於 idle 狀態（無任務可執行）
- **THEN** 該車的「閒置」累計時間增加該步進的 `dt`

#### Scenario: 重置歸零
- **WHEN** 使用者重置模擬或切換製程線
- **THEN** 所有 AGV 的各項累計時間歸零

### Requirement: AGV 稼動看板

系統 SHALL 提供一個全螢幕彈出看板，列出每台 AGV 的移動、取放、閒置時間與稼動率，並顯示總計/平均。稼動率定義為 動作時間 ÷（動作時間 + 閒置時間），其中動作時間 = 移動 + 取放。

#### Scenario: 開啟看板
- **WHEN** 使用者點擊 header 的「AGV 看板」按鈕
- **THEN** 顯示一個覆蓋畫面的看板視窗，含每台 AGV 一列（移動 / 取放 / 閒置 / 稼動率）

#### Scenario: 看板即時更新
- **WHEN** 看板開啟且模擬正在進行
- **THEN** 看板中的時間與稼動率隨模擬持續更新

#### Scenario: 總計與平均
- **WHEN** 看板開啟
- **THEN** 顯示全部 AGV 的時間總計與平均稼動率

#### Scenario: 關閉看板
- **WHEN** 使用者點擊看板的關閉鈕
- **THEN** 看板隱藏，模擬不受影響
