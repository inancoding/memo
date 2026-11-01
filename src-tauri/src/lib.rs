use tauri::{image::Image, Manager};

mod commands;
mod db;
mod tray;

fn load_window_icon() -> Option<Image<'static>> {
    let bytes = include_bytes!("../icons/icon.png");
    let img = image::load_from_memory_with_format(bytes, image::ImageFormat::Png).ok()?;
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    let pixels = rgba.into_raw();
    Some(Image::new_owned(pixels, w, h))
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

                    if let Some(monitor) = window.current_monitor().ok().flatten() {
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
                let _ = window.hide();
            }
        }
        _ => {}
    });
}
