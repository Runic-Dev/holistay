<script lang="ts">
  import { onMount } from "svelte";

  export let properties = [];
  export let error = false;
  import { Properties } from "./lib/test_data/properties";
  import PropertyOverview from "./PropertyOverview.svelte";
  import Tile from "./common/Tile.svelte";
  import { TileType } from "./enums/ui";
  import { emit, listen } from "@tauri-apps/api/event";

  let addingNewProperty = false;

  function addNewProperty(payload: any) {
    console.log("Add new property");
    console.log(payload);
    emit("add_new_property", payload.detail);
  }

  onMount(async () => {
    emit("get_properties");
    await listen("properties_loaded", (event) => {
      console.log(event);
    });
  });
</script>

<div class="properties-overview">
  <h2>Select your property:</h2>
  <div class="property-overview-container">
    {#each properties as property}
      <PropertyOverview {property} />
    {/each}
    {#if addingNewProperty}
      <Tile
        tileConfig={{
          type: TileType.NewProperty,
          title: "Create Property",
          imageUrl: null,
          clickAction: null,
        }}
        on:confirmedProperty={addNewProperty}
      />
    {/if}
  </div>
  <button on:click={() => (addingNewProperty = !addingNewProperty)}
    >Add New Property</button
  >
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
