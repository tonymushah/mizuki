import {graphql} from '$lib/gql'
import client from '$lib/gql.client'
import {readable} from 'svelte/store'
const subscription = graphql(`
  subscription watchMessages {
    watchMessages
  }
`)
const sub = readable<string | undefined>(undefined, set => {
  console.log('sd')
  const sub_ = client
    .subscribe({
      query: subscription
    })
    .subscribe(r => {
      set(r.data?.watchMessages)
    })
  return () => {
    sub_.unsubscribe()
    console.log('unsus')
  }
})

export default sub
