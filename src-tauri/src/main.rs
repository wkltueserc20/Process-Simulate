// 製程 AGV 模擬系統 — Tauri v2 桌面殼
// 前端為 ../ui 的靜態 HTML。提供 save_file 指令讓匯出可自選儲存路徑。
// 注意：刻意「不」啟用 withGlobalTauri（其注入機制在本前端會導致整頁空白）。
// 前端改用一律存在的 window.__TAURI_INTERNALS__.invoke 呼叫本指令。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::Engine;
use tauri_plugin_dialog::DialogExt;

// 開啟原生「另存新檔」對話框並把 base64 內容寫入所選檔案。
// 回傳所選路徑字串；使用者按取消時回傳 None。
#[tauri::command]
async fn save_file(
    app: tauri::AppHandle,
    default_name: String,
    b64: String,
    filter_name: String,
    exts: Vec<String>,
) -> Result<Option<String>, String> {
    let mut dlg = app.dialog().file().set_file_name(&default_name);
    if !exts.is_empty() {
        let ext_refs: Vec<&str> = exts.iter().map(|s| s.as_str()).collect();
        dlg = dlg.add_filter(&filter_name, &ext_refs);
    }
    match dlg.blocking_save_file() {
        Some(file_path) => {
            // 桌面儲存對話框一律回傳檔案系統路徑
            let path = file_path.to_string();
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(b64.as_bytes())
                .map_err(|e| e.to_string())?;
            std::fs::write(&path, bytes).map_err(|e| e.to_string())?;
            Ok(Some(path))
        }
        None => Ok(None),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![save_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
