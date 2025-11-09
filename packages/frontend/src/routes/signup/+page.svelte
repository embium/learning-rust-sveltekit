<script lang="ts">
  import { auth } from "$lib/stores/auth";
  import { authAPI } from "$lib/api/auth";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import * as Card from "$lib/components/ui/card";
  import LoadingSpinner from "$lib/components/LoadingSpinner.svelte";
  import {
    Mail,
    Lock,
    AlertCircle,
    CheckCircle,
    EyeOff,
    Eye,
  } from "lucide-svelte";
  import { onMount } from "svelte";
  import { browser } from "$app/environment";

  let email = $state("");
  let password = $state("");
  let confirmPassword = $state("");
  let isLoading = $state(false);
  let error = $state("");
  let success = $state(false);
  let showPassword = $state(false);
  let showConfirmPassword = $state(false);

  onMount(() => {
    // Handle Google OAuth callback
    if (browser) {
      const urlParams = new URLSearchParams(window.location.search);
      const code = urlParams.get("code");
      const error = urlParams.get("error");

      if (error) {
        console.error("OAuth error:", error);
        return;
      }

      if (code) {
        handleGoogleCallback(code);
      }
    }
  });

  async function handleSubmit(event: Event) {
    event.preventDefault();

    if (!email || !password || !confirmPassword) {
      error = "Please fill in all fields";
      return;
    }

    if (!isValidEmail(email)) {
      error = "Please enter a valid email address";
      return;
    }

    if (password.length < 8) {
      error = "Password must be at least 8 characters long";
      return;
    }

    if (password !== confirmPassword) {
      error = "Passwords do not match";
      return;
    }

    isLoading = true;
    error = "";

    try {
      await authAPI.signup({ email, password });
      success = true;

      // Auto-redirect after showing success message
      setTimeout(() => {
        goto(resolve("/login"));
      }, 2000);
    } catch (err) {
      error = err instanceof Error ? err.message : "Signup failed";
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

  async function handleGoogleLogin() {
    isLoading = true;
    error = "";

    try {
      const googleAuthUrl = await authAPI.getGoogleAuthUrl();
      if (googleAuthUrl) {
        // Redirect to Google OAuth
        window.location.href = googleAuthUrl;
      } else {
        throw new Error("Failed to get Google authentication URL");
      }
    } catch (err) {
      error = err instanceof Error ? err.message : "Google login failed";
      isLoading = false;
    }
  }

  async function handleGoogleCallback(code: string) {
    isLoading = true;
    error = "";

    try {
      const user = await authAPI.handleGoogleCallback(code);
      if (user) {
        auth.setUser(user);
      }

      // Redirect to dashboard after successful Google authentication
      await goto(resolve("/dashboard"));
    } catch (err) {
      error =
        err instanceof Error ? err.message : "Google authentication failed";
      isLoading = false;
    }
  }

  function getPasswordStrength(password: string): {
    score: number;
    text: string;
    color: string;
  } {
    let score = 0;
    if (password.length >= 8) score++;
    if (password.match(/[a-z]/)) score++;
    if (password.match(/[A-Z]/)) score++;
    if (password.match(/[0-9]/)) score++;
    if (password.match(/[^A-Za-z0-9]/)) score++;

    if (score < 2) return { score, text: "Weak", color: "text-red-500" };
    if (score < 4) return { score, text: "Medium", color: "text-yellow-500" };
    return { score, text: "Strong", color: "text-green-500" };
  }

  const passwordStrength = $derived(
    password ? getPasswordStrength(password) : null,
  );
</script>

<svelte:head>
  <title>Sign Up - SaaS Boilerplate</title>
</svelte:head>

<div
  class="container relative min-h-[calc(100vh-3.5rem)] flex-col items-center justify-center grid lg:max-w-none lg:grid-cols-2 lg:px-0"
>
  <div
    class="relative hidden h-full flex-col bg-muted p-10 text-white lg:flex dark:border-r"
  >
    <div
      class="absolute inset-0 bg-linear-to-br from-green-600 to-blue-700"
    ></div>
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
          "Join thousands of users who trust our platform for their project
          management needs. Get started in minutes."
        </p>
        <footer class="text-sm">Alex Chen, Product Manager</footer>
      </blockquote>
    </div>
  </div>

  <div class="lg:p-8">
    <div
      class="mx-auto flex w-full flex-col justify-center space-y-6 sm:w-[400px]"
    >
      <div class="flex flex-col space-y-2 text-center">
        <h1 class="text-2xl font-semibold tracking-tight">Create an account</h1>
        <p class="text-sm text-muted-foreground">
          Enter your information to create your account
        </p>
      </div>

      <Card.Root>
        <Card.Content class="pt-6">
          {#if success}
            <div class="flex flex-col items-center space-y-4 p-6">
              <CheckCircle class="h-12 w-12 text-green-500" />
              <div class="text-center space-y-2">
                <h3 class="text-lg font-semibold">
                  Account created successfully!
                </h3>
                <p class="text-sm text-muted-foreground">
                  Redirecting you to the login page...
                </p>
              </div>
            </div>
          {:else}
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
                    type={showPassword ? "text" : "password"}
                    placeholder="Create a password (min 8 characters)"
                    class="pl-10 pr-10"
                    bind:value={password}
                    oninput={clearError}
                    disabled={isLoading}
                    required
                  />
                  <button
                    type="button"
                    class="absolute right-3 top-3 h-4 w-4 text-muted-foreground hover:text-foreground"
                    onclick={() => (showPassword = !showPassword)}
                  >
                    {#if showPassword}
                      <EyeOff class="h-4 w-4" />
                    {:else}
                      <Eye class="h-4 w-4" />
                    {/if}
                  </button>
                </div>
                {#if passwordStrength && password}
                  <div class="flex items-center space-x-2 text-xs">
                    <div class="flex space-x-1">
                      {#each Array.from({ length: 5 }, (_, i) => i) as i (i)}
                        <div
                          class="h-1 w-6 rounded-full {i <
                          passwordStrength.score
                            ? passwordStrength.color.replace('text-', 'bg-')
                            : 'bg-gray-200'}"
                        ></div>
                      {/each}
                    </div>
                    <span class={passwordStrength.color}
                      >{passwordStrength.text}</span
                    >
                  </div>
                {/if}
              </div>

              <div class="space-y-2">
                <Label for="confirmPassword">Confirm Password</Label>
                <div class="relative">
                  <Lock
                    class="absolute left-3 top-3 h-4 w-4 text-muted-foreground"
                  />
                  <Input
                    id="confirmPassword"
                    type={showConfirmPassword ? "text" : "password"}
                    placeholder="Confirm your password"
                    class="pl-10 pr-10"
                    bind:value={confirmPassword}
                    oninput={clearError}
                    disabled={isLoading}
                    required
                  />
                  <button
                    type="button"
                    class="absolute right-3 top-3 h-4 w-4 text-muted-foreground hover:text-foreground"
                    onclick={() => (showConfirmPassword = !showConfirmPassword)}
                  >
                    {#if showConfirmPassword}
                      <EyeOff class="h-4 w-4" />
                    {:else}
                      <Eye class="h-4 w-4" />
                    {/if}
                  </button>
                </div>
              </div>

              <Button type="submit" class="w-full" disabled={isLoading}>
                {#if isLoading}
                  <LoadingSpinner size={16} class="mr-2" />
                  Creating account...
                {:else}
                  Create account
                {/if}
              </Button>

              <div class="relative">
                <div class="absolute inset-0 flex items-center">
                  <span class="w-full border-t" />
                </div>
                <div class="relative flex justify-center text-xs uppercase">
                  <span class="bg-background px-2 text-muted-foreground">
                    Or continue with
                  </span>
                </div>
              </div>

              <Button
                type="button"
                variant="outline"
                class="w-full"
                onclick={handleGoogleLogin}
                disabled={isLoading}
              >
                <svg class="mr-2 h-4 w-4" viewBox="0 0 24 24">
                  <path
                    fill="currentColor"
                    d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
                  />
                  <path
                    fill="currentColor"
                    d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
                  />
                  <path
                    fill="currentColor"
                    d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
                  />
                  <path
                    fill="currentColor"
                    d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
                  />
                </svg>
                Google
              </Button>
            </form>
          {/if}

          {#if !success}
            <div class="mt-6 text-center text-sm">
              <span class="text-muted-foreground"
                >Already have an account?
              </span>
              <a
                href={resolve("/login")}
                class="font-medium text-primary hover:underline"
              >
                Sign in
              </a>
            </div>
          {/if}
        </Card.Content>
      </Card.Root>

      <p class="px-8 text-center text-sm text-muted-foreground">
        By creating an account, you agree to our
        <a
          href={resolve("/terms")}
          class="underline underline-offset-4 hover:text-primary"
        >
          Terms of Service
        </a>
        and
        <a
          href={resolve("/privacy")}
          class="underline underline-offset-4 hover:text-primary"
        >
          Privacy Policy
        </a>
        .
      </p>
    </div>
  </div>
</div>
