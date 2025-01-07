use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use mizuki::{Builder, MizukiPlugin};
use std::{
  fs::File,
  io::{BufWriter, Write},
  path::Path,
};
use tauri::Runtime;

pub struct Query;

#[Object]
impl Query {
  async fn say(&self, name: Option<String>) -> String {
    if let Some(name) = name {
      format!("Hello {name}!")
    } else {
      "Insert your name please!".into()
    }
  }
}

pub fn init<R: Runtime>() -> MizukiPlugin<R, Query, EmptyMutation, EmptySubscription> {
  Builder::new(
    "mizuki-test",
    Schema::new(Query, EmptyMutation, EmptySubscription),
  )
  .setup(|_app, _, schema| {
    #[cfg(debug_assertions)]
    {
      let sdl = schema.sdl();
      let sdl_file = File::create(Path::new("../myschema.graphqls"))?;
      let mut buf_write = BufWriter::new(sdl_file);
      buf_write.write_all(sdl.as_bytes())?;
      buf_write.flush()?;
    }
    Ok(())
  })
  .build()
}
