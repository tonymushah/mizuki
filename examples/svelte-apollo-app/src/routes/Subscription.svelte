<script lang="ts">
  import {graphql} from '$lib/gql'
  import client from '$lib/gql.client'
  import {readable} from 'svelte/store'
  import Sub from './Sub.svelte'
  let listening = $state(true)
  
</script>

<div>
  <h4>Subscription</h4>
  {#if listening}
    <Sub/>
  {/if}

  <button
    onclick={() => {
      listening = !listening
    }}
  >
    {#if listening}
      Listening
    {:else}
      Not Listening
    {/if}
  </button>
</div>

<style>
  h4 {
    margin: 0px;
  }
  div {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }
  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }
</style>
