<script lang="ts">
  import * as Card from "$lib/components/ui/card";
  import { Button } from  "$lib/components/ui/button"
  import { handleImageEncodingForHtml } from "$lib/utils";
  import type { Property } from "@/models/Property";
  import { push } from "svelte-spa-router";

  export let property: Property;
</script>

<Card.Root class="w-full sm:w-64 lg:w-80">
  <Card.Header>
    <Card.Title>{property.name}</Card.Title>
  </Card.Header>
  <Card.Content>
    <img
      src={handleImageEncodingForHtml(property.image)}
      alt={property.name}
      class="w-full h-32 object-cover rounded-md"
    />
    <Button class="my-4" on:click={() => push(`/property/${property.id}`)}
      >View Property</Button
    >
  </Card.Content>
</Card.Root>
