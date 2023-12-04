import { createClient, Provider } from "@urql/preact";
import { render } from "preact";
import { invokeExchange, subscribe } from "@mizuki/urql";
import { App } from "./app";
import "./index.css";

const client = createClient({
  url: "graphql",
  exchanges: [
    invokeExchange("mizuki-test"),
    subscribe("mizuki-test")
  ],
});

render(
  <Provider value={client}>
    <App />
  </Provider>,
  document.getElementById("app")!
);
