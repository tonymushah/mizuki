import { createClient, Provider } from "@urql/preact";
import { render } from "preact";
import { getExchanges } from "@mizuki/urql";
import { App } from "./app";
import "./index.css";

const client = createClient({
  url: "graphql",
  exchanges: [
    ...getExchanges("mizuki-test"),
  ],
});

render(
  <Provider value={client}>
    <App />
  </Provider>,
  document.getElementById("app")!
);
