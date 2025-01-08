import { useClient, useSubscription } from "@urql/preact";
import { graphql } from "../gql";
import { useSignal } from "@preact/signals"
import { Fragment } from "preact";
import { useCallback, useMemo } from "preact/hooks"

const subscription = graphql(`
    subscription messages {
        watchMessages
    }
`);

const mutation = graphql(`
    mutation sendMessage($message: String!) {
        sendMessage(message: $message)
    }
`);

function Subscription() {
    const [data] = useSubscription({ query: subscription });
    const message = useMemo(() => data.data?.watchMessages ?? null, [data]);
    return <p>{message != null ? <strong>{message}</strong> : <i>No Message here</i>}</p>;
}

function Subchannel() {
    const shouldListen = useSignal(true);

    return (
        <div>
            <button onClick={() => {
                shouldListen.value = !shouldListen.value
            }}>
                {shouldListen.value ? <>Listening</> : <>Not Listening</>}
            </button>
            {
                shouldListen.value ? <Subscription /> : <Fragment />
            }
        </div>
    )
}

function Form() {
    const client = useClient();
    const message = useSignal<string | undefined>(undefined);
    const sendMessage = useCallback((message: string) => {
        client.mutation(mutation, {
            message
        }).toPromise().then(console.log).catch(console.error)
    }, [client]);
    return (<div>
        <input onSubmit={(e) => {
            e.preventDefault();
            const mess = message.peek();
            if (mess) {
                sendMessage(mess);
            }

        }} value={message.value} onInput={(e) => { message.value = e.currentTarget.value }} />
        <button onClick={(e) => {
            const mess = message.peek();
            if (mess) {
                sendMessage(mess);
            }
        }}>Send</button>
    </div>)
}

export default function TestMutationSubs() {
    return (
        <Fragment>
            <h2>Mutation subs test</h2>
            <Form />
            <Subchannel />
        </Fragment>
    )
}