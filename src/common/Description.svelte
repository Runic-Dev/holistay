<script lang="ts">
  import { listen, emit } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { DescribableEntity } from "./types";
  import { propertyStore } from "@/store";
  import { params } from "svelte-spa-router";

  export let type: DescribableEntity;
  export let id: string;
  export let description: string;
  let editable: boolean = false;
  let tempDescription: string = description;

  function saveChanges() {
    switch (type) {
      case DescribableEntity.Property: {
        emit("new_property_description", {
          id,
          description: tempDescription,
        });
      }
      case DescribableEntity.RoomGroup: {
        emit("new_room_group_description", {
          id,
          description: tempDescription,
        });
      }
      default: {
        console.error("Entity not describable");
      }
    }
    editable = false;
  }

  onMount(() => {
    if (type == DescribableEntity.Property) {
      listen<DescUpdated>(`property_desc_updated_for_${id}`, (event) => {
        propertyStore.update(ps => {
          let newProperties = ps.properties;
          newProperties.find((p) => p.id == id).description =
            event.payload.description;
          return { properties: newProperties };
        });
      });
    }

    if (type == DescribableEntity.RoomGroup) {
      listen<DescUpdated>(`room_group_desc_updated_for_${id}`, (event) => {
        propertyStore.update(ps => {
          let newProperties = ps.properties;
          newProperties.find(p => p.roomGroups.some(rg => rg.id == id))
          .roomGroups.find(rg => rg.id == id)
          .description = event.payload.description;
          return { properties: newProperties };
        });
      });
    }
  });

  type DescUpdated = {
    description: string;
  };
</script>

<div class="description-section">
  <h4>Description</h4>
  {#if editable}
    <textarea bind:value={tempDescription} class="current-description" />
    <button on:click={saveChanges}>Save Changes</button>
    <button on:click={() => (editable = !editable)}>Cancel</button>
  {:else}
    <p class="current-description">{description}</p>
    <button on:click={() => (editable = !editable)}>Edit Description</button>
  {/if}
</div>

<style lang="scss">
  @import "./src/lib/app.scss";
  .description-section {
    textarea {
      width: 100%;
      color: white;
      font-size: 1rem;
    }
    .current-description {
      padding: $padding;
      background-color: rgb(25, 25, 25);
    }
  }
</style>
