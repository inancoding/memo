use serde::{Deserialize, Serialize};
use tauri::{image::Image, Manager, Emitter};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use std::fs;

mod commands;
mod db;
mod tray;

#[derive(Debug, Serialize, Deserialize)]
struct WindowState {
    width: f64,
    height: f64,
    x: i32,
    y: i32,
}

// 桌面状态：记录最后打开的便签以及编辑窗口是否可见，用于开机自启时恢复
#[derive(Debug, Serialize, Deserialize, Default)]
struct DesktopState {
    note_id: Option<String>,
    editor_visible: bool,
}

fn is_autostart_launch() -> bool {
    std::env::args().any(|a| a == "--autostart")
}

fn load_window_icon() -> Option<Image<'static>> {
    let bytes = include_bytes!("../icons/icon.png");
    let img = image::load_from_memory_with_format(bytes, image::ImageFormat::Png).ok()?;
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    let pixels = rgba.into_raw();
    Some(Image::new_owned(pixels, w, h))
}

fn get_window_state_path(app_handle: &tauri::AppHandle) -> std::path::PathBuf {
    app_handle
        .path()
        .app_data_dir()
        .expect("failed to get app data dir")
        .join("window_state.json")
}

fn get_desktop_state_path(app_handle: &tauri::AppHandle) -> std::path::PathBuf {
    app_handle
        .path()
        .app_data_dir()
        .expect("failed to get app data dir")
        .join("desktop_state.json")
}

pub(crate) fn load_desktop_state(app_handle: &tauri::AppHandle) -> DesktopState {
    let path = get_desktop_state_path(app_handle);
    fs::read_to_string(path)
        .ok()
        .and_then(|c| serde_json::from_str(&c).ok())
        .unwrap_or_default()
}

pub(crate) fn save_desktop_state(app_handle: &tauri::AppHandle, state: &DesktopState) {
    let path = get_desktop_state_path(app_handle);
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, serde_json::to_string_pretty(state).unwrap_or_default());
}

fn save_window_state(app_handle: &tauri::AppHandle, window: &tauri::WebviewWindow) {
    let Ok(outer_size) = window.outer_size() else { return };
    let Ok(position) = window.outer_position() else { return };

    let state = WindowState {
        width: outer_size.width as f64,
        height: outer_size.height as f64,
        x: position.x,
        y: position.y,
    };

    let path = get_window_state_path(app_handle);
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, serde_json::to_string_pretty(&state).unwrap_or_default());
}

fn load_window_state(app_handle: &tauri::AppHandle) -> Option<WindowState> {
    let path = get_window_state_path(app_handle);
    let content = fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

// 显示编辑窗口并加载指定便签（预定义窗口，延迟发送事件等待前端监听就绪）
fn show_editor_with_note(app_handle: &tauri::AppHandle, note_id: &str) {
    if let Some(editor_window) = app_handle.get_webview_window("editor") {
        let _ = editor_window.show();
        let _ = editor_window.set_focus();
        let window = editor_window.clone();
        let note_id_owned = note_id.to_string();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(500));
            if let Err(e) = window.emit("open-note", &note_id_owned) {
                println!("Failed to emit open-note: {}", e);
            }
        });
    }
}

#[tauri::command]
fn save_window_state_cmd(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        save_window_state(&app_handle, &window);
    }
    Ok(())
}

#[tauri::command]
fn get_autostart(app_handle: tauri::AppHandle) -> bool {
    app_handle.autolaunch().is_enabled().unwrap_or(false)
}

#[tauri::command]
fn set_autostart(app_handle: tauri::AppHandle, enabled: bool) -> Result<(), String> {
    let autolaunch = app_handle.autolaunch();
    if enabled {
        autolaunch.enable().map_err(|e| e.to_string())?;
    } else {
        autolaunch.disable().map_err(|e| e.to_string())?;
    }
    Ok(())
}

// 前端编辑窗口隐藏时同步桌面状态
#[tauri::command]
fn set_editor_visible(app_handle: tauri::AppHandle, visible: bool) -> Result<(), String> {
    let mut state = load_desktop_state(&app_handle);
    state.editor_visible = visible;
    save_desktop_state(&app_handle, &state);
    Ok(())
}

#[tauri::command]
fn open_editor(app_handle: tauri::AppHandle, note_id: String) -> Result<(), String> {
    println!("open_editor called with note_id: {}", note_id);

    if app_handle.get_webview_window("editor").is_none() {
        return Err("Editor window not found".to_string());
    }

    // 记录最后打开的便签，供开机自启恢复
    let mut state = load_desktop_state(&app_handle);
    state.note_id = Some(note_id.clone());
    state.editor_visible = true;
    save_desktop_state(&app_handle, &state);

    show_editor_with_note(&app_handle, &note_id);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--autostart"]),
        ))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::notes::create_note,
            commands::notes::get_notes,
            commands::notes::update_note,
            commands::notes::delete_note,
            commands::notes::restore_note,
            commands::notes::empty_trash,
            save_window_state_cmd,
            open_editor,
            get_autostart,
            set_autostart,
            set_editor_visible,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();

            db::init_db(&app_handle)?;

            // 首次运行时植入一条欢迎便签（用户可自行删除，删除后不再重建）
            let welcome_marker = app_handle
                .path()
                .app_data_dir()
                .expect("failed to get app data dir")
                .join("welcome_note_seeded");
            if !welcome_marker.exists() {
                if let Some(db) = app_handle.try_state::<db::Database>() {
                    if let Err(e) = db.seed_welcome_note() {
                        println!("seed welcome note failed: {}", e);
                    }
                }
                let _ = fs::write(&welcome_marker, "1");
            }

            tray::setup_tray(&app_handle)?;

            // 全局快捷键
            {
                let gs = app_handle.global_shortcut();

                // Ctrl+Shift+N：新建便签并打开编辑窗口
                let _ = gs.on_shortcut(
                    Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyN),
                    |app, _sc, event| {
                        if event.state != ShortcutState::Pressed {
                            return;
                        }
                        let db = app.state::<db::Database>();
                        match db.create_note("glass", "") {
                            Ok(note) => {
                                let _ = app.emit("notes-changed", ());
                                show_editor_with_note(app, &note.id);
                            }
                            Err(e) => println!("shortcut create_note failed: {}", e),
                        }
                    },
                );

                // Ctrl+Shift+M：显示/隐藏主窗口
                let _ = gs.on_shortcut(
                    Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyM),
                    |app, _sc, event| {
                        if event.state != ShortcutState::Pressed {
                            return;
                        }
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    },
                );
            }

            // 首次运行时默认开启开机自启（之后尊重用户设置）
            let marker = app_handle
                .path()
                .app_data_dir()
                .expect("failed to get app data dir")
                .join("autostart_initialized");
            if !marker.exists() {
                let _ = app_handle.autolaunch().enable();
                let _ = fs::write(&marker, "1");
            } else if app_handle.autolaunch().is_enabled().unwrap_or(false) {
                // 已开启则刷新注册路径，适配 dev/prod 路径变化与应用更新
                let _ = app_handle.autolaunch().enable();
            }

            if let Some(icon) = load_window_icon() {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.set_icon(icon);

                    // Restore window state or use default
                    if let Some(state) = load_window_state(&app_handle) {
                        let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
                            width: state.width as u32,
                            height: state.height as u32,
                        }));
                        let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                            x: state.x,
                            y: state.y,
                        }));
                    } else if let Some(monitor) = window.current_monitor().ok().flatten() {
                        let scale = monitor.scale_factor();
                        let screen_width = monitor.size().width as f64 / scale;
                        let window_width = (screen_width * 0.25).clamp(360.0, 480.0) as u32;
                        let window_height = 600u32;
                        let _ = window.set_size(tauri::Size::Logical(tauri::LogicalSize {
                            width: window_width as f64,
                            height: window_height as f64,
                        }));
                    }
                }
            }

            // 启动窗口策略：开机自启恢复上次便签窗口（不显示主窗口）；手动启动显示主窗口
            if is_autostart_launch() {
                let state = load_desktop_state(&app_handle);
                if state.editor_visible {
                    if let Some(note_id) = state.note_id {
                        show_editor_with_note(&app_handle, &note_id);
                    }
                }
            } else if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|_app_handle, _event| {});
}
