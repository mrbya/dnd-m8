<script lang="ts">
  import {
    CharacterStore,
    type CharacterState,
  } from "$lib/stores/character.svelte";
  import { onMount } from "svelte";
  import {
    CharacterOverview,
    CharacterOverviewSkeleton,
  } from "$lib/components/views";
  import { Alert, Button } from "$lib/components/ui";

  const characterStore = new CharacterStore();

  let state = $derived(characterStore.state);

  onMount(() => {
    void characterStore.load();
  });

  function statusLabel(characterState: CharacterState): string {
    switch (characterState.status) {
      case "idle":
        return "Starting";
      case "loading":
        return "Loading character";
      case "ready":
        return "Ready";
      case "error":
        return "Needs attention";
    }
  }
</script>

{#snippet retryCharacterLoad()}
  <Button variant="secondary" onclick={() => void characterStore.load()}>
    Try again
  </Button>
{/snippet}

<div class="app-shell">
  <header class="app-header">
    <span class="app-name">D&amp;D Mate</span>
    <span class="app-status" aria-live="polite">
      {statusLabel(state)}
    </span>
  </header>

  <main class="app-content" aria-busy={state.status === "loading"}>
    {#if state.status === "ready"}
      <CharacterOverview character={state.character} />
    {:else if state.status === "error"}
      <div class="shell-alert">
        <Alert
          actions={retryCharacterLoad}
          description={state.error.message}
          role="alert"
          title="Unable to load character"
          tone="danger"
        />
      </div>
    {:else}
      <CharacterOverviewSkeleton />
    {/if}
  </main>
</div>

<style>
  .app-shell {
    display: grid;
    grid-template-rows: var(--header-height) minmax(0, 1fr);
    width: 100%;
    height: 100dvh;
    background: var(--color-canvas);
  }

  .app-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-4);
    padding-inline: var(--space-4);
    border-bottom: 1px solid var(--color-border);
    background: var(--color-shell);
  }

  .app-name {
    color: var(--color-primary);
    font-size: var(--font-size-md);
    font-weight: 700;
  }

  .app-status {
    color: var(--color-text-muted);
    font-size: var(--font-size-xs);
  }

  .app-content {
    min-height: 0;
    overflow: auto;
    padding: clamp(var(--space-4), 4vw, var(--space-8));
  }

  .shell-alert {
    width: min(100%, 42rem);
    margin-inline: auto;
  }
</style>
