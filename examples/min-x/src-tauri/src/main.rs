// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mizuki::MizukiPluginTrait;
use async_graphql::{EmptyMutation, Object, EmptySubscription, Schema};
use tauri::{plugin::Plugin, Runtime};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

struct Query;

#[Object]
impl Query{
    async fn value(&self) -> u32 {
        100
    }
}

struct MyPlug;
impl<R> MizukiPluginTrait<R, Query, EmptyMutation, EmptySubscription> for MyPlug 
where R : Runtime
{
    fn schema(&self) -> async_graphql::Schema<Query, EmptyMutation, EmptySubscription> {
        Schema::new(Query, EmptyMutation, EmptySubscription)
    }
}

impl<R> Plugin<R> for MyPlug 
where R: Runtime
{
    fn name(&self) -> &'static str {
        "my-plug"
    }
    fn extend_api(&mut self, invoke: tauri::Invoke<R>) {
        <MyPlug as MizukiPluginTrait<R, Query, EmptyMutation, EmptySubscription>>::extend_api(self, invoke)
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(MyPlug)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
