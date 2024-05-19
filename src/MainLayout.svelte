<script lang="ts">
  import { userStore, displayTopBarStore } from "./store";
  import * as Avatar from "$lib/components/ui/avatar";
    import { onMount } from "svelte";
  $: username = $userStore.user ? $userStore.user["name"] : null;
  $: topBar = false;

  onMount(() => {
    userStore.subscribe(us => {

    });
    displayTopBarStore.subscribe(x => topBar = x);
  })
</script>

<div class="">
  {#if topBar}
  <div class="top-bar">
    <h1>{username}</h1>

    {#if username != null}
      <Avatar.Root>
        <Avatar.Image src=""/>
        <Avatar.Fallback>CN</Avatar.Fallback>
      </Avatar.Root>
    {/if}
  </div>
  {/if}
  <slot {topBar} />
</div>

<!-- <div class="main-dashboard"> -->
<!--   <div class="top-bar" class:splash-topbar={header}> -->
<!--     <div on:click={() => push("/")} on:keydown={() => push("/")} class="center"> -->
<!--       Holistay -->
<!--     </div> -->
<!--     <div class="right-section"> -->
<!--       {#if username} -->
<!--         <h3 class="username">{username}</h3> -->
<!--       {/if} -->

<!--       <div class="notifications"> -->
<!--         <svg -->
<!--           xmlns="http://www.w3.org/2000/svg" -->
<!--           height="1em" -->
<!--           viewBox="0 0 448 512" -->
<!--         > -->
<!--           <style> -->
<!--             svg { -->
<!--               fill: #ffffff; -->
<!--             } -->
<!--           </style><path -->
<!--             d="M224 0c-17.7 0-32 14.3-32 32V51.2C119 66 64 130.6 64 208v18.8c0 47-17.3 92.4-48.5 127.6l-7.4 8.3c-8.4 9.4-10.4 22.9-5.3 34.4S19.4 416 32 416H416c12.6 0 24-7.4 29.2-18.9s3.1-25-5.3-34.4l-7.4-8.3C401.3 319.2 384 273.9 384 226.8V208c0-77.4-55-142-128-156.8V32c0-17.7-14.3-32-32-32zm45.3 493.3c12-12 18.7-28.3 18.7-45.3H224 160c0 17 6.7 33.3 18.7 45.3s28.3 18.7 45.3 18.7s33.3-6.7 45.3-18.7z" -->
<!--           /></svg -->
<!--         > -->
<!--       </div> -->
<!--     </div> -->
<!--   </div> -->

<!--   {#if imageUrl && header} -->
<!--     <div class="property-header" style="background-image: url({imageUrl});"> -->
<!--       <h3>{header}</h3> -->
<!--       <div class="overlay" /> -->
<!--     </div> -->
<!--   {:else if header != null} -->
<!--     <div class="property-header"> -->
<!--       <h3>{header}</h3> -->
<!--       <div class="overlay" /> -->
<!--     </div> -->
<!--   {/if} -->

<!--   <div class="main-content"> -->
<!--     <div class="property-overview"> -->
<!--       <slot /> -->
<!--     </div> -->
<!--   </div> -->
<!-- </div> -->

<style lang="scss">
  @import "./src/lib/app.scss";
  .main-dashboard {
    position: relative;
    .top-bar {
      display: flex;
      justify-content: space-between;
      align-items: center;
      .right-section {
        display: flex;
        justify-content: space-between;
        .username {
          padding: 0 15px;
        }
      }
      div {
        padding: 16px;

        &.center {
          font-size: 2rem;
        }
      }
    }

    .splash-topbar {
      width: 100%;
      z-index: 5;
      position: absolute;
      top: 0;
      left: 0;
      background-color: rgba(25, 25, 25, 0.4);
    }
  }
  .property-header {
    background-repeat: no-repeat;
    background-size: cover;
    background-position: center;
    height: 300px;
    position: relative;
    h3 {
      font-size: 5rem;
      margin: 0;
      position: absolute;
      bottom: 0;
      left: 0;
      padding: $padding * 2;
    }

    .overlay {
      width: 100%;
      height: 100%;
      background-color: rgba(25, 25, 25, 0.4);
    }
  }
</style>
