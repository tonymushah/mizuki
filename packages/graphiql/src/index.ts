import {Fetcher, Observable, Unsubscribable} from '@graphiql/toolkit'
import {ExecutionResult, GraphQLError, parse} from 'graphql'
import {invoke} from '@tauri-apps/api/core'
import {getCurrentWebviewWindow} from '@tauri-apps/api/webviewWindow'
import {Event} from '@tauri-apps/api/event'

type Response = [body: string, isOk: boolean]

export function getInvokeFetcher(pluginName: string) {
  const command = `plugin:${pluginName}|graphql`
  const fetcher: Fetcher = async function (params) {
    const r = await invoke<Response>(command, params)
      .then(response => {
        const [body] = response!
        const payload: ExecutionResult = JSON.parse(body)
        return payload
      })
      .catch(err => {
        return {
          errors: [new GraphQLError(String(err))]
        } satisfies ExecutionResult
      })
    return r
  }
  return fetcher
}

type SubNext = (value: ExecutionResult) => void
type SubErr = (error: any) => void
type SubCompl = () => void

class SubObs implements Observable<ExecutionResult> {
  private sub: (next: SubNext, error: SubErr, complete: SubCompl) => void

  private unlisten: () => void

  constructor(
    sub: (
      next: (value: ExecutionResult) => void,
      error: (error: any) => void,
      complete: () => void
    ) => void,
    unlisten: () => void
  ) {
    this.sub = sub
    this.unlisten = unlisten
  }

  subscribe(opts: {
    next: (value: ExecutionResult) => void
    error: (error: any) => void
    complete: () => void
  }): Unsubscribable

  // eslint-disable-next-line no-dupe-class-members
  subscribe(
    next: (value: ExecutionResult) => void,
    error: null | undefined,
    complete: () => void
  ): Unsubscribable

  // eslint-disable-next-line no-dupe-class-members
  subscribe(
    next?: ((value: ExecutionResult) => void) | undefined,
    error?: ((error: any) => void) | undefined,
    complete?: (() => void) | undefined
  ): Unsubscribable

  // eslint-disable-next-line no-dupe-class-members
  subscribe(
    next?: unknown,
    error?: unknown,
    complete?: unknown
  ): Unsubscribable {
    if (
      typeof next === 'function' &&
      typeof error === 'function' &&
      typeof complete === 'function'
    ) {
      this.sub(next as SubNext, error as SubErr, complete as SubCompl)
    }
    return {
      unsubscribe: this.unlisten
    }
  }
}

export function getSubscriptionFetcher(
  pluginName: string,
  subEndEventLabel: string = 'sub_end'
) {
  const appWebview = getCurrentWebviewWindow()
  const command = `plugin:${pluginName}|subscriptions`
  const fetcher: Fetcher = async function (params) {
    const id = Math.floor(Math.random() * 10000000)
    const subId = `${Math.floor(Math.random() * 10000000)}`
    let unlistens: (() => void)[] = [
      () => {
        appWebview.emit(subEndEventLabel, subId)
      }
    ]

    const unlisten = () => {
      unlistens.forEach(u => u())
      unlistens = []
    }
    window.addEventListener('beforeunload', unlisten)
    unlistens.push(() => window.removeEventListener('beforeunload', unlisten))
    const sub = function (
      next: (d: ExecutionResult) => void,
      error: (err: any) => void,
      complete: () => void
    ) {
      appWebview
        .listen(`graphql://${id}`, (event: Event<string | null>) => {
          if (event.payload === null) return complete()
          const res: ExecutionResult = JSON.parse(event.payload)
          // console.debug(res)
          next(res)
        })
        .then(_unlisten => unlistens.push(_unlisten))
        .then(() =>
          invoke(command, {
            ...params,
            id,
            sub_id: subId
          }).catch(e => {
            throw new Error(`Tauri Invoke Error ${String(e)}`)
          })
        )
        .catch(err => {
          error(err)
        })
    }
    return new SubObs(sub, unlisten)
  }
  return fetcher
}

export default function getMuzikiFetcher(
  pluginName: string,
  subEndEventLabel?: string
): Fetcher {
  const invokeFetcher = getInvokeFetcher(pluginName)
  const subFetcher = getSubscriptionFetcher(pluginName, subEndEventLabel)
  return async params => {
    const document = parse(params.query)
    const maybeSub = document.definitions.find(e => {
      if (e.kind === 'OperationDefinition') {
        return e.operation === 'subscription'
      } else {
        return false
      }
    })
    if (maybeSub !== undefined) {
      return subFetcher(params)
    } else {
      return invokeFetcher(params)
    }
  }
}
