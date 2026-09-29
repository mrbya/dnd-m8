<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect } from "storybook/test";
  import type { ComponentProps } from "svelte";

  import { ResourcePips } from "$lib/components/ui";

  type ResourcePipsProps = ComponentProps<typeof ResourcePips>;

  const { Story } = defineMeta({
    title: "ui/ResourcePips",
    component: ResourcePips,
    tags: ["autodocs"],
    args: {
      label: "Channel Divinity",
      max: 3,
      size: "medium",
      tone: "primary",
      value: 2,
    },
    argTypes: {
      size: {
        control: "select",
        options: ["small", "medium"],
      },
      tone: {
        control: "select",
        options: ["primary", "success", "warning", "danger"],
      },
    },
    render: template,
  });
</script>

{#snippet template(args: ResourcePipsProps)}
  <div class="story-frame">
    <p>{args.label}</p>
    <ResourcePips {...args} />
  </div>
{/snippet}

<Story name="Default" />

<Story
  name="Full"
  args={{
    label: "Bardic Inspiration",
    max: 4,
    value: 4,
  }}
/>

<Story
  name="Empty"
  args={{
    label: "Rage",
    max: 3,
    value: 0,
  }}
/>

<Story
  name="Success"
  args={{
    label: "Hit Dice",
    max: 6,
    tone: "success",
    value: 5,
  }}
/>

<Story
  name="Warning"
  args={{
    label: "First-level spell slots",
    max: 4,
    tone: "warning",
    value: 1,
  }}
/>

<Story
  name="Danger"
  args={{
    label: "Death save failures",
    max: 3,
    tone: "danger",
    value: 2,
    valueText: "Two of three death save failures",
  }}
  play={async ({ canvas }) => {
    const meter = canvas.getByRole("meter", {
      name: "Death save failures",
    });

    await expect(meter).toHaveAttribute(
      "aria-valuetext",
      "Two of three death save failures",
    );
  }}
/>

<Story
  name="Small"
  args={{
    label: "Sorcery Points",
    max: 8,
    size: "small",
    value: 5,
  }}
/>

<Story
  name="Large Pool"
  args={{
    label: "Ki Points",
    max: 20,
    value: 14,
  }}
/>

<Story
  name="Value Above Maximum"
  args={{
    label: "Clamped resource",
    max: 5,
    value: 8,
  }}
  play={async ({ canvas }) => {
    const meter = canvas.getByRole("meter", {
      name: "Clamped resource",
    });

    await expect(meter).toHaveAttribute("aria-valuemin", "0");
    await expect(meter).toHaveAttribute("aria-valuemax", "5");
    await expect(meter).toHaveAttribute("aria-valuenow", "5");
    await expect(meter).toHaveAttribute("aria-valuetext", "5 of 5 available");
  }}
/>

<Story
  name="Negative Value"
  args={{
    label: "Clamped empty resource",
    max: 5,
    value: -2,
  }}
  play={async ({ canvas }) => {
    const meter = canvas.getByRole("meter", {
      name: "Clamped empty resource",
    });

    await expect(meter).toHaveAttribute("aria-valuemin", "0");
    await expect(meter).toHaveAttribute("aria-valuemax", "5");
    await expect(meter).toHaveAttribute("aria-valuenow", "0");
    await expect(meter).toHaveAttribute("aria-valuetext", "0 of 5 available");
  }}
/>

<style>
  .story-frame {
    display: grid;
    gap: var(--space-3);
    width: min(24rem, calc(100vw - 2rem));
  }

  .story-frame p {
    margin: 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    font-weight: 600;
  }
</style>
