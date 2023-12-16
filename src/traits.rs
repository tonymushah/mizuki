use std::{path::PathBuf, io::{BufWriter, Write}, fs::File};

use crate::subscription::SubscriptionRequest;
use async_graphql::{futures_util::StreamExt, BatchRequest, ObjectType, Schema, SubscriptionType};
use tauri::{
  plugin::Plugin, InvokeError, Manager, Runtime,
};

pub trait MizukiPluginTrait<R, Query, Mutation, Subscription>: Plugin<R>
where
  Query: ObjectType + 'static,
  Mutation: ObjectType + 'static,
  Subscription: SubscriptionType + 'static,
  R: Runtime,
{
  fn schema(&self) -> Schema<Query, Mutation, Subscription>;
  fn sdl(&self) -> String {
    self.schema().sdl()
  }
  /// Export the schema to a new file
  fn export_sdl(&self, path: PathBuf) -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create(path)?);
    file.write_all(self.sdl().as_bytes())
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
}