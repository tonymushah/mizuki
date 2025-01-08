use std::marker::PhantomData;

use tauri::{EventId, Listener, Runtime};
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
pub struct CancellationTokenListener<L, R>
where
  L: Listener<R>,
  R: Runtime,
{
  listener: L,
  event_id: EventId,
  cancel_token: CancellationToken,
  runtime: PhantomData<R>,
}

impl<L, R> Drop for CancellationTokenListener<L, R>
where
  L: Listener<R>,
  R: Runtime,
{
  fn drop(&mut self) {
    self.listener.unlisten(self.event_id);
  }
}

impl<L, R> CancellationTokenListener<L, R>
where
  L: Listener<R>,
  R: Runtime,
{
  pub fn new(listener: L, event: String, payload_match: String) -> Self {
    let token = CancellationToken::new();
    let cloned_token = token.clone();
    let event_id = listener.listen(event.clone(), move |e| {
      // println!("{} - {}", event, e.payload());
      let Ok(payload) = serde_json::from_str::<String>(e.payload()) else {
        return;
      };
      if payload == payload_match {
        // println!("Cancel!");
        cloned_token.cancel();
      }
    });
    Self {
      listener,
      event_id,
      cancel_token: token,
      runtime: PhantomData::<R>,
    }
  }
  #[allow(dead_code)]
  pub fn listener(&self) -> &L {
    &self.listener
  }
  pub fn token(&self) -> CancellationToken {
    self.cancel_token.clone()
  }
}

unsafe impl<L, R> Send for CancellationTokenListener<L, R>
where
  L: Listener<R> + Send,
  R: Runtime,
{
}

unsafe impl<L, R> Sync for CancellationTokenListener<L, R>
where
  L: Listener<R> + Sync,
  R: Runtime,
{
}
