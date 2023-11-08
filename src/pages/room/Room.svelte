<script lang="ts">
  import type { Property, Room } from "src/types";
  import MainLayout from "../../../src/MainLayout.svelte";
  import { Properties } from "../../../src/lib/test_data/properties";
  import { beforeUpdate } from "svelte";
  export let room: Room;
  export let params: {
    roomId: string;
  };
  beforeUpdate(() => {
    console.log(params.roomId);
    room = Properties.data
      .map((property: Property) => {
        return property.roomGroups;
      })
      .flat()
      .map((rg) => rg.rooms)
      .flat()
      .filter((r) => r.id == params.roomId)
      .at(0);
  });
</script>

<MainLayout header={room.name} imageUrl={room.imageUrl}>
</MainLayout>
