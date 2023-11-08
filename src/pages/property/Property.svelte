<script lang="ts">
  import { Properties } from "../../lib/test_data/properties";
  import type { Property } from "src/types";
  import MainLayout from "../../MainLayout.svelte";
  import RoomGroup from "./RoomGroup.svelte";
  import { onMount } from "svelte";

  export let params: { propertyId: string };

  export let addingNewRoomGroup: boolean = false;

  let property: Property;

  // TODO: We need loading state, loaded state and error state for this
  $: roomGroupSummary = property
    ? `${property.name} has ${property.roomGroups.length} room group${
        property.roomGroups.length > 1 ? "s" : ""
      }`
    : "There was an issue loading the Property";

  function toggleNewRoomGroup() {
    addingNewRoomGroup = !addingNewRoomGroup;
  }

  onMount(() => {
    let properties: Property[] = Properties.data;
    property = properties.filter((p) => p.id == params.propertyId).at(0);
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
        {#if property.roomGroups.length > 0}
          {#each property.roomGroups as roomGroup}
            <RoomGroup {roomGroup} />
          {/each}
          {#if addingNewRoomGroup}
            <RoomGroup on:cancelNewRoomGroup={toggleNewRoomGroup} />
          {/if}
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
