<!----------------------------------------------------------------------------
  Top page
 !---------------------------------------------------------------------------->
<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from "$app/navigation";

  import Trash2 from 'lucide-svelte/icons/trash-2';

  import type { Target } from "$lib/types/target";

  import { deleteTarget } from "$lib/api/delete-target";
  import { listTargets } from "$lib/api/list-targets";
  import { fmtDateTime } from "$lib/utils/fmt";
  import HoverElement from "$lib/ui/HoverElement.svelte";
  import Modal from "$lib/ui/Modal.svelte";

  // Targets
  let targets: Target[] = $state([]);
  let error = $state("");

  async function fetchTargets() {
    try {
      targets = await listTargets();
    } catch(e) {
      if (e instanceof Error) {
        error = e.message;
      } else {
        error = String(e);
      }
    }
  }

  // OK Dialog
  let isOkModalOpen = $state(false);
  let okModalTitle = $state("");
  let okModalMessage = $state("");

  // Trash
  let isDeleteModalOpen = $state(false);
  let delTarget: Target | null = $state(null);
  let delConfirmation = $state("");
  let delConfirmError = $state("");

  async function resetDeleteModalParams() {
    delTarget = null;
    delConfirmation = "";
    delConfirmError = "";
  }

  async function handleDeleteRequest(target: Target) {
    delTarget = target;
    isDeleteModalOpen = true;
  }

  async function onCancelDelete() {
    resetDeleteModalParams();
    isDeleteModalOpen = false;
  }

  async function onConfirmDelete() {
    if (delTarget === null) {
      return;
    }

    if (delConfirmation === delTarget.name) {
      try {
        let target = await deleteTarget(delTarget.id);

        // Clean confirm modal.
        resetDeleteModalParams();
        isDeleteModalOpen = false;

        // Update targets.
        await fetchTargets();

        // Setup OK modal.
        okModalTitle = "Deletion completed!";
        okModalMessage = `The target '${target.name}' has been deleted.`;
        isOkModalOpen = true;
      } catch(e) {
        if (e instanceof Error) {
          delConfirmError = e.message;
        } else {
          delConfirmError = String(e);
        }
      }

    } else {
      delConfirmError = "Cofirmation failed. Please check again.";
    }
  }

  onMount(async () => {
    await fetchTargets();
  });
</script>

<main>
  {#if targets.length === 0}
    <hgroup>
      <h3>Welcome to Dirback!</h3>
      <p>Simple directory-based backup tool.</p>
    </hgroup>

    <p>
      No backup targets have been registered yet &#x1f440;
    </p>

    <button onclick={() => goto('register-target')}>
      &#x1f4c1; Register new target
    </button>

  {:else}
    <div class="header">
      <div>
        <h2>Targets</h2>
      </div>

      <div>
        <button onclick={() => goto('register-target')}>
          Register new target
        </button>
      </div>
    </div>

    <table class="striped">
      <thead>
        <tr>
          <th></th>
          <th>Name</th>
          <th>Last backup</th>
          <th>Backups</th>
          <th>Path</th>
          <th></th>
        </tr>
      </thead>

      <tbody>
        {#each targets as target}
          <tr>
            <td width="36px">
              <HoverElement>
                {#snippet normal()}
                  <span>&#x1f4c1;</span>
                {/snippet}
                {#snippet hover()}
                  <a href="/target/{target.id}">
                    <span>&#x1f4c2;</span>
                  </a>
                {/snippet}
              </HoverElement>
            </td>
            <td><a href="/target/{target.id}">{target.name}</a></td>
            <td>
              {#if target.backups.length > 0}
                {fmtDateTime(target.backups.at(-1)!.timestamp)}
              {:else}
                -
              {/if}
            </td>
            <td>{target.backups.length}</td>
            <td><code>{target.path}</code></td>
            <td width="36px">
              <button class="icon-btn" onclick={() => handleDeleteRequest(target)}>
                <Trash2 color="red" />
              </button>
            </td>
          </tr>
        {/each}
      </tbody>

      <tfoot>
        <tr>
          <td colspan="3">
            {targets.length} targets registered.
          </td>
        </tr>
      </tfoot>
    </table>
  {/if}

  <Modal title="Delete?" open={isDeleteModalOpen}>
    <p>Are you sure you want to delete this target?</p>

    {#if delTarget}
      <ul>
        <li>Name: {delTarget.name}</li>
        <li>Path: <code>{delTarget.path}</code></li>
        {#if delTarget.backups.length === 0}
          <li>Backups: no backups.</li>
        {:else}
          <li>Backups: {delTarget.backups.length}</li>
        {/if}
      </ul>

      <p>The backups will also be deleted.</p>

      <label for="confirm">Type the name of the target to confirm: <code>{delTarget.name}</code></label>
      <input id="confirm" bind:value={delConfirmation} />

      {#if delConfirmError}
        <p class="error">{delConfirmError}</p>
      {/if}

    {/if}

    <p class="warn">&#x26a0; This action cannot be undone!!!</p>

    <div slot="buttons">
      <button onclick={onCancelDelete} class="secondary">Cancel</button>
      <button onclick={onConfirmDelete} class="btn-delete">DELETE</button>
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

  .btn-delete {
    background-color: red;
  }

  tbody {
    tr {
      td:nth-child(3) {
        width: 20rem;
        font-family: monospace;
      }
      td:nth-child(4) {
        width: 7rem;
        text-align: right;
      }
    }
  }
</style>
