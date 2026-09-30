<script module lang="ts">
  import NavigationStateHarness from "./NavigationStateHarness.svelte";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, within } from "storybook/test";

  const { Story } = defineMeta({
    title: "navigation/SharedState",
    component: NavigationStateHarness,
    tags: ["autodocs"],
  });
</script>

<Story
  name="Selection Is Shared"
  play={async ({ canvas, userEvent }) => {
    const desktopRegion = canvas.getByRole("region", {
      name: "Desktop navigation preview",
    });
    const mobileRegion = canvas.getByRole("region", {
      name: "Mobile navigation preview",
    });

    const desktop = within(desktopRegion);
    const mobile = within(mobileRegion);

    const mobileSpells = mobile.getByRole("button", {
      name: "Spells",
    });

    await userEvent.click(mobileSpells);

    await expect(mobileSpells).toHaveAttribute("aria-current", "page");
    await expect(
      desktop.getByRole("button", {
        name: "Spells",
      }),
    ).toHaveAttribute("aria-current", "page");

    const desktopCombat = desktop.getByRole("button", {
      name: "Combat",
    });

    await userEvent.click(desktopCombat);

    await expect(desktopCombat).toHaveAttribute("aria-current", "page");
    await expect(
      mobile.getByRole("button", {
        name: "Combat",
      }),
    ).toHaveAttribute("aria-current", "page");

    await expect(
      canvas.getByText("Current section: combat"),
    ).toBeInTheDocument();
  }}
/>
