# line-blueprint-io Specification

## Purpose

提供 line blueprint 的 JSON 匯入／匯出、localStorage 持久化、靜態合法性驗證與乾跑（dry-run），讓不改程式即可新增/分享一條製程線並確保其可運作。

## Requirements

### Requirement: Blueprint JSON 匯入/匯出

系統 SHALL 支援將 line blueprint 以 JSON（含 schema 版本）匯出為檔案，以及匯入 JSON 新增為自訂線；自訂線 SHALL 儲存於 localStorage 並於選擇畫面列出。

#### Scenario: 匯出 blueprint
- **WHEN** 使用者對某條線選擇匯出
- **THEN** 下載一份含 `schemaVersion` 與完整 blueprint 內容的 JSON 檔

#### Scenario: 匯入 blueprint
- **WHEN** 使用者匯入一份合法的 blueprint JSON
- **THEN** 該線經驗證後加入自訂線清單並出現在選擇畫面，可立即模擬

#### Scenario: 自訂線持久化
- **WHEN** 使用者重新整理頁面
- **THEN** 先前匯入/建立的自訂線仍保留（由 localStorage 還原）

### Requirement: Blueprint 合法性驗證

系統 SHALL 在匯入或儲存 blueprint 前進行靜態驗證；不合法時 MUST 拒絕並提供可定位的錯誤訊息。

#### Scenario: 缺少起點或計產站
- **WHEN** blueprint 沒有補料(refill)起點或沒有指定計產站(sink)
- **THEN** 驗證失敗並指出缺少的元素

#### Scenario: 群組鏈不連續
- **WHEN** 群組無法串成有效鏈（首組來源非補料、組間目的與下一組來源不相符、或末組目的非計產站）
- **THEN** 驗證失敗並指出斷裂的群組

#### Scenario: 群組缺 AGV
- **WHEN** 任一群組的 AGV 數量小於 1
- **THEN** 驗證失敗並指出該群組

### Requirement: Blueprint 乾跑檢查

系統 SHALL 提供乾跑（dry-run）：以該 blueprint 建立暫時模擬並快轉有限步數，確認會產出且無立即死鎖。

#### Scenario: 乾跑通過
- **WHEN** 對一個配置合理的 blueprint 執行乾跑
- **THEN** 在限定步數內偵測到產出，回報通過

#### Scenario: 乾跑偵測無產出
- **WHEN** 對一個會卡死或無法出料的 blueprint 執行乾跑
- **THEN** 回報失敗並盡可能指出停滯的群組或站別
