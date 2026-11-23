mod scan;
mod settings;
mod trash;

use scan::scan_images;
use settings::{add_recent_path, get_recent_paths, get_trash_dir, set_trash_dir};
use trash::{move_to_trash, restore_from_trash};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            scan_images,
            move_to_trash,
            restore_from_trash,
            get_trash_dir,
            set_trash_dir,
            get_recent_paths,
            add_recent_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
