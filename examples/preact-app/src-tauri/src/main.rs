#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{fs::File, io::{BufWriter, Write}, path::Path};

use async_graphql::{
  futures_util::{self, stream::Stream}, EmptyMutation, Error, Object, Result as GraphQLResult, Schema, SimpleObject, Subscription
};
use mizuki::MizukiPlugin;
use tauri::Runtime;

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
  async fn not_hero(&self) -> GraphQLResult<Human> {
    Err(Error::new("Only heroes can be fetched!"))
  }
}

pub struct Subscription;

#[Subscription]
impl Subscription {
  async fn hello_world(&self) -> impl Stream<Item = &str> {
    futures_util::stream::iter(vec!["Hello", "World!"])
  }
}

fn new_mizuki_test<R: Runtime>() -> MizukiPlugin<R, Query, EmptyMutation, Subscription> {
  mizuki::Builder::new("mizuki-test", Schema::new(Query, EmptyMutation, Subscription)).setup(|_app, _config, s| {
    #[cfg(debug_assertions)]
    {
      let sdl = s.sdl();
      let sdl_file = File::create(Path::new("../myschema.graphqls"))?;
      let mut buf_write = BufWriter::new(sdl_file);
      buf_write.write_all(sdl.as_bytes())?;
      buf_write.flush()?;
    }
    Ok(())
  }).build()
}

fn main() {
  
  tauri::Builder::default()
    .plugin(new_mizuki_test())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
