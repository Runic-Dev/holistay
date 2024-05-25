<script lang="ts">
  import { userStore, displayTopBarStore } from "./store";
  import * as Avatar from "$lib/components/ui/avatar";
  import * as Sheet from "$lib/components/ui/sheet";
  import BreadcrumbGenerator from "./common/BreadcrumbGenerator.svelte";
  $: username = $userStore.user ? $userStore.user["name"] : null;
  $: displayTopBar = $displayTopBarStore;
</script>

<div>
  {#if displayTopBar}
    <div class="top-bar p-2 flex flex-row justify-end">
      {#if username != null}
        <Sheet.Root>
          <Sheet.Trigger>
            <Avatar.Root class="block">
              <Avatar.Image src="" />
              <Avatar.Fallback>{username[0].toUpperCase()}</Avatar.Fallback>
            </Avatar.Root>
          </Sheet.Trigger>
          <Sheet.Content>
            <Sheet.Header>
              <Sheet.Title>Here is the sheet</Sheet.Title>
              <Sheet.Description>The description</Sheet.Description>
            </Sheet.Header>
          </Sheet.Content>
        </Sheet.Root>
      {/if}
    </div>
  {/if}
  <BreadcrumbGenerator />
  <slot topBar={displayTopBar} />
</div>

<style lang="scss">
</style>
