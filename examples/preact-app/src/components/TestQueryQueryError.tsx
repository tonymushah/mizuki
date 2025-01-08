import { useQuery } from "@urql/preact";

import { graphql } from "../gql";

const testQueryErro = graphql(`
  query notHero {
    notHero {
      name
    }
  }
`);

export default function TestQueryQueryError() {
    const [res] = useQuery({
        query: testQueryErro,
    });

    const { data, fetching, error } = res;

    if (fetching) return <p>Loading...</p>;
    if (error) return <p>Oh no... {error.graphQLErrors[0].message}</p>;

    return (
        <p>The hero is {data?.notHero.name ?? "not here"}</p>
    );
}
