# station-uph-tooltip Specification

## Purpose

提供各製程站的每小時產出統計，並透過 tooltip 與站名下方常駐顯示其每小時 UPH 與平均 UPH，便於跨站比較與即時觀察。

## Requirements

### Requirement: 各站每小時產出統計

系統 SHALL 為每個製程站累計其每小時產出量。每當一個站的機台完成一次加工 cycle（產出滿框），系統 MUST 將該次產出（`curBatch()` 片）計入「以模擬時間 `simT` 換算的小時桶」（桶索引 = `floor(simT / 3600)`）。緩衝站（buffer）SHALL 以框變為可取出（ready）的時刻計入。

#### Scenario: 機台完成 cycle 計入當前小時桶
- **WHEN** 任一 transfer / refill / changer 站的機台計時器歸零並產出滿框
- **THEN** 該站當前小時桶的累計片數增加 `curBatch()` 片

#### Scenario: 跨小時邊界分桶
- **WHEN** 模擬時間從第 N 小時跨入第 N+1 小時後該站再次完成 cycle
- **THEN** 新的產出計入第 N+1 桶，不影響第 N 桶既有數值

#### Scenario: 重置時清空統計
- **WHEN** 使用者重置模擬或切換製程線
- **THEN** 所有站的每小時產出統計歸零

### Requirement: 站別 UPH tooltip 顯示

系統 SHALL 在使用者將滑鼠移到任一站時，以 tooltip 顯示該站各小時 UPH（每小時完成片數）、平均 UPH，並在離開時隱藏。

#### Scenario: 滑入站別顯示 tooltip
- **WHEN** 使用者將滑鼠移入某站的圖形區域
- **THEN** 顯示一個 tooltip，逐列列出每個已記錄小時的 UPH（例如「0~1hr 96」「1~2hr 144」）以及一列「平均」

#### Scenario: 當前未滿小時標註
- **WHEN** tooltip 顯示的最後一列對應的小時尚未跑滿一整小時
- **THEN** 該列顯示目前累計片數並加上標註（例如「← 現在」或灰字），與已完成的整點小時區別

#### Scenario: 尚無產出的站
- **WHEN** 使用者滑入一個目前尚無任何產出紀錄的站
- **THEN** tooltip 顯示該站名稱與「尚無產出」之類的提示，而非空白或錯誤

#### Scenario: 滑出隱藏 tooltip
- **WHEN** 使用者將滑鼠移出該站區域
- **THEN** tooltip 隱藏

### Requirement: 站別平均 UPH 常駐顯示

系統 SHALL 在詳細視圖中，於每個站的「站名下方」常駐顯示該站的平均 UPH（含單位 `片/hr`），免滑鼠即可跨站比較，並隨模擬即時更新。

#### Scenario: 各站常駐顯示平均 UPH
- **WHEN** 詳細視圖下模擬進行中
- **THEN** 每個站於站名下方顯示其平均 UPH 數值與單位 `片/hr`，且隨模擬持續更新

#### Scenario: 尚無產出的站
- **WHEN** 某站目前尚無任何產出
- **THEN** 該站的常駐平均 UPH 顯示為「—」
