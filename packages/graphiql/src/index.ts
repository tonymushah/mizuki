import {Fetcher, Observable, Unsubscribable} from '@graphiql/toolkit'
import {ExecutionResult, GraphQLError, parse} from 'graphql'
import {invoke} from '@tauri-apps/api/core'
import {Event} from '@tauri-apps/api/event'
import {getCurrentWebview} from '@tauri-apps/api/webview'

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
    // console.log('start')
    // console.log(next, error, complete)
    if (
      typeof next === 'object' &&
      next !== null &&
      'next' in next &&
      typeof next.next === 'function' &&
      'error' in next &&
      typeof next.error === 'function' &&
      'complete' in next &&
      typeof next.complete === 'function'
    ) {
      // console.log('can sub!')
      this.sub(
        next.next as SubNext,
        next.error as SubErr,
        next.complete as SubCompl
      )
    } else if (
      typeof next === 'function' &&
      typeof error === 'function' &&
      typeof complete === 'function'
    ) {
      // console.log('can sub!')
      this.sub(next as SubNext, error as SubErr, complete as SubCompl)
    } else {
      // console.error('cannot sub')
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
  const appWebview = getCurrentWebview()
  const command = `plugin:${pluginName}|subscriptions`
  const fetcher: Fetcher = async function (params) {
    // console.log('fetching')
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
      // console.log('stoping')
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
      // console.log('sub')
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
    // console.log('returned subObs')
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
  return params => {
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
