<script lang="ts">
  import "../app.css";
  import { auth } from "$lib/stores/auth";
  import UserMenu from "$lib/components/UserMenu.svelte";
  import { Button } from "$lib/components/ui/button";
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { resolve } from "$app/paths";

  let { children } = $props();

  onMount(() => {
    // Check auth status when app loads (non-blocking)
    // Don't await this to avoid interfering with navigation
    auth.initializeAuth();
  });
  // Pages that don't require authentication
  const publicRoutes = ["/login", "/signup", "/"];

  let isPublicRoute = $derived(publicRoutes.includes($page.url.pathname));
</script>

<svelte:head>
  <title>SaaS Boilerplate</title>
  <meta
    name="description"
    content="A modern SaaS boilerplate built with SvelteKit and Rust"
  />
</svelte:head>

<div class="min-h-screen bg-background flex flex-col">
  <!-- Navigation Header -->
  <header
    class="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-backdrop-filter:bg-background/60 shrink-0"
  >
    <div class="container mx-auto flex h-14 items-center">
      <!-- Simple Logo -->
      <div class="mr-4">
        <a href="{resolve('/')}]" class="mr-6 flex items-center space-x-2">
          <div
            class="h-6 w-6 rounded bg-primary flex items-center justify-center"
          >
            <span class="text-primary-foreground font-bold text-sm">S</span>
          </div>
          <span class="font-bold">SaaS Boilerplate</span>
        </a>
      </div>

      <!-- Simple Navigation -->
      <nav class="flex items-center space-x-6 text-sm font-medium">
        {#if $auth.isAuthenticated}
          <a href={resolve("/dashboard")} class="hover:text-primary"
            >Dashboard</a
          >
          <a href={resolve("/projects")} class="hover:text-primary">Projects</a>
        {/if}
      </nav>

      <!-- Simple Right side -->
      <div class="flex flex-1 items-center justify-end space-x-2">
        <nav class="flex items-center space-x-2">
          {#if $auth.isLoading}
            <div class="h-8 w-8 rounded-full bg-muted animate-pulse"></div>
          {:else if $auth.isAuthenticated}
            <UserMenu />
          {:else}
            <div class="flex items-center space-x-2">
              <Button variant="ghost" href="/login" size="sm">Sign In</Button>
              <Button href="/signup" size="sm">Sign Up</Button>
            </div>
          {/if}
        </nav>
      </div>
    </div>
  </header>

  <!-- Main Content -->
  <main class="flex-1 flex flex-col">
    {@render children()}
  </main>

  <!-- Footer -->
  <footer class="border-t shrink-0">
    <div
      class="container mx-auto flex flex-col items-center justify-center gap-4 py-10 md:h-24 md:flex-row md:py-0"
    >
      <div
        class="flex flex-col items-center gap-4 px-8 md:flex-row md:gap-2 md:px-0"
      >
        <p
          class="text-center text-sm leading-loose text-muted-foreground md:text-left"
        >
          Built with
          <a
            href="https://kit.svelte.dev"
            target="_blank"
            rel="noreferrer"
            class="font-medium underline underline-offset-4"
          >
            SvelteKit
          </a>
          and
          <a
            href="https://www.rust-lang.org"
            target="_blank"
            rel="noreferrer"
            class="font-medium underline underline-offset-4"
          >
            Rust
          </a>
        </p>
      </div>
      <div class="flex items-center space-x-4">
        <a
          href="#"
          class="text-sm text-muted-foreground hover:text-foreground transition-colors"
        >
          Privacy
        </a>
        <a
          href="#"
          class="text-sm text-muted-foreground hover:text-foreground transition-colors"
        >
          Terms
        </a>
        <a
          href="#"
          class="text-sm text-muted-foreground hover:text-foreground transition-colors"
        >
          Support
        </a>
      </div>
    </div>
  </footer>
</div>
