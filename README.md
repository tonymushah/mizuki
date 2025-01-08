# Mizuki

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/mizuki.svg
[crates-url]: https://crates.io/crates/mizuki
[docs-badge]: https://img.shields.io/docsrs/mizuki.svg
[docs-url]: https://docs.rs/mizuki
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE

A toolkit for building Tauri plugins that enables type-safe IPC through GraphQL.

## Notice

This project is a fork from [JonasKruckenberg/tauri-plugin-graphql][initial-repo].

But I thought that it would be a great a idea to push the plugin futher
and create a toolkit for building GraphQL Tauri Plugins.

## Usage

### Rust

With the introduction of [plugin command permissions][tauri-plugin-permission] in Tauri v2, you need your Mizuki plugin into a separate library crate. If you already have, then congrats!

1. modify the plugin `Cargo.toml`:

```toml
[package]
name = "my-todo-plugin"
version = "0.1.0"
edition = "2021"
## your plugin name
## Required or your plugin might not build
links = "todo-plugin"

[build-dependencies]
mizuki-build = "1"
## Other of your plugin build dependencies

[dependencies]
mizuki = "1"
async-graphql = "7"
tauri = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
## Other of your plugin dependecies
```

2. Make a `build.rs` file on your plugin root directory (Same as `Cargo.toml`):

```rust
fn main() {
    mizuki_build::build()
}
```

It will automaticly generate the required permission schemas.

3. Write your plugin code (mainly your lib.rs):

You need to register the plugin giving it a [`async_graphql::Schema`]. This schema will be used to fulfill requests.

```rust
use async_graphql::{Schema, Object, EmptySubscription, EmptyMutation, Result as GraphQLResult, SimpleObject};

#[derive(SimpleObject, Debug, Clone)]
struct ListItem {
    id: i32,
    text: String
}

impl ListItem {
    pub fn new(text: String) -> Self {
        Self {
            id: rand::random::<i32>(),
            text
        }
    }
}

struct Query;

#[Object]
impl Query {
    async fn list(&self) -> GraphQLResult<Vec<ListItem>> {
        let item = vec![
            ListItem::new("foo".to_string()),
            ListItem::new("bar".to_string())
        ];

        Ok(item)
    }
}

pub fn init_plugin<R: tauri::Runtime>() -> mizuki::MizukiPlugin<R, Query, EmptyMutation, EmptySubscription> {
    mizuki::Builder::new("todo-plugin", Schema::new(
        Query,
        EmptyMutation,
        EmptySubscription,
    )).build()
}
```

4. Add your plugin to main app `Cargo.toml`

5. Register the plugin:

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // The plugin name is required
        .plugin(my_todo_plugin::init_plugin())
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
```

6. Enable the `core:event:default`, `<your-plugin>:allow-graphql`, `<your-plugin>:allow-subscriptions` in your tauri app `capabilities`:

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Capability for the main window",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "core:event:default",
    "todo-plugin:allow-graphql",
    "todo-plugin:allow-subscriptions"
  ]
}    
```

`core:event:default` is required for subscriptions.

### JavaScript

The only client-side adapter currently are:

- `mizuki-urql-adapter`, a custom exchange for [`urql`]
- `mizuki-apollo-link`, a custom link for [`apollo`]

If you need adapters for other GraphQL clients, open a PR!

| Package                       | Version (click for changelogs) |
|-------------------------------|--------------------------------|
| [`mizuki-urql-adapter`] | [![urql adapter version][urql-adapter-version-badge]][urql-adapter-url] |
| [`mizuki-apollo-link`] | [![apollo link version][apollo-link-version-badge]] |

#### `mizuki-urql-adapter` usage

1. Install the `mizuki-urql-adapter` npm package with the package manager of your choice (mine is pnpm):

```sh
pnpm install mizuki-urql-adapter
```

2. Intialize the client:

```ts
import { Client } from "@urql/core" // or whatever urql framework packages
import { getExchanges } from "mizuki-urql-adapter"

const client = new Client({
    // Not required but needed if you want to some releive Typescript errors
    url: "",
    exchanges: [...getExchanges("todo-plugin")]
})
```

3. Use the client anyway you need it.

#### `mizuki-apollo-link` usage

1. Install the `mizuki-apollo-link` npm package with the package manager of your choice (mine is pnpm):

```sh
pnpm add mizuki-apollo-link
```

2. Intialize the client:

```ts
import { ApolloClient, InMemoryCache } from "@apollo/client/core"; // or @apollo/client if you want to
import { MizukiLink } from "mizuki-apollo-link";

const client = new ApolloClient({
    link: new MizukiLink("todo-plugin"),
    cache: new InMemoryCache()
});
```

## Contributing

PRs are welcome!

## License

[MIT © Tony Mushah](./LICENSE)

[`mizuki-urql-adapter`]: packages/urql
[urql-adapter-version-badge]: https://img.shields.io/npm/v/mizuki-urql-adapter?label=%20
[urql-adapter-url]: https://www.npmjs.com/package/mizuki-urql-adapter
[`urql`]: https://formidable.com/open-source/urql/
[`async_graphql::Schema`]: https://docs.rs/async-graphql/latest/async_graphql/struct.Schema.html
[initial-repo]: https://github.com/JonasKruckenberg/tauri-plugin-graphql
[`mizuki-apollo-link`]: packages/apollo
[apollo-link-version-badge]: https://img.shields.io/npm/v/mizuki-apollo-link?label=%20
[tauri-plugin-permission]: https://tauri.app/develop/plugins/#command-permissions
[`apollo`]: https://www.apollographql.com/
