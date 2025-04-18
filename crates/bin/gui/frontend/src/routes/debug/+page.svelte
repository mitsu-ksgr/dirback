<!----------------------------------------------------------------------------
  Debug page
 !---------------------------------------------------------------------------->
<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";

  import type { BackupEntry } from "$lib/types/backup-entry";
  import type { Target } from "$lib/types/target";

  import { IS_MOCK } from "$lib/config";
  import { getTarget } from "$lib/api/get-target";
  import { listTargets } from "$lib/api/list-targets";
  import { registerTarget } from "$lib/api/register-target";
  import { deleteTarget } from "$lib/api/delete-target";
  import { backupTarget } from "$lib/api/backup-target";
  import { deleteBackup } from "$lib/api/delete-backup";
  import { restoreTarget } from "$lib/api/restore-target";

  // Commands
  const apiCommands = [
    "ListTargets",
    "GetTarget",
    "RegisterTarget",
    "BackupTarget",
    "RestoreTarget",
    "DeleteTarget",
    "DeleteBackup",
  ];

  // Input
  let command = $state("ListTargets");
  let target_id = $state("");
  let backup_id = $state(0);
  let name = $state("");
  let path = $state("");
  let note = $state("");

  // Response
  let cmdResult: Target | Target[] | BackupEntry | string | null = $state(null);

  // Helpers
  function isNeed(param: string) {
    switch (param) {
      case "target_id":
        return [
          "GetTarget",
          "BackupTarget",
          "RestoreTarget",
          "DeleteTarget",
          "DeleteBackup",
        ].includes(command);

      case "backup_id":
        return ["RestoreTarget", "DeleteBackup"].includes(command);

      case "name":
      case "path":
        return command === "RegisterTarget";

      case "note":
        return command === "BackupTarget";
    }
    return false;
  }

  async function selectDirectory() {
    const selected = await open({
      directory: true,
      multiple: false,
    });

    if (typeof selected === "string") {
      path = selected;
    }
  }

  async function onSubmit(event: Event) {
    event.preventDefault();

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

        case "DeleteBackup":
          cmdResult = await deleteBackup(target_id, backup_id);
          break;

        case "RestoreTarget":
          await restoreTarget(target_id, backup_id);
          break;
      }
    } catch (e) {
      if (e instanceof Error) {
        cmdResult = e.message;
      } else {
        cmdResult = String(e);
      }
    }
  }
</script>

<main>
  <h2>Debug</h2>

  <p><a href="/">Top</a></p>

  <p>API mock? ... <mark>{IS_MOCK}</mark></p>

  <h3>API Command Test</h3>
  <form>
    <label for="commands">API Command</label>
    <select id="commands" bind:value={command}>
      {#each apiCommands as cmd}
        <option value={cmd}>{cmd}</option>
      {/each}
    </select>

    <div>
      <h4>Command Parameters</h4>

      {#if isNeed("target_id")}
        <label for="target_id">Target ID:</label>
        <input id="target_id" bind:value={target_id} />
      {/if}

      {#if isNeed("backup_id")}
        <label for="backup_id">Backup ID:</label>
        <input id="backup_id" type="number" bind:value={backup_id} />
      {/if}

      {#if isNeed("name")}
        <label for="name">Target Name:</label>
        <input id="name" bind:value={name} />
      {/if}

      {#if isNeed("path")}
        <label for="path">Path: </label>
        <div class="select-dir">
          <button onclick={selectDirectory}>Choose directory</button>
          <input id="path" bind:value={path} disabled />
        </div>
      {/if}

      {#if isNeed("note")}
        <label for="note">Note:</label>
        <input id="note" bind:value={note} />
      {/if}
    </div>

    <button type="submit" onclick={onSubmit}>Dispatch</button>
  </form>

  {#if cmdResult}
    <h3>API Command Result</h3>
    <article>
      <pre><code>{JSON.stringify(cmdResult, null, 4)}</code></pre>
    </article>
  {/if}
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

  pre,
  code {
    color: black;
    background-color: WhiteSmoke;
    text-align: left;
  }
</style>
