<script lang="ts">
  import Dashboard from "./Dashboard.svelte";
  import Property from "./pages/property/Property.svelte";
  import RoomGroup from "./pages/roomgroup/RoomGroup.svelte";
  import Router from "svelte-spa-router";
  import Room from "./pages/room/Room.svelte";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { userStore } from "./store";

  const routes = {
    "/": Dashboard,
    "/property/:propertyId": Property,
    "/roomgroup/:roomGroupId": RoomGroup,
    "/room/:roomId": Room,
    "*": Dashboard,
  };
  onMount(async () => {
    // TODO: Reconsider multiple listens in the frontend
    await listen("user_registered", (event) => {
      console.log("user_registered");
      console.log(event);
      userStore.set({
        user: {
          id: "testid",
          name: event.payload["username"],
        },
      });
    });
    await listen("user_logged_in", (event) => {
      console.log("user_logged_in");
      console.log(event);
      userStore.set({
        user: {
          id: "testid",
          name: event.payload["username"],
        },
      });
    });
    await listen("failed_user_registration", (event) => {
      console.log("failed_user_registration");
      console.log(event);
    });
  });
</script>

<main class="container">
  <Router {routes} />
</main>

<style lang="scss">
  @import "./src/lib/app.scss";
  main {
    padding-top: 0px;
  }
</style>
