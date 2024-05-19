<script lang="ts">
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import { Input } from "$lib/components/ui/input";
  import * as Card from "$lib/components/ui/card";
  import MainLayout from "@/MainLayout.svelte";
  import { propertyStore } from "@/store";
  import { emit } from "@tauri-apps/api/event";
  import type { Property } from "@/models/Property";
  import type { TileConfig } from "src/types";
  import { invoke } from "@tauri-apps/api";
  import { addBase64HtmlSyntax } from "@/utils";
  // import { TileType } from "@/enums/ui";

  export let params: { propertyId: string };
  export let addingNewRoomGroup: boolean = false;
  let newRoomGroupName: string = "";

  $: property = null;
  $: roomGroupSummary = "Loading...";

  function toggleNewRoomGroup() {
    addingNewRoomGroup = !addingNewRoomGroup;
  }

  function addNewRoomGroup(event: CustomEvent<TileConfig>) {
    console.log("New Room Group");
    console.log(event.detail.image);
    let newRoomGroupRequest = {
      name: event.detail.title,
      property_id: property.id,
      image: event.detail.image == "" ? null : event.detail.image,
    };

    emit("add_new_room_group", newRoomGroupRequest);
    emit("get_room_groups", {
      property_id: property.id,
    });

    toggleNewRoomGroup();
  }

  onMount(async () => {
    propertyStore.subscribe((x) => {
      property = x.properties.find((p) => p.id == params.propertyId);
      if (property.roomGroups) {
        roomGroupSummary = `${property.name} has ${property.roomGroups.length} room groups`;
      } else {
        roomGroupSummary = `${property.name} has 0 room groups`;
      }
      property = { ...property };
    });
    await invoke("get_property", {
      request: { property_id: params.propertyId },
    })
      .then((response: Property) => {
        propertyStore.update((x) => {
          let propertyIndex = x.properties.findIndex(
            (prop) => prop.id == params.propertyId,
          );
          let property = x.properties.at(propertyIndex);
          property.description = response.description;
          property.roomGroups = response.roomGroups;
          x.properties[propertyIndex] = property;
          return x;
        });
      })
      .catch((err) => console.error(err));
  });
</script>

{#if property}
  <MainLayout
    header={property.name}
    imageUrl={addBase64HtmlSyntax(property.image, "jpeg")}
  >
    <div class="manage-property p-4 bg-gray-100 rounded-lg">
      <div class="room-groups-controls flex justify-between items-center mb-4">
        <h4 class="room-group-summary text-xl font-semibold">
          {roomGroupSummary}
        </h4>
        <Button on:click={toggleNewRoomGroup} variant="outline"
          >Add RoomGroup</Button
        >
      </div>
      <div
        class="room-groups grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4"
      >
        {#if property.roomGroups}
          {#each property.roomGroups as roomGroup}
            <Card.Root class="w-full sm:w-64 lg:w-80">
              <Card.Header>
                <Card.Title>{roomGroup.name}</Card.Title>
              </Card.Header>
              <Card.Content>
                <img
                  src="./roomgroupplaceholder.png"
                  alt={property.name}
                  class="w-full h-32 object-cover rounded-md"
                />
              </Card.Content>
            </Card.Root>
          {/each}
        {/if}
        {#if addingNewRoomGroup}
          <Card.Root class="w-full sm:w-64 lg:w-80">
            <Card.Header>
              <Card.Title>Create New Room Group</Card.Title>
            </Card.Header>
            <Card.Content>
              <Label for="roomGroupName">Room Group Name</Label>
              <Input
                bind:value={newRoomGroupName}
                id="roomGroupName"
                type="text"
              />
              <Button>Confirm</Button>
            </Card.Content>
          </Card.Root>
        {/if}
      </div>
    </div>
  </MainLayout>
{/if}

<style lang="scss">
  @import "src/lib/app.scss";
  .manage-property {
    @apply p-4 bg-gray-100 rounded-lg;
    .room-groups-controls {
      @apply flex justify-between items-center mb-4;
      h4.room-group-summary {
        @apply text-xl font-semibold;
      }
    }
    .room-groups {
      @apply grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4;
    }
  }
</style>
