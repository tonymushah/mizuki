
import GraphiQL from "graphiql";
import "./App.css";
import getMuzikiFetcher from "mizuki-graphiql-fetcher";

const fetcher = getMuzikiFetcher("mizuki-test");

function App() {
  return <GraphiQL fetcher={fetcher} />
}

export default App;
