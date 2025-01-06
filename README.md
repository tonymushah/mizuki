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

## Install

### Rust

```toml
[dependencies]
mizuki = "1.0.0"
```

### JavaScript

The only client-side adapter currently is `@mizuki/urql`, a custom exchange for [`urql`].
If you need adapters for other GraphQL clients, open a PR!

| Package                       | Version (click for changelogs) |
|-------------------------------|--------------------------------|
| [`mizuki-urql-adapter`] | [![urql adapter version][urql-adapter-version-badge]][urql-adapter-url] |
| [`mizuki-apollo-link`] | [![apollo link version][apollo-link-version-badge]] |

## Usage

With the introduction of [plugin command permissions][tauri-plugin-permission] in Tauri v2, you need your Mizuki plugin into a separate library crate. If you already have, then congrats!

First, add those dependencies to the plugin `Cargo.toml`:

```toml 

```

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

fn init_plugin<R: tauri::Runtime>() -> mizuki::MizukiPlugin<R, Query, EmptyMutation, EmptySubscription> {
    mizuki::Builder::new("todo-plugin", Schema::new(
        Query,
        EmptyMutation,
        EmptySubscription,
    )).build()
}

fn main() {
    tauri::Builder::default()
        // The plugin name is required
        .plugin(init_plugin())
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
```

## Contributing

If you want to help out, there are a few areas that need improvement:

- **Client Adapters** - Currently, only a urql adapter exists; having adapters for more client libraries would be very nice.

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
