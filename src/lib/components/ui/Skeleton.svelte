<script module lang="ts">
  export type SkeletonShape = "circle" | "rectangle" | "text";
</script>

<script lang="ts">
  import type { HTMLAttributes } from "svelte/elements";

  type Props = Omit<
    HTMLAttributes<HTMLSpanElement>,
    "aria-hidden" | "children" | "class" | "role" | "style"
  > & {
    animated?: boolean;
    class?: HTMLAttributes<HTMLSpanElement>["class"];
    height?: string;
    shape?: SkeletonShape;
    width: string;
  };

  let {
    animated = true,
    class: className,
    height,
    shape = "text",
    width,
    ...attributes
  }: Props = $props();

  const resolvedWidth = $derived(
    width ?? (shape === "circle" ? "2.rem" : "100%"),
  );

  const resolvedHeight = $derived(
    height ??
      (shape === "text" ? "1em" : shape === "circle" ? resolvedWidth : "4rem"),
  );
</script>

<span
  {...attributes}
  class={[
    "skeleton",
    `skeleton--${shape}`,
    animated && `skeleton--animated`,
    className,
  ]}
  aria-hidden="true"
  style:--skeleton-width={resolvedWidth}
  style:--skeleton-height={resolvedHeight}
></span>

<style>
  .skeleton {
    display: block;
    width: var(--skeleton-width);
    max-width: 100%;
    height: var(--skeleton-height);
    background: var(--color-border);
    pointer-events: none;
    user-select: none;
  }

  .skeleton--text {
    border-radius: var(--radius-sm);
  }

  .skeleton--rectangle {
    border-radius: var(--radius-md);
  }

  .skeleton--circle {
    border-radius: 50%;
  }

  .skeleton--animated {
    background: linear-gradient(
      90deg,
      var(--color-border) 25%,
      var(--color-border-strong) 50%,
      var(--color-border) 75%
    );
    background-size: 200% 100%;
    animation: skeleton-shimmer 1.5s ease-in-out infinite;
  }

  @keyframes skeleton-shimmer {
    from {
      background-position: 200% 0;
    }

    to {
      background-position: -200% 0;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .skeleton--animated {
      background: var(--color-border);
      animation: none;
    }
  }
</style>
