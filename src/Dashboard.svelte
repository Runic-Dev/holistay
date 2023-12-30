<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import MainLayout from "./MainLayout.svelte";
    import PropertiesOverview from "./PropertiesOverview.svelte";
    import { onMount } from "svelte";
    import InitScreen from "./InitScreen.svelte";
    import { listen } from "@tauri-apps/api/event";

    let loggedInUser = null;

    onMount(async () => {
        await listen("user_logged_in", (payload) => {
            console.log(payload);
            loggedInUser = payload;
        });
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
