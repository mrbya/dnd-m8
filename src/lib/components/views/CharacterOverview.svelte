<script lang="ts">
  import type { CharacterSummaryDto } from "$lib/types";
  import { SummaryStat } from "$lib/components/ui";
  import {
    HitPointSummary,
    CurrentStateSummary,
  } from "$lib/components/character";

  interface Props {
    character: CharacterSummaryDto;
  }

  let { character }: Props = $props();
</script>

<section class="character-overview" aria-labelledby="overview-title">
  <h2 id="overview-title" class="visually-hidden">Overview</h2>
  <HitPointSummary
    currentHitPoints={character.currentHitPoints}
    maxHitPoints={character.maxHitPoints}
  />

  <CurrentStateSummary />

  <div class="stat-grid">
    <SummaryStat label="Armor Class" value={character.armorClass} />
    <SummaryStat label="Proficiency" value={`+${character.proficiencyBonus}`} />
    <SummaryStat label="Speed" value={character.speed} suffix="ft" />
  </div>
</section>

<style>
  .character-overview {
    display: grid;
    gap: var(--space-5);
  }

  .stat-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: var(--space-3);
    margin: 0;
  }

  @media (max-width: 32rem) {
    .stat-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
