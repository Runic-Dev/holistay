<script lang="ts">
    import { beforeUpdate } from "svelte";
    import { Properties } from "../../lib/test_data/properties";
    import MainLayout from "../../MainLayout.svelte";
    import type { Property } from "src/types";
    import { TileType } from "../../../src/enums/ui";
    import Description from "./Description.svelte";
    import Tile from "../../../src/common/Tile.svelte";
    import { push } from "svelte-spa-router";
    import RoomGroup from "../../../src/models/RoomGroup";
    export let roomGroup: RoomGroup;
    export let imageUrl: string = "";
    export let params: {
      roomGroupId: string
    };
    beforeUpdate(() => {
      console.log(params.roomGroupId);
      roomGroup = Properties.data.map((property: Property) => {
        return property.roomGroups
      }).flat().filter((rg) => rg.id == params.roomGroupId).at(0) ?? new RoomGroup("Default")
      imageUrl = roomGroup.imageUrl;
      console.log(imageUrl);
    })
</script>

<MainLayout header={roomGroup.name} {imageUrl}>
  <div class="room-group-container content-container">
    <Description description={roomGroup.description} />
    <div class="room-plan">
      <h4>Rooms</h4>
      <div class="room-plan-container">
        {#each roomGroup.rooms as room}
          <Tile tileConfig={{
            type: TileType.Default,
            title: room.name,
            imageUrl: room.imageUrl,
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
