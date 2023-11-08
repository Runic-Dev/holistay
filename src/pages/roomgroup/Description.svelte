<script lang="ts">
  export let description: string;
  export let editable: boolean = false;
  export let tempDescription: string = description;

  function saveChanges() {
    description = tempDescription;
    editable = false;
  }

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
