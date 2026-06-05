# live-vitals-bar Specification

## Purpose
TBD - created by archiving change ux-improvement-pass. Update Purpose after archive.
## Requirements
### Requirement: 即時生命徵象常駐顯示

系統 SHALL 在主畫面常駐顯示一條輕量「生命徵象」列，至少包含即時 UPH、已完成/目標片數、預估完成（ETA）與瓶頸指示，且其可見性 SHALL NOT 依賴統計圖表是否展開。

#### Scenario: 圖表收合時仍可見即時數據
- **WHEN** 統計圖表覆蓋層處於收合狀態
- **THEN** 使用者仍可在主畫面看到即時 UPH、已完成、預估完成與瓶頸狀態

#### Scenario: 隨模擬即時更新
- **WHEN** 模擬進行中
- **THEN** 生命徵象列的數值隨模擬時間即時更新，與統計面板的數據一致

#### Scenario: 重置與切換線歸零
- **WHEN** 使用者重置模擬或切換製程線
- **THEN** 生命徵象列的數值正確歸零或重設為新線初始狀態

### Requirement: 瓶頸指示與細節入口

生命徵象列 SHALL 以醒目的徽章顯示目前瓶頸狀態（無瓶頸／N 站堆積），並提供開啟細節（統計圖表/瓶頸警示）的方式。

#### Scenario: 顯示瓶頸數量
- **WHEN** 有站別出口料框堆積（PORT 堆積）
- **THEN** 瓶頸徽章以警示色顯示堆積站別數量

#### Scenario: 點擊查看細節
- **WHEN** 使用者點擊瓶頸徽章
- **THEN** 系統展開統計圖表/瓶頸細節面板供進一步檢視

