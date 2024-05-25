<script lang="ts">
  import * as Breadcrumb from "$lib/components/ui/breadcrumb";
  import { userStore, breadcrumbInfoStore } from "@/store";
</script>

{#if $userStore.user && $userStore.user.name && $breadcrumbInfoStore.propertyName}
  <Breadcrumb.Root class="p-4">
    <Breadcrumb.List>
      <Breadcrumb.Item link="/">
        {$userStore.user.name}
      </Breadcrumb.Item>
      <Breadcrumb.Separator />
      <Breadcrumb.Page>
        {$breadcrumbInfoStore.propertyName}
      </Breadcrumb.Page>
    </Breadcrumb.List>
  </Breadcrumb.Root>
{:else if $userStore.user && $userStore.user.name}
  <Breadcrumb.Root class="p-4">
    <Breadcrumb.List>
      <Breadcrumb.Page>{$userStore.user.name}</Breadcrumb.Page>
    </Breadcrumb.List>
  </Breadcrumb.Root>
{/if}
