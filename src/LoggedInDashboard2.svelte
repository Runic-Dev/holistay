<script lang="ts">
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import * as Breadcrumb from "$lib/components/ui/breadcrumb";
  import { push } from "svelte-spa-router";
  import { invoke } from "@tauri-apps/api/tauri";
  import { propertyStore, displayTopBarStore } from "./store";
  import { Property } from "./models/Property";
  import type { PropertyPartial } from "@/models/PropertyPartial";
  import type { ConfirmedPropertyToSend, TileConfig } from "./types";
  import { TileType } from "./enums/ui";
  import PropertyCard from "./common/PropertyCard.svelte";

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
      type: TileType.Default, title: property.name,
      image: property.image,
      clickAction: () => push(`/property/${property.id}`),
    } as TileConfig;
  }

  onMount(async () => {
    displayTopBarStore.set(true);
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

<Breadcrumb.Root class="p-4">
  <Breadcrumb.List>
    <Breadcrumb.Page>Properties</Breadcrumb.Page>
  </Breadcrumb.List>
</Breadcrumb.Root>
<div
  class="properties-overview h-screen w-screen flex flex-col p-4 bg-gray-100"
>
  <div
    class="property-overview-container grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4"
  >
    {#each properties as property}
      <PropertyCard {property} />
    {/each}
    {#if addingNewProperty}
      <Card.Root>
        <Card.Header>
          <Card.Title>Create Property</Card.Title>
        </Card.Header>
        <Card.Content>
          <Button on:click={() => addNewProperty}>Add New Property</Button>
        </Card.Content>
      </Card.Root>
    {/if}
  </div>
  <Button on:click={() => (addingNewProperty = !addingNewProperty)} class="mt-4"
    >Add New Property</Button
  >
</div>

<style lang="scss">
  @import "./styles.css";
  .properties-overview {
    display: flex;
    flex-direction: column;
    background: lightgray;
    // border-radius: 16px;
    padding: 16px;
    color: black;

    .property-overview-container {
      display: flex;
      flex-wrap: wrap;
    }
  }
</style>
