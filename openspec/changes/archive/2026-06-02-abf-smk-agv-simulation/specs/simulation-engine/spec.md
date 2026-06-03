## ADDED Requirements

### Requirement: AGV 狀態機
每台 AGV SHALL 維護以下狀態：IDLE → LOADING → MOVING → UNLOADING → IDLE。
狀態轉換由模擬事件觸發，不由時鐘輪詢。

#### Scenario: AGV 完成搬運後自動回到 IDLE
- **WHEN** AGV 完成 UNLOADING（卸料完成）
- **THEN** 狀態變為 IDLE，並立即觸發下一趟 LOADING 事件（若來源站有 WIP）

#### Scenario: 來源站無 WIP 時 AGV 等待
- **WHEN** AGV 進入 IDLE 且來源站 WIP = 0
- **THEN** AGV 保持 IDLE，等待來源站 WIP > 0 後才觸發 LOADING

---

### Requirement: WIP 計算
每個站點 SHALL 維護一個整數 WIP 計數器，代表該站點等待搬運的批次數量。

#### Scenario: AGV 取料時 WIP 減少
- **WHEN** AGV 對某站點執行 LOADING
- **THEN** 該站點 WIP -= 1（最小值為 0）

#### Scenario: 批次到達站點時 WIP 增加
- **WHEN** AGV 完成 MOVING 到達目標站點並執行 UNLOADING
- **THEN** 目標站點 WIP += 1

---

### Requirement: 離散事件佇列
模擬引擎 SHALL 使用優先佇列（min-heap by timestamp）管理所有事件，時間單位為秒。

#### Scenario: 事件按模擬時間排序執行
- **WHEN** 模擬推進時
- **THEN** 總是取出 timestamp 最小的事件先執行

#### Scenario: 重置清空所有事件
- **WHEN** 使用者點擊重置
- **THEN** 事件佇列清空、所有 AGV 回到初始站點、所有 WIP 歸零、模擬時間歸零

---

### Requirement: 製程數據初始化
模擬引擎 SHALL 依照以下預設值初始化，且可由參數面板覆寫。

#### Scenario: ABF 線預設資料載入
- **WHEN** 系統啟動或重置
- **THEN** ABF 線載入 6 個站點（CLN/Annealing/CZ/Baking/壓機/Postcure）、5 台 AGV、各段 cycle time（A→B:250s, B→C:290s, C→D:170s, D→E:200s, E→F:320s）

#### Scenario: SMK 線預設資料載入
- **WHEN** 系統啟動或重置
- **THEN** SMK 線載入 8 個站點（Baking/壓機/曝光/顯影/SoftBaking/UVQ/Postcure/PORT）、6 台 AGV、各段 cycle time（A→B:250s, B→C:290s, C→D:170s, D→G:260s, G→H:170s, H→I:170s, I→J:350s, J→K:200s, K→L:290s）
