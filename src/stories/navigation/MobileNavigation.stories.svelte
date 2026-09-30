<script module lang="ts">
  import { MobileNavigation } from "$lib/components/navigation";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import type { ComponentProps } from "svelte";
  import { expect, fn } from "storybook/test";

  type MobileNavigationArgs = ComponentProps<typeof MobileNavigation>;

  const { Story } = defineMeta({
    title: "navigation/MobileNavigation",
    component: MobileNavigation,
    tags: ["autodocs"],
    args: {
      activeView: "overview",
      onSelect: fn(),
    },
    render: template,
  });
</script>

{#snippet template(args: MobileNavigationArgs)}
  <div class="story-frame">
    <MobileNavigation {...args} />
  </div>
{/snippet}

<Story
  name="Overview Active"
  play={async ({ args, canvas, userEvent }) => {
    const overview = canvas.getByRole("button", {
      name: "Overview",
    });
    const spells = canvas.getByRole("button", {
      name: "Spells",
    });

    await expect(overview).toHaveAttribute("aria-current", "page");
    await expect(spells).not.toHaveAttribute("aria-current");

    await userEvent.click(spells);

    await expect(args.onSelect).toHaveBeenCalledWith("spells");
  }}
/>

<Story
  name="Inventory Active"
  args={{
    activeView: "inventory",
  }}
  play={async ({ canvas }) => {
    const inventory = canvas.getByRole("button", {
      name: "Inventory",
    });

    await expect(inventory).toHaveAttribute("aria-current", "page");
  }}
/>

<style>
  .story-frame {
    display: flex;
    align-items: end;
    width: 20rem;
    min-height: 12rem;
  }
</style>
