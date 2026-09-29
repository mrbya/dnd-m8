<script module lang="ts">
  export type ButtonSize = "small" | "medium" | "large";
  export type ButtonVariant = "primary" | "secondary" | "ghost" | "danger";
</script>

<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  type Props = Omit<
    HTMLButtonAttributes,
    "children" | "class" | "disabled" | "type"
  > & {
    children: Snippet;
    class?: HTMLButtonAttributes["class"];
    disabled?: boolean;
    loading?: boolean;
    square?: boolean;
    size?: ButtonSize;
    type?: HTMLButtonAttributes["type"];
    variant?: ButtonVariant;
  };

  let {
    children,
    class: className,
    disabled = false,
    loading = false,
    square = false,
    size = "medium",
    type = "button",
    variant = "primary",
    ...attributes
  }: Props = $props();
</script>

<button
  {...attributes}
  class={[
    "button",
    `button--${variant}`,
    `button--${size}`,
    loading && "button--loading",
    square && "button--square",
    className,
  ]}
  {type}
  disabled={disabled || loading}
  aria-busy={loading || undefined}
>
  <span class="button__content">
    {@render children()}
  </span>
</button>

<style>
  .button {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 0;
    border: 1px solid transparent;
    border-radius: var(--radius-md);
    font-weight: 700;
    line-height: 1;
    text-decoration: none;
    white-space: nowrap;
    user-select: none;
    transition:
      background-color var(--duration-fast),
      border-color var(--duration-fast),
      color var(--duration-fast),
      opacity var(--duration-fast),
      transform var(--duration-fast);
  }

  .button--small {
    --button-size: 2.25rem;

    min-height: var(--button-size);
    padding-inline: var(--space-3);
    font-size: var(--font-size-xs);
  }

  .button--medium {
    --button-size: var(--touch-target-size);

    min-height: var(--button-size);
    padding-inline: var(--space-4);
    font-size: var(--font-size-sm);
  }

  .button--large {
    --button-size: 3rem;

    min-height: var(--button-size);
    padding-inline: var(--space-5);
    font-size: var(--font-size-md);
  }

  .button--square {
    flex: 0 0 var(--button-size);
    width: var(--button-size);
    padding-inline: 0;
  }

  .button--primary {
    background: var(--color-primary);
    color: var(--color-text-on-accent);
  }

  .button--primary:not(:disabled):hover {
    background: var(--color-primary-hover);
  }

  .button--secondary {
    border-color: var(--color-border-strong);
    background: var(--color-surface);
    color: var(--color-text);
  }

  .button--secondary:not(:disabled):hover {
    background: var(--color-surface-hover);
  }

  .button--ghost {
    background: transparent;
    color: var(--color-primary-text);
  }

  .button--ghost:not(:disabled):hover {
    background: var(--color-surface);
  }

  .button--danger {
    background: var(--color-danger);
    color: var(--color-text-on-accent);
  }

  .button--danger:not(:disabled):hover {
    background: var(--color-danger-hover);
  }

  .button:not(:disabled):active {
    transform: translateY(1px);
  }

  .button:disabled {
    cursor: not-allowed;
    opacity: 0.55;
  }

  .button--loading:disabled {
    opacity: 1;
  }

  .button__content {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-2);
  }

  .button--loading .button__content {
    opacity: 0;
  }

  .button--loading::after {
    position: absolute;
    width: 1em;
    height: 1em;
    border: 2px solid currentcolor;
    border-right-color: transparent;
    border-radius: 50%;
    animation: button-spin 700ms linear infinite;
    content: "";
  }

  @keyframes button-spin {
    to {
      transform: rotate(360deg);
    }
  }
</style>
