use std::fs;
use std::path::{Path, PathBuf};

use crate::settings::resolve_trash_dir;

/// 指定されたファイルを設定済みのトラッシュフォルダ（未設定時は `~/tmp/Trash`）へ移動する。
/// 同名ファイルが既に存在する場合は連番サフィックス(_1, _2, ...)を付与する。
#[tauri::command]
pub fn move_to_trash(path: String) -> Result<String, String> {
    let source = Path::new(&path);
    if !source.exists() {
        return Err(format!("ファイルが存在しません: {}", path));
    }

    let trash_dir = resolve_trash_dir()?;
    fs::create_dir_all(&trash_dir)
        .map_err(|e| format!("トラッシュフォルダの作成に失敗しました: {}", e))?;

    let file_name = source
        .file_name()
        .ok_or_else(|| format!("ファイル名を取得できません: {}", path))?;
    let dest = unique_destination(&trash_dir, Path::new(file_name));

    fs::rename(source, &dest).map_err(|e| format!("ファイルの移動に失敗しました: {}", e))?;

    Ok(dest.to_string_lossy().to_string())
}

/// Restores a file previously moved by `move_to_trash` back to its original location.
/// Fails if the trashed file is missing or a file already exists at the original path.
#[tauri::command]
pub fn restore_from_trash(trashed_path: String, original_path: String) -> Result<String, String> {
    let source = Path::new(&trashed_path);
    if !source.exists() {
        return Err(format!("トラッシュ内にファイルが存在しません: {}", trashed_path));
    }
    let dest = Path::new(&original_path);
    if dest.exists() {
        return Err(format!("復元先に同名のファイルが既に存在します: {}", original_path));
    }
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("復元先フォルダの作成に失敗しました: {}", e))?;
    }

    fs::rename(source, dest).map_err(|e| format!("ファイルの復元に失敗しました: {}", e))?;

    Ok(original_path)
}

fn unique_destination(dir: &Path, file_name: &Path) -> PathBuf {
    let candidate = dir.join(file_name);
    if !candidate.exists() {
        return candidate;
    }

    let stem = file_name
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("file");
    let ext = file_name.extension().and_then(|s| s.to_str());

    let mut counter = 1;
    loop {
        let new_name = match ext {
            Some(ext) => format!("{}_{}.{}", stem, counter, ext),
            None => format!("{}_{}", stem, counter),
        };
        let candidate = dir.join(new_name);
        if !candidate.exists() {
            return candidate;
        }
        counter += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn unique_destination_returns_original_when_no_conflict() {
        let dir = std::env::temp_dir().join(format!("imv-trash-test-a-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();

        let dest = unique_destination(&dir, Path::new("photo.jpg"));
        assert_eq!(dest, dir.join("photo.jpg"));

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn unique_destination_appends_counter_on_conflict() {
        let dir = std::env::temp_dir().join(format!("imv-trash-test-b-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("photo.jpg"), b"x").unwrap();
        fs::write(dir.join("photo_1.jpg"), b"x").unwrap();

        let dest = unique_destination(&dir, Path::new("photo.jpg"));
        assert_eq!(dest, dir.join("photo_2.jpg"));

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn move_to_trash_moves_file_into_configured_trash_dir() {
        let trash_dir = resolve_trash_dir().unwrap();
        let unique_name = format!("imv-trash-cmd-test-{}.png", std::process::id());
        let source = std::env::temp_dir().join(&unique_name);
        fs::write(&source, b"x").unwrap();

        let dest = move_to_trash(source.to_string_lossy().to_string()).unwrap();

        assert!(Path::new(&dest).exists());
        assert!(!source.exists());
        assert!(dest.starts_with(trash_dir.to_string_lossy().as_ref()));

        fs::remove_file(&dest).ok();
    }

    #[test]
    fn restore_from_trash_moves_file_back_to_original_path() {
        let unique_name = format!("imv-restore-cmd-test-{}.png", std::process::id());
        let source = std::env::temp_dir().join(&unique_name);
        fs::write(&source, b"x").unwrap();
        let original = source.to_string_lossy().to_string();

        let trashed = move_to_trash(original.clone()).unwrap();
        let restored = restore_from_trash(trashed.clone(), original.clone()).unwrap();

        assert_eq!(restored, original);
        assert!(source.exists());
        assert!(!Path::new(&trashed).exists());

        fs::remove_file(&source).ok();
    }

    #[test]
    fn restore_from_trash_fails_when_original_path_is_occupied() {
        let dir = std::env::temp_dir().join(format!("imv-restore-test-c-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let trashed = dir.join("trashed.jpg");
        let original = dir.join("original.jpg");
        fs::write(&trashed, b"x").unwrap();
        fs::write(&original, b"y").unwrap();

        let result = restore_from_trash(
            trashed.to_string_lossy().to_string(),
            original.to_string_lossy().to_string(),
        );

        assert!(result.is_err());
        assert!(trashed.exists());

        fs::remove_dir_all(&dir).ok();
    }
}
