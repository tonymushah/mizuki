import GraphiQL from "graphiql";
import "graphiql/graphiql.min.css";
import getMuzikiFetcher from "mizuki-graphiql-fetcher";
import styles from "./App.module.css";

const fetcher = getMuzikiFetcher("mizuki-test-apollo");

function App() {
  return <div className={`${styles.graphiqlContainerExt}`}>
    <GraphiQL fetcher={fetcher} />
  </div>

}

export default App;
