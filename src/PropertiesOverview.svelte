<script lang="ts">
  import { onMount } from "svelte";

  export let properties = [];
  export let error = false;
  import { Properties } from "./lib/test_data/properties";
    import PropertyOverview from "./PropertyOverview.svelte";

  onMount(() => {
    properties = Properties.data;
  });
</script>

<div class="properties-overview">
  <h2>Properties</h2>
  {#if properties.length > 0}
    {#each properties as property}
      <PropertyOverview {property} />
    {/each}
  {:else if !error}
    <p>Loading..</p>
  {:else}
    <p>There was an error loading your properties</p>
  {/if}
</div>

<style lang="scss">
  .properties-overview {
    background: lightgray;
    border-radius: 16px;
    padding: 16px;
    color: black;
  }
</style>
