<script module lang="ts">
  export type AlertTone = "info" | "success" | "warning" | "danger";
</script>

<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";

  type Props = Omit<
    HTMLAttributes<HTMLElement>,
    "children" | "class" | "name"
  > & {
    actions?: Snippet;
    class?: HTMLAttributes<HTMLElement>["class"];
    description?: string;
    title: string;
    tone?: AlertTone;
  };

  let {
    actions,
    class: className,
    description,
    title,
    tone = "info",
    ...attributes
  }: Props = $props();
</script>

<div {...attributes} class={["alert", `alert--${tone}`, className]}>
  <div class="allert__content">
    <p class="alert__title">{title}</p>

    {#if description}
      <p class="alert__description">{description}</p>
    {/if}

    {#if actions}
      <div class="alert__actions">
        {@render actions()}
      </div>
    {/if}
  </div>
</div>

<style>
  .alert {
    --alert-accent: var(--color-info);

    min-width: 0;
    padding: var(--space-4);
    border: 1px solid
      color-mix(in srgb, var(--alert-accent) 65%, var(--color-border));
    border-left-width: 0.25rem;
    border-radius: var(--radius-md);
    background: color-mix(
      in srgb,
      var(--alert-accent) 6%,
      var(--color-surface)
    );
    color: var(--color-text);
  }

  .alert--info {
    --alert-accent: var(--color-info);
  }

  .alert--success {
    --alert-accent: var(--color-success);
  }

  .alert--warning {
    --alert-accent: var(--color-warning);
  }

  .alert--danger {
    --alert-accent: var(--color-danger);
  }

  .alert__content {
    display: grid;
    gap: var(--space-2);
  }

  .alert__title,
  .alert__description {
    margin: 0;
  }

  .alert__title {
    font-size: var(--font-size-sm);
    font-weight: 700;
    line-height: 1.4;
  }

  .alert__description {
    font-size: var(--font-size-sm);
    line-height: 1.5;
    white-space: pre-wrap;
  }

  .alert__actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2);
    margin-top: var(--space-1);
  }
</style>
