## ADDED Requirements

### Requirement: 製程線以設定資料描述
系統 SHALL 以一份 line config 資料描述一條製程線，內容包含站別定義、群組與 AGV 分配、路段行走時間表與版面佈局，使同一引擎可實例化不同製程線。

#### Scenario: 以 config 實例化 ABF 與 SMK
- **WHEN** 使用者選擇 ABF 或 SMK 製程線
- **THEN** 引擎讀取對應的 line config 建立站別、AGV、路段與版面，並以同一套引擎邏輯驅動

#### Scenario: 站別型態由 config 決定
- **WHEN** line config 的某站定義為 refill / transfer / changer / buffer
- **THEN** 該站依其型態具備對應的 PORT 與機台行為，無需為個別線撰寫專屬邏輯

### Requirement: 群組與 AGV 由 config 指定
line config SHALL 以群組描述滿框前進路徑（empties 反向回流）與該組專用 AGV。

#### Scenario: 指定群組站序與 AGV
- **WHEN** config 定義某群組的站序與 AGV 清單
- **THEN** 該組 AGV 僅在該站序內搬運，滿框沿站序前進、空框反向回流

#### Scenario: 相鄰群組共享站點
- **WHEN** 兩個相鄰群組共享一個交接站（如某 transfer 站為前組終點、後組起點）
- **THEN** 該站同時作為前組的入料目的地與後組的出料來源，搬運不重複不遺漏
