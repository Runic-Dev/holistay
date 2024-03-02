<script lang="ts">
    import MainLayout from "./MainLayout.svelte";
    import LoginScreen from "./LoginScreen.svelte";
    import { userStore } from "./store";
    import LoggedInDashboard from "./LoggedInDashboard.svelte";
    import { onMount } from "svelte";
    import { emit, listen } from "@tauri-apps/api/event";
    import type User from "./models/User";

    $: loggedInUser = $userStore.user;
    $: if (loggedInUser) {
        console.log($userStore);
    } else {
        console.log("No user");
    }

    onMount(async () => {
      await emit("init");
      listen<InitResponse>("init_response", (event) => {
        userStore.set({
          user: event.payload.user
        })
      });
    });
    
    type InitResponse = {
      user: User | null
    }

</script>

<MainLayout>
    {#if loggedInUser}
        <LoggedInDashboard />
    {:else}
        <LoginScreen />
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
