<script module lang="ts">
  import type { BadgeTone } from "$lib/components/ui";

  export interface CurrentStateItem {
    readonly id: string;
    readonly label: string;
    readonly tone?: BadgeTone;
  }
</script>

<script lang="ts">
  import { Badge, Card } from "$lib/components/ui";

  interface Props {
    conditions?: readonly CurrentStateItem[];
    concentration?: string | null;
    effects?: readonly CurrentStateItem[];
  }

  let { conditions = [], concentration = null, effects = [] }: Props = $props();

  const hasConcentration = $derived(Boolean(concentration?.trim()));

  const activeCount = $derived(
    conditions.length + effects.length + (hasConcentration ? 1 : 0),
  );

  const hasActiveState = $derived(activeCount > 0);
</script>

<Card as="section" aria-labelledby="current-state-title" padding="medium">
  <div class="current-state">
    <header class="current-state__header">
      <div>
        <p class="current-state__eyebrow">Session</p>
        <h2 id="current-state-title">Current state</h2>
      </div>

      {#if hasActiveState}
        <Badge label={`${activeCount} active`} tone="primary" />
      {/if}
    </header>

    {#if hasActiveState}
      <div class="current-state__groups">
        {#if hasConcentration}
          <section aria-labelledby="concentration-title">
            <h3 id="concentration-title">Concentration</h3>

            <Badge
              dot
              label={concentration ?? ""}
              size="medium"
              tone="primary"
            />
          </section>
        {/if}

        {#if conditions.length > 0}
          <section aria-labelledby="conditions-title">
            <h3 id="conditions-title">Conditions</h3>

            <ul class="current-state__items">
              {#each conditions as condition (condition.id)}
                <li>
                  <Badge
                    dot
                    label={condition.label}
                    size="medium"
                    tone={condition.tone ?? "warning"}
                  />
                </li>
              {/each}
            </ul>
          </section>
        {/if}

        {#if effects.length > 0}
          <section aria-labelledby="effects-title">
            <h3 id="effects-title">Effects</h3>

            <ul class="current-state__items">
              {#each effects as effect (effect.id)}
                <li>
                  <Badge
                    dot
                    label={effect.label}
                    size="medium"
                    tone={effect.tone ?? "success"}
                  />
                </li>
              {/each}
            </ul>
          </section>
        {/if}
      </div>
    {:else}
      <p class="current-state__empty">
        No active conditions, effects or concentration.
      </p>
    {/if}
  </div>
</Card>

<style>
  .current-state {
    display: grid;
    gap: var(--space-4);
  }

  .current-state__header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--space-4);
  }

  .current-state__eyebrow {
    margin: 0 0 var(--space-1);
    color: var(--color-primary-text);
    font-size: var(--font-size-xs);
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  h2,
  h3 {
    margin: 0;
  }

  h2 {
    font-size: var(--font-size-lg);
    line-height: 1.25;
  }

  h3 {
    margin-bottom: var(--space-2);
    color: var(--color-text-muted);
    font-size: var(--font-size-xs);
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }

  .current-state__groups {
    display: grid;
    gap: var(--space-4);
  }

  .current-state__items {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2);
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .current-state__empty {
    margin: 0;
    color: var(--color-text-muted);
  }
</style>
