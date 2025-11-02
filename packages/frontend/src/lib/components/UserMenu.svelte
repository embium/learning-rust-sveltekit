<script lang="ts">
  import { auth } from "$lib/stores/auth";
  import { authAPI } from "$lib/api/auth";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { Button } from "$lib/components/ui/button";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
  import { Avatar, AvatarFallback } from "$lib/components/ui/avatar";
  import { Home, FolderOpen, Settings, LogOut } from "lucide-svelte";

  let isLoggingOut = $state(false);

  async function handleLogout() {
    if (isLoggingOut) return;

    isLoggingOut = true;
    try {
      await authAPI.logout();
      auth.clearUser();
      await goto(resolve("/login"));
    } catch (error) {
      console.error("Logout failed:", error);
    } finally {
      isLoggingOut = false;
    }
  }

  function getInitials(email: string): string {
    return email.charAt(0).toUpperCase();
  }
</script>

{#if $auth.isAuthenticated && $auth.user}
  <DropdownMenu.Root>
    <DropdownMenu.Trigger>
      <Button variant="ghost" class="relative h-8 w-8 rounded-full">
        <Avatar class="h-8 w-8">
          <AvatarFallback class="bg-primary text-primary-foreground">
            {getInitials($auth.user.email)}
          </AvatarFallback>
        </Avatar>
      </Button>
    </DropdownMenu.Trigger>

    <DropdownMenu.Content class="w-56" align="end">
      <DropdownMenu.Label class="font-normal">
        <div class="flex flex-col space-y-1">
          <p class="text-sm font-medium leading-none">Account</p>
          <p class="text-xs leading-none text-muted-foreground">
            {$auth.user.email}
          </p>
        </div>
      </DropdownMenu.Label>

      <DropdownMenu.Separator />

      <DropdownMenu.Group>
        <DropdownMenu.Item class="cursor-pointer">
          <a href={resolve("/dashboard")} class="flex items-center w-full">
            <Home class="mr-2 h-4 w-4" />
            <span>Dashboard</span>
          </a>
        </DropdownMenu.Item>

        <DropdownMenu.Item class="cursor-pointer">
          <a href={resolve("/projects")} class="flex items-center w-full">
            <FolderOpen class="mr-2 h-4 w-4" />
            <span>Projects</span>
          </a>
        </DropdownMenu.Item>
      </DropdownMenu.Group>

      <DropdownMenu.Separator />

      <DropdownMenu.Group>
        <DropdownMenu.Item class="cursor-pointer">
          <a href={resolve("/account")} class="flex items-center w-full">
            <Settings class="mr-2 h-4 w-4" />
            <span>Settings</span>
          </a>
        </DropdownMenu.Item>
      </DropdownMenu.Group>

      <DropdownMenu.Separator />

      <DropdownMenu.Item
        class="cursor-pointer text-destructive focus:text-destructive"
      >
        <button
          onclick={handleLogout}
          disabled={isLoggingOut}
          class="flex items-center w-full"
        >
          <LogOut class="mr-2 h-4 w-4" />
          <span>{isLoggingOut ? "Signing out..." : "Sign out"}</span>
        </button>
      </DropdownMenu.Item>
    </DropdownMenu.Content>
  </DropdownMenu.Root>
{/if}
