<script lang="ts">
  import type {
    TileConfig,
    RoomGroupResponse as RoomGroupsDataEvent,
    PropertyResponse,
  } from "src/types";
  import { TileType } from "@/enums/ui";
  import MainLayout from "@/MainLayout.svelte";
  import RoomGroupComponent from "@/pages/property/RoomGroup.svelte";
  import { onMount } from "svelte";
  import Tile from "@/common/Tile.svelte";
  import RoomGroup from "@/models/RoomGroup";
  import { addBase64HtmlSyntax } from "@/utils/index";
  import { propertyStore } from "@/store";
  import { emit, listen } from "@tauri-apps/api/event";
  import Description from "@/common/Description.svelte";
  import { DescribableEntity } from "@/common/types";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Property } from "@/models/Property";

  export let params: { propertyId: string };

  export let addingNewRoomGroup: boolean = false;

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
    <div class="manage-property">
      <div class="room-groups-controls">
        <h4 class="room-group-summary">{roomGroupSummary}</h4>
        <button on:click={toggleNewRoomGroup} class="add-room-group">
          Add RoomGroup
        </button>
      </div>
      <Description
        type={DescribableEntity.Property}
        id={property.id}
        description={property.description ?? "No description"}
      />
      <div class="room-groups content-container">
        {#if property.roomGroups}
          {#each property.roomGroups as roomGroup}
            <RoomGroupComponent {roomGroup} propertyId={params.propertyId} />
          {/each}
        {/if}
        {#if addingNewRoomGroup}
          <Tile
            tileConfig={{
              type: TileType.NewRoomGroup,
              title: null,
              image: null,
              clickAction: null,
            }}
            on:confirmedRoomGroup={addNewRoomGroup}
          />
        {/if}
      </div>
    </div>
  </MainLayout>
{/if}

<style lang="scss">
  @import "src/lib/app.scss";

  .manage-property {
    .room-groups-controls {
      h4 {
        margin: $padding 0;
      }

      margin: $padding * 2;
    }

    .room-groups {
      text-align: left;
      display: flex;
      flex-wrap: wrap;
    }

    h4.room-group-summary {
      font-size: 2.5rem;
    }
  }
</style>
