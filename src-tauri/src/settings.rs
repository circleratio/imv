use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

const MAX_RECENT_PATHS: usize = 10;

#[derive(Serialize, Deserialize, Default)]
struct Settings {
    trash_dir: Option<String>,
    #[serde(default)]
    recent_paths: Vec<String>,
}

fn settings_path() -> Result<PathBuf, String> {
    let config_dir =
        dirs::config_dir().ok_or_else(|| "設定フォルダを取得できませんでした".to_string())?;
    let app_dir = config_dir.join("imv");
    fs::create_dir_all(&app_dir)
        .map_err(|e| format!("設定フォルダの作成に失敗しました: {}", e))?;
    Ok(app_dir.join("settings.json"))
}

/// デフォルトのトラッシュ置き場: `~/tmp/Trash`
fn default_trash_dir() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or_else(|| "ホームディレクトリを取得できませんでした".to_string())?;
    Ok(home.join("tmp").join("Trash"))
}

fn load_settings() -> Result<Settings, String> {
    let path = settings_path()?;
    if !path.exists() {
        return Ok(Settings::default());
    }
    let content =
        fs::read_to_string(&path).map_err(|e| format!("設定の読み込みに失敗しました: {}", e))?;
    serde_json::from_str(&content).map_err(|e| format!("設定の解析に失敗しました: {}", e))
}

fn save_settings(settings: &Settings) -> Result<(), String> {
    let settings_file = settings_path()?;
    let content = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("設定のシリアライズに失敗しました: {}", e))?;
    fs::write(&settings_file, content).map_err(|e| format!("設定の保存に失敗しました: {}", e))
}

/// 設定されたトラッシュ置き場を返す。未設定ならデフォルト値を返す。
pub fn resolve_trash_dir() -> Result<PathBuf, String> {
    let settings = load_settings()?;
    match settings.trash_dir {
        Some(dir) => Ok(PathBuf::from(dir)),
        None => default_trash_dir(),
    }
}

#[tauri::command]
pub fn get_trash_dir() -> Result<String, String> {
    Ok(resolve_trash_dir()?.to_string_lossy().to_string())
}

#[tauri::command]
pub fn set_trash_dir(path: String) -> Result<(), String> {
    let mut settings = load_settings()?;
    settings.trash_dir = Some(path);
    save_settings(&settings)
}

/// 既存エントリを取り除いた上で先頭に追加し、最大件数を超えたら末尾を切り捨てる。
fn upsert_recent_path(list: &mut Vec<String>, path: String) {
    list.retain(|p| p != &path);
    list.insert(0, path);
    list.truncate(MAX_RECENT_PATHS);
}

#[tauri::command]
pub fn get_recent_paths() -> Result<Vec<String>, String> {
    Ok(load_settings()?.recent_paths)
}

#[tauri::command]
pub fn add_recent_path(path: String) -> Result<(), String> {
    let mut settings = load_settings()?;
    upsert_recent_path(&mut settings.recent_paths, path);
    save_settings(&settings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_trash_dir_is_under_home_tmp() {
        let dir = default_trash_dir().unwrap();
        let home = dirs::home_dir().unwrap();
        assert_eq!(dir, home.join("tmp").join("Trash"));
    }

    #[test]
    fn upsert_recent_path_adds_to_front() {
        let mut list = vec!["b".to_string(), "c".to_string()];
        upsert_recent_path(&mut list, "a".to_string());
        assert_eq!(list, vec!["a", "b", "c"]);
    }

    #[test]
    fn upsert_recent_path_moves_existing_entry_to_front_without_duplicating() {
        let mut list = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        upsert_recent_path(&mut list, "b".to_string());
        assert_eq!(list, vec!["b", "a", "c"]);
    }

    #[test]
    fn upsert_recent_path_truncates_to_max() {
        let mut list: Vec<String> = (0..MAX_RECENT_PATHS).map(|i| i.to_string()).collect();
        upsert_recent_path(&mut list, "new".to_string());
        assert_eq!(list.len(), MAX_RECENT_PATHS);
        assert_eq!(list[0], "new");
        assert!(!list.contains(&(MAX_RECENT_PATHS - 1).to_string()));
    }
}
