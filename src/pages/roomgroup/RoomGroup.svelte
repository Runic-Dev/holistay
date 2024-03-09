<script lang="ts">
  import { onMount } from "svelte";
  import MainLayout from "@/MainLayout.svelte";
  import { TileType } from "@/enums/ui";
  import Tile from "@/common/Tile.svelte";
  import { push } from "svelte-spa-router";
  import { propertyStore } from "@/store";
  import { addBase64HtmlSyntax } from "@/utils";
  import { DescribableEntity } from "@/common/types";
  import Description from "@/common/Description.svelte";
  import { listen, emit } from "@tauri-apps/api/event";
  import type { RoomResponse, TileConfig } from "@/types";
  import Room from "@/models/Room";

  export let params: {
    propertyId: string;
    roomGroupId: string;
  };

  let addingNewRoom = false;

  $: roomGroup = null;

  function toggleNewRoom() {
    addingNewRoom = !addingNewRoom;
  }

  function addNewRoom(event: CustomEvent<TileConfig>) {
    let newRoomRequest = {
      name: event.detail.title,
      property_id: params.propertyId,
      room_group_id: roomGroup.id,
      image: event.detail.image,
    };

    emit("add_new_room", newRoomRequest);
    emit("get_rooms", {
      property_id: params.propertyId,
      room_group_id: roomGroup.id,
    });

    toggleNewRoom();
  }

  onMount(async () => {
    await listen<RoomResponse[]>("rooms_loaded", (event) => {
      propertyStore.update(x => {
        x.properties.find(p => p.id == params.propertyId).roomGroups.find(rg => rg.id == roomGroup.id).rooms = event.payload.map(y => {
          return Room.FromResponse(y);
        });
        return x;
      });
    });
    propertyStore.subscribe((x) => {
      let parentProperty = x.properties.find((p) => p.id == params.propertyId);
      if (parentProperty.roomGroups) {
        roomGroup = parentProperty.roomGroups?.find(
          (rm) => rm.id == params.roomGroupId,
        );
      }
    });
    emit("get_rooms", {
      property_id: params.propertyId,
      room_group_id: roomGroup.id
    })
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
      <Description
        type={DescribableEntity.RoomGroup}
        id={roomGroup.id}
        description={roomGroup.description ?? "No description"}
      />
      <div class="room-controls">
        <button on:click={toggleNewRoom} class="add-room"> Add Room </button>
      </div>
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
          {#if addingNewRoom}
            <Tile
              tileConfig={{
                type: TileType.NewRoom,
                title: null,
                image: null,
                clickAction: null,
              }}
              on:confirmedRoom={addNewRoom}
            />
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
