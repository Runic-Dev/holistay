<script lang="ts">
  import type { Room } from "src/types";
  import MainLayout from "../../../src/MainLayout.svelte";
  import { onMount } from "svelte";
  import { propertyStore } from "../../../src/store";
  export let room: Room;
  export let params: {
    roomId: string;
    roomGroupId: string;
    propertyId: string;
  };
  onMount(() => {
    propertyStore.subscribe(ps => 
      room = ps.properties.find(p => p.id == params.propertyId)
      .roomGroups.find(rg => rg.id == params.roomGroupId)
      .rooms.find(r => r.id == params.roomId)
    );
  });
</script>

<MainLayout header={room.name} imageUrl={room.imageUrl}>
</MainLayout>
