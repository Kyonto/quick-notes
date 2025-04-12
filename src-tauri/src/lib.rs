mod db;

use serde::{Deserialize, Serialize};
use rusqlite::Connection;
use tauri::AppHandle;
use tauri::Manager;

#[derive(Serialize, Deserialize)]
pub struct Note {
    id: i64,
    title: String,
    content: String,
    tags: Option<String>,
    created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct PaginatedNotes {
    notes: Vec<Note>,
    total: i64,
}

#[derive(Deserialize)]
pub struct NewNote {
    id: Option<i64>,
    title: String,
    content: String,
    tags: Option<String>,
}

#[tauri::command]
fn search_notes(app_handle: AppHandle, query: String) -> Result<Vec<Note>, String> {
    if query.trim().is_empty() {
        return Ok(Vec::new());
    }

    let app_local_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = app_local_dir.join("notes.db");
    
    let conn = Connection::open(&db_path)
        .map_err(|e| e.to_string())?;

    // Search with priority (title > tags > content)
    let mut stmt = conn.prepare(
        "SELECT n.id, n.title, n.content, n.tags, n.created_at, 
         (CASE 
            WHEN lower(n.title) LIKE lower(?) THEN 3
            WHEN n.tags IS NOT NULL AND lower(n.tags) LIKE lower(?) THEN 2
            WHEN lower(n.content) LIKE lower(?) THEN 1
            ELSE 0
         END) as priority
         FROM notes n
         WHERE lower(n.title) LIKE lower(?)
         OR (n.tags IS NOT NULL AND lower(n.tags) LIKE lower(?))
         OR lower(n.content) LIKE lower(?)
         ORDER BY priority DESC, created_at DESC"
    ).map_err(|e| e.to_string())?;

    let search_pattern = format!("%{}%", query);
    let notes = stmt.query_map(
        [&search_pattern, &search_pattern, &search_pattern, &search_pattern, &search_pattern, &search_pattern],
        |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                tags: row.get(3)?,
                created_at: row.get(4)?,
            })
        }
    ).map_err(|e| e.to_string())?
    .collect::<Result<Vec<Note>, _>>()
    .map_err(|e| e.to_string())?;

    Ok(notes)
}

#[tauri::command]
fn save_note(app_handle: AppHandle, note: NewNote) -> Result<(), String> {
    let app_local_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = app_local_dir.join("notes.db");
    
    let mut conn = Connection::open(&db_path)
        .map_err(|e| e.to_string())?;

    let tx = conn.transaction()
        .map_err(|e| e.to_string())?;

    // Get tags once to avoid multiple moves
    let tags = note.tags.unwrap_or_default();

    if let Some(id) = note.id {
        // Update existing note
        tx.execute(
            "UPDATE notes SET title = ?, content = ?, tags = ? WHERE id = ?",
            [&note.title, &note.content, &tags, &id.to_string()],
        ).map_err(|e| e.to_string())?;

        // Update FTS table
        tx.execute(
            "UPDATE notes_fts SET title = ?, content = ?, tags = ? WHERE rowid = ?",
            [&note.title, &note.content, &tags, &id.to_string()],
        ).map_err(|e| e.to_string())?;
    } else {
        // Insert new note
        tx.execute(
            "INSERT INTO notes (title, content, tags) VALUES (?, ?, ?)",
            [&note.title, &note.content, &tags],
        ).map_err(|e| e.to_string())?;

        // Get the last inserted row id
        let id = tx.last_insert_rowid();

        // Insert into FTS table
        tx.execute(
            "INSERT INTO notes_fts (rowid, title, content, tags) VALUES (?, ?, ?, ?)",
            [&id.to_string(), &note.title, &note.content, &tags],
        ).map_err(|e| e.to_string())?;
    }

    // Commit the transaction
    tx.commit()
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn get_paginated_notes(app_handle: AppHandle, page: i64, page_size: i64) -> Result<PaginatedNotes, String> {
    let app_local_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = app_local_dir.join("notes.db");
    
    let conn = Connection::open(&db_path)
        .map_err(|e| e.to_string())?;

    // Get total count
    let total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM notes",
        [],
        |row| row.get(0)
    ).map_err(|e| e.to_string())?;

    // Get paginated notes
    let mut stmt = conn.prepare(
        "SELECT id, title, content, tags, created_at FROM notes 
         ORDER BY created_at DESC 
         LIMIT ? OFFSET ?"
    ).map_err(|e| e.to_string())?;

    let offset = (page - 1) * page_size;
    let notes = stmt.query_map(
        [page_size, offset],
        |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                tags: row.get(3)?,
                created_at: row.get(4)?,
            })
        }
    ).map_err(|e| e.to_string())?
    .collect::<Result<Vec<Note>, _>>()
    .map_err(|e| e.to_string())?;

    Ok(PaginatedNotes { notes, total })
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn delete_note(app_handle: AppHandle, note_id: i64) -> Result<(), String> {
    let app_local_dir = app_handle.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = app_local_dir.join("notes.db");
    
    let mut conn = Connection::open(&db_path)
        .map_err(|e| e.to_string())?;

    let tx = conn.transaction()
        .map_err(|e| e.to_string())?;

    // Delete from the main notes table
    tx.execute(
        "DELETE FROM notes WHERE id = ?",
        [note_id],
    ).map_err(|e| e.to_string())?;

    // Delete from the FTS virtual table
    tx.execute(
        "DELETE FROM notes_fts WHERE rowid = ?",
        [note_id],
    ).map_err(|e| e.to_string())?;

    // Commit the transaction
    tx.commit()
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if let Err(e) = db::init_db(app.handle()) {
                eprintln!("Failed to initialize database: {}", e);
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_paginated_notes, save_note, search_notes, delete_note])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
