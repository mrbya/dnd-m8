<script lang="ts">
  import {
    CharacterStore,
    type CharacterState,
  } from "$lib/stores/caracter.svelte";
  import { onMount } from "svelte";
  import CharacterOverview from "$lib/components/views/CharacterOverview.svelte";

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
      <section class="shell-state shell-error" role="alert">
        <p class="eyebrow">Unable to load character</p>
        <h1>Something went wrong</h1>
        <p>{state.error.message}</p>

        <button type="button" onclick={() => void characterStore.load()}>
          Try again
        </button>
      </section>
    {:else}
      <section class="shell-state" aria-label="Loading character">
        <span class="loading-indicator" aria-hidden="true">
          <p>Loading character...</p>
        </span>
      </section>
    {/if}
  </main>
</div>

<style>
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

  .shell-state h1,
  .shell-state p {
    margin: 0;
  }

  .shell-state > p:not(.eyebrow) {
    color: var(--color-text-muted);
  }

  .shell-error {
    border-color: var(--color-danger);
  }

  .eyebrow {
    color: var(--color-primary);
    font-size: var(--font-size-xs);
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  .loading-indicator {
    width: 0.75rem;
    height: 0.75rem;
    border-radius: 50%;
    background: var(--color-primary);
    animation: pulse 1.2s ease-in-out infinite;
  }

  button {
    min-height: var(--touch-target-size);
    padding-inline: var(--space-4);
    border: 1px solid var(--color-primary);
    border-radius: var(--radius-md);
    background: transparent;
    color: var(--color-primary);
    font-weight: 600;
  }

  button:hover {
    background: var(--color-surface-hover);
    color: var(--color-primary-hover);
  }

  @keyframes pulse {
    50% {
      opacity: 0.35;
      transform: scale(0.8);
    }
  }
</style>
