<script lang="ts">
  import { onMount } from "svelte";

  export let properties = [];
  export let error = false;
  import { Properties } from "./lib/test_data/properties";
  import PropertyOverview from "./PropertyOverview.svelte";
  import { push } from "svelte-spa-router";
    import Tile from "./common/Tile.svelte";
    import { TileType } from "./enums/ui";

  onMount(() => {
    properties = Properties.data;
    if (properties.length == 1) {
      push("/property/" + properties[0].id);
    }
    if (properties.length == 0) {
      //TODO: Take them to create a property
      // push("/property/" + properties[0].id);
    }
  });
</script>

<div class="properties-overview">
  {#if properties.length > 0}
    <h2>Select your property:</h2>
    <div class="property-overview-container">
      {#each properties as property}
        <PropertyOverview {property} />
      {/each}
        <Tile tileConfig={{ type: TileType.NewProperty, title: null, imageUrl: null, clickAction: null }}/>
    </div>
  {:else if !error}
    <p>Loading..</p>
  {:else}
    <p>There was an error loading your properties</p>
  {/if}
</div>

<style lang="scss">
  .properties-overview {
    display: flex;
    flex-wrap: wrap;
    background: lightgray;
    border-radius: 16px;
    padding: 16px;
    color: black;

    .property-overview-container {
      display: flex;
      flex-wrap: wrap;
    }
  }
</style>
