<script lang="ts">
  import { auth } from "$lib/stores/auth";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { page } from "$app/stores";
  import * as Alert from "$lib/components/ui/alert";
  import LoadingSpinner from "$lib/components/LoadingSpinner.svelte";
  import { AlertCircle } from "lucide-svelte";
  import { onMount } from "svelte";

  let { children } = $props();

  onMount(async () => {
    // If not already authenticated, check auth status
    if (!$auth.isAuthenticated && !$auth.isLoading) {
      await auth.checkAuthStatus();
    }

    // After auth check, redirect if not authenticated
    if (!$auth.isAuthenticated) {
      const currentPath = $page.url.pathname;
      const redirectUrl =
        resolve("/login") + `?redirect=${encodeURIComponent(currentPath)}`;
      // eslint-disable-next-line svelte/no-navigation-without-resolve
      goto(redirectUrl, { replaceState: true });
      return;
    }
  });
</script>

<!-- Show loading while checking authentication -->
{#if $auth.isLoading}
  <div class="container mx-auto py-20">
    <div class="flex flex-col items-center justify-center space-y-4">
      <LoadingSpinner size={32} />
      <p class="text-sm text-muted-foreground">Checking authentication...</p>
    </div>
  </div>
  <!-- Show error if not authenticated and not loading -->
{:else if !$auth.isAuthenticated}
  <div class="container mx-auto py-20">
    <Alert.Root variant="destructive" class="max-w-md mx-auto">
      <AlertCircle class="h-4 w-4" />
      <Alert.Title>Authentication Required</Alert.Title>
      <Alert.Description>Please sign in to access this page.</Alert.Description>
    </Alert.Root>
  </div>
  <!-- Show protected content if authenticated -->
{:else}
  {@render children()}
{/if}
