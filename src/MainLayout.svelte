<script lang="ts">
  import { userStore, displayTopBarStore } from "./store";
  import * as Avatar from "$lib/components/ui/avatar";
  $: username = $userStore.user ? $userStore.user["name"] : null;
  $: displayTopBar = $displayTopBarStore;
</script>

<div>
  {#if displayTopBar}
    <div class="top-bar p-2 flex flex-row justify-end">
      {#if username != null}
        <Avatar.Root class="block">
          <Avatar.Image src="" />
          <Avatar.Fallback>{username[0].toUpperCase()}</Avatar.Fallback>
        </Avatar.Root>
      {/if}
    </div>
  {/if}
  <slot topBar={displayTopBar} />
</div>

<style lang="scss">
</style>
