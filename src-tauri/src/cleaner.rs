use serde::{Deserialize, Serialize};
use std::{env, fs, path::{Path, PathBuf}, time::{SystemTime, UNIX_EPOCH}};

#[derive(Clone, Serialize)]
pub struct ScanResult { pub id: String, pub name: String, pub description: String, pub icon: String, pub bytes: u64, pub files: u64, pub requires_admin: bool }
#[derive(Serialize)]
pub struct CleanupSummary { pub freed_bytes: u64, pub skipped: u64 }
#[derive(Clone, Serialize, Deserialize)]
pub struct HistoryEntry { pub timestamp: String, pub freed_bytes: u64, pub skipped: u64 }

struct Rule { id: &'static str, name: &'static str, description: &'static str, icon: &'static str, admin: bool, paths: Vec<PathBuf> }

fn local() -> PathBuf { env::var_os("LOCALAPPDATA").map(PathBuf::from).unwrap_or_default() }
fn rules() -> Vec<Rule> {
  let local = local(); let temp = env::var_os("TEMP").map(PathBuf::from).unwrap_or_default();
  vec![
    Rule { id:"user-temp", name:"临时文件", description:"应用运行时留下的安全临时数据", icon:"◌", admin:false, paths:vec![temp, local.join("Temp")] },
    Rule { id:"thumbnails", name:"缩略图缓存", description:"图片与视频预览会自动重新生成", icon:"▦", admin:false, paths:thumbnail_files(&local) },
    Rule { id:"reports", name:"错误报告", description:"程序崩溃后留下的报告与日志", icon:"!", admin:false, paths:vec![local.join("CrashDumps"), local.join("Microsoft/Windows/WER/ReportArchive")] },
    Rule { id:"chrome-cache", name:"Chrome 缓存", description:"不影响书签、密码、历史或登录状态", icon:"◎", admin:false, paths:vec![local.join("Google/Chrome/User Data/Default/Cache"), local.join("Google/Chrome/User Data/Default/Code Cache")] },
    Rule { id:"edge-cache", name:"Edge 缓存", description:"不影响书签、密码、历史或登录状态", icon:"◉", admin:false, paths:vec![local.join("Microsoft/Edge/User Data/Default/Cache"), local.join("Microsoft/Edge/User Data/Default/Code Cache")] },
    Rule { id:"firefox-cache", name:"Firefox 缓存", description:"不影响书签、密码、历史或登录状态", icon:"◍", admin:false, paths:firefox_cache_paths(&local) },
  ]
}

fn firefox_cache_paths(local: &Path) -> Vec<PathBuf> {
  let root = local.join("Mozilla/Firefox/Profiles");
  fs::read_dir(root).ok().into_iter().flatten().filter_map(Result::ok).map(|e| e.path().join("cache2")).collect()
}
fn thumbnail_files(local: &Path) -> Vec<PathBuf> {
  let root = local.join("Microsoft/Windows/Explorer");
  fs::read_dir(root).ok().into_iter().flatten().filter_map(Result::ok)
    .filter(|e| e.file_name().to_string_lossy().starts_with("thumbcache_"))
    .map(|e| e.path()).collect()
}
// The engine never receives paths from the frontend. These fixed roots are the only deletion targets.
fn protected(path: &Path) -> bool {
  let p = path.to_string_lossy().to_lowercase();
  ["\\windows\\system32", "\\program files", "\\programdata", "\\users\\default"].iter().any(|x| p.contains(x))
}
fn measure(path: &Path) -> (u64, u64) {
  let meta = match fs::symlink_metadata(path) { Ok(m) => m, Err(_) => return (0, 0) };
  if meta.file_type().is_symlink() || protected(path) { return (0, 0) }
  if meta.is_file() { return (meta.len(), 1) }
  let mut bytes = 0; let mut files = 0;
  if let Ok(entries) = fs::read_dir(path) { for e in entries.flatten() { let (b, f) = measure(&e.path()); bytes += b; files += f; } }
  (bytes, files)
}
pub fn scan() -> Vec<ScanResult> { rules().into_iter().map(|r| { let (bytes, files) = r.paths.iter().fold((0,0), |(b,f), p| { let (x,y)=measure(p); (b+x,f+y) }); ScanResult { id:r.id.into(), name:r.name.into(), description:r.description.into(), icon:r.icon.into(), bytes, files, requires_admin:r.admin } }).collect() }

fn delete_contents(root: &Path) -> (u64, u64) {
  if protected(root) { return (0, 1) }
  if root.is_file() {
    let (bytes, _) = measure(root);
    return match fs::remove_file(root) { Ok(_) => (bytes, 0), Err(_) => (0, 1) };
  }
  let mut freed = 0; let mut skipped = 0;
  let entries = match fs::read_dir(root) { Ok(e) => e, Err(_) => return (0, 0) };
  for entry in entries.flatten() {
    let p = entry.path(); let (bytes, _) = measure(&p);
    let result = if p.is_dir() { fs::remove_dir_all(&p) } else { fs::remove_file(&p) };
    match result { Ok(_) => freed += bytes, Err(_) => skipped += 1 }
  }
  (freed, skipped)
}
pub fn clean(ids: Vec<String>) -> CleanupSummary {
  let mut freed = 0; let mut skipped = 0;
  for r in rules().into_iter().filter(|r| ids.iter().any(|id| id == r.id)) { for p in r.paths { let (b,s)=delete_contents(&p); freed+=b; skipped+=s; } }
  let entry = HistoryEntry { timestamp: timestamp(), freed_bytes: freed, skipped }; append_history(&entry); CleanupSummary { freed_bytes:freed, skipped }
}
fn data_file() -> PathBuf { local().join("Qingli/history.json") }
pub fn history() -> Vec<HistoryEntry> { fs::read_to_string(data_file()).ok().and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default() }
fn append_history(item: &HistoryEntry) { let mut list = history(); list.insert(0, item.clone()); list.truncate(30); let file = data_file(); if let Some(p)=file.parent() { let _=fs::create_dir_all(p); } let _=fs::write(file, serde_json::to_string(&list).unwrap_or_default()); }
fn timestamp() -> String { SystemTime::now().duration_since(UNIX_EPOCH).map(|d| format!("{}", d.as_secs())).unwrap_or_else(|_| "未知时间".into()) }

#[cfg(test)]
mod tests { use super::*; #[test] fn protected_system_path_is_never_scanned() { assert!(protected(Path::new("C:\\Windows\\System32\\x"))); } #[test] fn absent_path_is_empty() { assert_eq!(measure(Path::new("Z:\\missing-qingli")), (0,0)); } }
