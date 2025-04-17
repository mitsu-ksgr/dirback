<!----------------------------------------------------------------------------
  Target page
 !---------------------------------------------------------------------------->
<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from "$app/navigation";
  import { open } from "@tauri-apps/plugin-dialog";

  import Trash2 from 'lucide-svelte/icons/trash-2';
  import Package from 'lucide-svelte/icons/package';
  import PackageOpen from 'lucide-svelte/icons/package-open';

  import type { BackupEntry } from "$lib/types/backup-entry";
  import type { Target } from "$lib/types/target";

  import { backupTarget } from "$lib/api/backup-target";
  import { deleteBackup } from "$lib/api/delete-backup";
  import { getTarget } from "$lib/api/get-target";
  import { restoreTarget } from "$lib/api/restore-target";
  import { fmtDateTime } from "$lib/utils/fmt";
  import HoverElement from "$lib/ui/HoverElement.svelte";
  import Modal from "$lib/ui/Modal.svelte";

  const { data } = $props();
  const target_id = data.target_id;

  // Target
  let target: Target | null = $state(null);
  let error = $state("");

  async function fetchTarget() {
    try {
      target = await getTarget(target_id);
    } catch(e) {
      if (e instanceof Error) {
        error = e.message;
      } else {
        error = String(e);
      }
    }
  }

  // Ok Dialog
  let isOkModalOpen = $state(false);
  let okModalTitle = $state("");
  let okModalMessage = $state("");

  // Trash
  let isDeleteModalOpen = $state(false);
  let delBackup: BackupEntry | null = $state(null);
  let delError = $state("");

  async function handleDeleteBackupRequest(backup: BackupEntry) {
    delBackup = backup;
    isDeleteModalOpen = true;
  }

  async function onCancelDeleteBackup() {
    delBackup = null;
    delError = "";
    isDeleteModalOpen = false;
  }

  async function onDeleteBackup() {
    if (target === null || delBackup === null) {
      return;
    }

    try {
      const backup = await deleteBackup(target.id, delBackup.id);

      // Clean modal params
      delBackup = null;
      delError = "";
      isDeleteModalOpen = false;

      // Update target info.
      await fetchTarget();

      // Setup OK modal.
      okModalTitle = "Deletion completed!";
      okModalMessage = `The backup[${backup.id}] has been deleted.`;
      isOkModalOpen = true;

    } catch (e) {
      if (e instanceof Error) {
        delError = e.message;
      } else {
        delError = String(e);
      }
    }
  }

  // Take a new backup
  let isBackupModalOpen = $state(false);
  let backupNote = $state("");
  let backupError = $state("");

  async function onCancelBackup() {
    backupNote = "";
    backupError = "";
    isBackupModalOpen = false;
  }

  async function onBackup() {
    if (target === null) {
      return;
    }

    try {
      target = await backupTarget(target.id, backupNote);
      const backup = target.backups.at(-1);

      // Clean modal params
      backupNote = "";
      backupError = "";
      isBackupModalOpen = false;

      // Setup OK modal.
      okModalTitle = "Back up successful!!";
      if (backup === undefined) {
        okModalMessage = "The backup was successful, but the latest backup information could not be obtained.";
      } else {
        okModalMessage = `A new backup[${backup.id}] has been created.`;
      }
      isOkModalOpen = true;

    } catch(e) {
      if (e instanceof Error) {
        backupError = e.message;
      } else {
        backupError = String(e);
      }
    }
  }

  // Restore
  let isRestoreModalOpen = $state(false);
  let resBackup: BackupEntry | null = $state(null);
  let resError = $state("");

  async function handleRestoreRequest(backup: BackupEntry) {
    resBackup = backup;
    isRestoreModalOpen = true;
  }

  async function onCancelRestore() {
    resBackup = null;
    resError = "";
    isRestoreModalOpen = false;
  }

  async function onRestore() {
    if (target === null || resBackup === null) {
      return;
    }

    try {
      await restoreTarget(target.id, resBackup.id);
      const backup_id = resBackup.id;

      // Clean modal params
      resBackup = null;
      resError = "";
      isRestoreModalOpen = false;

      // Setup OK modal.
      okModalTitle = "Restore completed!!";
      okModalMessage = `The target contents have been restored with the backup[${backup_id}].`;
      isOkModalOpen = true;

    } catch(e) {
      if (e instanceof Error) {
        resError = e.message;
      } else {
        resError = String(e);
      }
    }
  }

  onMount(async () => {
    await fetchTarget();
  });
</script>

<main>
  {#if target}
    <div class="header">
      <div>
        <h2>{target.name}</h2>
      </div>

      <div>
        <button class="outline secondary" onclick={() => goto('/')}>BACK</button>
      </div>
    </div>

    <div class="target-info">
      <div class="field">
        <h4>Target path</h4>
        <p><code>{target.path}</code></p>
      </div>

      <div class="field">
        <h4>Internal management ID</h4>
        <p><code>{target.id}</code></p>
      </div>

      <div class="field">
        <button onclick={() => isBackupModalOpen = true}>Take a new backup!</button>
      </div>
    </div>

    {#if target.backups.length > 0}
      <h2>Backups</h2>

      <table class="striped overflow-auto">
        <thead>
          <tr>
            <th>Restore</th>
            <th>Index</th>
            <th>Timestamp</th>
            <th>Note</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          {#each target.backups as backup}
            <tr>
              <td width="36px">
                <button class="icon-btn" onclick={() => handleRestoreRequest(backup)}>
                  <HoverElement>
                    {#snippet normal()}
                      <Package size={24} color="LimeGreen" />
                    {/snippet}
                    {#snippet hover()}
                      <PackageOpen size={24} color="LimeGreen" />
                    {/snippet}
                  </HoverElement>
                </button>
              </td>
              <td>{backup.id}</td>
              <td>{fmtDateTime(backup.timestamp)}</td>
              <td>{backup.note}</td>
              <td width="36px">
                <button class="icon-btn" onclick={() => handleDeleteBackupRequest(backup)}>
                  <Trash2 color="red" />
                </button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}

  {:else}
    <h3>Target not found.</h3>

    <p>Failed to get target information.</p>
    <p>{error}</p>

    <button onclick={() => goto('/')}>Back</button>
  {/if}

  <Modal title="Delete?" open={isDeleteModalOpen}>
    <p>Are you sure you want to delete this backup?</p>

    {#if delBackup}
      <ul>
        <li>Index: <code>{delBackup.id}</code></li>
        <li>Timestamp: {fmtDateTime(delBackup.timestamp)}</li>
      </ul>
      <p>Note:</p>
      <p class="note">{delBackup.note || "---"}</p>
    {/if}

    {#if delError}
      <p class="error">{delError}</p>
    {/if}

    <p class="warn">&#x26a0; This action cannot be undone!!!</p>

    <div slot="buttons">
      <button onclick={onCancelDeleteBackup} class="secondary">Cancel</button>
      <button onclick={onDeleteBackup} class="btn-delete">DELETE</button>
    </div>
  </Modal>

  <Modal title="Backup?" open={isBackupModalOpen}>
    <p>Create a new backup.</p>

    <label for="note">Note:</label>
    <textarea name="note" placeholder="..." bind:value={backupNote}>
    </textarea>

    {#if backupError}
      <p class="error">{backupError}</p>
    {/if}

    <div slot="buttons">
      <button onclick={onCancelBackup} class="secondary">Cancel</button>
      <button onclick={onBackup}>BACKUP</button>
    </div>
  </Modal>

  <Modal title="Restore?" open={isRestoreModalOpen}>
    {#if resBackup}
      <p>Do you want to retore with the backup[{resBackup.id}]?</p>

      <ul>
        <li>Index: <code>{resBackup.id}</code></li>
        <li>Timestamp: {fmtDateTime(resBackup.timestamp)}</li>
      </ul>
      <p>Note:</p>
      <p class="note">
        {resBackup.note || "---"}
      </p>
    {/if}

    {#if resError}
      <p class="error">{resError}</p>
    {/if}

    <ul class="warn-list">
      <li>&#x26a0; This action cannot be undone!!!</li>
      <li>&#x26a0; The target directory will be overwritten!!!</li>
    </ul>

    <div slot="buttons">
      <button onclick={onCancelRestore} class="secondary">Cancel</button>
      <button onclick={onRestore}>RESTORE</button>
    </div>
  </Modal>

  <Modal title={okModalTitle} open={isOkModalOpen} onClickOutside={() => isOkModalOpen = false}>
    <p>{okModalMessage}</p>

    <div slot="buttons">
      <button onclick={() => isOkModalOpen = false}>OK</button>
    </div>
  </Modal>
</main>

<style lang="scss">
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .target-info {
    padding-left: 1rem;
    margin-bottom: 2rem;
  }

  .note {
    border: 1px dashed darkgray;
    padding: 0.5rem;
  }

  tbody {
    tr {
      td:nth-child(1) {
        text-align: center;
        button {
          display: inline-block;
        }
      }
      td:nth-child(2) {
        width: 5rem;
        text-align: right;
      }
      td:nth-child(3) {
        width: 20rem;
        font-family: monospace;
      }
    }
  }
</style>
