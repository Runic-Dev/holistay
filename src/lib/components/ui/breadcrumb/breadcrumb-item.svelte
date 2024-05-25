<script lang="ts">
  import type { HTMLLiAttributes } from "svelte/elements";
  import { cn } from "$lib/utils.js";
  import { push } from "svelte-spa-router";
  import { breadcrumbInfoStore } from "@/store";

  type $$Props = HTMLLiAttributes & {
    el?: HTMLLIElement;
    link: string;
  };

  export let el: $$Props["el"] = undefined;
  let className: $$Props["class"] = undefined;
  export { className as class };
  export let link: $$Props["link"] = undefined;

</script>

<li
  on:click={() => {
    breadcrumbInfoStore.set({
      propertyName: null
    });
    push(link);
  }}
  bind:this={el}
  class={cn("inline-flex items-center gap-1.5", className)}
>
  <slot />
</li>
