<script lang="ts">
  import { onMount } from "svelte";
  import MainLayout from "@/MainLayout.svelte";
  import { TileType } from "@/enums/ui";
  import Description from "./Description.svelte";
  import Tile from "@/common/Tile.svelte";
  import { push } from "svelte-spa-router";
  import { propertyStore } from "@/store";
  import { addBase64HtmlSyntax } from "@/utils";

  export let params: {
    propertyId: string;
    roomGroupId: string;
  };

  $: roomGroup = null;

  onMount(() => {
    propertyStore.subscribe((x) => {
      roomGroup = x.properties
        .find((p) => p.id == params.propertyId)
        .roomGroups.find((rm) => rm.id == params.roomGroupId);
    });
  });
</script>

<MainLayout
  header={roomGroup?.name ?? "Loading..."}
  imageUrl={roomGroup?.imageUrl
    ? addBase64HtmlSyntax(roomGroup.imageUrl, "jpeg")
    : null}
>
  {#if roomGroup}
    <div class="room-group-container content-container">
      <div class="image-section">
        <h4>Image</h4>
        {#if roomGroup.imageUrl == "" || !roomGroup.imageUrl}
          <button>Upload Image</button>
        {:else}
          <button>Change Image</button>
        {/if}
      </div>
      <Description description={roomGroup.description ?? "No description"} />
      <div class="room-plan">
        <h4>Rooms</h4>
        <div class="room-plan-container">
          {#if roomGroup?.rooms}
            {#each roomGroup.rooms as room}
              <Tile
                tileConfig={{
                  type: TileType.Default,
                  title: room.name,
                  image: room.imageUrl,
                  clickAction: () =>
                    push(
                      `/property/${params.propertyId}/roomgroup/${params.roomGroupId}/room/${room.id}`,
                    ),
                }}
              />
            {/each}
          {/if}
        </div>
      </div>
    </div>
  {:else}
    <div>Loading...</div>
  {/if}
</MainLayout>

<style lang="scss">
  @import "./src/lib/app.scss";
  .room-plan-container {
    display: flex;
    flex-wrap: wrap;
  }
</style>
