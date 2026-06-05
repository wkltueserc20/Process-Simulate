## Context

整個前端是單一檔案 `ui/index.html`（約 3170 行），純前端、狀態全在記憶體。版面結構：

```
#hdr  (logo │ #hdr-info[標題·時鐘·進度條] │ #hdr-ctrl[~15 顆按鈕一列] )
#main
  #fw
    #stage  (relative)
      #fsvg            ← 流程圖主舞台
      #bot.collapsed   ← 統計圖表覆蓋層（absolute, inset:0, 預設隱藏）
        #cw  ( #chart-tools + #cwrap>canvas#uc )
        #sp  ( 即時統計：UPH/已完成/目標/預估 + #bnk-section 瓶頸警示 )   ← 被埋在這
    #bot-toggle        ← 底部「統計圖表」開關列（含 #bot-mini 迷你摘要）
  #sb  (右側參數抽屜)
#ov   (達標/結果視窗)、#agv-board、#cmp-table-ov、#line-wizard、#coach、toast…
```

關鍵現況與限制：
- `#sp`（即時統計＋瓶頸）目前是 `#bot` 覆蓋層的子節點 → 圖表收合時整塊看不到。`#bot-mini` 只在收合時於 toggle 列顯示「完成 N 片 / UPH / ETA」一行純文字摘要。
- `updateProgressAndStats(tot, uph)`（`:2055`）負責更新進度條、`#s-auph`、`#s-done`、`#s-eta`、`#bot-mini`。
- 只有 1 條 media query（`prefers-reduced-motion`）；其餘版面用固定/`clamp()` 寬度。
- 上一輪剛把 `#bot` 改為覆蓋層 + 圖表縮放/平移（`chartView`、`applyChartView`）；本次需避免破壞它。

## Goals / Non-Goals

**Goals:**
- 即時 UPH/完成/ETA/瓶頸常駐可見，與圖表覆蓋層解耦。
- Header 控制列主次分明、降低掃描成本。
- 版面在窄/寬視窗都不破；新增簡報大字模式。
- 抬升比較/建議的決策動線可發現性。
- 視覺一致、圖例可讀、站別色盲友善。
- 結果視窗成績單化 + 空狀態引導。

**Non-Goals:**
- 不動模擬引擎與數值。
- 不引入框架/外部相依。
- 不持久化新狀態（除非沿用既有 localStorage 主題模式那種輕量偏好）。

## Decisions

### D1：vitals 條獨立於 `#bot`，而非沿用 `#bot-mini`
把即時統計抽成一個獨立、常駐的列（建議掛在 `#fw` 底部或 `#hdr-info` 旁），不再依賴「圖表是否展開」。`#sp` 內的「即時統計」與「瓶頸警示」可保留在圖表覆蓋層作為細節版，但 vitals 條成為單一真實來源並隨 `updateProgressAndStats()` 即時刷新。
- 取捨：vitals 條會佔一點垂直空間；以極輕量（單列、tabular-nums、圖示+數字）控制。
- 瓶頸指示：以小圓點/徽章呈現「🟢無瓶頸 / 🔴N 站堆積」，點擊可展開圖表覆蓋層看細節（連動既有 `toggleBot`）。

### D2：控制列分群用「視覺分組 + 次要收納」，避免大改互動
維持按鈕都在 header（不強推下拉選單以免增加點擊深度），但用既有 `.cg` 群組 + `.hdr-sep` 把它們重排為三群：
```
[▶ 播放  1× 2× 5× 10×  ⏩ 立即結果]   |   [📊 看板  📋 比較]   |   [⤢ ▤ ⚙ 🌙 ?]
   主要動作（最高權重）                      分析                     工具（弱化）
```
- 工具群用較低視覺權重（icon-only、dim 色）。`重置/切換線` 歸主要或分析群待定。
- 窄視窗時工具群可收進「⋯ 更多」（與 D3 斷點連動）。

### D3：RWD 斷點 + 簡報模式是兩件事
- **斷點**：加 `@media (max-width: …)` 在窄視窗時讓控制列換行/收納次要群、縮短 label。目標是「不破版」，不是手機體驗。
- **簡報模式**（`presentation-mode`）：一個全域 class（如 `body.present`），切換後隱藏參數抽屜/次要控制，放大流程圖，並把 vitals 條換成超大字版（完成/UPH/ETA/瓶頸）。用快捷鍵（建議 `F` 或 header 一顆鈕）切換，`Esc` 退出。
- 取捨：簡報模式是疊加樣式，不改 DOM 結構 → 風險低、好回退。

### D4：決策動線用「情境觸發的引導」而非新頁面
- 達標結果視窗（成績單）直接內建「📌 釘選此方案以比較」「📋 開比較表」CTA（部分已有，強化呈現）。
- 瓶頸出現時，vitals 的瓶頸徽章/結果視窗提供「查看瓶頸建議」入口連到既有 `bottleneck-advisor`。
- 不新增資料能力，只改可發現性與入口位置。

### D5：站別色盲友善沿用既有「形狀疊加」手法
既有 `colorblind-agv-encoding` 已對 AGV 車頂用 ●/○ 形狀。站別狀態（滿框/空框/加工中）比照：除顏色外加上小符號或邊框樣式，使去色仍可分辨。屬 `display-refinements` 範疇。

### D6：圖例與字級
- 流程圖圖例（`abf-hdr2` 的一長串 10px 文字）改為更易讀的排版（分段/加大/或移到固定角落卡片），避免最該讀的東西最小。
- 盤點關鍵數值字級與對比（WCAG AA），必要處上調。

### D7：風險控管與驗證
- 本次動到剛建的 `#bot` 覆蓋層與 `#stage` 版面，且桌面 WebView2 對 inline script/版面敏感 → 每個版面類變更都要用既有 CDP 流程在真實 WebView2 驗證（選擇畫面正常載入、vitals 即時更新、簡報模式進出、圖表覆蓋層仍可開關縮放）。
- 簡報模式與斷點以「疊加 class/樣式」實作，確保可快速回退。

## Open Questions

- vitals 條的落點：常駐在「流程圖下緣一條」還是「併入 header 資訊列」？（影響垂直空間）
- 簡報模式觸發鍵與既有快捷鍵（Space/→/1/2/5/R/P/Esc）是否衝突？建議用 `F`。
- 控制列「重置 / 切換線」歸哪一群最直覺？
- 是否需要「簡報模式」記憶上次狀態（localStorage）？
