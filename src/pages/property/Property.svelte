<script lang="ts">
  import { Properties } from "../../lib/test_data/properties";

  import type { Property, TileConfig } from "src/types";
  import { TileType } from "../../enums/ui";
  import MainLayout from "../../MainLayout.svelte";
  import RoomGroupComponent from "./RoomGroup.svelte";
  import { onMount } from "svelte";
  import Tile from "../../common/Tile.svelte";
  import RoomGroup from "../../models/RoomGroup";

  export let params: { propertyId: string };

  export let addingNewRoomGroup: boolean = false;

  let property: Property;

  // TODO: We need loading state, loaded state and error state for this
  $: roomGroupSummary = property
    ? `${property.name} has ${property.roomGroups.length} room group${
        property.roomGroups.length > 1 ? "s" : ""
      }`
    : "There was an issue loading the Property";

  $: roomGroupArray = [];

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

  onMount(() => {
    let properties: Property[] = Properties.data;
    property = properties.filter((p) => p.id == params.propertyId).at(0);
    roomGroupArray = property.roomGroups;
  });
</script>

{#if property}
  <MainLayout header={property.name} imageUrl={property.imageUrl}>
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
              imageUrl: null,
              clickAction: null
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
