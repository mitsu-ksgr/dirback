<!----------------------------------------------------------------------------
  Simple modal dialog
 !---------------------------------------------------------------------------->
<script lang="ts">
  import { onMount } from "svelte";

  export let title = "Dialog";
  export let open = false;
  export let onClickOutside: () => void;

  let dialog: HTMLDialogElement;

  function handleBackgroundOnClick(event: MouseEvent) {
    if ((event.target as HTMLElement).closest('article')) return;
    onClickOutside();
  }

  onMount(() => {
    if (open && dialog?.showModal) {
      dialog.showModal();
    }
  });

  $: if (dialog) {
    if (open) dialog.showModal?.();
    else dialog.close?.();
  }
</script>

<dialog bind:this={dialog} class="modal" on:click={handleBackgroundOnClick}>
  <article>
    <header>
      <h3>{title}</h3>
    </header>

    <slot />

    <footer>
      <slot name="buttons" />
    </footer>
  </article>
</dialog>

<style lang="scss">
  dialog.modal {
    padding: 0;
    border: none;
    background: transparent;
  }

  dialog.modal::backdrop {
    background: rgba(0, 0, 0, 0.5);
  }
</style>
