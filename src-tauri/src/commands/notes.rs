use serde::{Deserialize, Serialize};
use tauri::State;
use crate::db::{Database, NoteRow};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub deleted_at: Option<String>,
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
            deleted_at: row.deleted_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub color: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
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
pub fn delete_note(db: State<Database>, id: String) -> Result<(), String> {
    db.delete_note(&id).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn restore_note(db: State<Database>, id: String) -> Result<(), String> {
    db.restore_note(&id).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn empty_trash(db: State<Database>) -> Result<(), String> {
    db.empty_trash().map_err(|e| e.to_string())?;
    Ok(())
}
