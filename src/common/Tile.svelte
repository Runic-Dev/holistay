<script lang="ts">
  export let onClickFunc: () => Promise<void>;
  export let imageUrl: string;
  export let name: string;
</script>
<div
  on:keydown={() => onClickFunc()}
  on:click={() => onClickFunc()}
  class="tile"
  style="background-image: url({imageUrl});"
>
  <h4 class="name">{name}</h4>
  <div class="overlay" />
</div>

<style lang="scss">
  @import "./src/lib/app.scss";
  .tile {
    position: relative;
    @include tile;
  }
  .overlay {
    position: absolute;
    width: 100%;
    height: 100%;
    background: linear-gradient(
      to top,
      rgba(25, 25, 25, 0.8) 10%,
      rgba(100, 25, 25, 0) 50%
    );
    z-index: 0;
  }
  .name {
    color: white;
    margin: 0;
    padding: $padding;
    position: absolute;
    bottom: 0;
    left: 0;
    padding: $padding;
    z-index: 5;
    font-size: 1.5rem;
  }
</style>
