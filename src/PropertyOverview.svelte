<script lang="ts">
  import type { Property } from "src/types";
  import { push } from "svelte-spa-router";
  import Tile from "./common/Tile.svelte";
  export let property: Property;
  export let propertyLink: string = `/property/${property.id}`;
</script>

<Tile
  onClickFunc={() => push(propertyLink)}
  name={property.name}
  imageUrl={property.imageUrl}
/>

<style lang="scss">
  @import "./src/lib/app.scss";
  .property-overview {
    @include tile;
  }
</style>
