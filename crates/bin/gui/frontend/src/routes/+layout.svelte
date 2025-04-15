<!----------------------------------------------------------------------------
  src/routes/+layout.svelte

  Page layout
 !---------------------------------------------------------------------------->
<script lang="ts">
  import "@picocss/pico";
  import "./style.css";
  import Header from "./Header.svelte";
  import Footer from "./Footer.svelte";
</script>

<div class="layout-container">
  <div class="header">
    <Header />
  </div>

  <div class="main">
    <slot />
  </div>

  <div class="footer">
    <Footer />
  </div>
</div>

<style lang="scss">
  .layout-container {
    display: flex;
    flex-direction: column;
    height: 100vh;

    .header, .footer {
      flex-shrink: 0;
      background: var(--pico-background-color);
      padding: 1rem;
    }

    .header {
      border-bottom: 5px solid var(--pico-muted-border-color);
    }

    .footer {
      border-top: 5px solid var(--pico-muted-border-color);
    }

    .main {
      flex: 1 1 auto;
      overflow-y: auto;
      padding: 1rem;
    }
  }
</style>
