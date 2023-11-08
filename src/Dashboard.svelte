<script>
  import MainLayout from "./MainLayout.svelte";
  import PropertiesOverview from "./PropertiesOverview.svelte";
</script>

<MainLayout>
  <PropertiesOverview />
</MainLayout>

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
