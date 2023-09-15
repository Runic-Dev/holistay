<script>
  import PropertiesOverview from "./PropertiesOverview.svelte";

</script>
<div class="main-dashboard">
  <div class="top-bar">
    <div class="center">Holistay</div>
    <div class="notifications">Notifications</div>
  </div>
  <nav>
    <li>Property Manager</li>
    <li>Bookings</li>
  </nav>

  <div class="main-content">
    <div class="property-overview">
      <PropertiesOverview />
    </div>
  </div>
  
</div>

<style lang="scss">
  .main-dashboard {
    .top-bar {
      display: flex;
      justify-content: space-between;
      div {
        padding: 16px;

        &.center {
          font-size: 2rem;
        }
      }
    }
    nav {
      display: flex;
      justify-content: flex-end;
      
      li {
        list-style: none;
        padding: 16px;
      }
    }
  }
</style>
