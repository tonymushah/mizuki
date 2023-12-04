// Copyright 2022 Jonas Kruckenberg
// SPDX-License-Identifier: MIT

//! This crate contains a Tauri plugin used to expose a [`async_graphql`]
//! GraphQL endpoint through Tauri's IPC system. This plugin can be used as
//! safer alternative to Tauri's existing Command API since both the Rust and
//! JavaScript side of the interface can be generated from a common schema.
//!
//! ## Rationale
//!
//! Especially in bigger projects that have specialized teams for the Frontend
//! and Rust core the existing command API falls short of being an optimal
//! solution. The Frontend is tightly coupled through `invoke()` calls to
//! backend commands, but there is no type-safety to alert Frontend developers
//! to changes in command signatures. This results in a very brittle interface
//! where changes on the Rust side will inadvertently break code in the
//! Frontend. This problem is similar exiting REST APIs, where the absence of a
//! formal contract between the server and the frontend makes future changes
//! very difficult.
//!
//! We can employ the same techniques used in traditional web development and
//! use shared schema that governs which types, methods, etc. are
//! available. GraphQL is such a schema language.
//!
//! ## Examples
//!
//! For the following examples, it is assumed you are familiar with [`Tauri
//! Commands`][`Commands`], [`Events`] and [`GraphQL`].
//!
//! ### Queries
//!
//! An example app that implements a very simple read-only todo-app using
//! GraphQL:
//!
//! ```rust
//! use async_graphql::{Schema, EmptySubscription, EmptyMutation, Object, SimpleObject, Result as GraphQLResult};
//!
//! #[derive(SimpleObject, Debug, Clone)]
//! struct ListItem {
//!     id: i32,
//!     text: String
//! }
//!
//! impl ListItem {
//!     pub fn new(text: String) -> Self {
//!         Self {
//!             id: rand::random::<i32>(),
//!             text
//!         }
//!     }
//! }
//!
//! struct Query;
//!
//! #[Object]
//! impl Query {
//!     async fn list(&self) -> GraphQLResult<Vec<ListItem>> {
//!         let item = vec![
//!             ListItem::new("foo".to_string()),
//!             ListItem::new("bar".to_string())
//!         ];
//!
//!         Ok(item)
//!     }
//! }
//!
//! let schema = Schema::new(
//!     Query,
//!     EmptyMutation,
//!     EmptySubscription,
//! );
//!
//! tauri::Builder::default()
//!     .plugin(tauri_plugin_graphql::init(schema));
//! ```
//!
//! ### Mutations
//!
//! GraphQL mutations provide a way to update or create state in the Core.
//!
//! Similarly to queries, mutations have access to a context object and can
//! manipulate windows, menus or global state.
//!
//! ```rust
//! use async_graphql::{Schema, Object, Context, EmptySubscription, EmptyMutation, SimpleObject, Result as GraphQLObject};
//! use tauri::{AppHandle, Manager};
//! use std::sync::Mutex;
//!
//! #[derive(Debug, Default)]
//! struct List(Mutex<Vec<ListItem>>);
//!
//! #[derive(SimpleObject, Debug, Clone)]
//! struct ListItem {
//!     id: i32,
//!     text: String
//! }
//!
//! impl ListItem {
//!     pub fn new(text: String) -> Self {
//!         Self {
//!             id: rand::random::<i32>(),
//!             text
//!         }
//!     }
//! }
//!
//! struct Query;
//!
//! #[Object]
//! impl Query {
//!     async fn list(&self, ctx: &Context<'_>) -> GraphQLObject<Vec<ListItem>> {
//!       let app = ctx.data::<AppHandle>().unwrap();
//!
//!       let list = app.state::<List>();
//!       let list = list.0.lock().unwrap();
//!         
//!       let items = list.iter().cloned().collect::<Vec<_>>();
//!
//!       Ok(items)
//!     }
//! }
//!
//! struct Mutation;
//!
//! #[Object]
//! impl Mutation {
//!   async fn add_entry(&self, ctx: &Context<'_>, text: String) -> GraphQLObject<ListItem> {
//!     let app = ctx.data::<AppHandle>().unwrap();
//!
//!     let list = app.state::<List>();
//!     let mut list = list.0.lock().unwrap();
//!
//!     let item = ListItem::new(text);
//!
//!     list.push(item.clone());
//!
//!     Ok(item)
//!   }
//! }
//!
//! let schema = Schema::new(
//!     Query,
//!     Mutation,
//!     EmptySubscription,
//! );
//!
//! tauri::Builder::default()
//!     .plugin(tauri_plugin_graphql::init(schema))
//!     .setup(|app| {
//!       app.manage(List::default());
//!
//!       Ok(())
//!     });
//! ```
//!
//! ### Subscriptions
//!
//! GraphQL subscriptions are a way to push real-time data to the Frontend.
//! Similarly to queries, a client can request a set of fields, but instead of
//! immediately returning a single answer, a new result is sent to the Frontend
//! every time the Core sends one.
//!
//! Subscription resolvers should be async and must return a [`Stream`].
//!
//! ```rust
//! use async_graphql::{
//!   futures_util::{self, stream::Stream},
//!   Schema, Object, Subscription, EmptySubscription,
//!   EmptyMutation, SimpleObject, Result as GraphQLResult
//! };
//!
//! struct Query;
//!
//! #[Object]
//! impl Query {
//!   async fn hello_world(&self) -> GraphQLResult<&str> {
//!     Ok("Hello World!")
//!   }
//! }
//!
//! struct Subscription;
//!
//! #[Subscription]
//! impl Subscription {
//!   async fn hello_world(&self) -> impl Stream<Item = &str> {
//!     futures_util::stream::iter(vec!["Hello", "World!"])
//!   }
//! }
//!
//! let schema = Schema::new(
//!   Query,
//!   EmptyMutation,
//!   Subscription,
//! );
//!
//! tauri::Builder::default()
//!   .plugin(tauri_plugin_graphql::init(schema));
//! ```
//!
//! ## Stability
//!
//! To work around limitations with the current command system, this plugin
//! directly implements an invoke handler instead of reyling on the
//! [`tauri::generate_handler`] macro.
//! Since the invoke handler implementation is not considered stable and might
//! change between releases **this plugin has no backwards compatibility
//! guarantees**.
//!
//! [`Stream`]: https://docs.rs/futures-util/latest/futures_util/stream/trait.Stream.html
//! [`Commands`]: https://tauri.studio/docs/guides/command
//! [`Events`]: https://tauri.studio/docs/guides/events
//! [`GraphQL`]: https://graphql.org

mod subscription;

use subscription::SubscriptionRequest;

pub use async_graphql;
use async_graphql::{
  futures_util::StreamExt, BatchRequest, ObjectType, Schema, SubscriptionType,
};
use std::{any::Any, io::{BufWriter, Write}, fs::File, path::Path};
use tauri::{
  plugin::Plugin,
  Invoke, InvokeError, Manager, Runtime,
};

pub struct Mizuki<D, Query, Mutation, Subscription>
where
  Query: ObjectType + 'static,
  Mutation: ObjectType + 'static,
  Subscription: SubscriptionType + 'static,
  D: Any + Clone + Send + Sync,
{
  name: &'static str,
  schema: Schema<Query, Mutation, Subscription>,
  context: Option<D>,
}

impl<Query, Mutation, Subscription, D> Mizuki<D, Query, Mutation, Subscription>
where
  Query: ObjectType + 'static,
  Mutation: ObjectType + 'static,
  Subscription: SubscriptionType + 'static,
  D: Any + Clone + Send + Sync,
{
  pub fn new_with_context(
    name: &'static str,
    schema: Schema<Query, Mutation, Subscription>,
    context: D,
  ) -> Self {
    Self {
      name,
      schema,
      context: Some(context),
    }
  }
  pub fn sdl(&self) -> String {
    self.schema.sdl()
  }
  pub fn export_sdl<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create(path)?);
    file.write_all(self.sdl().as_bytes())
  }
}

impl<Query, Mutation, Subscription> Mizuki<(), Query, Mutation, Subscription>
where
  Query: ObjectType + 'static,
  Mutation: ObjectType + 'static,
  Subscription: SubscriptionType + 'static,
{
  pub fn new(name: &'static str, schema: Schema<Query, Mutation, Subscription>) -> Self {
    Self {
      name,
      schema,
      context: None::<()>,
    }
  }
}

impl<R, Query, Mutation, Subscription, D> Plugin<R> for Mizuki<D, Query, Mutation, Subscription>
where
  R: Runtime,
  Query: ObjectType + 'static,
  Mutation: ObjectType + 'static,
  Subscription: SubscriptionType + 'static,
  D: Any + Clone + Send + Sync,
{
  fn name(&self) -> &'static str {
    self.name
  }
  fn extend_api(&mut self, invoke: Invoke<R>) {
    let context = self.context.clone();
    let window = invoke.message.window();

    let schema = self.schema.clone();

    match invoke.message.command() {
      "graphql" => invoke.resolver.respond_async(async move {
        let req: BatchRequest = serde_json::from_value(invoke.message.payload().clone())
          .map_err(InvokeError::from_serde_json)?;

        let resp = if let Some(data) = context {
          schema
            .execute_batch(
              req
                .data(window.app_handle())
                .data(window)
                .data(data.clone()),
            )
            .await
        } else {
          schema
            .execute_batch(req.data(window.app_handle()).data(window))
            .await
        };

        let str = serde_json::to_string(&resp).map_err(InvokeError::from_serde_json)?;

        Ok((str, resp.is_ok()))
      }),
      "subscriptions" => invoke.resolver.respond_async(async move {
        let req: SubscriptionRequest = serde_json::from_value(invoke.message.payload().clone())
          .map_err(InvokeError::from_serde_json)?;

        let subscription_window = window.clone();
        let mut stream = if let Some(data) = context {
          schema.execute_stream(req.inner.data(window.app_handle()).data(window).data(data))
        } else {
          schema.execute_stream(req.inner.data(window.app_handle()).data(window))
        };

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