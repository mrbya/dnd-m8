<script module lang="ts">
  import { HitPointSummary } from "$lib/components/character";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import type { ComponentProps } from "svelte";
  import { expect } from "storybook/test";

  type HitPointSummaryArgs = ComponentProps<typeof HitPointSummary>;

  const { Story } = defineMeta({
    title: "character/HitPointSummary",
    component: HitPointSummary,
    tags: ["autodocs"],
    args: {
      currentHitPoints: 31,
      maxHitPoints: 38,
    },
    render: template,
  });
</script>

{#snippet template(args: HitPointSummaryArgs)}
  <div class="story-frame">
    <HitPointSummary {...args} />
  </div>
{/snippet}

<Story
  name="High"
  play={async ({ canvas }) => {
    await expect(canvas.getByText("High")).toBeInTheDocument();
  }}
/>

<Story
  name="Low"
  args={{
    currentHitPoints: 19,
  }}
  play={async ({ canvas }) => {
    await expect(canvas.getByText("Low")).toBeInTheDocument();
  }}
/>

<Story
  name="Critical"
  args={{
    currentHitPoints: 9,
  }}
  play={async ({ canvas }) => {
    await expect(canvas.getByText("Critical")).toBeInTheDocument();
  }}
/>

<Story
  name="Zero"
  args={{
    currentHitPoints: 0,
  }}
  play={async ({ canvas }) => {
    const progress = canvas.getByRole("progressbar", {
      name: "Current hit points",
    });

    await expect(canvas.getByText("Critical")).toBeInTheDocument();
    await expect(progress).toHaveAttribute("value", "0");
  }}
/>

<style>
  .story-frame {
    width: min(30rem, 100%);
  }
</style>
