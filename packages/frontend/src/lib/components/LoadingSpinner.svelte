<script lang="ts">
  import { Loader2 } from "lucide-svelte";
  import { cn } from "$lib/utils";

  interface Props {
    size?: number;
    class?: string;
    text?: string;
    center?: boolean;
  }

  let {
    size = 16,
    class: className = "",
    text = "",
    center = false,
  }: Props = $props();

  const containerClass = cn(
    "flex items-center",
    center && "justify-center",
    className,
  );
</script>

<div class={containerClass}>
  <Loader2 class="animate-spin text-muted-foreground" {size} />
  {#if text}
    <span class="ml-2 text-sm text-muted-foreground">
      {text}
    </span>
  {/if}
</div>
