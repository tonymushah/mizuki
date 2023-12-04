import { useQuery, useSubscription } from "@urql/preact";

import { graphql } from "./gql";

const testQuery = graphql(`
query getHero {
  hero {
    name
  }
}
`);

function TestQuery() {
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

const newMessages = graphql(`
  subscription MessageSub {
    helloWorld
  }
`);

function handleSubscription(messages = [], response) {
  return [...messages, response.helloWorld];
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

export function App() {
  return <>
    <TestQuery />
    <TestSubscription />
  </>
}
