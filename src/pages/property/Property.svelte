<script lang="ts">
  import type { TileConfig } from "src/types";
  import { TileType } from "../../enums/ui";
  import MainLayout from "../../MainLayout.svelte";
  import RoomGroupComponent from "./RoomGroup.svelte";
  import { onMount } from "svelte";
  import Tile from "../../common/Tile.svelte";
  import type RoomGroup from "../../models/RoomGroup";
  import { emit, listen } from "@tauri-apps/api/event";
  import { addBase64HtmlSyntax } from "../../utils/index";
  import type { Property } from "../../models/Property";
  import type { PropertyDataEvent } from "../../events";

  export let params: { propertyId: string };

  export let addingNewRoomGroup: boolean = false;

  let property: Property;

  $: roomGroupSummary = "To be done later";

  $: roomGroupArray = [];

  function toggleNewRoomGroup() {
    addingNewRoomGroup = !addingNewRoomGroup;
  }

  function addNewRoomGroup(event: CustomEvent<TileConfig>) {
    console.log(event);

    let newRoomGroupRequest = {
      name: event.detail.title,
      property_id: property.id,
      image: event.detail.image,
    };

    console.log(newRoomGroupRequest);

    emit("add_new_room_group", newRoomGroupRequest);
    emit("get_room_groups", {
      property_id: property.id,
    });

    toggleNewRoomGroup();
  }

  onMount(async () => {
    console.log(property);
    await emit("get_property_data", params.propertyId);
    await emit("get_room_groups", {
      property_id: params.propertyId,
    });
    await listen<PropertyDataEvent>("property_data", (event) => {
      if (event.payload.success) {
        property = { ...event.payload.property };
        roomGroupArray = event.payload.property.roomGroups;
        console.log(roomGroupArray);
      }
    });
    await listen("room_groups_data", (event) => {
      roomGroupArray = event.payload as RoomGroup[];
    });
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
      <div class="room-groups content-container">
        {#if roomGroupArray}
          {#each roomGroupArray as roomGroup}
            <RoomGroupComponent {roomGroup} />
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
  @import "./src/lib/app.scss";
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
