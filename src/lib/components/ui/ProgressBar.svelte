<script module lang="ts">
  export type ProgressBarTone = "primary" | "success" | "warning" | "danger";
</script>

<script lang="ts">
  import type { HTMLProgressAttributes } from "svelte/elements";

  type Props = Omit<
    HTMLProgressAttributes,
    "aria-label" | "class" | "max" | "value"
  > & {
    class?: HTMLProgressAttributes["class"];
    label: string;
    max: number;
    tone?: ProgressBarTone;
    value: number;
  };

  let {
    class: className,
    label,
    max,
    tone = "primary",
    value,
    ...attributes
  }: Props = $props();

  const normalizedMax = $derived(Number.isFinite(max) && max > 0 ? max : 1);
  const normalizedValue = $derived(
    Number.isFinite(value) ? Math.min(Math.max(value, 0), normalizedMax) : 0,
  );
</script>

<progress
  {...attributes}
  class={["progress-bar", `progress-bar--${tone}`, className]}
  aria-label={label}
  max={normalizedMax}
  value={normalizedValue}
>
  {normalizedValue} of {normalizedMax}
</progress>

<style>
  .progress-bar {
    display: block;
    width: 100%;
    height: 0.75rem;
    overflow: hidden;
    appearance: none;
    border: 0;
    border-radius: var(--radius-sm);
    background: var(--color-shell);
    accent-color: var(--progress-bar-color);
  }

  .progress-bar--primary {
    --progress-bar-color: var(--color-primary);
  }

  .progress-bar--success {
    --progress-bar-color: var(--color-success);
  }

  .progress-bar--warning {
    --progress-bar-color: var(--color-warning);
  }

  .progress-bar--danger {
    --progress-bar-color: var(--color-danger);
  }

  .progress-bar::-webkit-progress-bar {
    background: var(--color-shell);
  }

  .progress-bar::-webkit-progress-value {
    border-radius: var(--radius-sm);
    background: var(--progress-bar-color);
  }

  .progress-bar::-moz-progress-bar {
    border-radius: var(--radius-sm);
    background: var(--progress-bar-color);
  }
</style>
