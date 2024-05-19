<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import { Checkbox } from "$lib/components/ui/checkbox";
  import { emit } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { userStore } from "./store";

  let isLogin = true;

  let username = "",
    password = "",
    repeatPassword = "";

  let stayLoggedIn = false;

  $: canSubmit = () => {
    if (isLogin) {
      username != "" && password != "";
    } else {
      username != "" && password != "" && password == repeatPassword;
    }
  };

  $: redField =
    password != repeatPassword &&
    username != "" &&
    password != "" &&
    repeatPassword != "";

  function toggleForm() {
    username = "";
    password = "";
    repeatPassword = "";
    isLogin = !isLogin;
  }

  function submitForm() {
    let event_name = isLogin ? "login_attempt" : "register_attempt";
    emit(event_name, {
      username,
      password,
      stay_logged_in: stayLoggedIn,
    });
  }
  onMount(async () => {
    // TODO: Reconsider multiple listens in the frontend
    await listen("user_registered", (event) => {
      console.log("user_registered");
      userStore.set({
        user: {
          id: event.payload["id"],
          name: event.payload["username"],
        },
      });
    });
    await listen("user_logged_in", (event) => {
      console.log("user_logged_in");
      userStore.set({
        user: {
          id: event.payload["id"],
          name: event.payload["username"],
        },
      });
    });
    await listen("failed_user_registration", (event) => {
      console.log(`There was a problem registering: ${event}`);
    });
  });
</script>

<div class="form-container">
  <div class="form-card">
    {#if isLogin}
      <h2 class="text-2xl mb-4">Login</h2>
      <form>
        <input
          bind:value={username}
          type="text"
          placeholder="Username"
          class="form-input"
          required
        />
        <input
          bind:value={password}
          type="password"
          placeholder="Password"
          class="form-input"
          required
        />

        {#if canSubmit}
          <Button
            type="submit"
            class="form-button bg-blue-500"
            on:click={submitForm}>Login</Button
          >
        {:else}
          <Button
            disabled
            type="submit"
            class="form-button bg-blue-500"
            on:click={submitForm}>Login</Button
          >
        {/if}
        <Checkbox bind:checked={stayLoggedIn} />
      </form>
      <div class="mt-4">
        <Button class="oauth-button bg-red-500">Sign in with Google</Button>
      </div>
      <p class="mt-4">
        Don't have an account? <a
          href="#"
          on:click={toggleForm}
          class="text-blue-500">Register</a
        >
      </p>
    {:else}
      <h2 class="text-2xl mb-4">Register</h2>
      <form>
        <input type="text" placeholder="Username" class="form-input" required />
        <input
          bind:value={username}
          type="password"
          placeholder="Password"
          class="form-input"
          required
        />
        <input
          bind:value={password}
          type="password"
          placeholder="Repeat Password"
          class="form-input"
          required
        />
        {#if canSubmit}
          <Button
            type="submit"
            class="form-button bg-blue-500"
            on:click={submitForm}>Register</Button
          >
        {:else}
          <Button
            disabled
            type="submit"
            class="form-button bg-blue-500"
            on:click={submitForm}>Register</Button
          >
        {/if}
        <Checkbox id="stayLoggedIn" bind:checked={stayLoggedIn} />
        <Label
          id="stayLoggedInLabel"
          for="stayLoggedIn"
          class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
        >
          Stay logged in
        </Label>
      </form>
      <div class="mt-4">
        <Button class="oauth-button bg-red-500">Sign up with Google</Button>
      </div>
      <p class="mt-4">
        Already have an account? <a
          href="#"
          on:click={toggleForm}
          class="text-blue-500">Login</a
        >
      </p>
    {/if}
  </div>
</div>

<style lang="scss">
  .form-container {
    @apply flex flex-col items-center justify-center min-h-screen bg-gray-100;
  }
  .form-card {
    @apply bg-white p-8 rounded-lg shadow-md w-full max-w-md;
  }
  .form-input {
    @apply mb-4 p-2 border border-gray-300 rounded;
  }
  .form-button {
    @apply bg-blue-500 text-white py-2 px-4 rounded hover:bg-blue-700;
  }
  .oauth-button {
    @apply bg-red-500 text-white py-2 px-4 rounded hover:bg-red-700;
  }
  .disabled {
    color: grey;
    &:hover {
      border: none;
    }
  }
</style>
