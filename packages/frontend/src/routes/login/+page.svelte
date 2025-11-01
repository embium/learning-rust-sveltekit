<script lang="ts">
  import { auth } from "$lib/stores/auth";
  import { authAPI } from "$lib/api/auth";
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import * as Card from "$lib/components/ui/card";
  import LoadingSpinner from "$lib/components/LoadingSpinner.svelte";
  import { Mail, Lock, AlertCircle } from "lucide-svelte";
  import { onMount } from "svelte";

  let email = $state("");
  let password = $state("");
  let isLoading = $state(false);
  let error = $state("");

  onMount(() => {
    // Redirect if already authenticated
    if ($auth.isAuthenticated) {
      goto("/dashboard");
    }
  });

  async function handleSubmit(event: Event) {
    event.preventDefault();

    if (!email || !password) {
      error = "Please fill in all fields";
      return;
    }

    if (!isValidEmail(email)) {
      error = "Please enter a valid email address";
      return;
    }

    isLoading = true;
    error = "";

    try {
      await authAPI.login({ email, password });

      // Check if there's a redirect URL in the query params
      const redirectTo = $page.url.searchParams.get("redirect") || "/dashboard";
      await goto(redirectTo);
    } catch (err) {
      error = err instanceof Error ? err.message : "Login failed";
    } finally {
      isLoading = false;
    }
  }

  function isValidEmail(email: string): boolean {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
  }

  function clearError() {
    error = "";
  }
</script>

<svelte:head>
  <title>Sign In - SaaS Boilerplate</title>
</svelte:head>

<div
  class="container relative min-h-[calc(100vh-3.5rem)] flex-col items-center justify-center grid lg:max-w-none lg:grid-cols-2 lg:px-0"
>
  <div
    class="relative hidden h-full flex-col bg-muted p-10 text-white lg:flex dark:border-r"
  >
    <div
      class="absolute inset-0 bg-linear-to-br from-blue-600 to-purple-700">
    </div>
    <div class="relative z-20 flex items-center text-lg font-medium">
      <div
        class="mr-2 h-6 w-6 rounded bg-white/10 flex items-center justify-center"
      >
        <span class="font-bold text-sm">S</span>
      </div>
      SaaS Boilerplate
    </div>
    <div class="relative z-20 mt-auto">
      <blockquote class="space-y-2">
        <p class="text-lg">
          "This platform has revolutionized how we manage our projects. The
          seamless integration and intuitive design make it a joy to use."
        </p>
        <footer class="text-sm">Sofia Davis</footer>
      </blockquote>
    </div>
  </div>

  <div class="lg:p-8">
    <div
      class="mx-auto flex w-full flex-col justify-center space-y-6 sm:w-[350px]"
    >
      <div class="flex flex-col space-y-2 text-center">
        <h1 class="text-2xl font-semibold tracking-tight">Welcome back</h1>
        <p class="text-sm text-muted-foreground">
          Enter your email and password to sign in to your account
        </p>
      </div>

      <Card.Root>
        <Card.Content class="pt-6">
          <form onsubmit={handleSubmit} class="space-y-4">
            {#if error}
              <div
                class="flex items-center space-x-2 p-3 text-sm text-destructive bg-destructive/10 border border-destructive/20 rounded-md"
              >
                <AlertCircle class="h-4 w-4" />
                <span>{error}</span>
              </div>
            {/if}

            <div class="space-y-2">
              <Label for="email">Email</Label>
              <div class="relative">
                <Mail
                  class="absolute left-3 top-3 h-4 w-4 text-muted-foreground"
                />
                <Input
                  id="email"
                  type="email"
                  placeholder="Enter your email"
                  class="pl-10"
                  bind:value={email}
                  oninput={clearError}
                  disabled={isLoading}
                  required
                />
              </div>
            </div>

            <div class="space-y-2">
              <Label for="password">Password</Label>
              <div class="relative">
                <Lock
                  class="absolute left-3 top-3 h-4 w-4 text-muted-foreground"
                />
                <Input
                  id="password"
                  type="password"
                  placeholder="Enter your password"
                  class="pl-10"
                  bind:value={password}
                  oninput={clearError}
                  disabled={isLoading}
                  required
                />
              </div>
            </div>

            <Button type="submit" class="w-full" disabled={isLoading}>
              {#if isLoading}
                <LoadingSpinner size={16} class="mr-2" />
                Signing in...
              {:else}
                Sign In
              {/if}
            </Button>
          </form>

          <div class="mt-6 text-center text-sm">
            <span class="text-muted-foreground">Don't have an account? </span>
            <a href="/signup" class="font-medium text-primary hover:underline">
              Sign up
            </a>
          </div>
        </Card.Content>
      </Card.Root>

      <p class="px-8 text-center text-sm text-muted-foreground">
        By clicking sign in, you agree to our{" "}
        <a
          href="/terms"
          class="underline underline-offset-4 hover:text-primary"
        >
          Terms of Service
        </a>
        {" "}and{" "}
        <a
          href="/privacy"
          class="underline underline-offset-4 hover:text-primary"
        >
          Privacy Policy
        </a>
        .
      </p>
    </div>
  </div>
</div>
