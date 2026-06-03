## ADDED Requirements

### Requirement: SMK ABF 式可調參數
參數面板 SHALL 提供 SMK 線的 ABF 式可調參數，並沿用既有 metadata、分區、驗證、stepper 與情境機制。

#### Scenario: SMK 製程時間可設定
- **WHEN** 使用者於 SMK 線調整 CLN 生產、CZ、SMK壓機、曝光、顯影、UVQ 處理時間、換框機拿料時間或每框片數
- **THEN** 這些參數套用後以新值重新模擬（屬需重跑類）

#### Scenario: SMK 緩衝與路段可設定
- **WHEN** 使用者調整 Annealing/Baking/SoftBaking/Postcure 的容量或 holdTime、11 站間路段行走時間、或機台/緩衝站取放料時間
- **THEN** 參數套用後生效，路段示意圖反映 SMK 拓樸

#### Scenario: 目標產量沿用
- **WHEN** 使用者於 SMK 線調整目標產量
- **THEN** 沿用即時生效行為，不重置

## REMOVED Requirements

### Requirement: SMK 輸送帶模型參數
**Reason**: SMK 改為 ABF 式 slot/PORT 引擎，輸送帶相關參數不再適用。
**Migration**: 移除 `SMK Cycle Time`、`每 Cycle 批量`、`SMK AGV 數量`、`WIP 警告閾值`、`WIP 危險閾值`；改用 SMK 的 ABF 式參數（各站處理時間、緩衝容量/holdTime、路段行走時間、取放料時間）。SMK AGV 改為固定分組 A1–A11，不再以數量參數調整，亦不再提供 SMK 即時加減 AGV。
