<script lang="ts">
  import sub from './subscription'
</script>

<p class:noMessage={$sub == undefined}>
  {#if $sub != undefined}
    {$sub}
  {:else}
    No Message
  {/if}
</p>

<style>
  .noMessage {
    font-style: italic;
  }
</style>
