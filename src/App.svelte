<script lang="ts">
  import WelcomeScreen from "./WelcomeScreen.svelte";
  import Property from "./pages/property/Property.svelte";
  import RoomGroup from "./pages/roomgroup/RoomGroup.svelte";
  import Router from "svelte-spa-router";
  import Room from "./pages/room/Room.svelte";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { userStore } from "./store";

  const routes = {
    "/": WelcomeScreen,
    "/property/:propertyId": Property,
    "/roomgroup/:roomGroupId": RoomGroup,
    "/room/:roomId": Room,
    "*": WelcomeScreen,
  };
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
