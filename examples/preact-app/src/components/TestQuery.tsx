import { useQuery } from "@urql/preact";

import { graphql } from "../gql";

const testQuery = graphql(`
query getHero {
  hero {
    name
  }
}
`);

export default function TestQuery() {
    const [res] = useQuery({
        query: testQuery,
    });

    const { data, fetching, error } = res;

    if (fetching) return <p>Loading...</p>;
    if (error) return <p>Oh no... {error.message}</p>;

    return (
        <p>The hero is {data?.hero.name ?? "not here"}</p>
    );
}
