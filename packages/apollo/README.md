# mizuki-apollo-link

[![Npm][npm-badge]][npm-url]

Custom `apollo` link that uses Tauri's IPC system to resolve queries against a GraphQL endpoint.

## Install

```console
$ pnpm add mizuki-apollo-link
# or
$ npm install mizuki-apollo-link
# or
$ yarn add mizuki-apollo-link
```

## Usage

You need to register the plugin with Tauri first! Please see the [top-level Readme] for a full example.

Import and use the custom link to connect to the GraphQL endpoint exposed over IPC.

```javascript
import { MizukiLink } from 'mizuki-apollo-link'
import { Client, InMemoryCache, useQuery, useSubscription } from "@apollo/client"

/// Don't forget to use the Apollo Client Provider to use the hooks!!!
const client = new ApolloClient({
    link: new MizukiLink("myPlugin"),
    cache: new InMemoryCache()
});

const heroQuery = `
query {
  hero {
    name
  }
}
`

function Hero() {
    const { data, fetching, error } = useQuery(heroQuery);

    if (fetching) return <p>Loading...</p>
    if (error) return <p>Oh no... {error.message}</p>

    return <p>The hero is {data.hero.name}</p>
}


const newMessages = `
  subscription MessageSub {
    helloWorld
  }
`

function Subscriptions() {
    const { data, error, loading } = useSubscription(newMessages);
    const [accumulatedData, setAccumulatedData] = useState([]);

    useEffect(() => {
        setAccumulatedData((prev) => [...prev, data.helloWorld]);
    }, [data]);

    return (
        <>
            {loading && <p>Loading...</p>}
            {JSON.stringify(accumulatedData, undefined, 2)}
        </>
    );
}
```

## License

[MIT © Tony Mushah](./LICENSE)

[top-level Readme]: ../../README.md
[npm-url]: https://www.npmjs.com/package/mizuki-apollo-link
[npm-badge]: https://img.shields.io/npm/v/mizuki-apollo-link
