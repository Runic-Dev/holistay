<script lang="ts">
  import { onMount } from "svelte";

  import PropertyOverview from "./PropertyOverview.svelte";
  import Tile from "./common/Tile.svelte";
  import { TileType } from "./enums/ui";
  import { emit, listen } from "@tauri-apps/api/event";
  import type { Property } from "./types";

  let properties: Property[] = [];
  let addingNewProperty = false;

  function addNewProperty(payload: any) {
    emit("add_new_property", payload.detail);
    emit("get_properties");
  }

  onMount(async () => {
    emit("get_properties");
    await listen("properties_loaded", (event) => {
      console.log(event);
      properties = event.payload as Property[];
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
          image: null,
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
