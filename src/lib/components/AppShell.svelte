<script lang="ts">
  import {
    CharacterStore,
    type CharacterState,
  } from "$lib/stores/character.svelte";
  import { onMount } from "svelte";
  import CharacterOverview from "$lib/components/views/CharacterOverview.svelte";
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
      <section class="shell-state" aria-label="Loading character">
        <span class="loading-indicator" aria-hidden="true"></span>
        <p>Loading character…</p>
      </section>
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

  .shell-state {
    display: grid;
    justify-items: start;
    gap: var(--space-3);
    width: min(100%, 42rem);
    margin-inline: auto;
    padding: var(--space-6);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    background: var(--color-surface);
  }

  .shell-state p {
    margin: 0;
  }

  .shell-state > p:not(.eyebrow) {
    color: var(--color-text-muted);
  }

  .loading-indicator {
    width: 0.75rem;
    height: 0.75rem;
    border-radius: 50%;
    background: var(--color-primary);
    animation: pulse 1.2s ease-in-out infinite;
  }

  .shell-alert {
    width: min(100%, 42rem);
    margin-inline: auto;
  }

  @keyframes pulse {
    50% {
      opacity: 0.35;
      transform: scale(0.8);
    }
  }
</style>
