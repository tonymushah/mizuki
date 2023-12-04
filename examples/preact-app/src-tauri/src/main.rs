#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use async_graphql::{
  futures_util::{self, stream::Stream},
  EmptyMutation, Object, Result as GraphQLResult, Schema, SimpleObject, Subscription,
};

#[derive(SimpleObject, Debug, Clone)]
struct Human {
  name: String,
}

struct Query;

#[Object]
impl Query {
  async fn hero(&self) -> GraphQLResult<Human> {
    Ok(Human {
      name: "Luke Skywalker".to_string(),
    })
  }
}

pub struct Subscription;

#[Subscription]
impl Subscription {
  async fn hello_world(&self) -> impl Stream<Item = &str> {
    futures_util::stream::iter(vec!["Hello", "World!"])
  }
}

fn main() {
  let my_plugin = mizuki::MizukiPlugin::new("mizuki-test", Schema::new(Query, EmptyMutation, Subscription));
  my_plugin.export_sdl("../myschema.graphqls").unwrap();
  tauri::Builder::default()
    .plugin(my_plugin)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
