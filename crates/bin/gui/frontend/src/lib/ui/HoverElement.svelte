<!----------------------------------------------------------------------------
  Hover Element
 !---------------------------------------------------------------------------->
<script lang="ts">
  let isHover = $state(false);
</script>

<span
  on:mouseenter={() => isHover = true}
  on:mouseleave={() => isHover = false}
  >
  {#if isHover}
    <slot name="hover" />
  {:else}
    <slot name="normal" />
  {/if}
</span>
