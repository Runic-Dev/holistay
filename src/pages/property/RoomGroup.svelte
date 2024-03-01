<script lang="ts">
  import type RoomGroup from "../../../src/models/RoomGroup";
  import { push } from "svelte-spa-router";
  import Tile from "../../../src/common/Tile.svelte";
  import type { TileConfig } from "src/types";
  import { TileType } from "../../../src/enums/ui";

  export let roomGroup: RoomGroup;
  export let propertyId: string;
  let tileConfig: TileConfig = {
    type: TileType.Default,
    title: roomGroup.name,
    image: roomGroup.imageUrl,
    clickAction: () => push(`/property/${propertyId}/roomgroup/${roomGroup.id}`)
  };
</script>

<Tile {tileConfig} />

<style lang="scss">
  @import "./src/lib/app.scss";
  .room-group {
    @include tile;

    overflow: hidden;
    position: relative;

    &.new-room-group {
      flex-direction: column;
      justify-content: center;
      border: 1px dashed black;
      .new-name {
        text-align: center;
      }
    }

    .overlay {
      position: absolute;
      width: 100%;
      height: 100%;
      background: linear-gradient(
        to top,
        rgba(25, 25, 25, 0.8) 10%,
        rgba(100, 25, 25, 0) 50%
      );
      z-index: 0;
    }

    .room-group-name {
      margin: 0;
      padding: $padding;
      position: absolute;
      bottom: 0;
      left: 0;
      padding: $padding;
      z-index: 5;
      font-size: 1.5rem;
    }
  }
  .rooms {
    display: flex;
    flex-direction: column;
  }
</style>
