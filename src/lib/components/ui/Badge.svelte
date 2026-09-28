<script module lang="ts">
  export type BadgeSize = "small" | "medium";

  export type BadgeTone =
    "neutral" | "primary" | "success" | "warning" | "danger";
</script>

<script lang="ts">
  import type { HTMLAttributes } from "svelte/elements";

  type Props = Omit<HTMLAttributes<HTMLSpanElement>, "children" | "class"> & {
    class?: HTMLAttributes<HTMLSpanElement>["class"];
    dot?: boolean;
    label: string;
    size?: BadgeSize;
    tone?: BadgeTone;
  };

  let {
    class: className,
    dot = false,
    label,
    size = "small",
    tone = "neutral",
    ...attributes
  }: Props = $props();
</script>

<span
  {...attributes}
  class={["badge", `badge--${size}`, `badge--${tone}`, className]}
>
  {#if dot}
    <span class="badge__dot" aria-hidden="true"></span>
  {/if}

  <span class="badge__label">{label}</span>
</span>

<style>
  .badge {
    --badge-accent: var(--color-border-strong);
    --badge-dot-size: 0.375rem;

    display: inline-flex;
    align-items: center;
    min-width: 0;
    max-width: 100%;
    border: 1px solid
      color-mix(in srgb, var(--badge-accent) 65%, var(--color-border));
    border-radius: var(--radius-pill);
    background: color-mix(
      in srgb,
      var(--badge-accent) 6%,
      var(--color-surface)
    );
    color: var(--color-text);
    font-weight: 600;
    line-height: 1;
    vertical-align: middle;
    white-space: nowrap;
  }

  .badge--small {
    min-height: 1.5rem;
    gap: var(--space-1);
    padding-inline: var(--space-2);
    font-size: var(--font-size-xs);
  }

  .badge--medium {
    --badge-dot-size: 0.5rem;

    min-height: 2rem;
    gap: var(--space-2);
    padding-inline: var(--space-3);
    font-size: var(--font-size-sm);
  }

  .badge--neutral {
    --badge-accent: var(--color-border-strong);
  }

  .badge--primary {
    --badge-accent: var(--color-primary);
  }

  .badge--success {
    --badge-accent: var(--color-success);
  }

  .badge--warning {
    --badge-accent: var(--color-warning);
  }

  .badge--danger {
    --badge-accent: var(--color-danger);
  }

  .badge__dot {
    flex: 0 0 var(--badge-dot-size);
    width: var(--badge-dot-size);
    height: var(--badge-dot-size);
    border-radius: 50%;
    background: var(--badge-accent);
  }

  .badge__label {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
