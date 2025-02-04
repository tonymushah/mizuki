use crate::{cancel_token::CancellationTokenListener, subscription::SubscriptionRequest};
mod builder;
pub use builder::{Builder, BuilderError};

use async_graphql::{
  futures_util::StreamExt, BatchRequest, ObjectType, Request, Schema, SubscriptionType,
};
use serde::{de::IntoDeserializer, Deserialize};
use serde_json::Value as JsonValue;
use std::sync::Arc;
use tauri::{
  ipc::{Invoke, InvokeError},
  plugin::Plugin,
  webview::PageLoadPayload,
  AppHandle, Emitter, EventTarget, Manager, RunEvent, Runtime, Url, Webview, Window, WindowEvent,
};

pub(crate) type SetupHook<R, Q, M, S> = dyn FnOnce(&AppHandle<R>, JsonValue, &Schema<Q, M, S>) -> Result<(), Box<dyn std::error::Error>>
  + Send;
pub(crate) type OnWebviewReady<R> = dyn FnMut(Webview<R>) + Send;
pub(crate) type OnEvent<R> = dyn FnMut(&AppHandle<R>, &RunEvent) + Send;
pub(crate) type OnPageLoad<R> = dyn FnMut(&Webview<R>, &PageLoadPayload<'_>) + Send;
pub(crate) type OnDrop<R> = dyn FnOnce(AppHandle<R>) + Send;
pub(crate) type OnBatchRequest = dyn Fn(BatchRequest) -> BatchRequest + Send + Sync;
pub(crate) type OnSubRequst = dyn Fn(Request) -> Request + Send + Sync;
pub(crate) type OnWindowReady<R> = dyn FnMut(Window<R>) + Send;
pub(crate) type OnNavigation<R> = dyn Fn(&Webview<R>, &Url) -> bool + Send;

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
  on_window_ready: Box<OnWindowReady<R>>,
  on_navigation: Box<OnNavigation<R>>,
  auto_cancel: bool,
  sub_end_event_label: String,
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

  fn initialize(
    &mut self,
    app: &AppHandle<R>,
    config: JsonValue,
  ) -> Result<(), Box<dyn std::error::Error>> {
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

  fn on_page_load(&mut self, window: &Webview<R>, payload: &PageLoadPayload<'_>) {
    (self.on_page_load)(window, payload)
  }

  fn on_event(&mut self, app: &AppHandle<R>, event: &RunEvent) {
    (self.on_event)(app, event)
  }

  fn extend_api(&mut self, invoke: Invoke<R>) -> bool {
    let on_batch_request = self.on_batch_request.clone();
    let on_sub_request = self.on_sub_request.clone();
    let sub_end_event_label = self.sub_end_event_label.clone();
    let auto_cancel = self.auto_cancel;

    let schema = self.schema.clone();

    match invoke.message.command() {
      "graphql" => invoke.resolver.respond_async(async move {
        let req: BatchRequest = match invoke.message.payload() {
          tauri::ipc::InvokeBody::Json(value) => {
            serde_json::from_value(value.clone()).map_err(InvokeError::from_error)?
          }
          tauri::ipc::InvokeBody::Raw(vec) => {
            Deserialize::deserialize(vec.clone().into_deserializer())
              .map_err(|e: serde_json::Error| InvokeError::from_error(e))?
          }
        };

        let resp = schema
          .execute_batch((on_batch_request)(
            req
              .data(invoke.message.webview().app_handle().clone())
              .data(invoke.message.webview())
              .data(invoke.message.webview().window()),
          ))
          .await;

        let str = serde_json::to_string(&resp).map_err(InvokeError::from_error)?;

        Ok((str, resp.is_ok()))
      }),
      "subscriptions" => invoke.resolver.respond_async(async move {
        let window = invoke.message.webview();
        let req: SubscriptionRequest = match invoke.message.payload() {
          tauri::ipc::InvokeBody::Json(value) => {
            serde_json::from_value(value.clone()).map_err(InvokeError::from_error)?
          }
          tauri::ipc::InvokeBody::Raw(vec) => {
            Deserialize::deserialize(vec.clone().into_deserializer())
              .map_err(|e: serde_json::Error| InvokeError::from_error(e))?
          }
        };

        let subscription_window = window.clone();
        let webwiew_cancel_token = CancellationTokenListener::new(
          subscription_window.clone(),
          sub_end_event_label,
          req.sub_id.clone(),
        );
        let cancel_token = webwiew_cancel_token.token();
        let mut stream = schema.execute_stream((on_sub_request)(
          req
            .inner
            .data(invoke.message.webview().app_handle().clone())
            .data(invoke.message.webview())
            .data(invoke.message.webview().window())
            .data(webwiew_cancel_token.token().clone()),
        ));

        {
          let cancel_token = cancel_token.clone();
          subscription_window.window().on_window_event(move |event| {
            if let WindowEvent::Destroyed = event {
              cancel_token.cancel();
            }
          });
        }

        let event_id = &format!("graphql://{}", req.id);
        if auto_cancel {
          loop {
            tokio::select! {
              _ = cancel_token.cancelled() => {
                // println!("end cancelled");
                break;
              },
              res = stream.next() => {
                if let Some(result) = res {
                  let str = serde_json::to_string(&result).map_err(InvokeError::from_error)?;

                  subscription_window.emit_to(EventTarget::Webview{label: subscription_window.label().into()},event_id, str)?;
                }else {
                  // println!("end stream");
                  break;
                }
              },
              else => {
                // println!("end!");
                break;
              }
            }
          }
        } else {
          while let Some(result) = stream.next().await {
            let str = serde_json::to_string(&result).map_err(InvokeError::from_error)?;

            subscription_window.emit_to(EventTarget::Webview{label: subscription_window.label().into()},event_id, str)?;
          }
        }

        subscription_window.emit_to(EventTarget::Webview{label: subscription_window.label().into()}, event_id, Option::<()>::None)?;

        Ok(())
      }),
      cmd => invoke.resolver.reject(format!(
        "Invalid endpoint \"{}\". Valid endpoints are: \"graphql\", \"subscriptions\".",
        cmd
      )),
    }
    true
  }

  fn window_created(&mut self, window: Window<R>) {
    (self.on_window_ready)(window)
  }

  fn webview_created(&mut self, webview: tauri::Webview<R>) {
    (self.on_webview_ready)(webview)
  }

  fn on_navigation(&mut self, webview: &tauri::Webview<R>, url: &tauri::Url) -> bool {
    (self.on_navigation)(webview, url)
  }
}
