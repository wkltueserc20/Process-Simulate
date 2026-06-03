// 製程 AGV 模擬系統 — Tauri v2 桌面殼（Level A 純包裝）
// 前端為 ../ui 的靜態 HTML；此處僅啟動視窗，無自訂指令。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
