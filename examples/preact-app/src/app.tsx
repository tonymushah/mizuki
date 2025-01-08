import TestMutationSubs from "./components/TestMutationSubs";
import TestQuery from "./components/TestQuery";
import TestQueryQueryError from "./components/TestQueryQueryError";
import TestSubscription from "./components/TestSubscription";
export function App() {
  return <>
    <TestQuery />
    <TestQueryQueryError />
    <TestSubscription />
    <TestMutationSubs />
  </>
}
