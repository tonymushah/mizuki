use async_graphql::{BatchRequest, ObjectType, Request, Schema, SubscriptionType};
use serde_json::Value as JsonValue;
use std::sync::Arc;
use tauri::{webview::PageLoadPayload, AppHandle, RunEvent, Runtime, Url, Webview, Window};

use super::{
  MizukiPlugin, OnBatchRequest, OnDrop, OnEvent, OnNavigation, OnPageLoad, OnSubRequst,
  OnWebviewReady, OnWindowReady, SetupHook,
};

///
/// This [`self::Builder`] struct share the same build function as [`tauri::plugin::Builder`]
///
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
  on_window_ready: Box<OnWindowReady<R>>,
  on_navigation: Box<OnNavigation<R>>,
  auto_cancel: bool,
  sub_event_label: String,
}

impl<R, Q, M, S> Builder<R, Q, M, S>
where
  R: Runtime,
  Q: ObjectType + 'static,
  M: ObjectType + 'static,
  S: SubscriptionType + 'static,
{
  /// Creates a new Plugin builder.
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
      on_window_ready: Box::new(|_| ()),
      on_navigation: Box::new(|_, _| true),
      auto_cancel: true,
      sub_event_label: "sub_end".into(),
    }
  }
  /// Same as [`tauri::plugin::Builder::js_init_script`]
  ///
  /// Sets the provided JavaScript to be run after the global object has been created,
  /// but before the HTML document has been parsed and before any other script included by the HTML document is run.
  ///
  /// Since it runs on all top-level document and child frame page navigations,
  /// it's recommended to check the `window.location` to guard your script from running on unexpected origins.
  ///
  /// The script is wrapped into its own context with `(function () { /* your script here */ })();`,
  /// so global variables must be assigned to `window` instead of implicitly declared.
  ///
  /// Note that calling this function multiple times overrides previous values.
  ///
  #[must_use]
  pub fn js_init_script(mut self, js_init_script: String) -> Self {
    self.js_init_script = Some(js_init_script);
    self
  }

  #[must_use]
  pub fn setup<F>(mut self, setup: F) -> Self
  where
    F: FnOnce(&AppHandle<R>, JsonValue, &Schema<Q, M, S>) -> Result<(), Box<dyn std::error::Error>>
      + Send
      + 'static,
  {
    self.setup.replace(Box::new(setup));
    self
  }
  /// Callback invoked when the webview performs a navigation to a page.
  /// Same as [`tauri::plugin::Builder::on_page_load`]
  #[must_use]
  pub fn on_page_load<F>(mut self, on_page_load: F) -> Self
  where
    F: FnMut(&Webview<R>, &PageLoadPayload<'_>) + Send + 'static,
  {
    self.on_page_load = Box::new(on_page_load);
    self
  }

  /// Same as [`tauri::plugin::Builder::on_webview_ready`]
  #[must_use]
  pub fn on_webview_ready<F>(mut self, on_webview_ready: F) -> Self
  where
    F: FnMut(Webview<R>) + Send + 'static,
  {
    self.on_webview_ready = Box::new(on_webview_ready);
    self
  }

  /// Same as [`tauri::plugin::Builder::on_event`]
  #[must_use]
  pub fn on_event<F>(mut self, on_event: F) -> Self
  where
    F: FnMut(&AppHandle<R>, &RunEvent) + Send + 'static,
  {
    self.on_event = Box::new(on_event);
    self
  }

  /// Same as [`tauri::plugin::Builder::on_drop`]
  #[must_use]
  pub fn on_drop<F>(mut self, on_drop: F) -> Self
  where
    F: FnOnce(AppHandle<R>) + Send + 'static,
  {
    self.on_drop.replace(Box::new(on_drop));
    self
  }

  /// Register a callback when a batch_request is invoked
  /// Might be useful if you want a request cache system
  #[must_use]
  pub fn on_batch_request<F>(mut self, on_batch_request: F) -> Self
  where
    F: Fn(BatchRequest) -> BatchRequest + Send + Sync + 'static,
  {
    self.on_batch_request = Box::new(on_batch_request);
    self
  }

  /// Register a callback when a subscription request is invoked
  /// Might be useful if you want a request cache system
  #[must_use]
  pub fn on_sub_request<F>(mut self, on_sub_request: F) -> Self
  where
    F: Fn(Request) -> Request + Send + Sync + 'static,
  {
    self.on_sub_request = Box::new(on_sub_request);
    self
  }
  /// Similar to [`tauri::plugin::Builder::on_navigation`]
  #[must_use]
  pub fn on_navigation<F>(mut self, on_navigation: F) -> Self
  where
    F: Fn(&Webview<R>, &Url) -> bool + Send + 'static,
  {
    self.on_navigation = Box::new(on_navigation);
    self
  }
  /// Similar to [`tauri::plugin::Builder::on_window_ready`]
  #[must_use]
  pub fn on_window_ready<F>(mut self, on_window_ready: F) -> Self
  where
    F: FnMut(Window<R>) + Send + 'static,
  {
    self.on_window_ready = Box::new(on_window_ready);
    self
  }
  /// Prevent the plugin for canceling subscription internally.
  /// But the subscription will still be cancelled if the window is reloaded or destroyed.
  #[must_use]
  pub fn auto_cancel(mut self, auto_cancel: bool) -> Self {
    self.auto_cancel = auto_cancel;
    self
  }

  /// Modify the subscription cancel event label
  ///
  /// Default: sub_event
  pub fn subscription_end_label(mut self, label: String) -> Self {
    self.sub_event_label = label;
    self
  }
  /// Build the [`crate::MizukiPlugin`]
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
      on_navigation: self.on_navigation,
      on_window_ready: self.on_window_ready,
      auto_cancel: self.auto_cancel,
      sub_end_event_label: self.sub_event_label,
    }
  }
}
