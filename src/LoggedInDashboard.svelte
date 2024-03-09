<script lang="ts">
  import { onMount } from "svelte";
  import Tile from "./common/Tile.svelte";
  import { TileType } from "./enums/ui";
  import { emit, listen } from "@tauri-apps/api/event";
  import type { TileConfig } from "./types";
  import { push } from "svelte-spa-router";
  import type { Property } from "./models/Property";
  import { propertyStore } from "./store";

  $: properties = [];
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
    propertyStore.subscribe((propStore) => {
      properties = propStore.properties;
    });
    emit("get_properties");
    await listen<Property[]>("properties_loaded", (event) => {
      console.log(event.payload);
      propertyStore.set({
        properties: event.payload,
      });
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
