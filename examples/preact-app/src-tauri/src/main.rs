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
  let schema = Schema::new(Query, EmptyMutation, Subscription);
  println!("{}", schema.sdl());
  tauri::Builder::default()
    .plugin(mizuki::Mizuki::new("graphql", schema))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
