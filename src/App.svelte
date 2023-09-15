<script lang="ts">
  import Dashboard from './Dashboard.svelte';
  import { AUTH_REQUIRED } from './lib/featureFlag';
</script>
  
<main class="container">
{#if AUTH_REQUIRED}
    Login
{:else}
    <Dashboard />
{/if}
</main>


<style lang="scss">
  main {
    padding-top: 0px;
  }
</style>
