use async_graphql::{BatchRequest, ObjectType, Request, Schema, SubscriptionType};
use serde_json::Value as JsonValue;
use std::sync::Arc;
use tauri::{plugin::Result, AppHandle, PageLoadPayload, RunEvent, Runtime, Window};

use super::{
  MizukiPlugin, OnBatchRequest, OnDrop, OnEvent, OnPageLoad, OnSubRequst, OnWebviewReady, SetupHook,
};

pub struct Builder<R, Q, M, S>
where
  R: Runtime,
  Q: ObjectType + 'static,
  M: ObjectType + 'static,
  S: SubscriptionType + 'static,
{
  name: &'static str,
  schema: Schema<Q, M, S>,
  setup: Option<Box<SetupHook<R, Q, M, S>>>,
  js_init_script: Option<String>,
  on_page_load: Box<OnPageLoad<R>>,
  on_webview_ready: Box<OnWebviewReady<R>>,
  on_event: Box<OnEvent<R>>,
  on_drop: Option<Box<OnDrop<R>>>,
  on_batch_request: Box<OnBatchRequest>,
  on_sub_request: Box<OnSubRequst>,
}

impl<R, Q, M, S> Builder<R, Q, M, S>
where
  R: Runtime,
  Q: ObjectType + 'static,
  M: ObjectType + 'static,
  S: SubscriptionType + 'static,
{
  pub fn new(name: &'static str, schema: Schema<Q, M, S>) -> Self {
    Self {
      name,
      schema,
      on_batch_request: Box::new(|r| r),
      on_sub_request: Box::new(|s| s),
      setup: None,
      js_init_script: None,
      on_page_load: Box::new(|_, _| ()),
      on_webview_ready: Box::new(|_| ()),
      on_event: Box::new(|_, _| ()),
      on_drop: None,
    }
  }

  #[must_use]
  pub fn js_init_script(mut self, js_init_script: String) -> Self {
    self.js_init_script = Some(js_init_script);
    self
  }

  #[must_use]
  pub fn setup<F>(mut self, setup: F) -> Self
  where
    F: FnOnce(&AppHandle<R>, JsonValue, &Schema<Q, M, S>) -> Result<()> + Send + 'static,
  {
    self.setup.replace(Box::new(setup));
    self
  }

  #[must_use]
  pub fn on_page_load<F>(mut self, on_page_load: F) -> Self
  where
    F: FnMut(Window<R>, PageLoadPayload) + Send + 'static,
  {
    self.on_page_load = Box::new(on_page_load);
    self
  }

  #[must_use]
  pub fn on_webview_ready<F>(mut self, on_webview_ready: F) -> Self
  where
    F: FnMut(Window<R>) + Send + 'static,
  {
    self.on_webview_ready = Box::new(on_webview_ready);
    self
  }

  #[must_use]
  pub fn on_event<F>(mut self, on_event: F) -> Self
  where
    F: FnMut(&AppHandle<R>, &RunEvent) + Send + 'static,
  {
    self.on_event = Box::new(on_event);
    self
  }

  #[must_use]
  pub fn on_drop<F>(mut self, on_drop: F) -> Self
  where
    F: FnOnce(AppHandle<R>) + Send + 'static,
  {
    self.on_drop.replace(Box::new(on_drop));
    self
  }

  #[must_use]
  pub fn on_batch_request<F>(mut self, on_batch_request: F) -> Self
  where
    F: Fn(BatchRequest) -> BatchRequest + Send + Sync + 'static,
  {
    self.on_batch_request = Box::new(on_batch_request);
    self
  }

  #[must_use]
  pub fn on_sub_request<F>(mut self, on_sub_request: F) -> Self
  where
    F: Fn(Request) -> Request + Send + Sync + 'static,
  {
    self.on_sub_request = Box::new(on_sub_request);
    self
  }
  pub fn build(self) -> MizukiPlugin<R, Q, M, S> {
    MizukiPlugin {
      name: self.name,
      app: None,
      schema: self.schema,
      setup: self.setup,
      js_init_script: self.js_init_script,
      on_page_load: self.on_page_load,
      on_webview_ready: self.on_webview_ready,
      on_event: self.on_event,
      on_drop: self.on_drop,
      on_batch_request: Arc::new(self.on_batch_request),
      on_sub_request: Arc::new(self.on_sub_request),
    }
  }
}
