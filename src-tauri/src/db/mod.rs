use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteRow {
    pub id: String,
    pub title: String,
    pub content: String,
    pub color: String,
    pub font_size: i32,
    pub position_x: i32,
    pub position_y: i32,
    pub width: i32,
    pub height: i32,
    pub is_pinned: bool,
    pub sticky_edge: Option<String>,
    pub is_hidden: bool,
    pub created_at: String,
    pub updated_at: String,
}

const NOTE_COLUMNS: &str =
    "id, title, content, color, font_size, position_x, position_y, width, height, is_pinned, sticky_edge, is_hidden, created_at, updated_at";

fn map_note_row(row: &rusqlite::Row<'_>) -> Result<NoteRow> {
    Ok(NoteRow {
        id: row.get(0)?,
        title: row.get(1)?,
        content: row.get(2)?,
        color: row.get(3)?,
        font_size: row.get(4)?,
        position_x: row.get(5)?,
        position_y: row.get(6)?,
        width: row.get(7)?,
        height: row.get(8)?,
        is_pinned: row.get::<_, i32>(9)? != 0,
        sticky_edge: row.get(10)?,
        is_hidden: row.get::<_, i32>(11)? != 0,
        created_at: row.get(12)?,
        updated_at: row.get(13)?,
    })
}

pub struct Database {
    conn: Mutex<Connection>,
}

// 首次安装时植入的欢迎便签（富文本 HTML，与编辑器格式一致）。
// 语言由前端启动时通过 set_language 告知：欢迎文案属于内置文案，可以本地化；
// 用户改写过的便签一律不动（判定条件见 localize_welcome_note）。
pub const WELCOME_TITLE_ZH: &str = "欢迎使用 Memo";
pub const WELCOME_TITLE_EN: &str = "Welcome to Memo";
const WELCOME_CONTENT_ZH: &str = r#"<p><strong>欢迎使用 Memo</strong></p><p>Memo 是一款轻量桌面便签应用，帮你随手记录灵感与待办事项。</p><p><strong>主要功能</strong></p><ul><li>富文本编辑：加粗、斜体、列表、复选框，支持拖拽插入图片并缩放</li><li>自动识别网址、邮箱、手机号并彩色高亮，点击即可打开</li><li>多色便签与毛玻璃主题，支持浅色 / 深色模式</li><li>便签窗口可置顶固定，自动记忆位置与大小</li><li>主窗口搜索：按标题与正文纯文本即时过滤</li><li>批量删除：卡片左上角勾选后可一次删除多条</li><li>开机自启动，并恢复上次打开的便签</li><li>系统托盘：新建便签、打开主窗口、开关开机自启、退出应用</li><li>界面语言支持简体中文 / English，可在「设置 - 通用」切换</li><li>边缘自动隐藏：在设置中开启后，便签贴屏幕左 / 右 / 上边缘时会收起为窄条，鼠标悬停窄条即可唤回，移开后自动收回</li></ul><p><strong>使用方法</strong></p><ul><li>点击右下角 + 按钮新建便签</li><li>点击任意便签打开独立编辑窗口</li><li>鼠标悬停便签卡片，可置顶或删除</li><li>关闭编辑窗口即隐藏，内容不会丢失</li></ul><p><strong>快捷键</strong></p><ul><li>Ctrl+Shift+N：新建便签</li><li>Ctrl+Shift+M：显示 / 隐藏主窗口</li><li>Ctrl+S：立即保存</li><li>Ctrl+W：关闭当前编辑窗</li><li>Esc：收起搜索</li></ul><p>这条便签可以随时删除，祝你使用愉快！</p>"#;
const WELCOME_CONTENT_EN: &str = r#"<p><strong>Welcome to Memo</strong></p><p>Memo is a lightweight desktop sticky-note app for capturing ideas and to-dos as they come.</p><p><strong>What it does</strong></p><ul><li>Rich text: bold, italic, lists, checkboxes; drop or pick images and resize them</li><li>URLs, emails and phone numbers are highlighted automatically and open on click</li><li>Several note colors plus a frosted-glass theme, with light / dark mode</li><li>Notes can stay on top and remember their position and size</li><li>Search in the main window filters titles and plain text as you type</li><li>Batch delete: tick the checkbox on cards to remove several at once</li><li>Launch at startup and restore the note you had open</li><li>System tray: new note, open main window, toggle startup, quit</li><li>Interface languages: 简体中文 / English, switchable in Settings - General</li><li>Edge auto-hide: once enabled in Settings, a note docked to the left / right / top screen edge collapses into a thin bar; hover the bar to bring it back, and it retracts when you leave</li></ul><p><strong>How to use</strong></p><ul><li>Click the + button in the corner to create a note</li><li>Click any note to open its own editor window</li><li>Hover a card to pin or delete it</li><li>Closing the editor only hides it; nothing is lost</li></ul><p><strong>Shortcuts</strong></p><ul><li>Ctrl+Shift+N: new note</li><li>Ctrl+Shift+M: show / hide the main window</li><li>Ctrl+S: save now</li><li>Ctrl+W: close the current note window</li><li>Esc: close search</li></ul><p>Delete this note whenever you like. Enjoy!</p>"#;

// 上一版欢迎内容：仅用于把"未被用户改写"的旧欢迎便签原地刷新为最新版，
// 精确匹配可避免覆盖用户自行编辑过的便签。升级欢迎内容时，把旧的中文版挪到这里。
const WELCOME_CONTENT_PREV: &str = r#"<p><strong>欢迎使用 Memo</strong></p><p>Memo 是一款轻量桌面便签应用，帮你随手记录灵感与待办事项。</p><p><strong>主要功能</strong></p><ul><li>富文本编辑：加粗、斜体、列表、复选框，支持拖拽插入图片并缩放</li><li>自动识别网址、邮箱、手机号并彩色高亮，点击即可打开</li><li>多色便签与毛玻璃主题，支持亮色 / 暗色模式</li><li>便签窗口可置顶固定，自动记忆位置与大小</li><li>开机自启动，并恢复上次打开的便签</li><li>系统托盘：打开主窗口或退出应用</li><li>边缘自动隐藏：在设置中开启后，便签贴屏幕左 / 右 / 上边缘时会收起为窄条，鼠标悬停窄条即可唤回，移开后自动收回</li></ul><p><strong>使用方法</strong></p><ul><li>点击右下角 + 按钮新建便签</li><li>点击任意便签打开独立编辑窗口</li><li>鼠标悬停便签卡片，可置顶或删除</li><li>关闭编辑窗口即隐藏，内容不会丢失</li></ul><p><strong>快捷键</strong></p><ul><li>Ctrl+Shift+N：新建便签</li><li>Ctrl+Shift+M：显示 / 隐藏主窗口</li></ul><p>这条便签可以随时删除，祝你使用愉快！</p>"#;

pub fn welcome_text(lang: &str) -> (&'static str, &'static str) {
    if lang == "en" {
        (WELCOME_TITLE_EN, WELCOME_CONTENT_EN)
    } else {
        (WELCOME_TITLE_ZH, WELCOME_CONTENT_ZH)
    }
}


impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS notes (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL DEFAULT '',
                content TEXT NOT NULL DEFAULT '',
                color TEXT NOT NULL DEFAULT 'yellow',
                font_size INTEGER NOT NULL DEFAULT 14,
                position_x INTEGER NOT NULL DEFAULT 100,
                position_y INTEGER NOT NULL DEFAULT 100,
                width INTEGER NOT NULL DEFAULT 500,
                height INTEGER NOT NULL DEFAULT 450,
                is_pinned INTEGER NOT NULL DEFAULT 0,
                sticky_edge TEXT,
                is_hidden INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                deleted_at TEXT
            );
            UPDATE notes SET color = 'silver' WHERE color = 'white';"
        )?;
        Ok(Database { conn: Mutex::new(conn) })
    }

    pub fn create_note(&self, color: &str, content: &str) -> Result<NoteRow> {
        let conn = self.conn.lock().unwrap();
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let title = content.lines().next().unwrap_or("").chars().take(50).collect::<String>();

        conn.execute(
            "INSERT INTO notes (id, title, content, color, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, title, content, color, now, now],
        )?;

        Ok(NoteRow {
            id,
            title,
            content: content.to_string(),
            color: color.to_string(),
            font_size: 14,
            position_x: 100,
            position_y: 100,
            width: 500,
            height: 450,
            is_pinned: false,
            sticky_edge: None,
            is_hidden: false,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn seed_welcome_note(&self, lang: &str, color: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let (title, content) = welcome_text(lang);
        conn.execute(
            "INSERT INTO notes (id, title, content, color, is_pinned, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![id, title, content, color, 1, now, now],
        )?;
        Ok(())
    }

    // 把仍是欢迎便签（标题与内容都未被用户改写）的行刷新成当前语言的最新文案，
    // 顺带完成旧版内容升级。匹配条件限定在内置文案集合内，用户编辑过的便签不会命中；
    // 有意不碰 updated_at，切语言不该打乱便签排序。
    pub fn localize_welcome_note(&self, lang: &str) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let (title, content) = welcome_text(lang);
        let changed = conn.execute(
            "UPDATE notes SET title = ?1, content = ?2 \
             WHERE content IN (?3, ?4, ?5) AND title IN (?6, ?7)",
            params![
                title,
                content,
                WELCOME_CONTENT_ZH,
                WELCOME_CONTENT_EN,
                WELCOME_CONTENT_PREV,
                WELCOME_TITLE_ZH,
                WELCOME_TITLE_EN,
            ],
        )?;
        Ok(changed)
    }

    pub fn get_notes(&self) -> Result<Vec<NoteRow>> {
        let conn = self.conn.lock().unwrap();
        // deleted_at 只用于过滤旧版本软删除留下的行，见 purge_legacy_deleted
        let sql = format!(
            "SELECT {} FROM notes WHERE deleted_at IS NULL ORDER BY updated_at DESC",
            NOTE_COLUMNS
        );
        let mut stmt = conn.prepare(&sql)?;

        let notes = stmt.query_map([], map_note_row)?;

        let mut result = Vec::new();
        for note in notes {
            result.push(note?);
        }
        Ok(result)
    }

    pub fn get_note(&self, id: &str) -> Result<NoteRow> {
        let conn = self.conn.lock().unwrap();
        let sql = format!(
            "SELECT {} FROM notes WHERE id = ? AND deleted_at IS NULL",
            NOTE_COLUMNS
        );
        conn.query_row(&sql, params![id], map_note_row)
    }

    pub fn update_note(
        &self,
        id: &str,
        title: Option<&str>,
        content: Option<&str>,
        color: Option<&str>,
        font_size: Option<i32>,
        position_x: Option<i32>,
        position_y: Option<i32>,
        width: Option<i32>,
        height: Option<i32>,
        is_pinned: Option<bool>,
        sticky_edge: Option<&str>,
        is_hidden: Option<bool>,
    ) -> Result<NoteRow> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();

        let mut updates = Vec::new();
        let mut params_vec: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        if let Some(v) = title { updates.push("title = ?"); params_vec.push(Box::new(v.to_string())); }
        if let Some(v) = content { updates.push("content = ?"); params_vec.push(Box::new(v.to_string())); }
        if let Some(v) = color { updates.push("color = ?"); params_vec.push(Box::new(v.to_string())); }
        if let Some(v) = font_size { updates.push("font_size = ?"); params_vec.push(Box::new(v)); }
        if let Some(v) = position_x { updates.push("position_x = ?"); params_vec.push(Box::new(v)); }
        if let Some(v) = position_y { updates.push("position_y = ?"); params_vec.push(Box::new(v)); }
        if let Some(v) = width { updates.push("width = ?"); params_vec.push(Box::new(v)); }
        if let Some(v) = height { updates.push("height = ?"); params_vec.push(Box::new(v)); }
        if let Some(v) = is_pinned { updates.push("is_pinned = ?"); params_vec.push(Box::new(if v { 1 } else { 0 })); }
        if let Some(v) = sticky_edge { updates.push("sticky_edge = ?"); params_vec.push(Box::new(v.to_string())); }
        if let Some(v) = is_hidden { updates.push("is_hidden = ?"); params_vec.push(Box::new(if v { 1 } else { 0 })); }

        updates.push("updated_at = ?");
        params_vec.push(Box::new(now));

        let params_refs: Vec<&dyn rusqlite::types::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
        let sql = format!("UPDATE notes SET {} WHERE id = ?", updates.join(", "));

        let mut final_params: Vec<&dyn rusqlite::types::ToSql> = params_refs;
        final_params.push(&id);

        let affected = conn.execute(&sql, final_params.as_slice())?;
        drop(conn);

        // 自动保存是高频调用，只读回被改的这一行，不能全表加载（便签含 base64 图片）
        if affected == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
        self.get_note(id)
    }

    pub fn delete_note(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM notes WHERE id = ?", params![id])?;
        Ok(())
    }

    /// 早期版本的删除是软删除，被删内容会永久留在 memo.db 里，升级后一次性物理清除
    pub fn purge_legacy_deleted(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        if conn.execute("DELETE FROM notes WHERE deleted_at IS NOT NULL", [])? > 0 {
            conn.execute("VACUUM", [])?;
        }
        Ok(())
    }
}

pub fn init_db(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let db_path = app
        .path()
        .app_data_dir()
        .expect("failed to get app data dir")
        .join("memo.db");

    std::fs::create_dir_all(db_path.parent().unwrap())?;

    let db = Database::new(db_path.to_str().unwrap())?;
    // 历史遗留清理属于维护性操作，失败也不能阻止应用启动
    if let Err(e) = db.purge_legacy_deleted() {
        eprintln!("purge legacy deleted notes failed: {}", e);
    }
    app.manage(db);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_db() -> (Database, std::path::PathBuf) {
        let path = std::env::temp_dir().join(format!("memo-test-{}.db", Uuid::new_v4()));
        let db = Database::new(path.to_str().unwrap()).unwrap();
        (db, path)
    }

    #[test]
    fn update_note_reads_back_only_its_own_row() {
        let (db, path) = temp_db();
        let a = db.create_note("yellow", "a").unwrap();
        let b = db.create_note("yellow", "b").unwrap();

        let updated = db
            .update_note(&a.id, None, Some("changed"), None, None, None, None, None, None, None, None, None)
            .unwrap();

        assert_eq!(updated.id, a.id);
        assert_eq!(updated.content, "changed");
        assert_eq!(db.get_note(&b.id).unwrap().content, "b");
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn update_note_on_missing_id_reports_no_rows() {
        let (db, path) = temp_db();
        let err = db
            .update_note("missing", None, Some("x"), None, None, None, None, None, None, None, None, None)
            .unwrap_err();
        assert!(matches!(err, rusqlite::Error::QueryReturnedNoRows));
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn localize_welcome_note_only_touches_untouched_welcome() {
        let (db, path) = temp_db();
        db.seed_welcome_note("zh", "glass").unwrap();
        let edited = db.create_note("yellow", WELCOME_CONTENT_ZH).unwrap();
        db.update_note(&edited.id, Some("我的便签"), None, None, None, None, None, None, None, None, None, None)
            .unwrap();

        db.localize_welcome_note("en").unwrap();

        let notes = db.get_notes().unwrap();
        let welcome = notes.iter().find(|n| n.title == WELCOME_TITLE_EN).unwrap();
        assert_eq!(welcome.content, WELCOME_CONTENT_EN);
        // 用户改过标题的便签保持原样
        assert!(notes.iter().any(|n| n.id == edited.id && n.title == "我的便签"));
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn localize_welcome_note_upgrades_legacy_content() {
        let (db, path) = temp_db();
        db.create_note("glass", "占位").unwrap();
        let legacy = db.get_notes().unwrap().remove(0);
        {
            let conn = db.conn.lock().unwrap();
            conn.execute(
                "UPDATE notes SET title = ?1, content = ?2 WHERE id = ?3",
                params![WELCOME_TITLE_ZH, WELCOME_CONTENT_PREV, legacy.id],
            )
            .unwrap();
        }

        db.localize_welcome_note("en").unwrap();

        let note = db.get_note(&legacy.id).unwrap();
        assert_eq!(note.title, WELCOME_TITLE_EN);
        assert_eq!(note.content, WELCOME_CONTENT_EN);
        let _ = std::fs::remove_file(path);
    }
}
