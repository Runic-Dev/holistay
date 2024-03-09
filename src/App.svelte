<script lang="ts">
  import WelcomeScreen from "./WelcomeScreen.svelte";
  import Property from "./pages/property/Property.svelte";
  import RoomGroup from "./pages/roomgroup/RoomGroup.svelte";
  import Router from "svelte-spa-router";
  import Room from "./pages/room/Room.svelte";
  import { TauriBackendHandler, WebBackendHandler, type BackendHandler } from "./backendHandlers";
  import { setContext } from "svelte";

  let handler =
    import.meta.env.VITE_HOLISTAY_PLATFORM == "desktop"
      ? new TauriBackendHandler()
      : new WebBackendHandler();

  setContext<BackendHandler>("backendHandler", handler);
  const routes = {
    "/": WelcomeScreen,
    "/property/:propertyId": Property,
    "/property/:propertyId/roomgroup/:roomGroupId": RoomGroup,
    "/property/:propertyId/roomgroup/:roomGroupId/room/:roomId": Room,
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
