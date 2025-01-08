#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{command, generate_handler};

#[command]
async fn just_print(smth: Option<String>) {
  println!("{:?}", smth);
}

fn main() {
  tauri::Builder::default()
    .plugin(mizuki_test::init())
    .invoke_handler(generate_handler![just_print])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
