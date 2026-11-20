use tauri::{
    image::Image,
    menu::{CheckMenuItemBuilder, Menu, MenuBuilder, MenuItemBuilder, MenuEvent, PredefinedMenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager, Wry,
};
use tauri_plugin_autostart::ManagerExt;

#[cfg(not(target_os = "linux"))]
use tauri::tray::{MouseButton, MouseButtonState, TrayIconEvent};

use crate::{create_and_open_note, snapshot_open_notes, AppLang, Lang};

// 托盘图标只建一个，重建菜单时按这个 id 找回它
pub const TRAY_ID: &str = "main";

// 新建便签 / 打开Memo / ─ / 开机自启动 / ─ / 退出Memo
fn labels(lang: Lang) -> [&'static str; 4] {
    match lang {
        Lang::Zh => ["新建便签", "打开Memo", "开机自启动", "退出Memo"],
        Lang::En => ["New Note", "Open Memo", "Launch at Startup", "Quit Memo"],
    }
}

// 自启勾选态每次都以插件（注册表 / 启动项文件）为准，菜单与设置面板因此不会漂移
fn build_menu(app: &AppHandle, lang: Lang) -> tauri::Result<Menu<Wry>> {
    let [new_note, open_main, autostart, quit] = labels(lang);

    let new_note_item = MenuItemBuilder::with_id("new_note", new_note).build(app)?;
    let open_main_item = MenuItemBuilder::with_id("open_main", open_main).build(app)?;
    let autostart_item = CheckMenuItemBuilder::with_id("autostart", autostart)
        .checked(app.autolaunch().is_enabled().unwrap_or(false))
        .build(app)?;
    let quit_item = MenuItemBuilder::with_id("quit", quit).build(app)?;

    let sep1 = PredefinedMenuItem::separator(app)?;
    let sep2 = PredefinedMenuItem::separator(app)?;

    MenuBuilder::new(app)
        .item(&new_note_item)
        .item(&open_main_item)
        .item(&sep1)
        .item(&autostart_item)
        .item(&sep2)
        .item(&quit_item)
        .build()
}

/// 语言或自启状态变化后重建托盘菜单。
/// 菜单项的构建与替换都要回到主线程完成，而调用点可能正是主线程上的菜单回调，
/// 所以统一另起工作线程，避免在主线程里等待主线程任务。
pub fn refresh_menu(app: &AppHandle) {
    let app = app.clone();
    std::thread::spawn(move || {
        let lang = app.state::<AppLang>().get();
        match build_menu(&app, lang) {
            Ok(menu) => {
                if let Some(tray) = app.tray_by_id(TRAY_ID) {
                    let _ = tray.set_menu(Some(menu));
                }
            }
            Err(e) => eprintln!("rebuild tray menu failed: {}", e),
        }
    });
}

fn handle_menu_event(app: &AppHandle, event: MenuEvent) {
    match event.id().as_ref() {
        "new_note" => create_and_open_note(app),
        "open_main" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        "autostart" => {
            let autolaunch = app.autolaunch();
            let enabled = !autolaunch.is_enabled().unwrap_or(false);
            let result = if enabled {
                autolaunch.enable()
            } else {
                autolaunch.disable()
            };
            match result {
                Ok(()) => {
                    refresh_menu(app);
                    // 通知「设置 - 通用」里的开关一起翻转
                    let _ = app.emit("autostart-changed", enabled);
                }
                Err(e) => eprintln!("tray autostart toggle failed: {}", e),
            }
        }
        "quit" => {
            // 退出前再快照一次（正常情况下开关窗口时已实时落盘，这里是兜底）
            snapshot_open_notes(app);
            app.exit(0);
        }
        _ => {}
    }
}

pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let lang = app.state::<AppLang>().get();
    let menu = build_menu(app, lang)?;

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

    let builder = TrayIconBuilder::with_id(TRAY_ID)
        .icon(icon)
        .menu(&menu)
        .on_menu_event(|app, event| handle_menu_event(app, event));

    // Windows/macOS：左键点击直接打开主窗口。
    // Linux：appindicator 左键点击原生弹出菜单（自定义左键事件通常不触发，
    // 且 show_menu_on_left_click 在 Linux 不受支持），因此统一走菜单里的“打开Memo”。
    #[cfg(not(target_os = "linux"))]
    let builder = builder.on_tray_icon_event(|tray, event| {
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
    });

    let _tray = builder.build(app)?;

    Ok(())
}
