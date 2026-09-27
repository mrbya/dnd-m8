<script lang="ts">
  interface Props {
    label: string;
    value: string | number;
    suffix?: string;
  }

  let { label, value, suffix }: Props = $props();
</script>

<div class="summary-stat">
  <dt>{label}</dt>

  <dd>
    {value}

    {#if suffix}
      <span>{suffix}</span>
    {/if}
  </dd>
</div>

<style>
  .summary-stat {
    min-width: 0;
    padding: var(--space-4);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background: var(--color-surface);
  }

  dt {
    overflow: hidden;
    color: var(--color-text-muted);
    font-size: var(--font-size-xs);
    font-weight: 600;
    letter-spacing: 0.06em;
    text-overflow: ellipsis;
    text-transform: uppercase;
    white-space: nowrap;
  }

  dd {
    margin: var(--space-1) 0 0;
    color: var(--color-text);
    font-size: var(--font-size-xl);
    font-weight: 700;
    line-height: 1.2;
  }

  dd span {
    margin-left: var(--space-1);
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    font-weight: 500;
  }
</style>
