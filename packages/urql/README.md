# mizuki-urql-adapter

[![Npm][npm-badge]][npm-url]

Custom `urql` exchange that uses Tauri's IPC system to resolve queries against a GraphQL endpoint.

## Install

```console
$ pnpm add mizuki-urql-adapter
# or
$ npm install mizuki-urql-adapter
# or
$ yarn add mizuki-urql-adapter
```

## Usage

You need to register the plugin with Tauri first! Please see the [top-level Readme] for a full example.

Import and use the custom exchange to connect to the GraphQL endpoint exposed over IPC.

```javascript
import { invokeExchange } from "mizuki-urql-adapter";

const client = createClient({
  url: "graphql", // this value is important, don't touch
  exchanges: [invokeExchange("myPlugin")],
});

const heroQuery = `
query {
  hero {
    name
  }
}
`;

function Hero() {
  const [result, reexecuteQuery] = useQuery({
    query: heroQuery,
  });

  const { data, fetching, error } = result;

  if (fetching) return <p>Loading...</p>;
  if (error) return <p>Oh no... {error.message}</p>;

  return (
    <p>The hero is {data.hero.name}</p>
  );
}
```

### Subscriptions

This adapter also supports subscriptions.

```javascript
import { subscriptionExchange } from "mizuki-urql-adapter";
import { createClient } from "@urql/preact";

const client = createClient({
  url: "graphql",
  exchanges: [
    subscriptionExchange("myPlugin")
  ],
});

const newMessages = `
  subscription MessageSub {
    helloWorld
  }
`;

function handleSubscription(messages = [], response) {
  return [response.helloWorld, ...messages];
};

function TestSubscription() {
  const [res] = useSubscription({ query: newMessages }, handleSubscription);

  if (!res.data) {
    return <p>No new messages</p>;
  }

  return (
        <p>
          {res.data}
        </p>
  ); 
}
```

## License

[MIT © Tony Mushah](./LICENSE)

[top-level Readme]: ../../README.md
[npm-url]: https://www.npmjs.com/package/mizuki-urql-adapter
[npm-badge]: https://img.shields.io/npm/v/mizuki-urql-adapter