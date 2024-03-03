<script lang="ts">
  import type { Room } from "@/types";

  export let room: Room;
</script>

<div class="room">
  <p>Room Name: {room.name}</p>
</div>
