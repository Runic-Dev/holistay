<script lang="ts">
  import type { Property, TileConfig } from "./types";
  import { TileType } from "./enums/ui";
  import { push } from "svelte-spa-router";
  import Tile from "./common/Tile.svelte";
  export let property: Property;
  export let propertyLink: string = `/property/${property.id}`;
  let tileConfig: TileConfig = {
    type: TileType.Default,
    title: property.name,
    image: property.image,
    clickAction: () => push(propertyLink),
  };
</script>

<Tile {tileConfig}/>

<style lang="scss">
  @import "./src/lib/app.scss";
  .property-overview {
    @include tile;
  }
</style>
