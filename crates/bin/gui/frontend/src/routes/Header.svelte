<!----------------------------------------------------------------------------
  Header
 !---------------------------------------------------------------------------->
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";

  import Github from "lucide-svelte/icons/github";
  import Moon from "lucide-svelte/icons/moon";
  import Sun from "lucide-svelte/icons/sun";
  import BookHeart from "lucide-svelte/icons/book-heart";

  import { getAppVersion } from "$lib/sys/get-app-version";

  const isDev = import.meta.env.DEV;

  let appVersion = $state("1.0.0");
  let curTheme = $state("dark");
  let fgColor = $derived(curTheme === "dark" ? "white" : "black");

  function setupTheme(theme: string) {
    curTheme = theme;
    localStorage.theme = theme;
    document.documentElement.setAttribute("data-theme", theme);
  }

  function toggleTheme() {
    setupTheme(curTheme === "dark" ? "light" : "dark");
  }

  onMount(async () => {
    appVersion = await getAppVersion();

    const theme = localStorage.getItem("theme");
    setupTheme(theme === "light" ? "light" : "dark");
  });
</script>

<header>
  <div class="left">
    <a class="brand-link" href="/">
      <div class="brand">
        <img src="/logo.png" alt="logo" />
        <div class="titles">
          <h1>Dirback</h1>
          <h3>
            v{appVersion}
            {#if isDev}
              (DEV)
            {/if}
          </h3>
        </div>
      </div>
    </a>
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

    <button class="icon-btn" onclick={toggleTheme}>
      {#if curTheme === "dark"}
        <Moon size={24} color="Yellow" />
      {:else}
        <Sun size={24} color="OrangeRed" />
      {/if}
    </button>
  </div>
</header>

<style lang="scss">
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;

    .left {
      display: flex;
      align-items: baseline;
      gap: 1rem;

      .brand-link {
        text-decoration: none;
      }

      .brand {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        cursor: pointer;

        img {
          width: 48px;
          height: auto;
          display: block;
        }

        .titles {
          display: flex;
          align-items: last baseline;
          gap: 0.5rem;

          h1,
          h3 {
            margin: 0;
          }
        }
      }
    }

    .right {
      display: flex;
      align-items: center;
      gap: 1rem;
    }
  }
</style>
