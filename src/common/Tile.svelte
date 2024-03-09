<script lang="ts">
  import type { TileConfig } from "src/types";
  import { TileType } from "@/enums/ui";
  import { onMount, createEventDispatcher } from "svelte";
  import { dialog } from "@tauri-apps/api";
  let onClickFunc: () => Promise<void> | void;
  export let tileConfig: TileConfig;

  let newRoomName: string = "";
  let newRoomGroupName: string = "";
  let newPropertyName: string = "";
  let tileImageFilePath: string | null;
  let loadedImage: string | null = null;

  const dispatch = createEventDispatcher();

  function getPropertyBackgroundImg(): string {
    return `data:image/jpeg;base64,${tileConfig.image}`;
  }

  function confirmNewRoom() {
    tileConfig.type = TileType.Default;
    tileConfig.title = newRoomName;
    dispatch("confirmedRoom", {
      title: newRoomName,
      image: tileImageFilePath
    });
  }


  function confirmNewRoomGroup() {
    tileConfig.type = TileType.Default;
    tileConfig.title = newRoomGroupName;
    dispatch("confirmedRoomGroup", {
      title: newRoomGroupName,
      image: tileImageFilePath
    });
  }

  function confirmNewProperty() {
    tileConfig.type = TileType.Default;
    tileConfig.title = newPropertyName;
    dispatch("confirmedProperty", {
      name: newPropertyName,
      image: tileImageFilePath,
    });
  }

  onMount(() => {
    if (!tileConfig.clickAction) {
      onClickFunc = () => {};
      return;
    }
    onClickFunc = tileConfig.clickAction;
    if(tileConfig.image) {
      loadedImage = getPropertyBackgroundImg();
    }
  });

  async function selectImage(
    _: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement },
  ) {
    let filePath = await dialog.open();
    if (Array.isArray(filePath)) {
      console.error("Only select one");
    } else {
      console.log(filePath);
      tileImageFilePath = filePath;
    }
  }
</script>

{#if tileConfig.type == TileType.Default}
  {#if loadedImage}
  <div
    on:keydown={() => tileConfig.clickAction()}
    on:click={() => tileConfig.clickAction()}
    class="tile"
    style="background-image: url({loadedImage});"
  >
    <h4 class="name">{tileConfig.title}</h4>
    <div class="overlay" />
  </div>
  {:else}
  <div
    on:keydown={() => tileConfig.clickAction()}
    on:click={() => tileConfig.clickAction()}
    class="tile"
  >
    <h4 class="name">{tileConfig.title}</h4>
    <div class="overlay" />
  </div>
  {/if}
{:else if tileConfig.type == TileType.NewRoom}
  <div class="tile newRoom">
    <label for="roomName">Room name:</label>
    <input bind:value={newRoomName} id="roomName" type="text" />
    <button on:click={confirmNewRoom}>Create</button>
    <button on:click={selectImage}>Select Image</button>
    <div class="overlay" />
  </div>
{:else if tileConfig.type == TileType.NewRoomGroup}
  <div class="tile newRoomGroup">
    <label for="roomGroupName">RoomGroup name:</label>
    <input bind:value={newRoomGroupName} id="roomGroupName" type="text" />
    <button on:click={confirmNewRoomGroup}>Create</button>
    <button on:click={selectImage}>Select Image</button>
    <div class="overlay" />
  </div>
{:else if tileConfig.type == TileType.NewProperty}
  <div class="tile newProperty">
    <label for="propertyName">Property name:</label>
    <input bind:value={newPropertyName} id="propertyName" type="text" />
    <button on:click={confirmNewProperty}>Create</button>
    <button on:click={selectImage}>Select Image</button>
    <div class="overlay" />
  </div>
{/if}

<style lang="scss">
  @import "./src/lib/app.scss";
  .tile {
    position: relative;
    @include tile;

    &.newRoomGroup,
    &.newProperty,
    &.newRoom {
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;

      input,
      button {
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
