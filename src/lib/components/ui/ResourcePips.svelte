<script module lang="ts">
  export type ResourcePipsSize = "small" | "medium";
  export type ResourcePipsTone = "primary" | "success" | "warning" | "danger";
</script>

<script lang="ts">
  import type { HTMLAttributes } from "svelte/elements";

  type Props = Omit<
    HTMLAttributes<HTMLDivElement>,
    | "aria-label"
    | "aria-valuemax"
    | "aria-valuemin"
    | "aria-valuenow"
    | "aria-valuetext"
    | "class"
    | "role"
  > & {
    class?: HTMLAttributes<HTMLDivElement>["class"];
    label: string;
    max: number;
    size?: ResourcePipsSize;
    tone?: ResourcePipsTone;
    value: number;
    valueText?: string;
  };

  let {
    class: className,
    label,
    max,
    size = "medium",
    tone = "primary",
    value,
    valueText,
    ...attributes
  }: Props = $props();

  const normalizedMax = $derived(
    Number.isFinite(max) ? Math.max(1, Math.trunc(max)) : 1,
  );

  const normalizedValue = $derived(
    Number.isFinite(value)
      ? Math.min(Math.max(Math.trunc(value), 0), normalizedMax)
      : 0,
  );

  const pips = $derived(
    Array.from({ length: normalizedMax }, (_, index) => index),
  );

  const accessibleValueText = $derived(
    valueText ?? `${normalizedValue} of ${normalizedMax} available`,
  );
</script>

<div
  {...attributes}
  class={[
    "resource-pips",
    `resource-pips--${size}`,
    `resource-pips--${tone}`,
    className,
  ]}
  role="meter"
  aria-label={label}
  aria-valuemin={0}
  aria-valuemax={normalizedMax}
  aria-valuenow={normalizedValue}
  aria-valuetext={accessibleValueText}
>
  {#each pips as index (index)}
    <span
      class={[
        "resource-pip",
        index < normalizedValue && "resource-pip--filled",
      ]}
      aria-hidden="true"
    ></span>
  {/each}
</div>

<style>
  .resource-pips {
    --resource-pip-color: var(--color-primary);
    --resource-pip-size: 0.875rem;

    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2);
    width: fit-content;
    max-width: 100%;
  }

  .resource-pips--small {
    --resource-pip-size: 0.625rem;

    gap: var(--space-1);
  }

  .resource-pips--medium {
    --resource-pip-size: 0.875rem;
  }

  .resource-pips--primary {
    --resource-pip-color: var(--color-primary);
  }

  .resource-pips--success {
    --resource-pip-color: var(--color-success);
  }

  .resource-pips--warning {
    --resource-pip-color: var(--color-warning);
  }

  .resource-pips--danger {
    --resource-pip-color: var(--color-danger);
  }

  .resource-pip {
    box-sizing: border-box;
    flex: 0 0 var(--resource-pip-size);
    width: var(--resource-pip-size);
    height: var(--resource-pip-size);
    border: 2px solid var(--color-text-muted);
    border-radius: 50%;
    background: transparent;
  }

  .resource-pip--filled {
    border-color: var(--resource-pip-color);
    background: var(--resource-pip-color);
  }
</style>
