<script lang="ts">
  import { emit } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { userStore } from "./store";

  let isRegister = true;

  let username = "",
    password = "",
    repeatPassword = "";

  let stayLoggedIn = false;

  $: toggleButtonText = isRegister
    ? "I already have an account"
    : "I don't have an account";

  $: submitButtonText = isRegister ? "Register" : "Login";

  $: canSubmit = () => {
    if (isRegister) {
      username != "" && password != "" && password == repeatPassword;
    } else {
      username != "" && password != "";
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
    isRegister = !isRegister;
  }

  function submitForm() {
    let event_name = isRegister ? "register_attempt" : "login_attempt";
    emit(event_name, {
      username,
      password,
      "stay_logged_in": stayLoggedIn
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

<div class="init-screen">
  <h1>Welcome to Holistay</h1>
  <div class="starter-form">
    <h2>Your solution for looking after your properties</h2>
    <p>Register as a user or login using the form below</p>

    {#if isRegister}
      <div class="register form">
        <label for="register-username">Username</label>
        <input id="register-username" bind:value={username} type="text" />
        <label for="register-password">Password</label>
        <input id="register-password" bind:value={password} type="password" />
        <label for="register-repeat-password">Repeat Password</label>
        <input
          id="register-repeat-password"
          class:red-highlight={redField}
          bind:value={repeatPassword}
          type="password"
        />
      </div>
    {:else}
      <div class="login form">
        <label for="login-username">Username</label>
        <input id="login-username" bind:value={username} type="text" />
        <label for="login-password">Password</label>
        <input id="login-password" bind:value={password} type="password" />
      </div>
    {/if}

    <label for="stay-logged-in">Stay logged in?</label>
    <input id="stay-logged-in" bind:checked={stayLoggedIn} on:click={() => console.log(stayLoggedIn)} type="checkbox" />

    <button on:click={toggleForm}>{toggleButtonText}</button>
    <button
      disabled={!canSubmit}
      class:disabled={!canSubmit}
      on:click={submitForm}>{submitButtonText}</button
    >
  </div>
</div>

<style lang="scss">
  .init-screen {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;

    .starter-form {
      background-color: rgba(0, 100, 250, 0.7);
      padding: 15px;
      max-width: 50%;
      border-radius: 15px;

      .disabled {
        color: grey;
        &:hover {
          border: none;
        }
      }

      .red-highlight {
        border: solid 1px red;
      }

      .form {
        display: flex;
        justify-content: center;
        flex-direction: column;
      }
    }
  }
</style>
