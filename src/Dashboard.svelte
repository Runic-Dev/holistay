<script lang="ts">
    import MainLayout from "./MainLayout.svelte";
    import InitScreen from "./InitScreen.svelte";
    import { userStore } from "./store";
    import PropertyDashboard from "./PropertyDashboard.svelte";

    $: loggedInUser = $userStore.user;
    $: if (loggedInUser) {
        console.log($userStore);
    } else {
        console.log("No user");
    }
</script>

<MainLayout>
    {#if loggedInUser}
        <PropertyDashboard />
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
