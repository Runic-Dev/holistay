<script lang="ts">
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Breadcrumb from "$lib/components/ui/breadcrumb";
  import { Label } from "$lib/components/ui/label";
  import { Input } from "$lib/components/ui/input";
  import * as Popover from "$lib/components/ui/popover";
  import * as Card from "$lib/components/ui/card";
  import MainLayout from "@/MainLayout.svelte";
  import { propertyStore } from "@/store";
  import { emit } from "@tauri-apps/api/event";
  import type { Property } from "@/models/Property";
  import type { TileConfig } from "src/types";
  import { invoke } from "@tauri-apps/api";
  import { handleImageEncodingForHtml } from "$lib/utils";
    import { push } from "svelte-spa-router";

  export let params: { propertyId: string };
  export let addingNewRoomGroup: boolean = false;
  let newRoomGroupName: string = "";

  $: property = null satisfies Property;
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
      console.log(property);
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
  <MainLayout>
    <div
      class="header-image"
      style="background-image: url({handleImageEncodingForHtml(
        property.image,
      )});"
    ></div>
    <Breadcrumb.Root class="p-4">
      <Breadcrumb.List>
        <Breadcrumb.Item link="/">
          Properties
        </Breadcrumb.Item>
        <Breadcrumb.Separator />
        <Breadcrumb.Page>
          <Breadcrumb.Item>{property.name}</Breadcrumb.Item>
        </Breadcrumb.Page>
      </Breadcrumb.List>
    </Breadcrumb.Root>
    <div
      style="background-image: url({handleImageEncodingForHtml(
        property.image,
      )}); background-color: rgba(255,255,255,0.7); background-blend-mode: lighten;"
      class="header-image manage-property p-4 bg-gray-100 rounded-lg"
    >
      <div class="room-groups-controls flex justify-between items-center mb-4">
        <h4 class="room-group-summary text-xl font-semibold">
          {roomGroupSummary}
        </h4>
        <Popover.Root>
          <Popover.Trigger>Add Room Group</Popover.Trigger>
          <Popover.Content>
            <Label for="newRoomGroupName">New Room Group</Label>
            <Input class="my-2" placeholder="Name" id="newRoomGroupName"
            ></Input>
            <Label for="newRoomGroupImage">Image</Label>
            <Input id="newRoomGroupImage" class="my-2" type="file"
              >RoomGroup Image</Input
            >
            <Button class="my-2">Confirm</Button>
          </Popover.Content>
        </Popover.Root>
        <Popover.Root>
          <Popover.Trigger>Add Room</Popover.Trigger>
          <Popover.Content>
            <Label for="newRoomName">New Room Group</Label>
            <Input class="my-2" placeholder="Name" id="newRoomName"></Input>
            <Label for="newRoomImage">Image</Label>
            <Input id="newRoomImage" class="my-2" type="file">Room Image</Input>
            <Button class="my-2">Confirm</Button>
          </Popover.Content>
        </Popover.Root>
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
  .header-image {
    background-size: cover;
    background-color: rgba(255, 255, 255, 0.5);
    // background-blend-mode: soft-light;
    backdrop-filter: blur(5px);
  }
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
