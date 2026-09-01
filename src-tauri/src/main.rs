mod cleaner;

#[tauri::command]
fn scan_cleanup() -> Vec<cleaner::ScanResult> { cleaner::scan() }
#[tauri::command]
fn run_cleanup(ids: Vec<String>) -> cleaner::CleanupSummary { cleaner::clean(ids) }
#[tauri::command]
fn get_history() -> Vec<cleaner::HistoryEntry> { cleaner::history() }

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![scan_cleanup, run_cleanup, get_history])
    .run(tauri::generate_context!())
    .expect("无法启动清理工具");
}
