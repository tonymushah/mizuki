import {
  ApolloLink,
  FetchResult,
  NextLink,
  Observable,
  Operation,
  fromPromise
} from '@apollo/client/core'
import {GraphQLError, print} from 'graphql'
import {invoke} from '@tauri-apps/api/core'
import {getCurrentWebviewWindow} from '@tauri-apps/api/webviewWindow'
import {Event} from '@tauri-apps/api/event'
import {getMainDefinition} from '@apollo/client/utilities'

type Response = [body: string, isOk: boolean]

export class InvokeLink extends ApolloLink {
  private pluginName: string

  constructor(pluginName: string) {
    super()
    this.pluginName = pluginName
  }

  public request(
    operation: Operation,
    forward?: NextLink | undefined
  ): Observable<FetchResult> | null {
    const command = `plugin:${this.pluginName}|graphql`
    const args = {
      query: print(operation.query),
      variables: operation.variables || undefined,
      extensions: operation.extensions
    }
    return fromPromise(
      invoke<Response>(command, args)
        .then(response => {
          console.debug(response)
          const [body] = response!
          const payload: FetchResult = JSON.parse(body)
          return payload
        })
        .catch(err => {
          return {
            errors: [new GraphQLError(String(err))],
            context: operation.getContext()
          }
        })
    )
  }
}

export class SubscriptionsLink extends ApolloLink {
  private pluginName: string
  private subEndEventLabel: string

  constructor(pluginName: string, subEndEventLabel: string = 'sub_end') {
    super()
    this.pluginName = pluginName
    this.subEndEventLabel = subEndEventLabel
  }

  public request(
    operation: Operation,
    forward?: NextLink | undefined
  ): Observable<FetchResult> | null {
    const args = {
      query: print(operation.query),
      variables: operation.variables || undefined,
      extensions: operation.extensions
    }
    return new Observable(subscriber => {
      const appWebview = getCurrentWebviewWindow()
      const command = `plugin:${this.pluginName}|subscriptions`
      const id = Math.floor(Math.random() * 10000000)
      const subId = `${Math.floor(Math.random() * 10000000)}`
      let unlistens: (() => void)[] = [
        () => {
          appWebview.emit(this.subEndEventLabel, subId)
        }
      ]

      const unlisten = () => {
        unlistens.forEach(u => u())
        unlistens = []
      }
      window.addEventListener('beforeunload', unlisten)
      unlistens.push(() => window.removeEventListener('beforeunload', unlisten))
      Promise.resolve()
        .then(async () =>
          appWebview.listen(
            `graphql://${id}`,
            (event: Event<string | null>) => {
              if (event.payload === null) return subscriber.complete()
              const res: FetchResult = JSON.parse(event.payload)
              console.debug(res)
              subscriber.next(res)
            }
          )
        )
        .then(_unlisten => unlistens.push(_unlisten))
        .then(() =>
          invoke(command, {
            ...args,
            id,
            sub_id: subId
          }).catch(e => {
            throw new Error(`Tauri Invoke Error ${String(e)}`)
          })
        )
        .catch(err => {
          subscriber.error(err)
        })
      return unlisten
    })
  }
}

export class MizukiLink extends ApolloLink {
  private inner: ApolloLink

  constructor(pluginName: string, subEndEventLabel: string = 'sub_end') {
    super()
    this.inner = ApolloLink.split(
      ({query}) => {
        const definition = getMainDefinition(query)

        return (
          definition.kind === 'OperationDefinition' &&
          definition.operation === 'subscription'
        )
      },
      new SubscriptionsLink(pluginName, subEndEventLabel),
      new InvokeLink(pluginName)
    )
  }

  request(
    operation: Operation,
    forward?: NextLink | undefined
  ): Observable<FetchResult> | null {
    return this.inner.request(operation, forward)
  }
}
