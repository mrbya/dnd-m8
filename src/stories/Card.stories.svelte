<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import type { ComponentProps } from "svelte";

  import { Card } from "$lib/components/ui";

  type CardProps = ComponentProps<typeof Card>;
  type CardStoryArgs = Omit<CardProps, "children">;

  const { Story } = defineMeta({
    title: "ui/Card",
    component: Card,
    tags: ["autodocs"],
    args: {
      as: "div",
      padding: "medium",
    },
    argTypes: {
      as: {
        control: "select",
        options: ["article", "aside", "div", "section"],
      },
      padding: {
        control: "select",
        options: ["none", "small", "medium", "large"],
      },
    },
    render: template,
  });
</script>

{#snippet template(args: CardStoryArgs)}
  <div class="story-frame">
    <Card {...args}>
      <div class="story-content">
        <p class="story-eyebrow">Character resource</p>
        <h2>Hit points</h2>
        <p>Track the character’s current and maximum hit points during play.</p>
      </div>
    </Card>
  </div>
{/snippet}

<Story name="Default" />

<Story name="Small Padding" args={{ padding: "small" }} />

<Story name="Large Padding" args={{ padding: "large" }} />

<Story name="No Padding" args={{ padding: "none" }} />

<Story
  name="Semantic Section"
  args={{
    as: "section",
    "aria-label": "Character resource",
  }}
/>

<Story
  name="Semantic Article"
  args={{
    as: "article",
    "aria-label": "Hit point summary",
  }}
/>

<style>
  .story-frame {
    width: min(30rem, calc(100vw - 2rem));
  }

  .story-content {
    display: grid;
    gap: var(--space-2);
  }

  .story-content h2,
  .story-content p {
    margin: 0;
  }

  .story-content > p:last-child {
    color: var(--color-text-muted);
  }

  .story-eyebrow {
    color: var(--color-primary);
    font-size: var(--font-size-xs);
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }
</style>
