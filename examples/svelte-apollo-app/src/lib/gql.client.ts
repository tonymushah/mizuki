import { ApolloClient, InMemoryCache } from "@apollo/client/core";
import {MizukiLink} from "mizuki-apollo-link";

const client = new ApolloClient({
    link: new MizukiLink("mizuki-test-apollo"),
    cache: new InMemoryCache()
});

export default client;