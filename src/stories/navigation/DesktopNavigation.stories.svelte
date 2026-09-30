<script module lang="ts">
  import { DesktopNavigation } from "$lib/components/navigation";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import type { ComponentProps } from "svelte";
  import { expect, fn } from "storybook/test";

  type DesktopNavigationArgs = ComponentProps<typeof DesktopNavigation>;

  const { Story } = defineMeta({
    title: "navigation/DesktopNavigation",
    component: DesktopNavigation,
    tags: ["autodocs"],
    args: {
      activeView: "overview",
      onSelect: fn(),
    },
    render: template,
  });
</script>

{#snippet template(args: DesktopNavigationArgs)}
  <div class="story-frame">
    <DesktopNavigation {...args} />
  </div>
{/snippet}

<Story
  name="Overview Active"
  play={async ({ args, canvas, userEvent }) => {
    const overview = canvas.getByRole("button", {
      name: "Overview",
    });
    const combat = canvas.getByRole("button", {
      name: "Combat",
    });

    await expect(overview).toHaveAttribute("aria-current", "page");
    await expect(combat).not.toHaveAttribute("aria-current");

    await userEvent.click(combat);

    await expect(args.onSelect).toHaveBeenCalledWith("combat");
  }}
/>

<Story
  name="Combat Active"
  args={{
    activeView: "combat",
  }}
  play={async ({ canvas }) => {
    const combat = canvas.getByRole("button", {
      name: "Combat",
    });

    await expect(combat).toHaveAttribute("aria-current", "page");
  }}
/>

<Story
  name="Keyboard Navigation"
  args={{
    onSelect: fn(),
  }}
  play={async ({ args, canvas, userEvent }) => {
    const overview = canvas.getByRole("button", {
      name: "Overview",
    });
    const combat = canvas.getByRole("button", {
      name: "Combat",
    });

    overview.focus();

    await expect(overview).toHaveFocus();

    await userEvent.keyboard("{Enter}");

    await expect(args.onSelect).toHaveBeenLastCalledWith("overview");

    await userEvent.tab();

    await expect(combat).toHaveFocus();

    await userEvent.keyboard("[Space]");

    await expect(args.onSelect).toHaveBeenLastCalledWith("combat");

    expect(overview.getBoundingClientRect().height).toBeGreaterThanOrEqual(44);
    expect(combat.getBoundingClientRect().height).toBeGreaterThanOrEqual(44);
  }}
/>

<style>
  .story-frame {
    width: 15rem;
    height: 32rem;
  }
</style>
