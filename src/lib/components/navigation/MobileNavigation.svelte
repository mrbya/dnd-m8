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

<nav class="mobile-navigation" aria-label="Character sections">
  <ul class="mobile-navigation__list">
    {#each appNavigationItems as item (item.id)}
      {@const Icon = item.icon}

      <li>
        <button
          class="mobile-navigation__button"
          type="button"
          aria-current={activeView === item.id ? "page" : undefined}
          onclick={() => onSelect(item.id)}
        >
          <span class="mobile-navigation__icon" aria-hidden="true">
            <Icon size={19} strokeWidth={1.75} />
          </span>

          <span class="mobile-navigation__label">
            {item.label}
          </span>
        </button>
      </li>
    {/each}
  </ul>
</nav>

<style>
  .mobile-navigation {
    width: 100%;
    padding: var(--space-1) var(--space-2)
      max(var(--space-1), env(safe-area-inset-bottom, 0px));
    border-top: 1px solid var(--color-border);
    background: var(--color-shell);
  }

  .mobile-navigation__list {
    display: grid;
    grid-template-columns: repeat(5, minmax(0, 1fr));
    gap: var(--space-1);
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .mobile-navigation__button {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: var(--space-1);
    width: 100%;
    min-width: 0;
    min-height: var(--touch-target-size);
    padding: var(--space-1);
    border: 1px solid transparent;
    border-radius: var(--radius-md);
    background: transparent;
    color: var(--color-text-muted);
    font-weight: 600;
    line-height: 1;
    text-align: center;
    transition:
      background-color var(--duration-fast),
      border-color var(--duration-fast),
      color var(--duration-fast),
      transform var(--duration-fast);
  }

  .mobile-navigation__button:hover {
    background: var(--color-surface);
    color: var(--color-text);
  }

  .mobile-navigation__button[aria-current="page"] {
    border-color: var(--color-border);
    background: var(--color-surface);
    color: var(--color-primary-text);
  }

  .mobile-navigation__button:active {
    transform: translateY(1px);
  }

  .mobile-navigation__icon {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .mobile-navigation__label {
    max-width: 100%;
    font-size: clamp(0.6875rem, 2.4vw, var(--font-size-xs));
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
