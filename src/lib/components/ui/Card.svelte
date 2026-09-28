<script module lang="ts">
  export type CardElement = "article" | "aside" | "div" | "section";
  export type CardPadding = "none" | "small" | "medium" | "large";
</script>

<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";

  type Props = Omit<HTMLAttributes<HTMLElement>, "children" | "class"> & {
    as?: CardElement;
    children: Snippet;
    class?: HTMLAttributes<HTMLElement>["class"];
    padding?: CardPadding;
  };

  let {
    as = "div",
    children,
    class: className,
    padding = "medium",
    ...attributes
  }: Props = $props();
</script>

<svelte:element
  this={as}
  {...attributes}
  class={["card", `card--${padding}`, className]}
>
  {@render children()}
</svelte:element>

<style>
  .card {
    min-width: 0;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    background: var(--color-surface);
  }

  .card--none {
    padding: 0;
  }

  .card--small {
    padding: var(--space-3);
  }

  .card--medium {
    padding: var(--space-5);
  }

  .card--large {
    padding: var(--space-6);
  }
</style>
