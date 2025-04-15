<!----------------------------------------------------------------------------
  Top page
 !---------------------------------------------------------------------------->
<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from "$app/navigation";

  import { listTargets } from "$lib/api/list-targets";
  import HoverElement from "$lib/ui/HoverElement.svelte";

  let targets = $state([]);
  let error = $state("");

  async function fetchTargets() {
    try {
      targets = await listTargets();
    } catch(e) {
      error = `Error: ${e}`;
    }
  }

  onMount(async () => {
    console.log("onMount!");
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
          <th>Path</th>
        </tr>
      </thead>

      <tbody>
        {#each targets as target}
          <tr>
            <td width="36px">
              <HoverElement>
                <span slot="normal">&#x1f4c1;</span>
                <span slot="hover">
                  <a href="/">
                    &#x1f4c2;
                  </a>
                </span>
              </HoverElement>
            </td>
            <td>{target.name}</td>
            <td><code>{target.path}</code></td>
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
</main>

<style lang="scss">
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
</style>
