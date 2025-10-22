import {invoke} from '@tauri-apps/api/core'
import {Event} from '@tauri-apps/api/event'
import {getCurrentWebview} from '@tauri-apps/api/webview'
import {
  Exchange,
  ExecutionResult,
  makeErrorResult,
  makeResult,
  Operation,
  OperationResult,
  subscriptionExchange as subEx
} from '@urql/core'
import {print} from 'graphql'
import {
  filter,
  make,
  merge,
  mergeMap,
  onPush,
  pipe,
  share,
  Source,
  takeUntil
} from 'wonka'

/**
 * An exchange for resolving GraphQL requests over Tauri's IPC bridge.
 *
 * **Example**
 *
 * ```javascript
 * import { createClient } from 'urql'
 * import { invokeExchange } from '@mizuki/urql'
 *
 * const client = createClient({
 *    url: 'graphql', // this endpoint is important, don't touch
 *    exchanges: [invokeExchange("<YOUR_PLUGIN_NAME_HERE>")]
 * })
 * ```
 * @param name Your plugin name
 */
export const invokeExchange: (name: string) => Exchange =
  name =>
  ({forward}) => {
    return ops$ => {
      const sharedOps$ = share(ops$)
      const fetchResults$ = pipe(
        sharedOps$,
        filter(op => op.kind === 'query' || op.kind === 'mutation'),
        mergeMap(operation => {
          const {key} = operation
          const teardown$ = pipe(
            sharedOps$,
            filter(op => op.kind === 'teardown' && op.key === key)
          )

          const args = {
            query: print(operation.query),
            variables: operation.variables || undefined,
            extensions: operation.extensions
          }

          const command = `plugin:${name}|graphql`

          console.debug({
            type: 'invokeRequest',
            message: 'An invoke request is being executed.',
            operation,
            data: {
              command,
              args
            }
          })

          return pipe(
            makeInvokeSource(operation, command, args),
            takeUntil(teardown$),
            onPush(result => {
              const error = !result.data ? result.error : undefined

              console.debug({
                type: error ? 'invokeError' : 'invokeSuccess',
                message: `A ${
                  error ? 'failed' : 'successful'
                } invoke response has been returned.`,
                operation,
                data: {
                  value: error || result
                }
              })
            })
          )
        })
      )

      const forward$ = pipe(
        sharedOps$,
        filter(op => op.kind !== 'query' && op.kind !== 'mutation'),
        forward
      )

      return merge([fetchResults$, forward$])
    }
  }

type Response = [body: string, isOk: boolean]

function makeInvokeSource(
  operation: Operation,
  command: string,
  invokeArgs: Record<string, any>
): Source<OperationResult> {
  return make(({next, complete}) => {
    let ended = false

    Promise.resolve()
      .then(() => {
        if (ended) return

        return invoke<Response>(command, invokeArgs)
      })
      .then(response => {
        const [body] = response!
        const payload: ExecutionResult = JSON.parse(body)

        console.debug(response)

        next(makeResult(operation, payload))
      })
      .then(complete)
      .catch(err => {
        const result = makeErrorResult(operation, err, null)

        next(result)
        complete()
      })

    return () => {
      ended = true
    }
  })
}

/**
 * A GraphQL Subscription transport that uses the Tauri IPC system.
 *
 * ## Example
 *
 * ```javascript
 * import { createClient } from 'urql'
 * import { subscriptionExchange } from '@mizuki/urql'
 *
 * const name = "<YOUR_PLUGIN_NAME_HERE>"
 *
 * const client = createClient({
 *  url: "graphql", // this endpoint is important, don't touch
 *  exchanges: [
 *    invokeExchange(name),
      subscriptionExchange(name)
 *  ],
 * });
 * ```
 *
 * @param name Your plugin name
 * @param [subEndEventLabel='sub_end'] the subscription end event label
 * @returns
 */

export function subscriptionExchange(
  name: string,
  subEndEventLabel: string = 'sub_end'
) {
  const appWebview = getCurrentWebview()
  return subEx({
    forwardSubscription: operation => ({
      subscribe: sink => {
        const id = Math.floor(Math.random() * 10000000)
        const subId = `${Math.floor(Math.random() * 10000000)}`
        let unlistens: (() => void)[] = [
          () => {
            appWebview.emitTo(
              {
                kind: 'Webview',
                label: appWebview.label
              },
              subEndEventLabel,
              subId
            )
          }
        ]

        const unlisten = () => {
          unlistens.forEach(u => u())
          unlistens = []
        }
        window.addEventListener('beforeunload', unlisten)
        unlistens.push(() =>
          window.removeEventListener('beforeunload', unlisten)
        )
        Promise.resolve()
          .then(async () =>
            appWebview.listen(
              `graphql://${id}`,
              (event: Event<string | null>) => {
                if (event.payload === null) return sink.complete()
                sink.next(JSON.parse(event.payload))
              }
            )
          )
          .then(_unlisten => unlistens.push(_unlisten))
          .then(() =>
            invoke(`plugin:${name}|subscriptions`, {
              ...operation,
              id,
              sub_id: subId
            })
          )
          // .then(() => sink.complete())
          .catch(err => {
            sink.error(err)
          })
        return {
          unsubscribe: unlisten
        }
      }
    })
  })
}

/**
 * Get the `invokeExchange` and the `subscriptionExchange` through one function
 *
 * ## Example
 *
 * ```javascript
 * import { createClient } from 'urql'
 * import { getExchanges } from '@mizuki/urql'
 *
 * const name = "<YOUR_PLUGIN_NAME_HERE>"
 *
 * const client = createClient({
 *  url: "graphql", // this endpoint is important, don't touch
 *  exchanges: getExchanges(name)
 * });
 * ```
 *
 * @param name Your plugin name
 * @param subEndEventLabel the subscription end event label
 * @returns
 */
export function getExchanges(name: string, subEndEventLabel?: string) {
  return [invokeExchange(name), subscriptionExchange(name, subEndEventLabel)]
}
