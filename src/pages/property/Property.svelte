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

  // TODO: We need loading state, loaded state and error state for this
  // $: roomGroupSummary = property
  //   ? `${property.name} has ${property.roomGroups.length} room group${
  //       property.roomGroups.length > 1 ? "s" : ""
  //     }`
  //   : "There was an issue loading the Property";
  $: roomGroupSummary = "To be done later"

  $: roomGroupArray = [];

  // TODO: What the hell is this?!
  function toggleNewRoomGroup() {
    addingNewRoomGroup = !addingNewRoomGroup;
  }

  function addNewRoomGroup(event: CustomEvent<TileConfig>) {
    let newRoomGroup = new RoomGroup(event.detail.title);
    let newRoomGroupArray = roomGroupArray;
    newRoomGroupArray.push(newRoomGroup);
    roomGroupArray = newRoomGroupArray;
    toggleNewRoomGroup();
  }

  onMount(async () => {
    await emit("get_property_data", params.propertyId);
    await listen("property_data", (event) => {
      console.log(event);
      property = event.payload as Property;
    });
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
