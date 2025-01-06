import {
  ApolloLink,
  FetchResult,
  NextLink,
  Observable,
  Operation
} from '@apollo/client'
import {print} from 'graphql'
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
    return new Observable(observer => {
      const command = `plugin:${this.pluginName}|graphql`
      const args = {
        query: print(operation.query),
        variables: operation.variables || undefined,
        extensions: operation.extensions
      }
      let ended = false
      Promise.resolve()
        .then(() => {
          if (ended) return
          return invoke<Response>(command, args)
        })
        .then(response => {
          const [body] = response!
          const payload: FetchResult = JSON.parse(body)

          console.debug(response)

          observer.next(payload)
        })
        .then(observer.complete)
        .catch(err => {
          observer.error(err)
          observer.complete()
        })

      return () => {
        ended = true
      }
    })
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
              subscriber.next(JSON.parse(event.payload))
            }
          )
        )
        .then(_unlisten => unlistens.push(_unlisten))
        .then(() =>
          invoke(command, {
            ...operation,
            id,
            sub_id: subId
          })
        )
        .catch(err => console.error(err))
      return unlisten
    })
  }
}

export class MizukiLink extends ApolloLink {
  private pluginName: string
  private subEndEventLabel: string

  constructor(pluginName: string, subEndEventLabel: string = 'sub_end') {
    super()
    this.pluginName = pluginName
    this.subEndEventLabel = subEndEventLabel
  }

  request(
    operation: Operation,
    forward?: NextLink | undefined
  ): Observable<FetchResult> | null {
    return this.split(
      ({query}) => {
        const definition = getMainDefinition(query)

        return (
          definition.kind === 'OperationDefinition' &&
          definition.operation === 'subscription'
        )
      },
      new SubscriptionsLink(this.pluginName, this.subEndEventLabel),
      new InvokeLink(this.pluginName)
    ).request(operation, forward)
  }
}
