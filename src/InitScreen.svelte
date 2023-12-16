<script lang="ts">
  import { appWindow } from '@tauri-apps/api/window';
  let isRegister = true;

  let username = "",
    password = "",
    repeatPassword = "";

  $: toggleButtonText = isRegister
    ? "I already have an account"
    : "I don't have an account";

  $: submitButtonText = isRegister ? "Register" : "Login";

  function toggleForm() {
    username = "";
    password = "";
    repeatPassword = "";
    isRegister = !isRegister;
  }

  function submitForm() {
    let eventType = isRegister ? 'register_attempt' : 'login_attempt';
    appWindow.emit(eventType, { username, password });
  }
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
        <input id="register-repeat-password" bind:value={repeatPassword} type="password" />
      </div>
    {:else}
      <div class="login form">
        <label for="login-username">Username</label>
        <input id="login-username" bind:value={username} type="text" />
        <label for="login-password">Password</label>
        <input id="login-password" bind:value={password} type="password" />
      </div>
    {/if}

    <button on:click={toggleForm}>{toggleButtonText}</button>
    <button on:click={submitForm}>{submitButtonText}</button>
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

      .form {
        display: flex;
        justify-content: center;
        flex-direction: column;
      }
    }
  }
</style>
