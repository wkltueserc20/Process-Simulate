## MODIFIED Requirements

### Requirement: 瓶頸偵測與視覺警示
系統 SHALL 以 PORT 堆積為依據偵測瓶頸並提供視覺警示，對 ABF 與 SMK 採一致語彙，取代原 SMK 以 WIP 閾值判定的方式。

#### Scenario: PORT 堆積標示
- **WHEN** 某站的出滿框/退空框 PORT 有料持續未被取走，或某 buffer 接近滿載達一定時間
- **THEN** 該站於畫面標示堆積警示，並列入瓶頸清單顯示站名與堆積情形

#### Scenario: 無堆積時
- **WHEN** 各站 PORT 與 buffer 皆正常流動、無持續堆積
- **THEN** 瓶頸清單顯示「目前無瓶頸」

#### Scenario: 移除 WIP 閾值判定
- **WHEN** 顯示 SMK 線
- **THEN** 不再以 WIP 警告/危險閾值對站點上色或判定瓶頸（該機制已由 PORT 堆積式取代）
