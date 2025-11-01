<script lang="ts">
  import { auth } from "$lib/stores/auth";
  import { projectsAPI, type Project } from "$lib/api/projects";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Textarea } from "$lib/components/ui/textarea";
  import * as Card from "$lib/components/ui/card";
  import * as Dialog from "$lib/components/ui/dialog";
  import LoadingSpinner from "$lib/components/LoadingSpinner.svelte";
  import {
    Plus,
    FolderOpen,
    Search,
    Edit,
    Trash2,
    AlertCircle,
    Calendar,
    Filter,
  } from "lucide-svelte";
  import { onMount } from "svelte";

  let projects = $state<Project[]>([]);
  let filteredProjects = $state<Project[]>([]);
  let isLoadingProjects = $state(false);
  let projectsError = $state("");
  let searchQuery = $state("");

  // Create/Edit Modal State
  let showCreateModal = $state(false);
  let showEditModal = $state(false);
  let editingProject = $state<Project | null>(null);
  let isCreating = $state(false);
  let isUpdating = $state(false);
  let modalError = $state("");

  // Form fields
  let projectName = $state("");
  let projectDescription = $state("");

  // Delete confirmation
  let showDeleteModal = $state(false);
  let deletingProject = $state<Project | null>(null);
  let isDeleting = $state(false);

  onMount(async () => {
    await loadProjects();
  });

  async function loadProjects() {
    if (!$auth.user) return;

    isLoadingProjects = true;
    projectsError = "";

    try {
      projects = await projectsAPI.listProjects();
      filteredProjects = [...projects];
    } catch (error) {
      projectsError =
        error instanceof Error ? error.message : "Failed to load projects";
      console.error("Failed to load projects:", error);
    } finally {
      isLoadingProjects = false;
    }
  }

  function openCreateModal() {
    projectName = "";
    projectDescription = "";
    modalError = "";
    showCreateModal = true;
  }

  function openEditModal(project: Project) {
    editingProject = project;
    projectName = project.name;
    projectDescription = project.description || "";
    modalError = "";
    showEditModal = true;
  }

  function openDeleteModal(project: Project) {
    deletingProject = project;
    showDeleteModal = true;
  }

  async function handleCreateProject(event: Event) {
    event.preventDefault();

    if (!projectName.trim()) {
      modalError = "Project name is required";
      return;
    }

    if (!$auth.user) {
      modalError = "User not authenticated";
      return;
    }

    isCreating = true;
    modalError = "";

    try {
      await projectsAPI.createProject({
        user_email: $auth.user.email,
        name: projectName.trim(),
        description: projectDescription.trim() || undefined,
      });

      showCreateModal = false;
      await loadProjects();
    } catch (error) {
      modalError =
        error instanceof Error ? error.message : "Failed to create project";
    } finally {
      isCreating = false;
    }
  }

  async function handleUpdateProject(event: Event) {
    event.preventDefault();

    if (!projectName.trim()) {
      modalError = "Project name is required";
      return;
    }

    if (!editingProject) return;

    isUpdating = true;
    modalError = "";

    try {
      await projectsAPI.updateProject(editingProject.id!, {
        name: projectName.trim(),
        description: projectDescription.trim() || undefined,
        update_timestamp: Date.now(),
      });

      showEditModal = false;
      await loadProjects();
    } catch (error) {
      modalError =
        error instanceof Error ? error.message : "Failed to update project";
    } finally {
      isUpdating = false;
    }
  }

  async function handleDeleteProject() {
    if (!deletingProject) return;

    isDeleting = true;

    try {
      await projectsAPI.deleteProject(deletingProject.id!);
      showDeleteModal = false;
      await loadProjects();
    } catch (error) {
      console.error("Failed to delete project:", error);
      // Handle error - maybe show toast
    } finally {
      isDeleting = false;
    }
  }

  function handleSearch() {
    if (!searchQuery.trim()) {
      filteredProjects = [...projects];
    } else {
      const query = searchQuery.toLowerCase().trim();
      filteredProjects = projects.filter(
        (project) =>
          project.name.toLowerCase().includes(query) ||
          (project.description &&
            project.description.toLowerCase().includes(query)),
      );
    }
  }

  // React to search query changes
  $effect(() => {
    handleSearch();
  });

  function getInitials(name: string) {
    return name
      .split(" ")
      .map((word) => word.charAt(0))
      .join("")
      .toUpperCase()
      .substring(0, 2);
  }

  function formatDate(dateString: string) {
    return new Date(dateString).toLocaleDateString("en-US", {
      year: "numeric",
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  }
</script>

<svelte:head>
  <title>Projects - SaaS Boilerplate</title>
</svelte:head>

<div class="container mx-auto py-8 space-y-6">
  <!-- Header -->
  <div
    class="flex flex-col space-y-4 sm:flex-row sm:items-center sm:justify-between sm:space-y-0"
  >
    <div>
      <h1 class="text-3xl font-bold tracking-tight">Projects</h1>
      <p class="text-muted-foreground">Manage and organize your projects</p>
    </div>
    <Button onclick={openCreateModal}>
      <Plus class="mr-2 h-4 w-4" />
      New Project
    </Button>
  </div>

  <!-- Search and Filters -->
  <div
    class="flex flex-col space-y-4 sm:flex-row sm:items-center sm:space-x-4 sm:space-y-0"
  >
    <div class="relative flex-1 max-w-sm">
      <Search class="absolute left-3 top-3 h-4 w-4 text-muted-foreground" />
      <Input
        placeholder="Search projects..."
        class="pl-10"
        bind:value={searchQuery}
      />
    </div>
    <Button variant="outline" size="sm">
      <Filter class="mr-2 h-4 w-4" />
      Filter
    </Button>
  </div>

  <!-- Projects Content -->
  {#if isLoadingProjects}
    <div class="flex items-center justify-center py-12">
      <LoadingSpinner size={32} text="Loading projects..." />
    </div>
  {:else if projectsError}
    <Card.Root>
      <Card.Content class="py-12">
        <div class="flex flex-col items-center justify-center space-y-4">
          <AlertCircle class="h-12 w-12 text-destructive" />
          <div class="text-center space-y-2">
            <h3 class="font-medium">Failed to load projects</h3>
            <p class="text-sm text-muted-foreground">{projectsError}</p>
            <Button variant="outline" onclick={loadProjects}>Try Again</Button>
          </div>
        </div>
      </Card.Content>
    </Card.Root>
  {:else if filteredProjects.length === 0}
    <Card.Root>
      <Card.Content class="py-12">
        <div class="flex flex-col items-center justify-center space-y-4">
          <FolderOpen class="h-12 w-12 text-muted-foreground" />
          <div class="text-center space-y-2">
            <h3 class="font-medium">
              {searchQuery ? "No projects found" : "No projects yet"}
            </h3>
            <p class="text-sm text-muted-foreground">
              {searchQuery
                ? `No projects match "${searchQuery}"`
                : "Get started by creating your first project"}
            </p>
            {#if !searchQuery}
              <Button onclick={openCreateModal}>
                <Plus class="mr-2 h-4 w-4" />
                Create Project
              </Button>
            {/if}
          </div>
        </div>
      </Card.Content>
    </Card.Root>
  {:else}
    <!-- Projects Grid -->
    <div class="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
      {#each filteredProjects as project (project.id)}
        <Card.Root class="hover:shadow-md transition-shadow">
          <Card.Header class="pb-3">
            <div class="flex items-start justify-between">
              <div class="flex items-center space-x-3">
                <div
                  class="h-10 w-10 rounded-lg bg-primary/10 flex items-center justify-center"
                >
                  <span class="text-sm font-medium text-primary">
                    {getInitials(project.name)}
                  </span>
                </div>
                <div>
                  <Card.Title class="text-base">{project.name}</Card.Title>
                </div>
              </div>
              <div class="flex items-center space-x-1">
                <Button
                  variant="ghost"
                  size="sm"
                  class="h-8 w-8 p-0"
                  onclick={() => openEditModal(project)}
                >
                  <Edit class="h-4 w-4" />
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  class="h-8 w-8 p-0 text-destructive hover:text-destructive"
                  onclick={() => openDeleteModal(project)}
                >
                  <Trash2 class="h-4 w-4" />
                </Button>
              </div>
            </div>
          </Card.Header>
          <Card.Content class="pt-0">
            {#if project.description}
              <p class="text-sm text-muted-foreground mb-4 line-clamp-2">
                {project.description}
              </p>
            {:else}
              <p class="text-sm text-muted-foreground mb-4 italic">
                No description
              </p>
            {/if}
            <div class="flex items-center text-xs text-muted-foreground">
              <Calendar class="mr-1 h-3 w-3" />
              {#if project.last_updated}
                Updated {formatDate(project.last_updated)}
              {:else if project.created_at}
                Created {formatDate(project.created_at)}
              {/if}
            </div>
          </Card.Content>
        </Card.Root>
      {/each}
    </div>
  {/if}
</div>

<!-- Create Project Modal -->
<Dialog.Root bind:open={showCreateModal}>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Create New Project</Dialog.Title>
      <Dialog.Description>
        Add a new project to your workspace. You can edit these details later.
      </Dialog.Description>
    </Dialog.Header>
    <form onsubmit={handleCreateProject}>
      <div class="grid gap-4 py-4">
        {#if modalError}
          <div
            class="flex items-center space-x-2 p-3 text-sm text-destructive bg-destructive/10 border border-destructive/20 rounded-md"
          >
            <AlertCircle class="h-4 w-4" />
            <span>{modalError}</span>
          </div>
        {/if}

        <div class="space-y-2">
          <Label for="name">Project Name</Label>
          <Input
            id="name"
            placeholder="Enter project name"
            bind:value={projectName}
            disabled={isCreating}
            required
          />
        </div>

        <div class="space-y-2">
          <Label for="description">Description</Label>
          <Textarea
            id="description"
            placeholder="Describe your project (optional)"
            bind:value={projectDescription}
            disabled={isCreating}
            rows={3}
          />
        </div>
      </div>
      <Dialog.Footer>
        <Button
          type="button"
          variant="outline"
          onclick={() => (showCreateModal = false)}
          disabled={isCreating}
        >
          Cancel
        </Button>
        <Button type="submit" disabled={isCreating}>
          {#if isCreating}
            <LoadingSpinner size={16} class="mr-2" />
            Creating...
          {:else}
            Create Project
          {/if}
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>

<!-- Edit Project Modal -->
<Dialog.Root bind:open={showEditModal}>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Edit Project</Dialog.Title>
      <Dialog.Description>Update your project information.</Dialog.Description>
    </Dialog.Header>
    <form onsubmit={handleUpdateProject}>
      <div class="grid gap-4 py-4">
        {#if modalError}
          <div
            class="flex items-center space-x-2 p-3 text-sm text-destructive bg-destructive/10 border border-destructive/20 rounded-md"
          >
            <AlertCircle class="h-4 w-4" />
            <span>{modalError}</span>
          </div>
        {/if}

        <div class="space-y-2">
          <Label for="edit-name">Project Name</Label>
          <Input
            id="edit-name"
            placeholder="Enter project name"
            bind:value={projectName}
            disabled={isUpdating}
            required
          />
        </div>

        <div class="space-y-2">
          <Label for="edit-description">Description</Label>
          <Textarea
            id="edit-description"
            placeholder="Describe your project (optional)"
            bind:value={projectDescription}
            disabled={isUpdating}
            rows={3}
          />
        </div>
      </div>
      <Dialog.Footer>
        <Button
          type="button"
          variant="outline"
          onclick={() => (showEditModal = false)}
          disabled={isUpdating}
        >
          Cancel
        </Button>
        <Button type="submit" disabled={isUpdating}>
          {#if isUpdating}
            <LoadingSpinner size={16} class="mr-2" />
            Updating...
          {:else}
            Update Project
          {/if}
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>

<!-- Delete Confirmation Modal -->
<Dialog.Root bind:open={showDeleteModal}>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Delete Project</Dialog.Title>
      <Dialog.Description>
        Are you sure you want to delete "{deletingProject?.name}"? This action
        cannot be undone.
      </Dialog.Description>
    </Dialog.Header>
    <Dialog.Footer>
      <Button
        variant="outline"
        onclick={() => (showDeleteModal = false)}
        disabled={isDeleting}
      >
        Cancel
      </Button>
      <Button
        variant="destructive"
        onclick={handleDeleteProject}
        disabled={isDeleting}
      >
        {#if isDeleting}
          <LoadingSpinner size={16} class="mr-2" />
          Deleting...
        {:else}
          Delete Project
        {/if}
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
