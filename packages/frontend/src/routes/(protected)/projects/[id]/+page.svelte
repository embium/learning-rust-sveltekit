<script lang="ts">
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { projectsAPI, type Project } from "$lib/api/projects";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Textarea } from "$lib/components/ui/textarea";
  import * as Card from "$lib/components/ui/card";
  import * as Dialog from "$lib/components/ui/dialog";
  import * as Alert from "$lib/components/ui/alert";
  import LoadingSpinner from "$lib/components/LoadingSpinner.svelte";
  import {
    ArrowLeft,
    Edit,
    Trash2,
    Calendar,
    User,
    FileText,
    AlertCircle,
    CheckCircle2,
    Settings,
    Activity,
    AlertTriangle,
    FileX,
    Lock,
    Server,
  } from "lucide-svelte";

  interface DataProps {
    data: {
      project: Project | null;
    };
  }

  let { data }: DataProps = $props();

  // Get the project ID from the URL params
  let projectId = $derived($page.params.id);

  let project = $state<Project | null>(data.project);
  let isLoading = $state(false);
  let error = $state("");
  let isRefreshing = $state(false);

  // Edit Modal State
  let showEditModal = $state(false);
  let isUpdating = $state(false);
  let modalError = $state("");
  let projectName = $state("");
  let projectDescription = $state("");

  // Delete confirmation
  let showDeleteModal = $state(false);
  let isDeleting = $state(false);

  async function loadProject() {
    if (!projectId) return;

    isLoading = true;
    error = "";

    try {
      const loadedProject = await projectsAPI.getProject(projectId);
      project = loadedProject;
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to load project";
      console.error("Failed to load project:", err);
    } finally {
      isLoading = false;
    }
  }

  function openEditModal() {
    if (!project) return;

    projectName = project.name;
    projectDescription = project.description || "";
    modalError = "";
    showEditModal = true;
  }

  function openDeleteModal() {
    showDeleteModal = true;
  }

  async function handleUpdateProject(event: Event) {
    event.preventDefault();

    if (!project?.id) return;

    if (!projectName.trim()) {
      modalError = "Project name is required";
      return;
    }

    isUpdating = true;
    modalError = "";

    try {
      const updatedProject = await projectsAPI.updateProject(project.id, {
        name: projectName.trim(),
        description: projectDescription.trim() || undefined,
      });

      project = updatedProject;
      showEditModal = false;
    } catch (err) {
      modalError =
        err instanceof Error ? err.message : "Failed to update project";
    } finally {
      isUpdating = false;
    }
  }

  async function handleDeleteProject() {
    if (!project?.id) return;

    isDeleting = true;

    try {
      await projectsAPI.deleteProject(project.id);
      // Redirect to projects list after successful deletion
      goto(resolve("/projects"));
    } catch (err) {
      console.error("Failed to delete project:", err);
      error = err instanceof Error ? err.message : "Failed to delete project";
    } finally {
      isDeleting = false;
      showDeleteModal = false;
    }
  }

  async function refreshProject() {
    if (!projectId) return;

    isRefreshing = true;
    error = "";

    try {
      const refreshedProject = await projectsAPI.getProject(projectId);
      project = refreshedProject;
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to refresh project";
    } finally {
      isRefreshing = false;
    }
  }

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
      month: "long",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  async function copyProjectId() {
    if (project?.id) {
      try {
        await navigator.clipboard.writeText(project.id);
        // Success feedback could be added here
      } catch (err) {
        console.error("Failed to copy project ID:", err);
        // Fallback for older browsers
        const textArea = document.createElement("textarea");
        textArea.value = project.id;
        document.body.appendChild(textArea);
        textArea.select();
        try {
          document.execCommand("copy");
        } catch (fallbackErr) {
          console.error("Fallback copy failed:", fallbackErr);
        }
        document.body.removeChild(textArea);
      }
    }
  }

  function getErrorIcon() {
    if (error.includes("404") || error.includes("not found")) return FileX;
    if (error.includes("403") || error.includes("denied")) return Lock;
    if (error.includes("500") || error.includes("server")) return Server;
    return AlertTriangle;
  }
</script>

<svelte:head>
  <title>{project?.name || "Loading..."} - Project Details</title>
</svelte:head>

<div class="container mx-auto py-8 space-y-6">
  <!-- Breadcrumb Navigation -->
  <nav class="flex items-center space-x-2 text-sm text-muted-foreground">
    <button
      onclick={() => {
        goto(resolve("/projects"));
      }}
      class="hover:text-foreground transition-colors"
    >
      Projects
    </button>
    <span>/</span>
    <span class="text-foreground font-medium">
      {project?.name || "Loading..."}
    </span>
  </nav>

  <!-- Header with Back Button -->
  <div class="flex items-center space-x-4">
    <Button
      variant="ghost"
      size="sm"
      onclick={() => {
        goto(resolve("/projects"));
      }}
    >
      <ArrowLeft class="mr-2 h-4 w-4" />
      Back to Projects
    </Button>
  </div>

  {#if isLoading}
    <div class="flex items-center justify-center py-12">
      <LoadingSpinner size={32} text="Loading project..." />
    </div>
  {:else if error}
    <!-- Error State -->
    <Card.Root>
      <Card.Content class="py-12">
        <div class="flex flex-col items-center justify-center space-y-6">
          <div
            class="h-16 w-16 rounded-full bg-destructive/10 flex items-center justify-center"
          >
            {#if true}
              {@const ErrorIcon = getErrorIcon()}
              <ErrorIcon class="h-8 w-8 text-destructive" />
            {/if}
          </div>
          <div class="text-center space-y-2">
            <h1 class="text-2xl font-bold tracking-tight">
              Failed to Load Project
            </h1>
            <p class="text-muted-foreground max-w-md">{error}</p>
          </div>
          <div class="flex gap-3">
            <Button onclick={loadProject}>
              <Activity class="mr-2 h-4 w-4" />
              Try Again
            </Button>
            <Button
              variant="outline"
              onclick={() => goto(resolve("/projects"))}
            >
              <ArrowLeft class="mr-2 h-4 w-4" />
              Go to Projects
            </Button>
          </div>
        </div>
      </Card.Content>
    </Card.Root>
  {:else if project}
    <!-- Project Content -->
    <div class="flex flex-col space-y-6 lg:flex-row lg:space-x-8 lg:space-y-0">
      <!-- Main Project Info -->
      <div class="flex-1">
        <Card.Root>
          <Card.Header>
            <div class="flex items-start justify-between">
              <div class="flex items-center space-x-4">
                <div
                  class="h-16 w-16 rounded-xl bg-primary/10 flex items-center justify-center"
                >
                  <span class="text-xl font-semibold text-primary">
                    {getInitials(project.name)}
                  </span>
                </div>
                <div>
                  <Card.Title class="text-2xl">{project.name}</Card.Title>
                  <Card.Description class="text-base mt-1">
                    Project ID: <button
                      class="font-mono text-xs bg-muted px-2 py-1 rounded hover:bg-muted/80 transition-colors"
                      onclick={copyProjectId}
                      title="Click to copy"
                    >
                      {project.id}
                    </button>
                  </Card.Description>
                </div>
              </div>
              <div class="flex items-center space-x-2">
                <Button
                  variant="outline"
                  size="sm"
                  onclick={refreshProject}
                  disabled={isRefreshing}
                >
                  {#if isRefreshing}
                    <LoadingSpinner size={14} class="mr-2" />
                  {:else}
                    <Activity class="mr-2 h-4 w-4" />
                  {/if}
                  Refresh
                </Button>
                <Button variant="outline" size="sm" onclick={openEditModal}>
                  <Edit class="mr-2 h-4 w-4" />
                  Edit
                </Button>
                <Button variant="outline" size="sm" onclick={openDeleteModal}>
                  <Trash2 class="mr-2 h-4 w-4" />
                  Delete
                </Button>
              </div>
            </div>
          </Card.Header>
          <Card.Content>
            <div class="space-y-6">
              <!-- Description -->
              <div>
                <h3
                  class="text-sm font-medium text-muted-foreground mb-2 flex items-center"
                >
                  <FileText class="mr-2 h-4 w-4" />
                  Description
                </h3>
                {#if project.description}
                  <p class="text-sm leading-relaxed">{project.description}</p>
                {:else}
                  <p class="text-sm text-muted-foreground italic">
                    No description provided
                  </p>
                {/if}
              </div>

              <!-- Project Details -->
              <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div>
                  <h3
                    class="text-sm font-medium text-muted-foreground mb-2 flex items-center"
                  >
                    <User class="mr-2 h-4 w-4" />
                    Owner
                  </h3>
                  <p class="text-sm">{project.user_email}</p>
                </div>

                {#if project.created_at}
                  <div>
                    <h3
                      class="text-sm font-medium text-muted-foreground mb-2 flex items-center"
                    >
                      <Calendar class="mr-2 h-4 w-4" />
                      Created
                    </h3>
                    <p class="text-sm">{formatDate(project.created_at)}</p>
                  </div>
                {/if}

                {#if project.updated_at}
                  <div>
                    <h3
                      class="text-sm font-medium text-muted-foreground mb-2 flex items-center"
                    >
                      <Activity class="mr-2 h-4 w-4" />
                      Last Updated
                    </h3>
                    <p class="text-sm">{formatDate(project.updated_at)}</p>
                  </div>
                {/if}
              </div>
            </div>
          </Card.Content>
        </Card.Root>
      </div>

      <!-- Sidebar with Quick Actions -->
      <div class="lg:w-80">
        <Card.Root>
          <Card.Header>
            <Card.Title class="flex items-center text-base">
              <Settings class="mr-2 h-4 w-4" />
              Quick Actions
            </Card.Title>
          </Card.Header>
          <Card.Content class="space-y-3">
            <Button
              variant="outline"
              size="sm"
              class="w-full justify-start"
              onclick={refreshProject}
              disabled={isRefreshing}
            >
              {#if isRefreshing}
                <LoadingSpinner size={14} class="mr-2" />
              {:else}
                <Activity class="mr-2 h-4 w-4" />
              {/if}
              Refresh Project
            </Button>
            <Button
              variant="outline"
              size="sm"
              class="w-full justify-start"
              onclick={openEditModal}
            >
              <Edit class="mr-2 h-4 w-4" />
              Edit Project
            </Button>
            <Button
              variant="outline"
              size="sm"
              class="w-full justify-start"
              onclick={copyProjectId}
            >
              <FileText class="mr-2 h-4 w-4" />
              Copy Project ID
            </Button>
            <Button
              variant="destructive"
              size="sm"
              class="w-full justify-start"
              onclick={openDeleteModal}
            >
              <Trash2 class="mr-2 h-4 w-4" />
              Delete Project
            </Button>
          </Card.Content>
        </Card.Root>

        <!-- Project Stats -->
        <Card.Root class="mt-4">
          <Card.Header>
            <Card.Title class="flex items-center text-base">
              <Activity class="mr-2 h-4 w-4" />
              Project Stats
            </Card.Title>
          </Card.Header>
          <Card.Content class="space-y-4">
            <div class="flex justify-between items-center">
              <span class="text-sm text-muted-foreground">Status</span>
              <div class="flex items-center text-sm text-green-600">
                <CheckCircle2 class="mr-1 h-3 w-3" />
                Active
              </div>
            </div>
            <div class="flex justify-between items-center">
              <span class="text-sm text-muted-foreground">Tasks</span>
              <span class="text-sm font-medium">Coming soon</span>
            </div>
            <div class="flex justify-between items-center">
              <span class="text-sm text-muted-foreground">Contributors</span>
              <span class="text-sm font-medium">1</span>
            </div>
          </Card.Content>
        </Card.Root>
      </div>
    </div>
  {/if}
</div>

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
          <Alert.Root variant="destructive">
            <AlertCircle class="h-4 w-4" />
            <Alert.Description>{modalError}</Alert.Description>
          </Alert.Root>
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
        Are you sure you want to delete "{project?.name}"? This action cannot be
        undone.
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
