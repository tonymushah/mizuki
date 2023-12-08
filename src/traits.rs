use std::{path::PathBuf, io::{BufWriter, Write}, fs::File};

use crate::subscription::SubscriptionRequest;
use async_graphql::{futures_util::StreamExt, BatchRequest, ObjectType, Schema, SubscriptionType};
use tauri::{
  plugin::Plugin, AppHandle, InvokeError, Manager, PageLoadPayload, RunEvent, Runtime, Window,
};

pub trait MizukiPluginTrait<R, Query, Mutation, Subscription>: Send
where
  Query: ObjectType + 'static,
  Mutation: ObjectType + 'static,
  Subscription: SubscriptionType + 'static,
  R: Runtime,
{
  // Your plugin name
  fn name(&self) -> &'static str;
  fn schema(&self) -> Schema<Query, Mutation, Subscription>;
  fn sdl(&self) -> String {
    self.schema().sdl()
  }
  /// Export the schema to a new file
  fn export_sdl(&self, path: PathBuf) -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create(path)?);
    file.write_all(self.sdl().as_bytes())
  }
  #[allow(unused_variables)]
  fn initialize(
    &mut self,
    app: &AppHandle<R>,
    config: serde_json::Value,
  ) -> tauri::plugin::Result<()> {
    Ok(())
  }

  /// Add the provided JavaScript to a list of scripts that should be run after the global object has been created,
  /// but before the HTML document has been parsed and before any other script included by the HTML document is run.
  ///
  /// Since it runs on all top-level document and child frame page navigations,
  /// it's recommended to check the `window.location` to guard your script from running on unexpected origins.
  ///
  /// The script is wrapped into its own context with `(function () { /* your script here */ })();`,
  /// so global variables must be assigned to `window` instead of implicitly declared.
  fn initialization_script(&self) -> Option<String> {
    None
  }

  /// Callback invoked when the webview is created.
  #[allow(unused_variables)]
  fn created(&mut self, window: Window<R>) {}

  /// Callback invoked when the webview performs a navigation to a page.
  #[allow(unused_variables)]
  fn on_page_load(&mut self, window: Window<R>, payload: PageLoadPayload) {}

  /// Callback invoked when the event loop receives a new event.
  #[allow(unused_variables)]
  fn on_event(&mut self, app: &AppHandle<R>, event: &RunEvent) {}
}

impl<R, Query, Mutation, Subscription> Plugin<R>
  for dyn MizukiPluginTrait<R, Query, Mutation, Subscription>
where
  R: Runtime,
  Query: ObjectType + 'static,
  Mutation: ObjectType + 'static,
  Subscription: SubscriptionType + 'static,
{
  fn name(&self) -> &'static str {
    <Self as MizukiPluginTrait<R, Query, Mutation, Subscription>>::name(self)
  }
  fn extend_api(&mut self, invoke: tauri::Invoke<R>) {
    let window = invoke.message.window();

    let schema = self.schema().clone();

    match invoke.message.command() {
      "graphql" => invoke.resolver.respond_async(async move {
        let req: BatchRequest = serde_json::from_value(invoke.message.payload().clone())
          .map_err(InvokeError::from_serde_json)?;

        let resp = schema
          .execute_batch(req.data(window.app_handle()).data(window))
          .await;

        let str = serde_json::to_string(&resp).map_err(InvokeError::from_serde_json)?;

        Ok((str, resp.is_ok()))
      }),
      "subscriptions" => invoke.resolver.respond_async(async move {
        let req: SubscriptionRequest = serde_json::from_value(invoke.message.payload().clone())
          .map_err(InvokeError::from_serde_json)?;

        let subscription_window = window.clone();
        let mut stream = schema.execute_stream(req.inner.data(window.app_handle()).data(window));

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
  fn created(&mut self, window: tauri::Window<R>) {
    <Self as MizukiPluginTrait<R, Query, Mutation, Subscription>>::created(self, window);
  }
  fn initialization_script(&self) -> Option<String> {
    <Self as MizukiPluginTrait<R, Query, Mutation, Subscription>>::initialization_script(self)
  }
  fn initialize(
    &mut self,
    app: &tauri::AppHandle<R>,
    config: serde_json::Value,
  ) -> tauri::plugin::Result<()> {
    <Self as MizukiPluginTrait<R, Query, Mutation, Subscription>>::initialize(self, app, config)
  }
  fn on_event(&mut self, app: &tauri::AppHandle<R>, event: &tauri::RunEvent) {
    <Self as MizukiPluginTrait<R, Query, Mutation, Subscription>>::on_event(self, app, event)
  }
  fn on_page_load(&mut self, window: tauri::Window<R>, payload: tauri::PageLoadPayload) {
    <Self as MizukiPluginTrait<R, Query, Mutation, Subscription>>::on_page_load(
      self, window, payload,
    )
  }
}
