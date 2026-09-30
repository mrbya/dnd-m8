<script lang="ts">
  import type { CharacterSummaryDto } from "$lib/types";

  interface Props {
    character: CharacterSummaryDto;
  }

  let { character }: Props = $props();
</script>

<header class="character-header">
  <p class="character-header__eyebrow">Active character</p>

  <h1 id="character-name">{character.name}</h1>

  <p class="character-header__description">
    Level {character.level} · {character.className}
  </p>
</header>

<style>
  .character-header {
    min-width: 0;
    padding-block: var(--space-2);
  }

  .character-header__eyebrow {
    margin: 0;
    color: var(--color-primary-text);
    font-size: var(--font-size-xs);
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  h1 {
    margin: var(--space-2) 0 var(--space-1);
    color: var(--color-text);
    font-size: clamp(var(--font-size-2xl), 6vw, 2.5rem);
    line-height: 1.15;
    overflow-wrap: anywhere;
  }

  .character-header__description {
    margin: 0;
    color: var(--color-text-muted);
  }
</style>
