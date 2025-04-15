<!----------------------------------------------------------------------------
  Top page
 !---------------------------------------------------------------------------->
<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from "$app/navigation";

  import Trash2 from 'lucide-svelte/icons/trash-2';

  import { deleteTarget } from "$lib/api/delete-target";
  import { listTargets } from "$lib/api/list-targets";
  import { fmtDateTime } from "$lib/utils/fmt";
  import HoverElement from "$lib/ui/HoverElement.svelte";
  import Modal from "$lib/ui/Modal.svelte";

  // Targets
  let targets = $state([]);
  let error = $state("");

  async function fetchTargets() {
    try {
      targets = await listTargets();
    } catch(e) {
      error = `Error: ${e}`;
    }
  }

  // OK Dialog
  let isOkModalOpen = $state(false);
  let okModalTitle = $state("");
  let okModalMessage = $state("");

  // Trash
  let isDeleteModalOpen = $state(false);
  let delTarget = $state(null);
  let delConfirmation = $state("");
  let delConfirmError = $state("");

  async function resetDeleteModalParams() {
    delTarget = null;
    delConfirmation = "";
    delConfirmError = "";
  }

  async function handleDeleteRequest(target) {
    delTarget = target;
    isDeleteModalOpen = true;
  }

  async function onCancelDelete() {
    resetDeleteModalParams();
    isDeleteModalOpen = false;
  }

  async function onConfirmDelete() {
    if (delConfirmation === delTarget?.name) {
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
        delConfirmError = e;
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

    <button on:click={() => goto('register-target')}>
      &#x1f4c1; Register new target
    </button>

  {:else}
    <div class="header">
      <div>
        <h3>Targets</h3>
      </div>

      <div>
        <button on:click={() => goto('register-target')}>New target</button>
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
                <span slot="normal">&#x1f4c1;</span>
                <span slot="hover">
                  <a href="/target/{target.id}">
                    &#x1f4c2;
                  </a>
                </span>
              </HoverElement>
            </td>
            <td>{target.name}</td>
            <td>
              {#if target.backups.length > 0}
                {fmtDateTime(target.backups.at(-1).timestamp)}
              {:else}
                -
              {/if}
            </td>
            <td>{target.backups.length}</td>
            <td><code>{target.path}</code></td>
            <td width="36px">
              <div class="icon" on:click={() => handleDeleteRequest(target)}>
                <Trash2 color="red" />
              </div>
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
      <button on:click={onCancelDelete} class="secondary">Cancel</button>
      <button on:click={onConfirmDelete} class="btn-delete">DELETE</button>
    </div>
  </Modal>

  <Modal title={okModalTitle} open={isOkModalOpen} onClickOutside={() => isOkModalOpen = false}>
    <p>{okModalMessage}</p>

    <div slot="buttons">
      <button on:click={() => isOkModalOpen = false}>OK</button>
    </div>
  </Modal>
</main>

<style lang="scss">
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .icon {
    width: 24px;
    height: 24px;
    display: flex;
    justify-content: center;
    transition: transform 0.2s ease;
    transform-origin: center center;

    &:hover {
      transform: scale(1.3);
      cursor: pointer;
    }
  }

  .btn-delete {
    background-color: red;
  }

  .warn {
    color: yellow;
  }

  .error {
    color: red;
  }

  tbody {
    tr {
      td:nth-child(3) {
        font-family: monospace;
      }
      td:nth-child(4) {
        text-align: center;
      }
    }
  }
</style>
