<!----------------------------------------------------------------------------
  Register target page
 !---------------------------------------------------------------------------->
<script lang="ts">
  import { goto } from "$app/navigation";
  import { open } from "@tauri-apps/plugin-dialog";

  import { registerTarget } from "$lib/api/register-target";
  import Modal from "$lib/ui/Modal.svelte";

  // Form params
  let name = $state("");
  let path = $state("");
  let error = $state("");

  // Dialog params
  let targetName = $state("");
  let targetPath = $state("");
  let showDialog = $state(false);

  async function selectDirectory() {
    const selected = await open({
      directory: true,
      multiple: false,
    });

    if (typeof selected === "string") {
      path = selected;
    }
  }

  async function onSubmit() {
    try {
      const target = await registerTarget(name, path);
      targetName = target.name;
      targetPath = target.path;
      showDialog = true;
    } catch (e) {
      if (e instanceof Error) {
        error = e.message;
      } else {
        error = String(e);
      }
    }
  }

  function handleOk() {
    showDialog = false;
    goto("/");
  }
</script>

<main>
  <h2>Register new target</h2>

  <form>
    <label for="name">Target Name:</label>
    <input id="name" bind:value={name} />

    <label for="path">Target directory path: </label>
    <div class="select-dir">
      <button onclick={selectDirectory}>Choose directory</button>
      <input id="path" bind:value={path} />
    </div>

    {#if error}
      <div class="errors">
        <p>{error}</p>
      </div>
    {/if}

    <div class="grid">
      <button type="button" onclick={() => goto("/")} class="secondary"
        >Cancel</button
      >
      <button onclick={onSubmit}>Register</button>
    </div>
  </form>

  <Modal title="Registration complete!!" open={showDialog}>
    <p>A new target has been registered!</p>

    <ul>
      <li>Name: {targetName}</li>
      <li>Path: {targetPath}</li>
    </ul>

    <div slot="buttons">
      <button onclick={handleOk}>OK</button>
    </div>
  </Modal>
</main>

<style lang="scss">
  .select-dir {
    display: flex;
    align-items: left;
    gap: 0.5rem;
    margin-bottom: 1rem;

    button {
      white-space: nowrap;
    }

    input {
      margin-bottom: 0;
    }
  }

  .errors {
    p {
      color: Crimson;
    }
  }
</style>
