import { useSubscription } from "@urql/preact";

import { graphql } from "../gql";

const newMessages = graphql(`
  subscription MessageSub {
    helloWorld
  }
`);

function handleSubscription(messages = [], response) {
    return [...messages, response.helloWorld];
};

export default function TestSubscription() {
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
