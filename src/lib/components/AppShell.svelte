<script lang="ts">
  import {
    CharacterStore,
    type CharacterState,
  } from "$lib/stores/character.svelte";
  import { onMount } from "svelte";
  import {
    DesktopNavigation,
    MobileNavigation,
    type AppViewId,
  } from "$lib/components/navigation";
  import {
    CharacterOverview,
    CharacterOverviewSkeleton,
    SectionPlaceholder,
  } from "$lib/components/views";
  import CharacterHeader from "./character/CharacterHeader.svelte";
  import { Alert, Button } from "$lib/components/ui";

  const characterStore = new CharacterStore();

  let characterState = $derived(characterStore.state);

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

  let activeView = $state<AppViewId>("overview");

  function selectView(view: AppViewId): void {
    activeView = view;
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
      {statusLabel(characterState)}
    </span>
  </header>

  <aside class="desktop-navigation-region">
    <DesktopNavigation {activeView} onSelect={selectView} />
  </aside>

  <main class="app-content" aria-busy={characterState.status === "loading"}>
    {#if characterState.status === "ready"}
      <div class="character-workspace">
        <CharacterHeader character={characterState.character} />

        {#if activeView === "overview"}
          <CharacterOverview character={characterState.character} />
        {:else}
          <SectionPlaceholder view={activeView} />
        {/if}
      </div>
    {:else if characterState.status === "error"}
      <div class="shell-alert">
        <Alert
          actions={retryCharacterLoad}
          description={characterState.error.message}
          role="alert"
          title="Unable to load character"
          tone="danger"
        />
      </div>
    {:else}
      <CharacterOverviewSkeleton />
    {/if}
  </main>

  <div class="mobile-navigation-region">
    <MobileNavigation {activeView} onSelect={selectView} />
  </div>
</div>

<style>
  .app-shell {
    display: grid;
    grid-template:
      "header" var(--header-height)
      "content" minmax(0, 1fr)
      "mobile-navigation" auto
      / minmax(0, 1fr);
    width: 100%;
    height: 100dvh;
    overflow: hidden;
    background: var(--color-canvas);
  }

  .app-header {
    grid-area: header;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-4);
    padding-inline: var(--space-4);
    border-bottom: 1px solid var(--color-border);
    background: var(--color-shell);
  }

  .app-name {
    color: var(--color-primary-text);
    font-size: var(--font-size-md);
    font-weight: 700;
  }

  .app-status {
    color: var(--color-text-muted);
    font-size: var(--font-size-xs);
  }

  .app-content {
    grid-area: content;
    min-width: 0;
    min-height: 0;
    min-height: 0;
    overflow: auto;
    padding: clamp(var(--space-4), 4vw, var(--space-8));
  }

  .shell-alert {
    width: min(100%, 42rem);
    margin-inline: auto;
  }

  .character-workspace {
    display: grid;
    gap: var(--space-5);
    width: min(100%, 48rem);
    margin-inline: auto;
  }

  .desktop-navigation-region {
    display: none;
    grid-area: desktop-navigation;
    min-width: 0;
    min-height: 0;
  }

  .mobile-navigation-region {
    grid-area: mobile-navigation;
    min-width: 0;
  }

  @media (min-width: 48rem) {
    .app-shell {
      grid-template:
        "header header" var(--header-height)
        "desktop-navigation content" minmax(0, 1fr)
        / 14rem minmax(0, 1fr);
    }

    .desktop-navigation-region {
      display: block;
    }

    .mobile-navigation-region {
      display: none;
    }
  }
</style>
