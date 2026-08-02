use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir};

const IMAGE_EXTENSIONS: [&str; 6] = ["jpg", "jpeg", "png", "gif", "bmp", "webp"];

/// 指定されたファイル/ディレクトリを基準に、配下(サブディレクトリ含む)の
/// 対応画像ファイルを再帰的に検索して絶対パス一覧を返す。
/// 隠しファイル・隠しディレクトリは除外する。
#[tauri::command]
pub fn scan_images(path: String) -> Result<Vec<String>, String> {
    let input = Path::new(&path);
    if !input.exists() {
        return Err(format!("パスが存在しません: {}", path));
    }

    let base_dir: PathBuf = if input.is_dir() {
        input.to_path_buf()
    } else {
        input
            .parent()
            .map(|p| p.to_path_buf())
            .ok_or_else(|| format!("親ディレクトリを取得できません: {}", path))?
    };

    let mut files: Vec<String> = WalkDir::new(&base_dir)
        .into_iter()
        .filter_entry(|entry| !is_hidden(entry))
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| is_image(entry.path()))
        .map(|entry| entry.path().to_string_lossy().to_string())
        .collect();

    files.sort();
    Ok(files)
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

fn is_image(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn finds_images_recursively_and_excludes_hidden_and_non_images() {
        let dir = std::env::temp_dir().join(format!("imv-scan-test-{}", std::process::id()));
        let sub = dir.join("sub");
        fs::create_dir_all(&sub).unwrap();

        fs::write(dir.join("a.png"), b"x").unwrap();
        fs::write(dir.join("b.PNG"), b"x").unwrap();
        fs::write(sub.join("c.jpg"), b"x").unwrap();
        fs::write(dir.join("notes.txt"), b"x").unwrap();
        fs::write(dir.join(".hidden.png"), b"x").unwrap();

        let result = scan_images(dir.to_string_lossy().to_string()).unwrap();

        assert_eq!(result.len(), 3);
        assert!(result.iter().any(|p| p.ends_with("a.png")));
        assert!(result.iter().any(|p| p.ends_with("b.PNG")));
        assert!(result.iter().any(|p| p.ends_with("c.jpg")));
        assert!(!result.iter().any(|p| p.contains("notes.txt")));
        assert!(!result.iter().any(|p| p.contains(".hidden.png")));

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn resolves_base_dir_from_a_file_path() {
        let dir = std::env::temp_dir().join(format!("imv-scan-test-file-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let file = dir.join("a.png");
        fs::write(&file, b"x").unwrap();
        fs::write(dir.join("b.png"), b"x").unwrap();

        let result = scan_images(file.to_string_lossy().to_string()).unwrap();
        assert_eq!(result.len(), 2);

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn errors_on_missing_path() {
        let result = scan_images("C:/does/not/exist-imv".to_string());
        assert!(result.is_err());
    }
}
