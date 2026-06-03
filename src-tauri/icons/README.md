# 圖示（必要）

`tauri.conf.json` 的 `bundle.icon` 會引用本資料夾的圖示檔；**首次建置前需先產生**，否則 `cargo tauri build` / `dev` 會找不到圖示而失敗。

準備一張方形 PNG（建議 ≥512×512，例如公司 logo），於專案根目錄執行：

```powershell
cargo tauri icon path\to\logo.png
```

會自動產生本資料夾所需的：
`32x32.png`、`128x128.png`、`128x128@2x.png`、`icon.icns`、`icon.ico`（以及 Windows Store 用的多種尺寸）。

> 這些產生出的二進位圖示通常**不需**進版控（可加入 .gitignore），或視團隊習慣保留。
