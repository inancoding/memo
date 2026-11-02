use serde::{Deserialize, Serialize};
use tauri::{image::Image, Manager};
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::notes::create_note,
            commands::notes::get_notes,
            commands::notes::update_note,
            commands::notes::delete_note,
            commands::notes::restore_note,
            commands::notes::empty_trash,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();

            db::init_db(&app_handle)?;
            tray::setup_tray(&app_handle)?;

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

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
            if let Some(window) = app_handle.get_webview_window("main") {
                save_window_state(app_handle, &window);
                let _ = window.hide();
            }
        }
        _ => {}
    });
}
