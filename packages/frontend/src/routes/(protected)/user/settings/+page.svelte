<script lang="ts">
  import { auth } from "$lib/stores/auth";
  import { onMount } from "svelte";
  import { userAPI, type UserUpdateRequest } from "$lib/api/user";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
  } from "$lib/components/ui/card";
  import { AlertCircle, Check } from "lucide-svelte";
  import Icon from "@iconify/svelte";
  import LoadingSpinner from "$lib/components/LoadingSpinner.svelte";

  let userFullName: string | null = $state("");
  let userEmail = $state("");
  let userProvider = $state("");
  let currentPassword = $state("");
  let newPassword = $state("");
  let confirmNewPassword = $state("");
  let isUpdating = $state(false);
  let isLoadingUser = $state(false);
  let updateError = $state("");
  let userError = $state("");
  let successMessage = $state("");
  let changePassword = $state(false);
  let hasAttemptedLoad = false;

  $effect(() => {
    if ($auth.user && !hasAttemptedLoad && !$auth.isLoading) {
      hasAttemptedLoad = true;
      loadUser();
    }
  });

  async function loadUser() {
    if (!$auth.user) return;

    isLoadingUser = true;
    userError = "";

    try {
      const user = await userAPI.getUserSettings();
      console.log(user);
      userFullName = user.fullname;
      userEmail = user.email;
      userProvider = user.provider;
    } catch (error) {
      userError =
        error instanceof Error ? error.message : "Failed to load user";
      console.error("Failed to load user:", error);
    } finally {
      isLoadingUser = false;
    }
  }

  async function handleUpdateAccount(event: Event) {
    event.preventDefault();
    isUpdating = true;
    updateError = "";
    successMessage = "";

    // Validation
    if (changePassword) {
      if (!currentPassword) {
        updateError = "Current password is required when changing password";
        isUpdating = false;
        return;
      }
      if (!newPassword) {
        updateError = "New password is required";
        isUpdating = false;
        return;
      }
      if (newPassword !== confirmNewPassword) {
        updateError = "New passwords do not match";
        isUpdating = false;
        return;
      }
      if (newPassword.length < 6) {
        updateError = "New password must be at least 6 characters";
        isUpdating = false;
        return;
      }
    }

    try {
      const updateRequest: UserUpdateRequest = {
        name: userFullName,
        email: userEmail,
      };

      if (changePassword && newPassword) {
        updateRequest.current_password = currentPassword;
        updateRequest.new_password = newPassword;
      }

      await userAPI.updateUserSettings(updateRequest);

      successMessage = changePassword
        ? "User settings and password updated successfully!"
        : "User settings updated successfully!";

      // Reset password fields
      currentPassword = "";
      newPassword = "";
      confirmNewPassword = "";
      changePassword = false;
    } catch (error) {
      updateError =
        error instanceof Error ? error.message : "Failed to update user";
      console.error("Failed to update user:", error);
    } finally {
      isUpdating = false;
    }
  }

  function togglePasswordChange() {
    changePassword = !changePassword;
    if (!changePassword) {
      currentPassword = "";
      newPassword = "";
      confirmNewPassword = "";
    }
  }
</script>

<svelte:head>
  <title>User Settings - SaaS Boilerplate</title>
</svelte:head>

<div class="container mx-auto py-8 px-4 max-w-2xl">
  <div class="mb-6">
    <h1 class="text-3xl font-bold">User Settings</h1>
    <p class="text-muted-foreground">
      Manage your user information and security settings.
    </p>
  </div>

  {#if isLoadingUser}
    <div class="flex justify-center py-8">
      <LoadingSpinner size={32} />
    </div>
  {:else if userError}
    <div
      class="flex items-center space-x-2 p-4 text-sm text-destructive bg-destructive/10 border border-destructive/20 rounded-md mb-6"
    >
      <AlertCircle class="h-4 w-4" />
      <span>{userError}</span>
    </div>
  {:else}
    <Card>
      <CardHeader>
        <CardTitle>Profile Information</CardTitle>
        <CardDescription>Update your user details below.</CardDescription>
      </CardHeader>
      <CardContent>
        <form onsubmit={handleUpdateAccount} class="space-y-6">
          {#if updateError}
            <div
              class="flex items-center space-x-2 p-3 text-sm text-destructive bg-destructive/10 border border-destructive/20 rounded-md"
            >
              <AlertCircle class="h-4 w-4" />
              <span>{updateError}</span>
            </div>
          {/if}

          {#if successMessage}
            <div
              class="flex items-center space-x-2 p-3 text-sm text-green-700 bg-green-50 border border-green-200 rounded-md"
            >
              <Check class="h-4 w-4" />
              <span>{successMessage}</span>
            </div>
          {/if}

          <div class="space-y-4">
            <div class="space-y-2">
              <Label for="name">Name</Label>
              <Input
                id="name"
                placeholder="Enter your name"
                bind:value={userFullName}
                disabled={isUpdating}
              />
            </div>

            <div class="space-y-2">
              <Label for="email">Email Address</Label>
              {#if userProvider === "email"}
                <Input
                  id="email"
                  type="email"
                  placeholder="Enter your email"
                  bind:value={userEmail}
                  disabled={isUpdating}
                  required
                />
              {/if}
              {#if userProvider === "google"}
                <div class="flex items-center space-x-2">
                  <Icon icon="mdi:google" class="h-4 w-4" />
                  <span>{userEmail}</span>
                </div>
              {/if}
            </div>
          </div>

          {#if userProvider === "email"}
            <div class="border-t pt-6">
              <div class="flex items-center justify-between mb-4">
                <div>
                  <h3 class="text-lg font-medium">Password</h3>
                  <p class="text-sm text-muted-foreground">
                    {changePassword
                      ? "Enter your current password and choose a new one"
                      : "Keep your current password"}
                  </p>
                </div>
                <Button
                  type="button"
                  variant="outline"
                  onclick={togglePasswordChange}
                  disabled={isUpdating}
                >
                  {changePassword ? "Cancel" : "Change Password"}
                </Button>
              </div>

              {#if changePassword}
                <div class="space-y-4">
                  <div class="space-y-2">
                    <Label for="current-password">Current Password</Label>
                    <Input
                      id="current-password"
                      type="password"
                      placeholder="Enter your current password"
                      bind:value={currentPassword}
                      disabled={isUpdating}
                      required={changePassword}
                    />
                  </div>

                  <div class="space-y-2">
                    <Label for="new-password">New Password</Label>
                    <Input
                      id="new-password"
                      type="password"
                      placeholder="Enter your new password"
                      bind:value={newPassword}
                      disabled={isUpdating}
                      required={changePassword}
                    />
                    <p class="text-xs text-muted-foreground">
                      Password must be at least 6 characters long
                    </p>
                  </div>

                  <div class="space-y-2">
                    <Label for="confirm-password">Confirm New Password</Label>
                    <Input
                      id="confirm-password"
                      type="password"
                      placeholder="Confirm your new password"
                      bind:value={confirmNewPassword}
                      disabled={isUpdating}
                      required={changePassword}
                    />
                  </div>
                </div>
              {/if}
            </div>
          {/if}

          <div class="flex justify-end pt-6">
            <Button type="submit" disabled={isUpdating} class="min-w-[120px]">
              {#if isUpdating}
                <LoadingSpinner size={16} class="mr-2" />
                Updating...
              {:else}
                Update Account
              {/if}
            </Button>
          </div>
        </form>
      </CardContent>
    </Card>
  {/if}
</div>
