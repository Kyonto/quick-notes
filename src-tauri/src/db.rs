use std::fs;
use rusqlite::{Connection, Result};
use tauri::AppHandle;
use tauri::Manager;

pub fn init_db(app_handle: &AppHandle) -> Result<()> {
    let app_local_dir: std::path::PathBuf = app_handle.path().app_data_dir()
        .expect("Failed to get local data dir");
    let db_path = app_local_dir.join("notes.db");
    println!("AppLocalData path: {}", app_local_dir.display());
    println!("Attempting to create/access database at: {}", db_path.display());

    if std::path::Path::new(&db_path).exists() {
        println!("Database already exists!");
        return Ok(());
    } else {
        println!("Database does not exist, creating new one...");
    }

    // Create parent directory if needed
    if let Some(parent) = std::path::Path::new(&db_path).parent() {
        println!("Creating directory: {}", parent.display());
        fs::create_dir_all(parent).expect("Failed to create app data directory");
    }

    println!("Creating database connection...");
    let conn = Connection::open(&db_path)?;
    println!("Database connection successful!");

    // Main notes table
    println!("Creating tables...");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            tags TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    // Full-text search virtual table
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS notes_fts USING fts5(
            title, content, tags,
            content='notes', content_rowid='id'
        )",
        [],
    )?;

    println!("Tables created successfully!");
    Ok(())
}