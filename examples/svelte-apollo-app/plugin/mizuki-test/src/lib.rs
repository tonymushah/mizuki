use async_graphql::{
  futures_util::stream::Stream, Context, EmptyMutation, Error, Object, Schema, Subscription,
};
use mizuki::Builder;
use mizuki::{AsyncGQLContextExt, MizukiPlugin};
use std::{
  fs::File,
  io::{BufWriter, Write},
  path::Path,
  task::ready,
  task::Poll,
};
use tauri::Runtime;
use tauri::{Emitter, EventId, Listener, Webview};
use tokio::sync::watch::{self, channel, Receiver};
use tokio_util::sync::ReusableBoxFuture;

const CHANNEL_NAME: &str = "message_channel";

pub struct Query;

#[Object]
impl Query {
  async fn say(&self, cx: &Context<'_>, name: Option<String>) -> String {
    if let Some(name) = name {
      if let Some(e) = cx.webview::<tauri::Wry>() {
        let _ = e.emit(CHANNEL_NAME, name.clone());
      }
      format!("Hello {name}!")
    } else {
      "Insert your name please!".into()
    }
  }
}

type EventListenerInnerFut = ReusableBoxFuture<
  'static,
  (
    Result<(), watch::error::RecvError>,
    Receiver<Option<String>>,
  ),
>;

struct EventListener<R>
where
  R: Runtime,
{
  webview: Webview<R>,
  event_id: EventId,
  fut: EventListenerInnerFut,
}

async fn make_future(
  mut rx: Receiver<Option<String>>,
) -> (
  Result<(), watch::error::RecvError>,
  Receiver<Option<String>>,
) {
  let res = rx.changed().await;

  (res, rx)
}

impl<R: Runtime> Drop for EventListener<R> {
  fn drop(&mut self) {
    self.webview.unlisten(self.event_id);
    println!("dropped and unlistened {}", self.event_id);
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
      fut: ReusableBoxFuture::new(async move {
        if rx.borrow().is_some() {
          (Ok(()), rx)
        } else {
          make_future(rx).await
        }
      }),
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
    let (result, mut rx) = ready!(self.fut.poll(cx));
    match result {
      Ok(_) => {
        let received = (*rx.borrow_and_update()).clone();
        self.fut.set(make_future(rx));
        Poll::Ready(received.map(|e| {
          if let Ok(strg) = serde_json::from_str::<String>(&e) {
            strg
          } else {
            e
          }
        }))
      }
      Err(_) => {
        self.fut.set(make_future(rx));
        Poll::Ready(None)
      }
    }
  }
}

pub struct Subscriptions;

#[Subscription]
impl Subscriptions {
  async fn watch_messages(
    &self,
    ctx: &Context<'_>,
  ) -> async_graphql::Result<impl Stream<Item = String>> {
    let webview = ctx
      .webview::<tauri::Wry>()
      .ok_or(Error::new("Cannot access webview ref"))?;
    Ok(EventListener::new(webview.clone(), CHANNEL_NAME.into()))
  }
}

pub fn init<R: Runtime>() -> MizukiPlugin<R, Query, EmptyMutation, Subscriptions> {
  Builder::new(
    "mizuki-test-apollo",
    Schema::new(Query, EmptyMutation, Subscriptions),
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
