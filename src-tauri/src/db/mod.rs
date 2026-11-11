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

// 首次安装时植入的欢迎便签内容（富文本 HTML，与编辑器格式一致）
const WELCOME_CONTENT: &str = r#"<p><strong>欢迎使用 Memo</strong></p><p>Memo 是一款轻量桌面便签应用，帮你随手记录灵感与待办事项。</p><p><strong>主要功能</strong></p><ul><li>富文本编辑：加粗、斜体、列表、复选框，支持拖拽插入图片并缩放</li><li>自动识别网址、邮箱、手机号并彩色高亮，点击即可打开</li><li>多色便签与毛玻璃主题，支持亮色 / 暗色模式</li><li>便签窗口可置顶固定，自动记忆位置与大小</li><li>开机自启动，并恢复上次打开的便签</li><li>系统托盘：打开主窗口或退出应用</li><li>边缘自动隐藏：在设置中开启后，便签贴屏幕左 / 右 / 上边缘时会收起为窄条，鼠标悬停窄条即可唤回，移开后自动收回</li></ul><p><strong>使用方法</strong></p><ul><li>点击右下角 + 按钮新建便签</li><li>点击任意便签打开独立编辑窗口</li><li>鼠标悬停便签卡片，可置顶或删除</li><li>关闭编辑窗口即隐藏，内容不会丢失</li></ul><p><strong>快捷键</strong></p><ul><li>Ctrl+Shift+N：新建便签</li><li>Ctrl+Shift+M：显示 / 隐藏主窗口</li></ul><p>这条便签可以随时删除，祝你使用愉快！</p>"#;

// 上一版欢迎便签内容：仅用于把"未被用户改写"的旧欢迎便签原地刷新为最新版，
// 精确匹配可避免覆盖用户自行编辑过的便签。升级欢迎内容时，把旧的 WELCOME_CONTENT 挪到这里。
const WELCOME_CONTENT_PREV: &str = r#"<p><strong>欢迎使用 Memo</strong></p><p>Memo 是一款轻量桌面便签应用，帮你随手记录灵感与待办事项。</p><p><strong>主要功能</strong></p><ul><li>富文本编辑：加粗、斜体、列表、复选框，支持拖拽插入图片并缩放</li><li>自动识别网址、邮箱、手机号并彩色高亮，点击即可打开</li><li>多色便签与毛玻璃主题，支持亮色 / 暗色模式</li><li>便签窗口可置顶固定，自动记忆位置与大小</li><li>开机自启动，并恢复上次打开的便签</li><li>系统托盘：打开主窗口或退出应用</li></ul><p><strong>使用方法</strong></p><ul><li>点击右下角 + 按钮新建便签</li><li>点击任意便签打开独立编辑窗口</li><li>鼠标悬停便签卡片，可置顶或删除</li><li>关闭编辑窗口即隐藏，内容不会丢失</li></ul><p><strong>快捷键</strong></p><ul><li>Ctrl+Shift+N：新建便签</li><li>Ctrl+Shift+M：显示 / 隐藏主窗口</li></ul><p>这条便签可以随时删除，祝你使用愉快！</p>"#;

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

    pub fn seed_welcome_note(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO notes (id, title, content, color, is_pinned, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![id, "欢迎使用 Memo", WELCOME_CONTENT, "glass", 1, now, now],
        )?;
        Ok(())
    }

    // 把内容仍与上一版欢迎便签完全一致（即用户未改写）的便签原地刷新为最新内容。
    // 精确匹配旧内容可避免覆盖用户自行编辑过的便签。
    pub fn refresh_welcome_note(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE notes SET content = ?1 WHERE content = ?2",
            params![WELCOME_CONTENT, WELCOME_CONTENT_PREV],
        )?;
        Ok(())
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
}
