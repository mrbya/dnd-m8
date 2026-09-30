<script lang="ts">
  import {
    appNavigationItems,
    type AppViewId,
  } from "$lib/components/navigation";

  interface Props {
    activeView: AppViewId;
    onSelect: (view: AppViewId) => void;
  }

  let { activeView, onSelect }: Props = $props();
</script>

<nav class="desktop-navigation" aria-label="Character sections">
  <ul class="desktop-navigation__list">
    {#each appNavigationItems as item (item.id)}
      {@const Icon = item.icon}

      <li>
        <button
          class="desktop-navigation__button"
          type="button"
          aria-current={activeView === item.id ? "page" : undefined}
          onclick={() => onSelect(item.id)}
        >
          <span class="desktop-navigation__icon" aria-hidden="true">
            <Icon size={20} strokeWidth={1.75} />
          </span>

          <span>{item.label}</span>
        </button>
      </li>
    {/each}
  </ul>
</nav>

<style>
  .desktop-navigation {
    width: 100%;
    height: 100%;
    padding: var(--space-3);
    border-right: 1px solid var(--color-border);
    background: var(--color-shell);
  }

  .desktop-navigation__list {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .desktop-navigation__button {
    display: grid;
    grid-template-columns: 1.25rem minmax(0, 1fr);
    align-items: center;
    gap: var(--space-3);
    width: 100%;
    min-height: var(--touch-target-size);
    padding: var(--space-2) var(--space-3);
    border: 1px solid transparent;
    border-radius: var(--radius-md);
    background: transparent;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    font-weight: 600;
    line-height: 1.25;
    text-align: left;
    transition:
      background-color var(--duration-fast),
      border-color var(--duration-fast),
      color var(--duration-fast);
  }

  .desktop-navigation__button:hover {
    background: var(--color-surface);
    color: var(--color-text);
  }

  .desktop-navigation__button[aria-current="page"] {
    border-color: var(--color-border);
    background: var(--color-surface);
    color: var(--color-primary-text);
  }

  .desktop-navigation__icon {
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
