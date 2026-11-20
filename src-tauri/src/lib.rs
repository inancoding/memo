use serde::{Deserialize, Serialize};
use tauri::{image::Image, Manager, Emitter};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use std::fs;
use std::sync::Mutex;

mod commands;
mod db;
mod tray;

/// 界面语言。托盘菜单与欢迎便签的文案在 Rust 侧生成，需要知道当前语言。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Zh,
    En,
}

impl Default for Lang {
    // 启动时先按中文建好菜单与欢迎便签，前端挂载后立即用真实语言调用 set_language 覆盖
    fn default() -> Self {
        Lang::Zh
    }
}

impl Lang {
    pub fn from_code(code: &str) -> Self {
        if code == "en" {
            Lang::En
        } else {
            Lang::Zh
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            Lang::Zh => "zh",
            Lang::En => "en",
        }
    }
}

// 启动时先按中文建好菜单与欢迎便签，前端挂载后立即用真实语言调用 set_language 覆盖
#[derive(Default)]
pub struct AppLang(Mutex<Lang>);

impl AppLang {
    pub fn get(&self) -> Lang {
        *self.0.lock().unwrap()
    }

    pub fn set(&self, lang: Lang) {
        *self.0.lock().unwrap() = lang;
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct WindowState {
    width: f64,
    height: f64,
    x: i32,
    y: i32,
}

// 桌面状态：记录退出前未关闭的便签窗口，用于开机自启时恢复
#[derive(Debug, Serialize, Deserialize, Default)]
struct DesktopState {
    open_notes: Option<Vec<String>>,
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

fn get_desktop_state_path(app: &impl Manager<tauri::Wry>) -> std::path::PathBuf {
    app.path()
        .app_data_dir()
        .expect("failed to get app data dir")
        .join("desktop_state.json")
}

fn load_desktop_state(app: &impl Manager<tauri::Wry>) -> DesktopState {
    let path = get_desktop_state_path(app);
    fs::read_to_string(path)
        .ok()
        .and_then(|c| serde_json::from_str(&c).ok())
        .unwrap_or_default()
}

fn save_desktop_state(app: &impl Manager<tauri::Wry>, state: &DesktopState) {
    let path = get_desktop_state_path(app);
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, serde_json::to_string_pretty(state).unwrap_or_default());
}

// 快照当前打开的便签编辑窗（label 形如 editor-<id>）并落盘。
// 每次窗口创建 / 销毁都调用它，使状态文件始终等于「此刻真正开着的便签」，
// 这样关机 / 重启 / 断电 / 强杀都不需要先走托盘退出也能在下次开机恢复。
// AppHandle 与 Window 都实现 Manager，故删除便签（从 Window 事件）与新建（从命令）共用此函数。
pub(crate) fn snapshot_open_notes(app: &impl Manager<tauri::Wry>) {
    let open_notes: Vec<String> = app
        .webview_windows()
        .iter()
        .filter_map(|(label, _)| label.strip_prefix("editor-").map(|id| id.to_string()))
        .collect();
    let mut state = load_desktop_state(app);
    state.open_notes = Some(open_notes);
    save_desktop_state(app, &state);
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

// 便签已彻底删除，若它正开在自己的窗口里，关掉窗口
pub(crate) fn forget_desktop_note(app_handle: &tauri::AppHandle, note_id: &str) {
    if let Some(win) = app_handle.get_webview_window(&format!("editor-{note_id}")) {
        let _ = win.destroy();
    }
}

// 新便签默认色：Linux 的 WebKitGTK 合成不了毛玻璃，玻璃色已从颜色栏移除，取颜色栏第一个实色
fn default_note_color() -> &'static str {
    if cfg!(target_os = "linux") {
        "yellow"
    } else {
        "glass"
    }
}

// 新建便签并打开编辑窗口：全局快捷键与托盘菜单共用
// 独立窗口模型下只负责建库 + 通知主窗口，由前端用 WebviewWindow 创建专属窗口
pub(crate) fn create_and_open_note(app: &tauri::AppHandle) {
    let note = {
        let db = app.state::<db::Database>();
        match db.create_note(default_note_color(), "") {
            Ok(note) => note,
            Err(e) => {
                eprintln!("create_note failed: {}", e);
                return;
            }
        }
    };
    let _ = app.emit("notes-changed", ());
    let _ = app.emit("request-new-note", &note.id);
}

#[tauri::command]
fn save_window_state_cmd(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        save_window_state(&app_handle, &window);
    }
    Ok(())
}

// 前端新建便签窗口（tauri://created 之后）调用，把「当前开着的便签」实时写进状态文件。
#[tauri::command]
fn record_open_notes_cmd(app_handle: tauri::AppHandle) {
    snapshot_open_notes(&app_handle);
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
    // 让托盘菜单里的勾选项跟着变
    tray::refresh_menu(&app_handle);
    Ok(())
}

// 前端（设置 - 通用 / 启动挂载）把语言推给 Rust：托盘菜单在这里重建，欢迎便签在这里换语言。
// 每个用到文案的组件都会调用，语言没变时直接返回，避免反复重建菜单。
#[tauri::command]
fn set_language(app_handle: tauri::AppHandle, lang: String) -> Result<(), String> {
    let next = Lang::from_code(&lang);
    let current = app_handle.state::<AppLang>().get();
    if current == next {
        return Ok(());
    }
    app_handle.state::<AppLang>().set(next);

    if let Some(db) = app_handle.try_state::<db::Database>() {
        let changed = db
            .localize_welcome_note(next.code())
            .map_err(|e| e.to_string())?;
        // 欢迎便签换了语言，让主窗口重新读一次列表
        if changed > 0 {
            let _ = app_handle.emit("notes-changed", ());
        }
    }
    tray::refresh_menu(&app_handle);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        // 必须最先注册：再次启动时退出新进程并唤起已运行的实例
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }))
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
            commands::notes::write_recovery_content,
            commands::notes::read_recovery_content,
            commands::notes::clear_recovery_content,
            save_window_state_cmd,
            record_open_notes_cmd,
            get_autostart,
            set_autostart,
            set_language,
        ])
        // 便签窗口一旦被销毁（用户关闭、删除便签、程序化 destroy），立即刷新状态文件。
        // 全局钩子对所有窗口生效，含 setup 之后由前端 WebviewWindow 新建的窗口。
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                if window.label().starts_with("editor-") {
                    snapshot_open_notes(window);
                }
            }
        })
        .setup(|app| {
            let app_handle = app.handle().clone();

            db::init_db(&app_handle)?;
            app_handle.manage(AppLang::default());

            // 首次运行时植入一条欢迎便签（用户可自行删除，删除后不再重建）
            let welcome_marker = app_handle
                .path()
                .app_data_dir()
                .expect("failed to get app data dir")
                .join("welcome_note_seeded");
            let lang = app_handle.state::<AppLang>().get();
            if !welcome_marker.exists() {
                if let Some(db) = app_handle.try_state::<db::Database>() {
                    if let Err(e) = db.seed_welcome_note(lang.code(), default_note_color()) {
                        eprintln!("seed welcome note failed: {}", e);
                    }
                }
                let _ = fs::write(&welcome_marker, "1");
            }

            // 每次启动都把"未被用户改写"的欢迎便签刷新为当前语言的最新文案
            if let Some(db) = app_handle.try_state::<db::Database>() {
                if let Err(e) = db.localize_welcome_note(lang.code()) {
                    eprintln!("localize welcome note failed: {}", e);
                }
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
                        create_and_open_note(app);
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

            // 启动窗口策略：无论哪种启动方式都恢复上次挂在桌面的便签窗口；
            // 额外地，手动启动显示主窗口，开机自启只还原便签、不弹主窗口。
            let state = load_desktop_state(&app_handle);
            let note_ids = state.open_notes.unwrap_or_default();
            if !note_ids.is_empty() {
                let handle = app_handle.clone();
                std::thread::spawn(move || {
                    // 等前端挂载并注册监听器
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                    let _ = handle.emit("request-restore-notes", &note_ids);
                });
            }
            if !is_autostart_launch() {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|_app_handle, event| {
        // 便签窗口关掉就销毁，最后一个销毁会让运行时发出 code 为 None 的自然退出，
        // 但应用要常驻托盘，只放行 app.exit()（带 code）这条主动退出的路。
        if let tauri::RunEvent::ExitRequested { api, code, .. } = event {
            if code.is_none() {
                api.prevent_exit();
            }
        }
    });
}
