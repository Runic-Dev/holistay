<script lang="ts">
  import MainLayout from "@/MainLayout.svelte";
  import {getContext, onDestroy, onMount} from "svelte";
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
  onMount(async () => {
    // Having to do null / undefined checks everywhere due to weird JS behaviour
    propertyStore.subscribe((ps) => {
      if(ps) {
        let property = ps.properties.find(p => p.id == params.propertyId);
        if(property && property.roomGroups) {
          let roomGroup = property.roomGroups.find(rg => rg.id == params.roomGroupId);
          if(roomGroup && roomGroup.rooms) {
            room = roomGroup.rooms.find(r => r.id == params.roomId);
          }
        }
      }
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
