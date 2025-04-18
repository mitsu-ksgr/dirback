<!----------------------------------------------------------------------------
  Hover Element
 !---------------------------------------------------------------------------->
<script lang="ts">
  let { hover, normal } = $props();
  let isHover = $state(false);
</script>

<span
  role="presentation"
  onmouseenter={() => (isHover = true)}
  onmouseleave={() => (isHover = false)}
>
  {#if isHover}
    {@render hover()}
  {:else}
    {@render normal()}
  {/if}
</span>
