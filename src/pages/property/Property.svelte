<script lang="ts">
  import type { Property, TileConfig } from "src/types";
  import { TileType } from "../../enums/ui";
  import MainLayout from "../../MainLayout.svelte";
  import RoomGroupComponent from "./RoomGroup.svelte";
  import { onMount } from "svelte";
  import Tile from "../../common/Tile.svelte";
  import RoomGroup from "../../models/RoomGroup";
  import { emit, listen } from "@tauri-apps/api/event";
  import { addBase64HtmlSyntax } from "../../utils/index";

  export let params: { propertyId: string };

  export let addingNewRoomGroup: boolean = false;

  let property: Property;

  $: roomGroupSummary = "To be done later"

  $: roomGroupArray = [];

  function toggleNewRoomGroup() {
    addingNewRoomGroup = !addingNewRoomGroup;
  }

  function addNewRoomGroup(event: CustomEvent<TileConfig>) {

    console.log(event.detail);
    console.log(property.id);

    let newRoomGroupRequest = {
      name: event.detail.title,
      property_id: property.id,
      image: event.detail.image
    };

    console.log(`Prepared newRoomGroupRequest: `);
    console.table(newRoomGroupRequest);

    emit("add_new_room_group", newRoomGroupRequest);
    emit("get_room_groups", {
      property_id: property.id
    })

    toggleNewRoomGroup();
  }

  onMount(async () => {
    await emit("get_property_data", params.propertyId);
    await listen("property_data", (event) => {
      console.log(event);
      property = event.payload as Property;
    });
    await listen("room_groups_data", (event) => {
      console.log(event);
    })
  });
</script>

{#if property}
  <MainLayout header={property.name} imageUrl={addBase64HtmlSyntax(property.image, "jpeg")}>
    <div class="manage-property">
      <div class="room-groups-controls">
        <h4 class="room-group-summary">{roomGroupSummary}</h4>
        <button on:click={toggleNewRoomGroup} class="add-room-group">
          Add RoomGroup
        </button>
      </div>
      <div class="room-groups content-container">
        {#each roomGroupArray as roomGroup}
          <RoomGroupComponent {roomGroup} />
        {/each}
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
