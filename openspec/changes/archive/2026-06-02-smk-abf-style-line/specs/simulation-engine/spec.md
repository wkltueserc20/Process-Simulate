## ADDED Requirements

### Requirement: 一般化引擎驅動多製程線
模擬引擎 SHALL 接受 line config 與該線 runtime 狀態進行驅動，使 ABF 與 SMK 共用同一套機台邏輯、AGV 步進與取放料/預訂機制。

#### Scenario: ABF 行為等價
- **WHEN** 以 ABF line config 執行一般化引擎
- **THEN** ABF 的站別行為、AGV 搬運、產出統計與一般化前完全一致（零回歸）

#### Scenario: SMK 以同引擎運作
- **WHEN** 以 SMK line config 執行一般化引擎
- **THEN** SMK 的 11 站 PORT 行為、6 組 AGV 搬運與產出統計，依 ABF 相同規則運作

### Requirement: 2 站直送群組（無中間緩衝）
引擎 SHALL 支援群組站序僅含來源與目的兩站（無中間 buffer）的直送型態。

#### Scenario: transfer 直送 transfer
- **WHEN** 某群組站序為 [來源 transfer, 目的 transfer]
- **THEN** 滿框由來源出料 PORT 直接搬至目的入料 PORT，空框由目的退空框 PORT 直接回流至來源收空框 PORT，且套用與一般 transfer 相同的保護期

#### Scenario: 直送不需經緩衝即計產與回流
- **WHEN** 直送群組完成一次搬運
- **THEN** 不經任何 buffer 站，產出與空框回流時序正確、不卡死

### Requirement: SMK 線性鏈與分組搬運
引擎 SHALL 依 SMK config 驅動線性主鏈 CLN→Annealing→CZ→Baking→SMK壓機→曝光機→顯影機→SoftBaking→UVQ→Postcure→換框機 的六組搬運。

#### Scenario: 六組各司其職
- **WHEN** SMK 模擬執行中
- **THEN** G1(CLN→Annealing→CZ)、G2(CZ→Baking→壓機)、G3(壓機→曝光)、G4(曝光→顯影)、G5(顯影→SoftBaking→UVQ)、G6(UVQ→Postcure→換框機) 各以其專用 AGV 搬運，換框機完成即計產
