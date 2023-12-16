<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import MainLayout from "./MainLayout.svelte";
  import PropertiesOverview from "./PropertiesOverview.svelte";
  import { onMount } from "svelte";
    import InitScreen from "./InitScreen.svelte";

  async function getCurrentUser() {
    let response = await invoke("get_current_user");
    console.log(response);
  }

  let loggedInUser = null;

  onMount(async () => {
    let response = await getCurrentUser();
    if(response["status"] == "success") {
      loggedInUser = response["body"];
    }
  });
</script>

<MainLayout>
  {#if loggedInUser}
  <PropertiesOverview />
  {:else}
  <InitScreen />
  {/if}
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
