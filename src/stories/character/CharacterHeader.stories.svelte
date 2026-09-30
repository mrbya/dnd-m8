<script module lang="ts">
  import { CharacterHeader } from "$lib/components/character";
  import type { CharacterSummaryDto } from "$lib/types";
  import { defineMeta } from "@storybook/addon-svelte-csf";

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
    title: "character/CharacterHeader",
    component: CharacterHeader,
    tags: ["autodocs"],
    args: {
      character,
    },
  });
</script>

<Story name="Default" />

<Story
  name="Long Identity"
  args={{
    character: {
      ...character,
      name: "Alexandria Cassiopeia Ravenscroft",
      className: "Cleric / Wizard / Eldritch Knight",
    },
  }}
/>
