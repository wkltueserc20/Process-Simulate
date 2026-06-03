## ADDED Requirements

### Requirement: 播放控制列
系統 SHALL 在頁面頂部顯示控制列，包含：模擬時鐘、播放/暫停、速度選擇、重置按鈕。

#### Scenario: 模擬時鐘顯示
- **WHEN** 模擬正在執行
- **THEN** 時鐘顯示格式為 `HH:MM:SS`（模擬時間，非真實時間），每幀更新

#### Scenario: 播放/暫停切換
- **WHEN** 使用者點擊播放/暫停按鈕
- **THEN** 模擬在執行與暫停之間切換；按鈕圖示同步變更（▶ / ⏸）

---

### Requirement: 播放速度控制
系統 SHALL 支援 1x / 2x / 5x 三種播放速度。

#### Scenario: 切換速度
- **WHEN** 使用者點擊速度按鈕（1x / 2x / 5x）
- **THEN** 當前速度按鈕高亮，模擬時間推進速率立即改變，AGV 動畫 duration 同步調整

#### Scenario: 1x 速度定義
- **WHEN** 播放速度為 1x
- **THEN** 真實 1 秒 = 模擬 10 秒（讓 1 小時模擬在 6 分鐘內完成）

---

### Requirement: 重置功能
系統 SHALL 提供重置按鈕，將模擬恢復到初始狀態。

#### Scenario: 重置後狀態
- **WHEN** 使用者點擊重置
- **THEN** 模擬時間歸零、所有 AGV 回到起點、所有 WIP 清零、UPH 圖表清空、模擬自動暫停

---

### Requirement: 進度條
系統 SHALL 顯示一條進度條，顯示目前模擬進度（以目標產量為基準）。

#### Scenario: 進度條更新
- **WHEN** 有新批次從 PORT 輸出
- **THEN** 進度條按比例增加（已完成片數 / 目標片數），並顯示百分比
