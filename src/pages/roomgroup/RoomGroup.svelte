<script lang="ts">
  import { onMount } from "svelte";
  import MainLayout from "@/MainLayout.svelte";
  import { propertyStore } from "@/store";
  import { addBase64HtmlSyntax } from "@/utils";
  import RoomGroupContent from "@/pages/roomgroup/RoomGroupContent.svelte";

  export let params: {
    propertyId: string;
    roomGroupId: string;
  };

  $: roomGroup = null;

  onMount(async () => {
    propertyStore.subscribe((x) => {
      let parentProperty = x.properties.find((p) => p.id == params.propertyId);
      if (parentProperty.roomGroups) {
        roomGroup = parentProperty.roomGroups?.find(
          (rm) => rm.id == params.roomGroupId,
        );
      }
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
    <RoomGroupContent roomGroup={roomGroup} params={params} />
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
