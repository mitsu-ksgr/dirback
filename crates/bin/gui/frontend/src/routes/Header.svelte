<!----------------------------------------------------------------------------
  Header
 !---------------------------------------------------------------------------->
<script lang="ts">
  import { onMount } from 'svelte';
  import { getVersion } from '@tauri-apps/api/app';

  import Github from 'lucide-svelte/icons/github';
  import Moon from 'lucide-svelte/icons/moon';
  import Sun from 'lucide-svelte/icons/sun';
  import BookHeart from 'lucide-svelte/icons/book-heart';

  const isDev = import.meta.env.DEV;

  let appVersion = $state('1.0.0');
  let theme = $state('dark');
  let fgColor = $derived(theme === 'dark' ? 'white' : 'black');

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
      <a href="/">
        <h1>Dirback</h1>
      </a>
    </div>
    <div>
      <h3>
        v{appVersion}
        {#if isDev}
          (DEV)
        {/if}
      </h3>
    </div>
  </div>

  <div class="right">
    {#if isDev}
      <div>
        <a href="debug">
          <BookHeart size={24} color="DeepPink" />
        </a>
      </div>
    {/if}

    <div>
      <a
        href="https://github.com/mitsu-ksgr/dirback"
        target="_blank"
        rel="noopener noreferrer"
        class="gh-link"
        aria-label="GitHub"
        >
        <Github size={24} color={fgColor} />
      </a>
    </div>

    <div on:click={toggleTheme}>
      {#if theme === 'dark'}
        <Moon size={24} color="Yellow" />
      {:else}
        <Sun size={24} color="OrangeRed" />
      {/if}
    </div>
  </div>
</header>

<style lang="scss">
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;

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
