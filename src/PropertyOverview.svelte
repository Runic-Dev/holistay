<script lang="ts">
  import type { PropertyOverviewData } from "src/types";
  export let property: PropertyOverviewData;
</script>

<div class="property-overview">
  <div class="splash">
    <h2>{property.name}</h2>
    <img src={property.imageUrl} alt="A Hotel" />
  </div>
  <div class="menu">
    <ul>
      <a href="/manage"><li>Manage</li></a>
      <li>Bookings</li>
    </ul>
  </div>
</div>

<style lang="scss">
  @import "./src/lib/app.scss";
  .property-overview {
    display: flex;
    padding: $padding;
    text-align: left;

    div {
      max-width: 50%;

      &.menu {
        width: 100%;
        display: flex;
        flex-direction: column;
        justify-content: center;

        ul {
          display: flex;
          width: 100%;
          margin: 0;
          justify-content: space-evenly;
        }

        li {
          list-style: none;
          padding: $padding;
          background-color: $lightblue;
          border-radius: $padding;
          color: white;
        }
      }
    }

    img {
      width: 100%;
      border-radius: $padding;
    }
  }
</style>
