<script lang="ts">
  import type { CharacterSummaryDto } from "$lib/types";
  import { SummaryStat } from "$lib/components/ui";

  interface Props {
    character: CharacterSummaryDto;
  }

  let { character }: Props = $props();

  let boundedHitPoints = $derived(
    Math.max(0, Math.min(character.currentHitPoints, character.maxHitPoints)),
  );
</script>

<article class="character-overview" aria-labelledby="character-name">
  <header class="character-header">
    <div>
      <p class="eyebrow">Active character</p>
      <h1 id="character-name">{character.name}</h1>
      <p class="character-description">
        Level {character.level} · {character.className}
      </p>
    </div>
  </header>

  <section class="hit-points" aria-labelledby="hit-points-title">
    <div class="hit-points-header">
      <p id="hit-points-title">Hit points</p>

      <p class="hit-points-value">
        {character.currentHitPoints}
        <span>/ {character.maxHitPoints}</span>
      </p>

      <progress
        aria-label="Current hit points"
        max={Math.max(character.maxHitPoints, 1)}
        value={boundedHitPoints}
      >
        {boundedHitPoints} of {character.maxHitPoints}
      </progress>
    </div>
  </section>

  <dl class="stat-grid" aria-label="character statistics">
    <SummaryStat label="Armor Class" value={character.armorClass} />
    <SummaryStat label="Proficiency" value={`+${character.proficiencyBonus}`} />
    <SummaryStat label="Speed" value={character.speed} suffix="ft" />
  </dl>
</article>

<style>
  .character-overview {
    display: grid;
    gap: var(--space-5);
    width: min(100%, 48rem);
    margin-inline: auto;
  }

  .character-header {
    padding-block: var(--space-2);
  }

  .eyebrow {
    margin: 0;
    color: var(--color-primary);
    font-size: var(--font-size-xs);
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  h1 {
    margin: var(--space-2) 0 var(--space-1);
    font-size: clamp(var(--font-size-2xl), 6vw, 2.5rem);
    line-height: 1.15;
  }

  .character-description {
    margin: 0;
    color: var(--color-text-muted);
  }

  .hit-points {
    display: grid;
    gap: var(--space-3);
    padding: var(--space-5);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    background: var(--color-surface);
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

  progress {
    width: 100%;
    height: 0.75rem;
    overflow: hidden;
    border: 0;
    border-radius: var(--radius-sm);
    background: var(--color-shell);
    accent-color: var(--color-success);
  }

  progress::-webkit-progress-bar {
    background: var(--color-shell);
  }

  progress::-webkit-progress-value {
    border-radius: var(--radius-sm);
    background: var(--color-success);
  }

  progress::-moz-progress-bar {
    border-radius: var(--radius-sm);
    background: var(--color-success);
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
