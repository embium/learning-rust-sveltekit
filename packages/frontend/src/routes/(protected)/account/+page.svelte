<script lang="ts">
  import { auth } from "$lib/stores/auth";
  import { onMount } from "svelte";
  import {
    accountAPI,
    type AccountSettings,
    type AccountUpdateRequest,
  } from "$lib/api/account";
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
  import LoadingSpinner from "$lib/components/LoadingSpinner.svelte";

  let accountName: string | null = "";
  let accountEmail = "";
  let currentPassword = "";
  let newPassword = "";
  let confirmNewPassword = "";
  let isUpdating = false;
  let isLoadingAccount = false;
  let updateError = "";
  let accountError = "";
  let successMessage = "";
  let changePassword = false;

  onMount(async () => {
    await loadAccount();
  });

  async function loadAccount() {
    if (!$auth.user) return;

    isLoadingAccount = true;
    accountError = "";

    try {
      const account = await accountAPI.getAccountSettings();
      accountName = account.name;
      accountEmail = account.email;
    } catch (error) {
      accountError =
        error instanceof Error ? error.message : "Failed to load account";
      console.error("Failed to load account:", error);
    } finally {
      isLoadingAccount = false;
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
      const updateRequest: AccountUpdateRequest = {
        name: accountName,
        email: accountEmail,
      };

      if (changePassword && newPassword) {
        updateRequest.current_password = currentPassword;
        updateRequest.new_password = newPassword;
      }

      await accountAPI.updateAccountSettings(updateRequest);

      successMessage = changePassword
        ? "Account settings and password updated successfully!"
        : "Account settings updated successfully!";

      // Reset password fields
      currentPassword = "";
      newPassword = "";
      confirmNewPassword = "";
      changePassword = false;
    } catch (error) {
      updateError =
        error instanceof Error ? error.message : "Failed to update account";
      console.error("Failed to update account:", error);
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
  <title>Account Settings - SaaS Boilerplate</title>
</svelte:head>

<div class="container mx-auto py-8 px-4 max-w-2xl">
  <div class="mb-6">
    <h1 class="text-3xl font-bold">Account Settings</h1>
    <p class="text-muted-foreground">
      Manage your account information and security settings.
    </p>
  </div>

  {#if isLoadingAccount}
    <div class="flex justify-center py-8">
      <LoadingSpinner size={32} />
    </div>
  {:else if accountError}
    <div
      class="flex items-center space-x-2 p-4 text-sm text-destructive bg-destructive/10 border border-destructive/20 rounded-md mb-6"
    >
      <AlertCircle class="h-4 w-4" />
      <span>{accountError}</span>
    </div>
  {:else}
    <Card>
      <CardHeader>
        <CardTitle>Profile Information</CardTitle>
        <CardDescription>Update your account details below.</CardDescription>
      </CardHeader>
      <CardContent>
        <form on:submit={handleUpdateAccount} class="space-y-6">
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
                bind:value={accountName}
                disabled={isUpdating}
              />
            </div>

            <div class="space-y-2">
              <Label for="email">Email Address</Label>
              <Input
                id="email"
                type="email"
                placeholder="Enter your email"
                bind:value={accountEmail}
                disabled={isUpdating}
                required
              />
            </div>
          </div>

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
