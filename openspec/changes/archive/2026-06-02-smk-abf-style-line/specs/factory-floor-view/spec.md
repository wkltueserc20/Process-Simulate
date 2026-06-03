## ADDED Requirements

### Requirement: SMK 採 ABF 式 PORT 站卡渲染
SMK 製程線 SHALL 以與 ABF 相同的 slot/PORT 站卡、分組 zone、AGV 與載貨指示渲染，取代原輸送帶呈現。

#### Scenario: SMK 站卡顯示 PORT
- **WHEN** 顯示 SMK 線
- **THEN** 各 transfer 站顯示入料端（入滿框/拿料位/退空框）與出料端（收空框/放置位/出滿框）；refill 顯示收空框/空框放置/出滿框；buffer 顯示容量與待命；changer 顯示入料端，且 slot 狀態以亮色圖例上色

#### Scenario: SMK 分組 zone 與 AGV 標示
- **WHEN** 顯示 SMK 線
- **THEN** 六個群組以 zone 區隔並標示群組與所屬 AGV（A1–A11），AGV 以群組色環、載貨菱形與狀態標籤呈現

### Requirement: 多站佈局策略
factory-floor-view SHALL 以佈局策略容納 SMK 的 11 站，避免站卡重疊且維持可讀字級。

#### Scenario: 蛇形換行佈局
- **WHEN** 製程線站數超過單列可容納上限
- **THEN** 站點以蛇形（多列）佈局排列，主軌道依序連接，群組標示隨列繪製，整線仍可一眼瀏覽

#### Scenario: AGV 沿蛇形移動
- **WHEN** AGV 於蛇形佈局中由一站移動至下一站
- **THEN** AGV 沿對應路徑平滑移動，轉列在站點處銜接，位置與載貨狀態正確
