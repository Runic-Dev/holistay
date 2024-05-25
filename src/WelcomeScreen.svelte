<script lang="ts">
  import MainLayout from "./MainLayout.svelte";
  import LoginScreen from "./LoginScreen.svelte";
  import { userStore } from "./store";
  import LoggedInDashboard from "./LoggedInDashboard.svelte";
  import { onMount } from "svelte";
  import { emit, listen } from "@tauri-apps/api/event";
  import type User from "./models/User";

  onMount(async () => {
    if (!$userStore.user) {
      await listen<InitResponse>("init_response", (event) => {
        userStore.set(event.payload);
      });
      await emit("init");
    }
  });

  type InitResponse = {
    user: User | null;
  };
</script>

{#if $userStore.user}
  <MainLayout>
    <LoggedInDashboard />
  </MainLayout>
{:else}
  <LoginScreen />
{/if}

<style lang="scss">
  @import "./styles.css";
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
