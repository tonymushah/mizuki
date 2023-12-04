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
mizuki = "0.1.0"
```

### JavaScript

The only client-side adapter currently is `@mizuki/urql`, a custom exchange for [`urql`].
If you need adapters for other GraphQL clients, open a PR!

| Package                       | Version (click for changelogs) |
|-------------------------------|--------------------------------|
| [`@mizuki/urql`] | [![urql adapter version][urql-adapter-version-badge]][urql-adapter-changelog]

## Usage

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

fn main() {
    let schema = Schema::new(
        Query,
        EmptyMutation,
        EmptySubscription,
    );

    tauri::Builder::default()
        // The plugin name is required
        .plugin(mizuki::MizukiPlugin::new("todo-plugin", schema))
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

[`@mizuki/urql`]: packages/urql
[urql-adapter-version-badge]: https://img.shields.io/npm/v/mizuki-urql?label=%20
[urql-adapter-changelog]: packages/urql/CHANGELOG.md
[`urql`]: https://formidable.com/open-source/urql/
[`async_graphql::Schema`]: https://docs.rs/async-graphql/latest/async_graphql/struct.Schema.html
[initial-repo]: https://github.com/JonasKruckenberg/tauri-plugin-graphql
