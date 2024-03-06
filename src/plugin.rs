use crate::subscription::SubscriptionRequest;
mod builder;
pub use builder::Builder;

use async_graphql::{
  futures_util::StreamExt, BatchRequest, ObjectType, Request, Schema, SubscriptionType,
};
use serde_json::Value as JsonValue;
use std::sync::Arc;
use tauri::{
  plugin::{Plugin, Result},
  AppHandle, Invoke, InvokeError, Manager, PageLoadPayload, RunEvent, Runtime, Window,
};

pub(crate) type SetupHook<R, Q, M, S> =
  dyn FnOnce(&AppHandle<R>, JsonValue, &Schema<Q, M, S>) -> Result<()> + Send;
pub(crate) type OnWebviewReady<R> = dyn FnMut(Window<R>) + Send;
pub(crate) type OnEvent<R> = dyn FnMut(&AppHandle<R>, &RunEvent) + Send;
pub(crate) type OnPageLoad<R> = dyn FnMut(Window<R>, PageLoadPayload) + Send;
pub(crate) type OnDrop<R> = dyn FnOnce(AppHandle<R>) + Send;
pub(crate) type OnBatchRequest = dyn Fn(BatchRequest) -> BatchRequest + Send + Sync;
pub(crate) type OnSubRequst = dyn Fn(Request) -> Request + Send + Sync;

pub struct MizukiPlugin<R, Q, M, S>
where
  R: Runtime,
  Q: ObjectType + 'static,
  M: ObjectType + 'static,
  S: SubscriptionType + 'static,
{
  name: &'static str,
  app: Option<AppHandle<R>>,
  schema: Schema<Q, M, S>,
  setup: Option<Box<SetupHook<R, Q, M, S>>>,
  js_init_script: Option<String>,
  on_page_load: Box<OnPageLoad<R>>,
  on_webview_ready: Box<OnWebviewReady<R>>,
  on_event: Box<OnEvent<R>>,
  on_drop: Option<Box<OnDrop<R>>>,
  on_batch_request: Arc<Box<OnBatchRequest>>,
  on_sub_request: Arc<Box<OnSubRequst>>,
}

impl<R, Q, M, S> Drop for MizukiPlugin<R, Q, M, S>
where
  R: Runtime,
  Q: ObjectType + 'static,
  M: ObjectType + 'static,
  S: SubscriptionType + 'static,
{
  fn drop(&mut self) {
    if let (Some(on_drop), Some(app)) = (self.on_drop.take(), self.app.take()) {
      on_drop(app);
    }
  }
}

impl<R, Q, M, S> Plugin<R> for MizukiPlugin<R, Q, M, S>
where
  R: Runtime,
  Q: ObjectType + 'static,
  M: ObjectType + 'static,
  S: SubscriptionType + 'static,
{
  fn name(&self) -> &'static str {
    self.name
  }

  fn initialize(&mut self, app: &AppHandle<R>, config: JsonValue) -> Result<()> {
    let _ = config;
    self.app.replace(app.clone());
    if let Some(s) = self.setup.take() {
      (s)(app, config, &self.schema)?;
    }
    Ok(())
  }

  fn initialization_script(&self) -> Option<String> {
    self.js_init_script.clone()
  }

  fn created(&mut self, window: Window<R>) {
    (self.on_webview_ready)(window)
  }

  fn on_page_load(&mut self, window: Window<R>, payload: PageLoadPayload) {
    (self.on_page_load)(window, payload)
  }

  fn on_event(&mut self, app: &AppHandle<R>, event: &RunEvent) {
    (self.on_event)(app, event)
  }

  fn extend_api(&mut self, invoke: Invoke<R>) {
    let on_batch_request = self.on_batch_request.clone();
    let on_sub_request = self.on_sub_request.clone();
    let window = invoke.message.window();

    let schema = self.schema.clone();

    match invoke.message.command() {
      "graphql" => invoke.resolver.respond_async(async move {
        let req: BatchRequest = serde_json::from_value(invoke.message.payload().clone())
          .map_err(InvokeError::from_serde_json)?;

        let resp = schema
          .execute_batch((on_batch_request)(
            req.data(window.app_handle()).data(window),
          ))
          .await;

        let str = serde_json::to_string(&resp).map_err(InvokeError::from_serde_json)?;

        Ok((str, resp.is_ok()))
      }),
      "subscriptions" => invoke.resolver.respond_async(async move {
        let req: SubscriptionRequest = serde_json::from_value(invoke.message.payload().clone())
          .map_err(InvokeError::from_serde_json)?;

        let subscription_window = window.clone();
        let mut stream = schema.execute_stream((on_sub_request)(
          req.inner.data(window.app_handle()).data(window),
        ));

        let event_id = &format!("graphql://{}", req.id);
        while let Some(result) = stream.next().await {
          let str = serde_json::to_string(&result).map_err(InvokeError::from_serde_json)?;

          subscription_window.emit(event_id, str)?;
        }
        subscription_window.emit(event_id, Option::<()>::None)?;

        Ok(())
      }),
      cmd => invoke.resolver.reject(format!(
        "Invalid endpoint \"{}\". Valid endpoints are: \"graphql\", \"subscriptions\".",
        cmd
      )),
    }
  }
}
