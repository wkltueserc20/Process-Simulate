# command-bar-hierarchy Specification

## Purpose
TBD - created by archiving change ux-improvement-pass. Update Purpose after archive.
## Requirements
### Requirement: 控制列依性質分群與分層

系統 SHALL 將 Header 控制列依使用性質分為主要動作、分析、工具三群，並以視覺權重區分（主要動作最顯眼、工具群弱化）。

#### Scenario: 主要動作最顯眼
- **WHEN** 使用者檢視 Header
- **THEN** 播放/暫停與速度等主要動作具有最高視覺權重，最易被找到

#### Scenario: 工具群弱化
- **WHEN** 使用者檢視 Header
- **THEN** 主題、說明、視角等工具類控制以較低視覺權重呈現，不與主要動作競爭注意力

### Requirement: 分群的無障礙標示

控制列各群 SHALL 以語意分組（group 與標籤）標示，且 Tab 焦點順序 SHALL 維持合理。

#### Scenario: 群組可被輔助技術辨識
- **WHEN** 使用螢幕報讀器或鍵盤瀏覽控制列
- **THEN** 各功能群有可辨識的群組語意與標籤，焦點順序符合視覺順序

