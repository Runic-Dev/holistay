<script lang="ts">
    import MainLayout from "./MainLayout.svelte";
    import LoginScreen2 from "./LoginScreen2.svelte";
    import { userStore } from "./store";
    import LoggedInDashboard2 from "./LoggedInDashboard2.svelte";
    import { onMount } from "svelte";
    import { emit, listen } from "@tauri-apps/api/event";
    import type User from "./models/User";

    $: loggedInUser = null;

    onMount(async () => {
      userStore.subscribe(us => loggedInUser = us.user);
      await listen<InitResponse>("init_response", (event) => {
        userStore.set(event.payload);
      });
      await emit("init");
    });

    type InitResponse = {
      user: User | null
    }

</script>

<MainLayout>
    {#if loggedInUser}
        <LoggedInDashboard2 />
    {:else}
        <LoginScreen2 />
    {/if}
</MainLayout>

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
