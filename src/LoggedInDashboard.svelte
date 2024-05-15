<script lang="ts">
  import { onMount } from "svelte";
  import Tile from "./common/Tile.svelte";
  import { TileType } from "./enums/ui";
  import type { ConfirmedPropertyToSend, TileConfig } from "./types";
  import { push } from "svelte-spa-router";
  import { Property } from "./models/Property";
  import { propertyStore } from "./store";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { PropertyPartial } from "@/models/PropertyPartial";

  $: properties = [];
  let addingNewProperty = false;

  function addNewProperty(payload: { detail: ConfirmedPropertyToSend }) {
    invoke("add_new_property", { request: payload.detail })
      .then(
        ({
          propertyId,
          imageOption,
        }: {
          propertyId: string;
          imageOption: string | null;
        }) => {
          propertyStore.set({
            properties: [
              ...properties,
              new Property(propertyId, payload.detail.name, "", imageOption),
            ],
          });
        },
      )
      .catch((err: string) => console.error(err));
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
    await invoke("get_property_partials")
      .then((property_partials: PropertyPartial[]) => {
        propertyStore.subscribe((ps) => (properties = ps.properties));
        propertyStore.set({
          properties: property_partials.map(
            (pp) => new Property(pp.id, pp.name, "", pp.image),
          ),
        });
      })
      .catch((err: string) => console.log(err));
  });
</script>

<div class="properties-overview">
  <h2 class="underline">Select your property:</h2>
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
    >Add New Property
  </button>
</div>

<style lang="scss">
  @import "./styles.css";
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
