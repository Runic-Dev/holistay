<script lang="ts">
    import { beforeUpdate } from "svelte";
    import MainLayout from "../../MainLayout.svelte";
    import { TileType } from "../../../src/enums/ui";
    import Description from "./Description.svelte";
    import Tile from "../../../src/common/Tile.svelte";
    import { push } from "svelte-spa-router";
    import type RoomGroup from "../../../src/models/RoomGroup";
    export let roomGroup: RoomGroup;
    export let imageUrl: string = "";
    export let params: {
      roomGroupId: string
    };
    beforeUpdate(() => {
    })
    console.log(params.roomGroupId);
</script>

<MainLayout header={roomGroup.name} {imageUrl}>
  <div class="room-group-container content-container">
    <div class="image-section">
      <h4>Image</h4>
      {#if roomGroup.imageUrl == "" || !roomGroup.imageUrl }
        <button>Upload Image</button>
      {:else}
        <button>Change Image</button>
      {/if}
    </div>
    <Description description={roomGroup.description} />
    <div class="room-plan">
      <h4>Rooms</h4>
      <div class="room-plan-container">
        {#each roomGroup.rooms as room}
          <Tile tileConfig={{
            type: TileType.Default,
            title: room.name,
            image: room.imageUrl,
            clickAction: () => push(`/room/${room.id}`)
          }}/>
        {/each}
      </div>
    </div>
  </div>
</MainLayout>

<style lang="scss">
  @import "./src/lib/app.scss";
  .room-plan-container {
    display: flex;
    flex-wrap: wrap;
  }
</style>
