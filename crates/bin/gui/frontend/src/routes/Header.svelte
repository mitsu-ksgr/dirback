<!----------------------------------------------------------------------------
  Header
 !---------------------------------------------------------------------------->
<script lang="ts">
  import { onMount } from 'svelte';
  import { getVersion } from '@tauri-apps/api/app';

  import Github from 'lucide-svelte/icons/github';
  import Moon from 'lucide-svelte/icons/moon';
  import Sun from 'lucide-svelte/icons/sun';

  let appVersion = '1.0.0';
  let theme = 'dark';
  let fgColor = 'white';

  function updateFgColor() {
    fgColor = theme === 'dark' ? 'white' : 'black';
  }

  function toggleTheme() {
    theme = theme === 'dark' ? 'light' : 'dark';
    localStorage.theme = theme;
    document.documentElement.setAttribute('data-theme', theme);
    updateFgColor();
  }

  onMount(async () => {
    appVersion = await getVersion();

    const lsTheme = localStorage.getItem('theme');
    if (lsTheme === 'light') {
      theme = 'light';
    } else {
      theme = 'dark';
    }
    updateFgColor();
  });
</script>

<header>
  <div class="left">

    <div>
      <h1>Dirback</h1>
    </div>
    <div>
      <h3>
        ver{appVersion}
      </h3>
    </div>
  </div>

  <div class="right">
    <div>
      <a
        href="https://github.com"
        target="_blank"
        rel="noopener noreferrer"
        class="gh-link"
        aria-label="GitHub"
        >
      </a>
      <Github size={24} color={fgColor} />
    </div>

    <div on:click={toggleTheme}>
      {#if theme === 'dark'}
        <Moon size={24} color="orange" />
      {:else}
        <Sun size={24} color="orangered" />
      {/if}
    </div>
  </div>
</header>

<style lang="scss">
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;

    .left {
      display: flex;
      align-items: end;
      gap: 1rem;

      h1, h3 {
        margin: 0;
      }
    }

    .right {
      display: flex;
      align-items: center;
      gap: 1rem;
    }
  }
</style>
