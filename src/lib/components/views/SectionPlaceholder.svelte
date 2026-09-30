<script lang="ts">
  import type { AppViewId } from "$lib/components/navigation";

  type PlaceholderViewId = Exclude<AppViewId, "overview">;

  interface Props {
    view: PlaceholderViewId;
  }

  const sectionContent = {
    combat: {
      eyebrow: "Session tools",
      title: "Combat",
      description:
        "Attacks, defenses, current effects and combat resources will live here.",
    },
    spells: {
      eyebrow: "Spellcasting",
      title: "Spells",
      description:
        "Prepared spells, spell slots and concentration tracking will live here.",
    },
    inventory: {
      eyebrow: "Equipment",
      title: "Inventory",
      description:
        "Carried items, equipped gear and item resources will live here.",
    },
    features: {
      eyebrow: "Character abilities",
      title: "Features",
      description:
        "Class, species, background and feat abilities will live here.",
    },
  } satisfies Record<
    PlaceholderViewId,
    {
      eyebrow: string;
      title: string;
      description: string;
    }
  >;

  let { view }: Props = $props();

  let content = $derived(sectionContent[view]);
</script>

<section
  class="section-placeholder"
  aria-labelledby="section-placeholder-title"
>
  <p class="section-placeholder__eyebrow">{content.eyebrow}</p>

  <h1 id="section-placeholder-title">{content.title}</h1>

  <p class="section-placeholder__description">
    {content.description}
  </p>
</section>

<style>
  .section-placeholder {
    width: min(100%, 42rem);
    margin-inline: auto;
    padding-block: clamp(var(--space-8), 10vh, 6rem);
    text-align: center;
  }

  .section-placeholder__eyebrow {
    margin: 0 0 var(--space-2);
    color: var(--color-primary-text);
    font-size: var(--font-size-xs);
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  h1 {
    margin: 0;
    color: var(--color-text);
    font-size: var(--font-size-2xl);
    line-height: 1.2;
  }

  .section-placeholder__description {
    margin: var(--space-3) auto 0;
    color: var(--color-text-muted);
    line-height: 1.6;
  }
</style>
