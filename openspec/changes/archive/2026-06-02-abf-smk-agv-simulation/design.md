## Context

欣興電子 ABF + SMK 兩條 PCB 製程線目前以靜態報表呈現產能數據，無法讓管理層直觀理解 WIP 堆積與瓶頸形成的動態過程。本系統以單一 HTML 檔案實作，可在任何瀏覽器執行，無需安裝。

製程數據：
- ABF 線：CLN → Annealing → CZ → Baking → 壓機 → Postcure → PORT，cycle 20:30，5 台 AGV
- SMK 線：Baking → 壓機 → 曝光 → 顯影 → SoftBaking → UVQ → Postcure → PORT，cycle 35:50，6 台 AGV
- 瓶頸：SMK I→J（5:50）與 K→L（4:50）

## Goals / Non-Goals

**Goals:**
- 在瀏覽器中完整展示 AGV 移動、WIP 變化、瓶頸高亮
- 可調整 cycle time、AGV 數量等參數後重新模擬
- 播放 / 暫停 / 快轉 / 重置控制
- 輸出為單一 `index.html`，可離線使用

**Non-Goals:**
- 不連接真實工廠 MES 或感測器資料
- 不支援隨機故障事件模擬
- 不需要登入或後端服務
- 不需要儲存歷史模擬記錄

## Decisions

### 決策 1：純 HTML + Vanilla JS，不用框架

**選擇**：單一 `index.html`，內嵌 CSS 和 JS，Chart.js 從 CDN 引入。

**理由**：客戶端只需雙擊檔案即可執行，無需 Node.js、npm 或網路伺服器。降低分發成本。

**考慮過的替代方案**：
- React + Vite：開發體驗好，但需要 build 流程，客戶端難以直接開啟
- Vue CDN 版：可行但增加學習門檻，對此規模專案無必要

---

### 決策 2：離散事件模擬（DES）引擎

**選擇**：以「事件佇列」為核心，每個 AGV 完成搬運後觸發下一個事件。

```
事件佇列（按模擬時間排序）
[t=0: AGV1開始移動] → [t=250: AGV1到達，AGV2出發] → ...
```

**理由**：比逐幀更新（frame-by-frame）精確；WIP 計算直接從事件推導，不會有累積誤差。

**考慮過的替代方案**：
- 逐秒 tick 模擬：實作簡單，但模擬誤差會隨時間放大，且快轉時效能差

---

### 決策 3：SVG 工廠平面圖 + CSS transition 做 AGV 動畫

**選擇**：工廠站點用 SVG rect/text 繪製，AGV 用 SVG circle + CSS `transition: transform` 實現平滑移動。

**理由**：SVG 可任意縮放，CSS transition 由瀏覽器硬體加速，效能最佳。

**考慮過的替代方案**：
- Canvas 2D：繪製靈活但難以做點擊互動、縮放
- CSS Flexbox 佈局：無法精確控制 AGV 位置

---

### 決策 4：AGV 專車制（每台 AGV 綁定固定路段）

**選擇**：每台 AGV 只在指定兩個站點間來回，不做排班調度。

```
ABF AGV1: CLN ↔ Annealing
ABF AGV2: Annealing ↔ CZ
...
```

**理由**：符合原始製程設計，模擬邏輯簡單，視覺上也清楚。

**考慮過的替代方案**：
- 動態調度（最短等待優先）：更接近真實，但超出展示用途的複雜度

---

### 決策 5：時間壓縮比（playback speed）

**選擇**：1x = 1 秒真實時間對應 10 秒模擬時間；2x/5x 等比加速。

```
1x  → 模擬 1小時 需 6 分鐘觀看
5x  → 模擬 1小時 需 72 秒觀看
```

**理由**：讓觀眾在 1~2 分鐘內看完完整瓶頸形成過程。

## Risks / Trade-offs

| 風險 | 緩解方式 |
|------|---------|
| Chart.js CDN 在離線環境無法載入 | 提供內嵌版本選項（將 Chart.js 壓縮後直接嵌入 HTML） |
| AGV 動畫在低效能電腦卡頓 | 快轉時改用直接跳轉位置（不用 transition），確保 60fps |
| 參數調整後模擬狀態不一致 | 每次參數變更都完全重置模擬引擎，不做增量更新 |
| SVG 在高 DPI 螢幕顯示模糊 | 使用 viewBox 確保向量渲染 |

## Open Questions

- 是否需要「比較模式」（調整前 vs 調整後同時顯示）？
- ABF 線的 PORT 和 SMK 線的 PORT 是否為同一個出口？
- 是否需要中文 / 英文切換？
