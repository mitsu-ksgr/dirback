<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  import { dispatch } from "$lib/api/dispatcher";
  import { IS_MOCK } from "$lib/config";

  import type { Target } from "$lib/api/types/target";
  import { getTarget } from "$lib/api/get-target";
  import { listTargets } from "$lib/api/list-targets";
  import { registerTarget } from "$lib/api/register-target";
  import { deleteTarget } from "$lib/api/delete-target";
  import { backupTarget } from "$lib/api/backup-target";

  // Rust: Command dispatcher test.
  let command = $state("");
  let cmdResult = $state("");

  let target_id = $state("");
  let name = $state("");
  let path = $state("");
  let note = $state("");

  async function onSubmit(event: Event) {
    event.preventDefault();

    console.log(`Command: ${command}`);
    console.log(`TargetID: ${target_id}`);
    console.log(`Name: ${name}`);
    console.log(`Path: ${path}`);

    try {
      switch (command) {
        case "GetTarget":
          cmdResult = await getTarget(target_id);
          break;

        case "ListTargets":
          cmdResult = await listTargets();
          break;

        case "RegisterTarget":
          cmdResult = await registerTarget(name, path);
          break;

        case "DeleteTarget":
          cmdResult = await deleteTarget(target_id);
          break;

        case "BackupTarget":
          cmdResult = await backupTarget(target_id, note);
          break;
      }
    } catch (error) {
      cmdResult = `ERROR: ${error}`;
    }
  }
</script>

<main class="container">
  <h1>Welcome to Tauri + Svelte</h1>

  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://kit.svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" />
    </a>
  </div>

  <h2>Command Dispatcher Testing</h2>

  <p>IsMock? {IS_MOCK}</p>

  <form onsubmit={onSubmit}>
    <div class="row">
      <label for="commands">Command:</label>
      <select id="commands" bind:value={command}>
        <option value="GetTarget">GetTarget</option>
        <option value="ListTargets">ListTargets</option>
        <option value="RegisterTarget">RegisterTarget</option>
        <option value="DeleteTarget">DeleteTarget</option>
        <option value="BackupTarget">BackupTarget</option>
      </select>
    </div>

    <div class="row">
      <label for="target_id">Target ID:</label>
      <input id="target_id" placeholder="target-id" bind:value={target_id} />
    </div>

    <div class="row">
      <label for="name">Name:</label>
      <input id="name" placeholder="name" bind:value={name} />
    </div>

    <div class="row">
      <label for="path">Path:</label>
      <input id="path" placeholder="path" bind:value={path} />
    </div>

    <div class="row">
      <label for="note">Note:</label>
      <input id="note" placeholder="note" bind:value={note} />
    </div>

    <button type="submit">Dispatch</button>
  </form>
  <pre><code>{JSON.stringify(cmdResult, null, 2)}</code></pre>
</main>

<style>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.svelte-kit:hover {
  filter: drop-shadow(0 0 2em #ff3e00);
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
  margin-bottom: 1.0em;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

pre, code {
  color: black;
  background-color: WhiteSmoke;
  text-align: left;
}

</style>
