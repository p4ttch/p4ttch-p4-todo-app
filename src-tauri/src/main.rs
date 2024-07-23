// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri_plugin_sql::{Migration, MigrationKind};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn add_todo(task_name: &str) -> String {
    format!("Added task, {}!", task_name)
}

fn main() {
    let migrations = vec![
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: "
            CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT); 

            CREATE TABLE tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                category_id INTEGER,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT DEFAULT 'pending', -- 'pending', 'in_progress', 'completed'
                start_time TEXT,
                end_time TEXT,
                task_time TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                -- FOREIGN KEY (user_id) REFERENCES users(id),
                -- FOREIGN KEY (category_id) REFERENCES categories(id)
            );

            CREATE TABLE settings (id INTEGER PRIMARY KEY, name TEXT);
            ",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "add_due_date_to_todos",
            sql: "ALTER TABLE todos ADD COLUMN due_date TEXT;",
            kind: MigrationKind::Up,
        },
    ];

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:p4todo.db", migrations)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![add_todo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
