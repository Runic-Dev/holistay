<script lang="ts">
    import {DescribableEntity} from "@/common/types.js";
    import {TileType} from "@/enums/ui.js";
    import {push} from "svelte-spa-router";
    import Tile from "@/common/Tile.svelte";
    import Description from "@/common/Description.svelte";
    import {emit, listen} from "@tauri-apps/api/event";
    import type { RoomResponse, TileConfig } from "@/types";
    import type RoomGroup from "@/models/RoomGroup";
    import {onMount} from "svelte";
    import {propertyStore} from "@/store";
    import Room from "@/models/Room";
    let addingNewRoom = false;
    function toggleNewRoom() {
        addingNewRoom = !addingNewRoom;
    }
    export let roomGroup: RoomGroup;
    export let params: {
        propertyId: string;
        roomGroupId: string;
    };

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
        await emit("get_rooms", {
            property_id: params.propertyId,
            room_group_id: roomGroup.id
        })
    })
</script>

<div class="room-group-container content-container">
    <div class="image-section">
        <h4>Image</h4>
        {#if roomGroup.imageUrl === "" || !roomGroup.imageUrl}
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
                    <Tile tileConfig={{
                  type: TileType.Default,
                  title: room.name,
                  image: room.image,
                  clickAction: () =>
                    push(
                      `/property/${params.propertyId}/roomgroup/${params.roomGroupId}/room/${room.id}`,
                    ),
                }} />
                {/each}
            {/if}
            {#if addingNewRoom}
                <Tile tileConfig={{
                type: TileType.NewRoom,
                title: null,
                image: null,
                clickAction: null,
              }} on:confirmedRoom={addNewRoom} />
            {/if}
        </div>
    </div>
</div>
