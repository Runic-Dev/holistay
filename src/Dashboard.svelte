<script lang="ts">
    import MainLayout from "./MainLayout.svelte";
    import PropertiesOverview from "./PropertiesOverview.svelte";
    import InitScreen from "./InitScreen.svelte";
    import { userStore } from "./store";

    $: loggedInUser = $userStore.user;
    $: if (loggedInUser) {
        console.log($userStore);
    } else {
        console.log("No user");
    }
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
