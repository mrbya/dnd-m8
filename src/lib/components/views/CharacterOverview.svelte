<script lang="ts">
  import type { CharacterSummaryDto } from "$lib/types";
  import { SummaryStat, ProgressBar, Card } from "$lib/components/ui";

  interface Props {
    character: CharacterSummaryDto;
  }

  let { character }: Props = $props();
</script>

<section class="character-overview" aria-labelledby="overview-title">
  <h2 id="overview-title" class="visually-hidden">Overview</h2>
  <Card as="section" aria-labelledby="hit-points-title" padding="medium">
    <div class="hit-points">
      <div class="hit-points-header">
        <p id="hit-points-title">Hit points</p>

        <p class="hit-points-value">
          {character.currentHitPoints}
          <span>/ {character.maxHitPoints}</span>
        </p>
      </div>

      <ProgressBar
        label="Current hit points"
        max={character.maxHitPoints}
        tone="success"
        value={character.currentHitPoints}
      />
    </div>
  </Card>

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

  .hit-points {
    display: grid;
    gap: var(--space-3);
  }

  .hit-points-header {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: var(--space-4);
  }

  .hit-points-header > p {
    margin: 0;
  }

  #hit-points-title {
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    font-weight: 600;
    text-transform: uppercase;
  }

  .hit-points-value {
    color: var(--color-success);
    font-size: var(--font-size-2xl);
    font-weight: 700;
  }

  .hit-points-value span {
    color: var(--color-text-muted);
    font-size: var(--font-size-md);
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
