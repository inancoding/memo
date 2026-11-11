use serde::{Deserialize, Serialize};
use tauri::{Manager, State};
use crate::db::{Database, NoteRow};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Note {
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

impl From<NoteRow> for Note {
    fn from(row: NoteRow) -> Self {
        Note {
            id: row.id,
            title: row.title,
            content: row.content,
            color: row.color,
            font_size: row.font_size,
            position_x: row.position_x,
            position_y: row.position_y,
            width: row.width,
            height: row.height,
            is_pinned: row.is_pinned,
            sticky_edge: row.sticky_edge,
            is_hidden: row.is_hidden,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNoteRequest {
    pub color: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNoteRequest {
    pub id: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub color: Option<String>,
    pub font_size: Option<i32>,
    pub position_x: Option<i32>,
    pub position_y: Option<i32>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub is_pinned: Option<bool>,
    pub sticky_edge: Option<String>,
    pub is_hidden: Option<bool>,
}

#[tauri::command]
pub fn create_note(
    db: State<Database>,
    request: CreateNoteRequest,
) -> Result<Note, String> {
    let note = db.create_note(
        request.color.as_deref().unwrap_or("yellow"),
        request.content.as_deref().unwrap_or(""),
    ).map_err(|e| e.to_string())?;
    Ok(note.into())
}

#[tauri::command]
pub fn get_notes(db: State<Database>) -> Result<Vec<Note>, String> {
    let notes = db.get_notes().map_err(|e| e.to_string())?;
    Ok(notes.into_iter().map(Note::from).collect())
}

#[tauri::command]
pub fn update_note(
    db: State<Database>,
    request: UpdateNoteRequest,
) -> Result<Note, String> {
    let note = db.update_note(
        &request.id,
        request.title.as_deref(),
        request.content.as_deref(),
        request.color.as_deref(),
        request.font_size,
        request.position_x,
        request.position_y,
        request.width,
        request.height,
        request.is_pinned,
        request.sticky_edge.as_deref(),
        request.is_hidden,
    ).map_err(|e| e.to_string())?;
    Ok(note.into())
}

#[tauri::command]
pub fn delete_note(
    app_handle: tauri::AppHandle,
    db: State<Database>,
    id: String,
) -> Result<(), String> {
    db.delete_note(&id).map_err(|e| e.to_string())?;
    // 删除是彻底的，保存失败留下的兜底文件也一并清掉，否则会残留明文
    let _ = remove_recovery_file(&app_handle, &id);
    crate::forget_desktop_note(&app_handle, &id);
    Ok(())
}

// ===== 自动保存失败时的兜底落盘 =====
// 数据库写入持续失败时把最新内容写成独立文件，下次打开该便签再读回，避免用户输入丢失。
fn recovery_file_in(base: &std::path::Path, note_id: &str) -> Option<std::path::PathBuf> {
    // id 来自前端，限制为 UUID 字符，避免拼路径越界
    if note_id.is_empty()
        || !note_id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-')
    {
        return None;
    }
    Some(base.join("recovery").join(format!("{}.html", note_id)))
}

fn recovery_path(app_handle: &tauri::AppHandle, note_id: &str) -> Option<std::path::PathBuf> {
    recovery_file_in(&app_handle.path().app_data_dir().ok()?, note_id)
}

fn remove_recovery_file(app_handle: &tauri::AppHandle, note_id: &str) -> Result<(), String> {
    let Some(path) = recovery_path(app_handle, note_id) else {
        return Ok(());
    };
    match std::fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn write_recovery_content(
    app_handle: tauri::AppHandle,
    note_id: String,
    content: String,
) -> Result<(), String> {
    let path = recovery_path(&app_handle, &note_id).ok_or_else(|| "invalid note id".to_string())?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn read_recovery_content(
    app_handle: tauri::AppHandle,
    note_id: String,
) -> Option<String> {
    recovery_path(&app_handle, &note_id).and_then(|p| std::fs::read_to_string(p).ok())
}

#[tauri::command]
pub fn clear_recovery_content(
    app_handle: tauri::AppHandle,
    note_id: String,
) -> Result<(), String> {
    remove_recovery_file(&app_handle, &note_id)
}

#[cfg(test)]
mod tests {
    use super::recovery_file_in;
    use std::path::Path;

    #[test]
    fn recovery_file_stays_inside_app_data_dir() {
        let base = Path::new("data");
        assert_eq!(
            recovery_file_in(base, "6f1c-2").unwrap(),
            base.join("recovery").join("6f1c-2.html")
        );
        for bad in ["", "../evil", "a\\b", "a/b", "a b"] {
            assert!(recovery_file_in(base, bad).is_none(), "accepted {:?}", bad);
        }
    }
}
