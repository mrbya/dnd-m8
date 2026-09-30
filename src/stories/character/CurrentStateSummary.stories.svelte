<script module lang="ts">
  import { CurrentStateSummary } from "$lib/components/character";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect } from "storybook/test";

  const { Story } = defineMeta({
    title: "character/CurrentStateSummary",
    component: CurrentStateSummary,
    tags: ["autodocs"],
  });
</script>

<Story
  name="Empty"
  play={async ({ canvas }) => {
    await expect(
      canvas.getByText("No active conditions, effects or concentration."),
    ).toBeInTheDocument();
  }}
/>

<Story
  name="Concentrating"
  args={{
    concentration: "Bless",
  }}
  play={async ({ canvas }) => {
    await expect(canvas.getByText("Bless")).toBeInTheDocument();
    await expect(canvas.getByText("1 active")).toBeInTheDocument();
  }}
/>

<Story
  name="Active State"
  args={{
    concentration: "Spirit Guardians",
    conditions: [
      {
        id: "poisoned",
        label: "Poisoned",
        tone: "danger",
      },
      {
        id: "prone",
        label: "Prone",
        tone: "warning",
      },
    ],
    effects: [
      {
        id: "bless",
        label: "Blessed",
        tone: "success",
      },
      {
        id: "shield-of-faith",
        label: "Shield of Faith",
        tone: "primary",
      },
    ],
  }}
  play={async ({ canvas }) => {
    await expect(canvas.getByText("5 active")).toBeInTheDocument();
    await expect(canvas.getByText("Poisoned")).toBeInTheDocument();
    await expect(canvas.getByText("Blessed")).toBeInTheDocument();
  }}
/>

<Story
  name="Dense State"
  args={{
    concentration: "Spirit Guardians",
    conditions: [
      { id: "blinded", label: "Blinded", tone: "danger" },
      { id: "charmed", label: "Charmed", tone: "warning" },
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
        id: "temporary-defense",
        label: "Defensive bonus",
        tone: "primary",
      },
    ],
  }}
/>
