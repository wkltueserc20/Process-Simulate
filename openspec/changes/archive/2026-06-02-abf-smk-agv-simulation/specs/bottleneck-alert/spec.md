## ADDED Requirements

### Requirement: 瓶頸偵測邏輯
系統 SHALL 自動偵測 WIP 超過危險閾值的站點，並判定為瓶頸。

#### Scenario: 偵測瓶頸站點
- **WHEN** 任一站點 WIP >= 危險閾值（預設 10）
- **THEN** 該站點被標記為「瓶頸」，列入瓶頸警示面板

#### Scenario: 瓶頸解除
- **WHEN** 瓶頸站點 WIP 降回 < 警告閾值（預設 5）
- **THEN** 從瓶頸清單移除，站點顏色恢復正常

---

### Requirement: 瓶頸警示面板
系統 SHALL 在頁面右下角顯示瓶頸警示面板，列出當前所有瓶頸站點。

#### Scenario: 面板內容
- **WHEN** 有瓶頸站點
- **THEN** 面板顯示：站點名稱、當前 WIP、平均等待時間（min）、持續時間（模擬秒）

#### Scenario: 無瓶頸時面板狀態
- **WHEN** 所有站點 WIP < 警告閾值
- **THEN** 面板顯示「✅ 目前無瓶頸」綠色狀態

---

### Requirement: 瓶頸段落高亮
系統 SHALL 在工廠平面圖中高亮顯示瓶頸站點之間的軌道線。

#### Scenario: 瓶頸軌道線變色
- **WHEN** 某段路段（如 I→J 或 K→L）的終點站成為瓶頸
- **THEN** 該段軌道線從灰色變為紅色虛線，並有脈衝動畫效果
