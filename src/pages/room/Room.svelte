<script lang="ts">
  import MainLayout from "@/MainLayout.svelte";
  import { getContext, onMount } from "svelte";
  import { propertyStore } from "@/store";
  import { addBase64HtmlSyntax } from "@/utils";
  import type { BackendHandler } from "@/backendHandlers";
  export let params: {
    roomId: string;
    roomGroupId: string;
    propertyId: string;
  };
  $: room = null;
  getContext<BackendHandler>("backendHandler").foo();
  onMount(() => {
    propertyStore.subscribe((ps) => {
      room = ps.properties
        .find((p) => p.id == params.propertyId)
        .roomGroups.find((rg) => rg.id == params.roomGroupId)
        .rooms.find((r) => r.id == params.roomId);
    });
  });
</script>

{#if room}
  <MainLayout
    header={room?.name ?? "Loading..."}
    imageUrl={room?.imageUrl
      ? addBase64HtmlSyntax(room.imageUrl, "jpeg")
      : null}
  ></MainLayout>
{:else}
  Loading..
{/if}
