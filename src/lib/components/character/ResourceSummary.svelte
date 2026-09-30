<script module lang="ts">
  import type { ResourcePipsTone } from "$lib/components/ui";

  export interface ResourceSummaryItem {
    readonly id: string;
    readonly label: string;
    readonly max: number;
    readonly value: number;
    readonly detail?: string;
    readonly tone?: ResourcePipsTone;
    readonly valueText?: string;
  }
</script>

<script lang="ts">
  import { Badge, Card, ResourcePips } from "$lib/components/ui";

  interface Props {
    resources?: readonly ResourceSummaryItem[];
  }

  let { resources = [] }: Props = $props();

  function normalizeMaximum(max: number): number {
    return Number.isFinite(max) ? Math.max(1, Math.trunc(max)) : 1;
  }

  function normalizeValue(value: number, max: number): number {
    if (!Number.isFinite(value)) {
      return 0;
    }

    return Math.min(Math.max(Math.trunc(value), 0), max);
  }
</script>

<Card as="section" aria-labelledby="resources-title" padding="medium">
  <div class="resource-summary">
    <header class="resource-summary__header">
      <div>
        <p class="resource-summary__eyebrow">Session</p>
        <h2 id="resources-title">Resources</h2>
      </div>

      {#if resources.length > 0}
        <Badge label={`${resources.length} tracked`} />
      {/if}
    </header>

    {#if resources.length > 0}
      <ul class="resource-summary__list">
        {#each resources as resource (resource.id)}
          {@const normalizedMax = normalizeMaximum(resource.max)}
          {@const normalizedValue = normalizeValue(
            resource.value,
            normalizedMax,
          )}

          <li class="resource-summary__item">
            <div class="resource-summary__item-header">
              <div>
                <h3>{resource.label}</h3>

                {#if resource.detail}
                  <p>{resource.detail}</p>
                {/if}
              </div>

              <span class="resource-summary__value">
                {normalizedValue}
                <span>/ {normalizedMax}</span>
              </span>
            </div>

            <ResourcePips
              label={resource.label}
              max={normalizedMax}
              tone={resource.tone ?? "primary"}
              value={normalizedValue}
              valueText={resource.valueText}
            />
          </li>
        {/each}
      </ul>
    {:else}
      <p class="resource-summary__empty">
        No limited-use resources are currently tracked.
      </p>
    {/if}
  </div>
</Card>

<style>
  .resource-summary {
    display: grid;
    gap: var(--space-4);
  }

  .resource-summary__header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--space-4);
  }

  .resource-summary__eyebrow {
    margin: 0 0 var(--space-1);
    color: var(--color-primary-text);
    font-size: var(--font-size-xs);
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  h2,
  h3,
  p {
    margin: 0;
  }

  h2 {
    font-size: var(--font-size-lg);
    line-height: 1.25;
  }

  .resource-summary__list {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(min(100%, 14rem), 1fr));
    gap: var(--space-3);
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .resource-summary__item {
    display: grid;
    align-content: start;
    gap: var(--space-3);
    min-width: 0;
    padding: var(--space-3);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background: var(--color-shell);
  }

  .resource-summary__item-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--space-3);
  }

  h3 {
    color: var(--color-text);
    font-size: var(--font-size-sm);
    line-height: 1.3;
  }

  .resource-summary__item-header p {
    margin-top: var(--space-1);
    color: var(--color-text-muted);
    font-size: var(--font-size-xs);
  }

  .resource-summary__value {
    display: flex;
    flex: 0 0 auto;
    align-items: baseline;
    color: var(--color-text);
    font-size: var(--font-size-lg);
    font-weight: 700;
    line-height: 1;
  }

  .resource-summary__value span {
    margin-left: var(--space-1);
    color: var(--color-text-muted);
    font-size: var(--font-size-xs);
    font-weight: 500;
  }

  .resource-summary__empty {
    color: var(--color-text-muted);
  }
</style>
