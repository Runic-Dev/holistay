<script lang="ts">
  import type { TileConfig } from "src/types";
  import { TileType } from "../enums/ui";
  import { onMount } from "svelte";
  let onClickFunc: () => Promise<void> | void;
  export let tileConfig: TileConfig;
  let newRoomGroupName: string = "";

  function confirmNewRoomGroup() {
    tileConfig.type = TileType.Default;
    tileConfig.title = newRoomGroupName;
  }

  onMount(() => {
    if (!tileConfig.clickAction) {
      onClickFunc = () => {};
      return;
    }
    onClickFunc = tileConfig.clickAction;
  });
</script>

{#if tileConfig.type == TileType.Default}
  <div
    on:keydown={() => tileConfig.clickAction()}
    on:click={() => tileConfig.clickAction()}
    class="tile"
    style="background-image: url({tileConfig.imageUrl});"
  >
    <h4 class="name">{tileConfig.title}</h4>
    <div class="overlay" />
  </div>
{:else if tileConfig.type == TileType.NewRoomGroup}
  <div class="tile newRoomGroup">
    <label for="roomGroupName">RoomGroup name:</label>
    <input bind:value={newRoomGroupName} id="roomGroupName" type="text" />
    <button on:click={() => confirmNewRoomGroup()}>Create</button>
    <div class="overlay" />
  </div>
{/if}

<style lang="scss">
  @import "./src/lib/app.scss";
  .tile {
    position: relative;
    @include tile;

    &.newRoomGroup {
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;

      input, button {
        z-index: 5;
      }
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
  .name {
    color: white;
    margin: 0;
    padding: $padding;
    position: absolute;
    bottom: 0;
    left: 0;
    padding: $padding;
    z-index: 5;
    font-size: 1.5rem;
  }
</style>
