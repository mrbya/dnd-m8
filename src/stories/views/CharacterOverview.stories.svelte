<script module lang="ts">
  import { CharacterOverview } from "$lib/components/views";
  import type { CharacterSummaryDto } from "$lib/types";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect } from "storybook/test";

  const character = {
    id: "sample-character",
    name: "Arannis Moonwhisper",
    level: 5,
    className: "Cleric",
    currentHitPoints: 31,
    maxHitPoints: 38,
    armorClass: 18,
    proficiencyBonus: 3,
    speed: 30,
  } satisfies CharacterSummaryDto;

  const { Story } = defineMeta({
    title: "views/CharacterOverview",
    component: CharacterOverview,
    tags: ["autodocs"],
    args: {
      character,
    },
  });
</script>

<Story
  name="Empty Session State"
  play={async ({ canvas }) => {
    await expect(
      canvas.getByText("No active conditions, effects or concentration."),
    ).toBeInTheDocument();

    await expect(
      canvas.getByText("No limited-use resources are currently tracked."),
    ).toBeInTheDocument();
  }}
/>

<Story
  name="Active Session"
  args={{
    concentration: "Spirit Guardians",
    conditions: [
      {
        id: "poisoned",
        label: "Poisoned",
        tone: "danger",
      },
    ],
    effects: [
      {
        id: "bless",
        label: "Blessed",
        tone: "success",
      },
    ],
    resources: [
      {
        id: "channel-divinity",
        label: "Channel Divinity",
        value: 1,
        max: 2,
        detail: "Short or long rest",
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
        value: 1,
        max: 3,
        tone: "warning",
      },
    ],
  }}
  play={async ({ canvas }) => {
    await expect(canvas.getByText("Spirit Guardians")).toBeInTheDocument();
    await expect(canvas.getByText("Poisoned")).toBeInTheDocument();

    await expect(
      canvas.getByRole("meter", {
        name: "Channel Divinity",
      }),
    ).toHaveAttribute("aria-valuenow", "1");
  }}
/>

<Story
  name="Critical And Dense"
  args={{
    character: {
      ...character,
      currentHitPoints: 7,
    },
    concentration: "Spirit Guardians",
    conditions: [
      { id: "blinded", label: "Blinded", tone: "danger" },
      { id: "frightened", label: "Frightened", tone: "danger" },
      { id: "grappled", label: "Grappled", tone: "warning" },
      { id: "poisoned", label: "Poisoned", tone: "danger" },
      { id: "prone", label: "Prone", tone: "warning" },
      { id: "restrained", label: "Restrained", tone: "danger" },
    ],
    effects: [
      { id: "bless", label: "Blessed", tone: "success" },
      { id: "heroism", label: "Heroism", tone: "success" },
      {
        id: "shield-of-faith",
        label: "Shield of Faith",
        tone: "primary",
      },
    ],
    resources: [
      {
        id: "channel-divinity",
        label: "Channel Divinity",
        value: 0,
        max: 2,
        tone: "danger",
      },
      {
        id: "first-level-slots",
        label: "First-level slots",
        value: 1,
        max: 4,
        tone: "warning",
      },
      {
        id: "second-level-slots",
        label: "Second-level slots",
        value: 0,
        max: 3,
        tone: "danger",
      },
      {
        id: "third-level-slots",
        label: "Third-level slots",
        value: 1,
        max: 2,
      },
      {
        id: "hit-dice",
        label: "Hit Dice",
        value: 2,
        max: 5,
      },
      {
        id: "special-feature",
        label: "Special feature with a long resource name",
        value: 1,
        max: 3,
      },
    ],
  }}
  play={async ({ canvas }) => {
    await expect(canvas.getByText("Critical")).toBeInTheDocument();
    await expect(canvas.getByText("10 active")).toBeInTheDocument();
    await expect(canvas.getByText("6 tracked")).toBeInTheDocument();
  }}
/>
