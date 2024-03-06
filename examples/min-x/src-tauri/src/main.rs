// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use mizuki::MizukiPlugin;
use tauri::Runtime;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}! You've been greeted from Rust!", name)
}

struct Query;

#[Object]
impl Query {
  async fn value(&self) -> u32 {
    100
  }
}

fn init_my_plug<R: Runtime>() -> MizukiPlugin<R, Query, EmptyMutation, EmptySubscription> {
  mizuki::Builder::<R, Query, EmptyMutation, EmptySubscription>::new(
    "my-plug",
    Schema::new(Query, EmptyMutation, EmptySubscription),
  )
  .build()
}

fn main() {
  tauri::Builder::default()
    .plugin(init_my_plug())
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
