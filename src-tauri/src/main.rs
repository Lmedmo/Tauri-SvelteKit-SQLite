// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod commands;

#[async_std::main]
async fn main() {
  db::init().await;

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      commands::get_tasks
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
