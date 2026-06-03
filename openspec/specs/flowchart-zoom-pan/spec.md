# flowchart-zoom-pan Specification

## Purpose

提供流程圖的滑鼠滾輪縮放、拖曳平移與視角重置能力，讓使用者調整觀看視野，且僅影響攝影機（viewBox）而不改變站別與 AGV 的實際座標。

## Requirements

### Requirement: 流程圖滾輪縮放

系統 SHALL 允許使用者在流程圖 SVG 上以滑鼠滾輪縮放，並以游標位置為縮放中心。縮放僅影響流程圖的攝影機（viewBox），不改變站別與 AGV 的實際座標。

#### Scenario: 向上滾動放大
- **WHEN** 使用者在流程圖上向上滾動滾輪
- **THEN** 流程圖以游標所在位置為中心放大，游標下的內容點維持在原螢幕位置附近

#### Scenario: 向下滾動縮小
- **WHEN** 使用者在流程圖上向下滾動滾輪
- **THEN** 流程圖以游標所在位置為中心縮小

#### Scenario: 縮放範圍限制
- **WHEN** 使用者持續放大或縮小
- **THEN** 縮放被限制在合理上下限內，不會縮到看不見或無限放大

### Requirement: 流程圖拖曳平移

系統 SHALL 允許使用者在流程圖上按住滑鼠拖曳以平移視野（四方向）。

#### Scenario: 拖曳平移
- **WHEN** 使用者在流程圖上按住滑鼠並移動
- **THEN** 流程圖視野跟隨拖曳方向平移

#### Scenario: 平移不誤觸 tooltip 或站別互動
- **WHEN** 使用者進行拖曳平移
- **THEN** 拖曳行為不被誤判為單純 hover，平移結束後 hover tooltip 行為恢復正常

### Requirement: 視角重置

系統 SHALL 提供重置視角的方式，並在切換製程線或切換精簡/詳細視圖時自動重置攝影機為該佈局的預設 viewBox。

#### Scenario: 手動重置
- **WHEN** 使用者點擊「重置視角」按鈕
- **THEN** 流程圖回到該佈局的預設 viewBox（縮放與平移歸零）

#### Scenario: 切換視圖自動重置
- **WHEN** 使用者切換製程線或切換精簡/詳細視圖
- **THEN** 攝影機自動重置為目標佈局的預設 viewBox
