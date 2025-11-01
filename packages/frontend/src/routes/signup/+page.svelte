<script lang="ts">
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
    Eye,
    EyeOff,
  } from "lucide-svelte";
  import { onMount } from "svelte";
  import { auth } from "$lib/stores/auth";

  let email = $state("");
  let password = $state("");
  let confirmPassword = $state("");
  let isLoading = $state(false);
  let error = $state("");
  let success = $state(false);
  let showPassword = $state(false);
  let showConfirmPassword = $state(false);

  onMount(() => {
    // Redirect if already authenticated
    if ($auth.isAuthenticated) {
      goto(resolve("/dashboard"));
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
                  Create Account
                {/if}
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
