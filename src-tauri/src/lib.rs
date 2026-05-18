pub mod desktop;
pub mod services;

use services::notes::{default_store, AppConfig, AppError, Note, NoteMetadata, SaveNoteRequest};
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};

#[tauri::command]
fn list_system_fonts() -> Result<Vec<String>, AppError> {
    desktop::list_system_fonts()
}

#[tauri::command]
fn app_name() -> &'static str {
    "拾芥"
}

#[tauri::command]
fn notes_list() -> Result<Vec<NoteMetadata>, AppError> {
    default_store()?.list_notes()
}

#[tauri::command]
fn notes_get(id: String) -> Result<Note, AppError> {
    default_store()?.read_note(&id)
}

#[tauri::command]
fn notes_create(app: AppHandle, request: SaveNoteRequest) -> Result<Note, AppError> {
    let note = default_store()?.create_note(request)?;
    let _ = app.emit("notes-changed", ());
    Ok(note)
}

#[tauri::command]
fn notes_update(app: AppHandle, id: String, request: SaveNoteRequest) -> Result<Note, AppError> {
    let note = default_store()?.update_note(&id, request)?;
    let _ = app.emit("notes-changed", ());
    Ok(note)
}

#[tauri::command]
fn notes_delete(app: AppHandle, id: String) -> Result<(), AppError> {
    default_store()?.delete_note(&id)?;
    let _ = app.emit("notes-changed", ());
    Ok(())
}

#[tauri::command]
fn notes_import_markdown(
    app: AppHandle,
    path: String,
    category: Option<String>,
) -> Result<Note, AppError> {
    let note = default_store()?
        .import_markdown_file(&PathBuf::from(path), &category.unwrap_or_default())?;
    let _ = app.emit("notes-changed", ());
    Ok(note)
}

#[tauri::command]
fn notes_export_markdown(id: String, path: String) -> Result<(), AppError> {
    default_store()?.export_markdown_file(&id, &PathBuf::from(path))
}

#[tauri::command]
fn read_external_file(path: String) -> Result<String, AppError> {
    std::fs::read_to_string(&path).map_err(|e| AppError {
        code: "io".into(),
        message: e.to_string(),
    })
}

#[tauri::command]
fn get_file_modified_time(path: String) -> Result<f64, AppError> {
    let metadata = std::fs::metadata(&path).map_err(|e| AppError {
        code: "io".into(),
        message: e.to_string(),
    })?;
    let modified = metadata.modified().map_err(|e| AppError {
        code: "io".into(),
        message: e.to_string(),
    })?;
    let duration = modified
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    Ok(duration.as_secs_f64() * 1000.0)
}

#[tauri::command]
fn save_external_file(path: String, content: String) -> Result<(), AppError> {
    if let Some(parent) = PathBuf::from(&path).parent() {
        std::fs::create_dir_all(parent).map_err(|e| AppError {
            code: "io".into(),
            message: e.to_string(),
        })?;
    }
    std::fs::write(&path, content).map_err(|e| AppError {
        code: "io".into(),
        message: e.to_string(),
    })
}

#[tauri::command]
fn categories_list() -> Result<Vec<String>, AppError> {
    default_store()?.list_categories()
}

#[tauri::command]
fn categories_create(app: AppHandle, name: String) -> Result<(), AppError> {
    default_store()?.create_category(&name)?;
    let _ = app.emit("notes-changed", ());
    Ok(())
}

#[tauri::command]
fn categories_rename(app: AppHandle, old_name: String, new_name: String) -> Result<(), AppError> {
    default_store()?.rename_category(&old_name, &new_name)?;
    let _ = app.emit("notes-changed", ());
    Ok(())
}

#[tauri::command]
fn categories_delete(app: AppHandle, name: String) -> Result<(), AppError> {
    default_store()?.delete_category(&name)?;
    let _ = app.emit("notes-changed", ());
    Ok(())
}

#[tauri::command]
fn tags_list() -> Result<Vec<String>, AppError> {
    default_store()?.list_tags()
}

#[tauri::command]
fn save_asset_bytes(name: String, data: Vec<u8>) -> Result<String, AppError> {
    default_store()?.save_asset_bytes(&name, &data)
}

#[tauri::command]
fn copy_asset_file(source: String) -> Result<String, AppError> {
    default_store()?.copy_asset_file(&std::path::PathBuf::from(source))
}

#[tauri::command]
fn asset_base64(relative_path: String) -> Result<String, AppError> {
    default_store()?.asset_base64(&relative_path)
}

#[tauri::command]
fn open_in_explorer(id: String) -> Result<(), AppError> {
    let path = default_store()?.get_note_path(&id)?;
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg("/select,")
            .arg(&path)
            .spawn()
            .map_err(|e| AppError {
                code: "openExplorer".into(),
                message: format!("打开资源管理器失败: {}", e),
            })?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("-R")
            .arg(&path)
            .spawn()
            .map_err(|e| AppError {
                code: "openExplorer".into(),
                message: format!("打开访达失败: {}", e),
            })?;
    }
    #[cfg(target_os = "linux")]
    {
        let parent = path.parent().unwrap_or_else(|| std::path::Path::new("/"));
        std::process::Command::new("xdg-open")
            .arg(parent)
            .spawn()
            .map_err(|e| AppError {
                code: "openExplorer".into(),
                message: format!("打开文件管理器失败: {}", e),
            })?;
    }
    Ok(())
}

#[tauri::command]
fn notes_move_category(
    app: AppHandle,
    id: String,
    category: String,
) -> Result<NoteMetadata, AppError> {
    let result = default_store()?.move_note_to_category(&id, &category)?;
    let _ = app.emit("notes-changed", ());
    Ok(result)
}

#[tauri::command]
fn config_get() -> Result<AppConfig, AppError> {
    default_store()?.load_config()
}

#[tauri::command]
fn config_save(app: AppHandle, config: AppConfig) -> Result<AppConfig, AppError> {
    let store = default_store()?;
    let previous = store.load_config()?;
    desktop::apply_runtime_config(&app, &previous, &config).map_err(|error| AppError {
        code: "desktopConfig".into(),
        message: error.to_string(),
    })?;
    store.save_config(config.clone())?;
    let _ = app.emit("config-changed", &config);
    Ok(config)
}

#[tauri::command]
async fn open_notepad_window(
    app: AppHandle,
    note_id: Option<String>,
    bounds: Option<desktop::WindowBounds>,
) -> Result<String, AppError> {
    desktop::open_notepad_window(app, note_id, bounds).await
}

#[tauri::command]
async fn recycle_notepad_window(app: AppHandle, label: String) -> Result<(), AppError> {
    desktop::recycle_notepad_window(&app, &label)
}

#[tauri::command]
async fn open_tile_window(
    app: AppHandle,
    note_id: String,
    bounds: Option<desktop::WindowBounds>,
) -> Result<String, AppError> {
    desktop::open_tile_window(app, note_id, bounds).await
}

#[tauri::command]
async fn open_note_in_editor(app: AppHandle, note_id: String) -> Result<(), AppError> {
    desktop::show_main_window(&app)?;
    let _ = app.emit("open-note", &note_id);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            if let Some(file_path) = desktop::extract_file_arg(&args) {
                let _ = app.emit("open-external-file", file_path);
            }
            let _ = desktop::show_main_window(app);
        }))
        .setup(|app| {
            desktop::setup_desktop(app)?;
            Ok(())
        })
        .on_window_event(desktop::handle_window_event)
        .invoke_handler(tauri::generate_handler![
            app_name,
            list_system_fonts,
            notes_list,
            notes_get,
            notes_create,
            notes_update,
            notes_delete,
            notes_import_markdown,
            notes_export_markdown,
            notes_move_category,
            read_external_file,
            save_external_file,
            get_file_modified_time,
            categories_list,
            categories_create,
            categories_rename,
            categories_delete,
            tags_list,
            save_asset_bytes,
            copy_asset_file,
            asset_base64,
            open_in_explorer,
            config_get,
            config_save,
            open_notepad_window,
            recycle_notepad_window,
            open_tile_window,
            open_note_in_editor
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
