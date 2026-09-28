<script lang="ts">
  import type { LucideIcon } from "@lucide/svelte";
  import type { ComponentProps } from "svelte";

  import Button from "./Button.svelte";

  type Props = Omit<
    ComponentProps<typeof Button>,
    "aria-label" | "children" | "square"
  > & {
    icon: LucideIcon;
    label: string;
  };

  let { icon: Icon, label, size = "medium", ...buttonProps }: Props = $props();

  const iconSize = $derived(size === "small" ? 16 : size === "large" ? 24 : 20);
</script>

<Button {...buttonProps} {size} square aria-label={label}>
  <Icon size={iconSize} strokeWidth={2} aria-hidden="true" />
</Button>
