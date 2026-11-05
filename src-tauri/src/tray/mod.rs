use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

use crate::{load_desktop_state, save_desktop_state};

pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let open_main = MenuItemBuilder::with_id("open_main", "打开Memo").build(app)?;
    let separator = tauri::menu::PredefinedMenuItem::separator(app)?;
    let quit = MenuItemBuilder::with_id("quit", "退出Memo").build(app)?;

    let menu = MenuBuilder::new(app)
        .item(&open_main)
        .item(&separator)
        .item(&quit)
        .build()?;

    let icon_bytes = include_bytes!("../../icons/icon.png");
    let icon = image::load_from_memory_with_format(icon_bytes, image::ImageFormat::Png)
        .map(|img| {
            let rgba = img.to_rgba8();
            let (width, height) = rgba.dimensions();
            Image::new_owned(rgba.into_raw(), width, height)
        })
        .unwrap_or_else(|_| {
            let raw = [13u8, 148, 136, 255];
            let mut pixels = Vec::with_capacity(256 * 256 * 4);
            for _ in 0..(256 * 256) {
                pixels.extend_from_slice(&raw);
            }
            Image::new_owned(pixels, 256, 256)
        });

    let _tray = TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .on_menu_event(move |app, event| match event.id().as_ref() {
            "open_main" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                // 退出前记录编辑窗口可见状态，供开机自启恢复
                let mut state = load_desktop_state(app);
                if let Some(editor) = app.get_webview_window("editor") {
                    state.editor_visible = editor.is_visible().unwrap_or(false);
                }
                save_desktop_state(app, &state);
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}
