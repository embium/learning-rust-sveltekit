<script lang="ts">
  import { auth } from "$lib/stores/auth";
  import { projectsAPI, type Project } from "$lib/api/projects";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import LoadingSpinner from "$lib/components/LoadingSpinner.svelte";
  import { Plus, FolderOpen, BarChart3, Users, Activity } from "lucide-svelte";
  import { onMount } from "svelte";

  let projects = $state<Project[]>([]);
  let isLoadingProjects = $state(false);
  let projectsError = $state("");

  onMount(async () => {
    await loadProjects();
  });

  async function loadProjects() {
    if (!$auth.user) return;

    isLoadingProjects = true;
    projectsError = "";

    try {
      projects = await projectsAPI.listProjects();
    } catch (error) {
      projectsError =
        error instanceof Error ? error.message : "Failed to load projects";
      console.error("Failed to load projects:", error);
    } finally {
      isLoadingProjects = false;
    }
  }

  // Mock stats for demonstration
  let stats = $derived([
    {
      title: "Total Projects",
      value: projects.length.toString(),
      icon: FolderOpen,
      description: "Active projects",
    },
    {
      title: "Team Members",
      value: "4",
      icon: Users,
      description: "Collaborators",
    },
    {
      title: "Tasks Completed",
      value: "23",
      icon: Activity,
      description: "This week",
    },
    {
      title: "Success Rate",
      value: "98%",
      icon: BarChart3,
      description: "Project completion",
    },
  ]);

  function getInitials(name: string): string {
    return name
      .split(" ")
      .map((word) => word.charAt(0))
      .join("")
      .toUpperCase()
      .substring(0, 2);
  }

  function formatDate(dateString: string): string {
    return new Date(dateString).toLocaleDateString("en-US", {
      year: "numeric",
      month: "short",
      day: "numeric",
    });
  }
</script>

<svelte:head>
  <title>Dashboard - SaaS Boilerplate</title>
</svelte:head>

<div class="container mx-auto py-8 space-y-8">
  <!-- Welcome Section -->
  <div
    class="flex flex-col space-y-4 md:flex-row md:items-center md:justify-between md:space-y-0"
  >
    <div>
      <h1 class="text-3xl font-bold tracking-tight">
        Welcome back{$auth.user ? `, ${$auth.user.email.split("@")[0]}` : ""}!
      </h1>
      <p class="text-muted-foreground">
        Here's what's happening with your projects today.
      </p>
    </div>
    <Button href="/projects" class="w-fit">
      <Plus class="mr-2 h-4 w-4" />
      New Project
    </Button>
  </div>

  <!-- Stats Grid -->
  <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
    {#each stats as stat (stat.title)}
      <Card.Root>
        <Card.Content class="p-6">
          <div class="flex items-center space-x-2">
            <stat.icon class="h-4 w-4 text-muted-foreground" />
            <h3 class="text-sm font-medium">{stat.title}</h3>
          </div>
          <div class="mt-2">
            <p class="text-2xl font-bold">{stat.value}</p>
            <p class="text-xs text-muted-foreground mt-1">{stat.description}</p>
          </div>
        </Card.Content>
      </Card.Root>
    {/each}
  </div>

  <!-- Recent Projects -->
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <h2 class="text-xl font-semibold">Recent Projects</h2>
      <Button variant="outline" href="/projects" size="sm">View All</Button>
    </div>

    <Card.Root>
      <Card.Header>
        <Card.Title>Your Projects</Card.Title>
        <Card.Description>
          Manage and track your active projects
        </Card.Description>
      </Card.Header>
      <Card.Content>
        {#if isLoadingProjects}
          <div class="flex items-center justify-center py-8">
            <LoadingSpinner size={24} text="Loading projects..." />
          </div>
        {:else if projectsError}
          <div class="flex flex-col items-center justify-center py-8 space-y-2">
            <p class="text-sm text-destructive">{projectsError}</p>
            <Button variant="outline" size="sm" onclick={loadProjects}>
              Try Again
            </Button>
          </div>
        {:else if projects.length === 0}
          <div class="flex flex-col items-center justify-center py-8 space-y-4">
            <FolderOpen class="h-12 w-12 text-muted-foreground" />
            <div class="text-center space-y-2">
              <h3 class="font-medium">No projects yet</h3>
              <p class="text-sm text-muted-foreground">
                Get started by creating your first project
              </p>
              <Button href="/projects" size="sm">
                <Plus class="mr-2 h-4 w-4" />
                Create Project
              </Button>
            </div>
          </div>
        {:else}
          <div class="space-y-4">
            {#each projects.slice(0, 5) as project (project.id)}
              <div
                class="flex items-center space-x-4 p-4 border rounded-lg hover:bg-muted/50 transition-colors"
              >
                <div
                  class="h-10 w-10 rounded-full bg-primary/10 flex items-center justify-center"
                >
                  <span class="text-sm font-medium text-primary">
                    {getInitials(project.name)}
                  </span>
                </div>
                <div class="flex-1 space-y-1">
                  <h4 class="font-medium">{project.name}</h4>
                  {#if project.description}
                    <p class="text-sm text-muted-foreground line-clamp-1">
                      {project.description}
                    </p>
                  {/if}
                </div>
                <div class="text-sm text-muted-foreground">
                  {#if project.created_at}
                    Created {formatDate(project.created_at)}
                  {/if}
                </div>
                <Button variant="ghost" size="sm" href="/projects/{project.id}">
                  View
                </Button>
              </div>
            {/each}
          </div>
        {/if}
      </Card.Content>
    </Card.Root>
  </div>

  <!-- Quick Actions -->
  <Card.Root>
    <Card.Header>
      <Card.Title>Quick Actions</Card.Title>
      <Card.Description>
        Common tasks to help you get things done
      </Card.Description>
    </Card.Header>
    <Card.Content>
      <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        <Button
          variant="outline"
          class="h-24 flex-col space-y-2"
          href="/projects"
        >
          <Plus class="h-6 w-6" />
          <span>Create Project</span>
        </Button>

        <Button
          variant="outline"
          class="h-24 flex-col space-y-2"
          href="/projects"
        >
          <FolderOpen class="h-6 w-6" />
          <span>Browse Projects</span>
        </Button>

        <Button
          variant="outline"
          class="h-24 flex-col space-y-2"
          href="/settings"
        >
          <Activity class="h-6 w-6" />
          <span>View Settings</span>
        </Button>
      </div>
    </Card.Content>
  </Card.Root>
</div>
