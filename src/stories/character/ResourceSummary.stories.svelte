<script module lang="ts">
  import { ResourceSummary } from "$lib/components/character";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import type { ComponentProps } from "svelte";
  import { expect } from "storybook/test";

  type ResourceSummaryArgs = ComponentProps<typeof ResourceSummary>;

  const { Story } = defineMeta({
    title: "character/ResourceSummary",
    component: ResourceSummary,
    tags: ["autodocs"],
    render: template,
  });
</script>

{#snippet template(args: ResourceSummaryArgs)}
  <div class="story-frame">
    <ResourceSummary {...args} />
  </div>
{/snippet}

<Story
  name="Empty"
  play={async ({ canvas }) => {
    await expect(
      canvas.getByText("No limited-use resources are currently tracked."),
    ).toBeInTheDocument();
  }}
/>

<Story
  name="Typical"
  args={{
    resources: [
      {
        id: "channel-divinity",
        label: "Channel Divinity",
        value: 1,
        max: 2,
        detail: "Short or long rest",
        tone: "primary",
      },
      {
        id: "bardic-inspiration",
        label: "Bardic Inspiration",
        value: 3,
        max: 4,
        detail: "Long rest",
        tone: "success",
      },
      {
        id: "hit-dice",
        label: "Hit Dice",
        value: 4,
        max: 5,
        detail: "Long rest",
        tone: "warning",
      },
    ],
  }}
  play={async ({ canvas }) => {
    const channelDivinity = canvas.getByRole("meter", {
      name: "Channel Divinity",
    });

    await expect(channelDivinity).toHaveAttribute("aria-valuenow", "1");
    await expect(channelDivinity).toHaveAttribute("aria-valuemax", "2");
    await expect(canvas.getByText("3 tracked")).toBeInTheDocument();
  }}
/>

<Story
  name="Depleted"
  args={{
    resources: [
      {
        id: "channel-divinity",
        label: "Channel Divinity",
        value: 0,
        max: 2,
        detail: "Short or long rest",
        tone: "danger",
        valueText: "No uses available",
      },
    ],
  }}
  play={async ({ canvas }) => {
    const resource = canvas.getByRole("meter", {
      name: "Channel Divinity",
    });

    await expect(resource).toHaveAttribute(
      "aria-valuetext",
      "No uses available",
    );
  }}
/>

<Story
  name="Dense"
  args={{
    resources: [
      {
        id: "channel-divinity",
        label: "Channel Divinity",
        value: 1,
        max: 2,
      },
      {
        id: "sorcery-points",
        label: "Sorcery Points",
        value: 5,
        max: 7,
      },
      {
        id: "first-level-slots",
        label: "First-level slots",
        value: 3,
        max: 4,
      },
      {
        id: "second-level-slots",
        label: "Second-level slots",
        value: 2,
        max: 3,
      },
      {
        id: "third-level-slots",
        label: "Third-level slots",
        value: 1,
        max: 3,
        tone: "warning",
      },
      {
        id: "hit-dice",
        label: "Hit Dice",
        value: 4,
        max: 7,
        tone: "success",
      },
    ],
  }}
/>

<style>
  .story-frame {
    width: min(48rem, 100%);
  }
</style>
