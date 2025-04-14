<!----------------------------------------------------------------------------
  src/routes/+layout.svelte

  Layout
 !---------------------------------------------------------------------------->
<script lang="ts">
  import "@picocss/pico";
</script>

<div class="app container">
  <header>
    <hgroup>
      <h1>Dirback</h1>
      <p>Simple directory-based backup tool.</p>
    </hgroup>
  </header>

  <slot />

  <footer>
    <p>Footer information</p>
  </footer>
</div>

<style lang="scss">
.app {
  margin-top: 2rem;
  margin-bottom: 2rem;
}
</style>

