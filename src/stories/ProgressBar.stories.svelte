<script module lang="ts">
  import type { ComponentProps } from "svelte";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect } from "storybook/test";

  import ProgressBar from "$lib/components/ui/ProgressBar.svelte";

  type ProgressBarProps = ComponentProps<typeof ProgressBar>;

  const { Story } = defineMeta({
    title: "ui/ProgressBar",
    component: ProgressBar,
    tags: ["autodocs"],
    args: {
      label: "Resource remaining",
      max: 10,
      tone: "primary",
      value: 7,
    },
    argTypes: {
      tone: {
        control: "select",
        options: ["primary", "success", "warning", "danger"],
      },
    },
    render: template,
  });
</script>

{#snippet template(args: ProgressBarProps)}
  <div class="story-frame">
    <ProgressBar {...args} />
  </div>
{/snippet}

<Story name="Primary" />

<Story
  name="Success"
  args={{
    label: "Current hit points",
    max: 45,
    tone: "success",
    value: 38,
  }}
/>

<Story
  name="Warning"
  args={{
    label: "Current hit points",
    max: 45,
    tone: "warning",
    value: 20,
  }}
/>

<Story
  name="Danger"
  args={{
    label: "Current hit points",
    max: 45,
    tone: "danger",
    value: 8,
  }}
/>

<Story name="Empty" args={{ value: 0 }} />

<Story name="Full" args={{ value: 10 }} />

<Story
  name="Value Above Maximum"
  args={{
    label: "Clamped resource",
    max: 10,
    value: 14,
  }}
  play={async ({ canvas }) => {
    const progress = canvas.getByRole("progressbar", {
      name: "Clamped resource",
    });

    await expect(progress).toHaveProperty("max", 10);
    await expect(progress).toHaveProperty("value", 10);
  }}
/>

<Story
  name="Negative Value"
  args={{
    label: "Empty resource",
    max: 10,
    value: -4,
  }}
  play={async ({ canvas }) => {
    const progress = canvas.getByRole("progressbar", {
      name: "Empty resource",
    });

    await expect(progress).toHaveProperty("max", 10);
    await expect(progress).toHaveProperty("value", 0);
  }}
/>

<style>
  .story-frame {
    width: min(24rem, calc(100vw - 2rem));
  }
</style>
