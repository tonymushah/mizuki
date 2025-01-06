use std::{
  fs::File,
  future::Future,
  io::{BufWriter, Write},
  path::Path,
  task::{ready, Poll},
};

use async_graphql::{
  futures_util::{self, stream::Stream},
  Error, Object, Result as GraphQLResult, Schema, SimpleObject, Subscription,
};
use mizuki::{AsyncGQLContextExt, MizukiPlugin};
use tauri::{Emitter, EventId, Listener, Runtime, Webview};
use tokio::sync::watch::{channel, Receiver};

#[derive(SimpleObject, Debug, Clone)]
struct Human {
  name: String,
}

pub struct Query;

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

const CHANNEL_NAME: &str = "message_channel";

pub struct Mutation;

#[Object]
impl Mutation {
  async fn send_message(
    &self,
    context: &async_graphql::Context<'_>,
    message: String,
  ) -> async_graphql::Result<bool> {
    let webview = context
      .webview::<tauri::Wry>()
      .ok_or(async_graphql::Error::new("Cannot access webview ref"))?;
    webview.emit(CHANNEL_NAME, message)?;
    Ok(true)
  }
}

pub struct Subscription;

struct EventListener<R>
where
  R: Runtime,
{
  webview: Webview<R>,
  event_id: EventId,
  receiver: Receiver<Option<String>>,
}

impl<R: Runtime> Drop for EventListener<R> {
  fn drop(&mut self) {
    self.webview.unlisten(self.event_id);
  }
}

impl<R: Runtime> Unpin for EventListener<R> {}

impl<R: Runtime> EventListener<R> {
  pub fn new(webview: Webview<R>, event_label: String) -> Self {
    let (tx, rx) = channel(None::<String>);
    Self {
      event_id: webview.listen(event_label, move |e| {
        let _ = tx.send(Some(e.payload().into()));
      }),
      receiver: rx,
      webview,
    }
  }
}

impl<R: Runtime> Stream for EventListener<R> {
  type Item = String;
  fn poll_next(
    mut self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> Poll<Option<Self::Item>> {
    let mut d = Box::pin(self.receiver.changed());
    match ready!(d.as_mut().poll(cx)) {
      Ok(_) => {
        drop(d);
        if let Some(e) = self.receiver.borrow().as_ref() {
          Poll::Ready(Some(e.clone()))
        } else {
          cx.waker().wake_by_ref();
          Poll::Pending
        }
      }
      Err(_) => Poll::Ready(None),
    }
  }
}

#[Subscription]
impl Subscription {
  async fn hello_world(&self) -> impl Stream<Item = &str> {
    futures_util::stream::iter(vec!["Hello", "World!"])
  }
  async fn watch_messages(
    &self,
    context: &async_graphql::Context<'_>,
  ) -> async_graphql::Result<impl Stream<Item = String>> {
    let webview = context
      .webview::<tauri::Wry>()
      .ok_or(async_graphql::Error::new("Cannot access webview ref"))?
      .clone();
    Ok(EventListener::new(webview, CHANNEL_NAME.into()))
  }
}

pub fn init<R: Runtime>() -> MizukiPlugin<R, Query, Mutation, Subscription> {
  mizuki::Builder::new("mizuki-test", Schema::new(Query, Mutation, Subscription))
    .setup(|_app, _config, s| {
      #[cfg(debug_assertions)]
      {
        let sdl = s.sdl();
        let sdl_file = File::create(Path::new("../myschema.graphqls"))?;
        let mut buf_write = BufWriter::new(sdl_file);
        buf_write.write_all(sdl.as_bytes())?;
        buf_write.flush()?;
      }
      Ok(())
    })
    .build()
}
