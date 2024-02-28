<script lang="ts">
  import { onMount } from "svelte";
  import Tile from "./common/Tile.svelte";
  import { TileType } from "./enums/ui";
  import { emit, listen } from "@tauri-apps/api/event";
  import type { TileConfig } from "./types";
  import { push } from "svelte-spa-router";
  import type { Property } from "./models/Property";

  let properties: Property[] = [];
  let addingNewProperty = false;

  function addNewProperty(payload: any) {
    emit("add_new_property", payload.detail);
    emit("get_properties");
  }

  function propertyToTileConfig(property: Property) {
    return {
      type: TileType.Default,
      title: property.name,
      image: property.image,
      clickAction: () => push(`/property/${property.id}`),
    } as TileConfig;
  }

  onMount(async () => {
    emit("get_properties");
    await listen("properties_loaded", (event) => {
      properties = event.payload as Property[];
    });
  });
</script>

<div class="properties-overview">
  <h2>Select your property:</h2>
  <div class="property-overview-container">
    {#each properties as property}
      <Tile tileConfig={propertyToTileConfig(property)} />
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
