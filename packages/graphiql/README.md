# mizuki-graphiql-fetcher

[![Npm][npm-badge]][npm-url]

Custom `graphiql` fetcher that uses Tauri's IPC system to resolve queries against a GraphQL endpoint.

## Install

```console
$ pnpm add mizuki-graphiql-fetcher
# or
$ npm install mizuki-graphiql-fetcher
# or
$ yarn add mizuki-graphiql-fetcher
```

## Usage

You need to register the plugin with Tauri first! Please see the [top-level Readme] for a full example.

Import and use the custom fetcher to connect to the GraphQL endpoint exposed over IPC.

```javascript
import makeMizukiFetcher from 'mizuki-graphiql-fetcher'
import GraphiQL from "graphiql";

const fetcher = makeMizukiFetcher("my-plugin");

function MyGraphIQL() {
    return <GraphiQL fetcher={fetcher}>
}

```

## License

[MIT © Tony Mushah](./LICENSE)

[top-level Readme]: ../../README.md
[npm-url]: https://www.npmjs.com/package/mizuki-apollo-link
[npm-badge]: https://img.shields.io/npm/v/mizuki-apollo-link
