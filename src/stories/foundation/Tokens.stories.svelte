<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";

  type Token = {
    label: string;
    variable: `--${string}`;
  };

  type TokenGroup = {
    description: string;
    label: string;
    tokens: readonly Token[];
  };

  const colourGroups = [
    {
      label: "Surfaces",
      description: "Application layers and structural boundaries.",
      tokens: [
        { label: "Canvas", variable: "--color-canvas" },
        { label: "Shell", variable: "--color-shell" },
        { label: "Surface", variable: "--color-surface" },
        { label: "Surface hover", variable: "--color-surface-hover" },
        { label: "Border", variable: "--color-border" },
        { label: "Strong border", variable: "--color-border-strong" },
      ],
    },
    {
      label: "Text",
      description: "Semantic text colours for different levels of emphasis.",
      tokens: [
        { label: "Text", variable: "--color-text" },
        { label: "Muted text", variable: "--color-text-muted" },
        { label: "Disabled text", variable: "--color-text-disabled" },
        { label: "Text on accent", variable: "--color-text-on-accent" },
      ],
    },
    {
      label: "Interaction",
      description: "Primary actions, links and focus indicators.",
      tokens: [
        { label: "Primary", variable: "--color-primary" },
        { label: "Primary hover", variable: "--color-primary-hover" },
        { label: "Primary text", variable: "--color-primary-text" },
        {
          label: "Primary text hover",
          variable: "--color-primary-text-hover",
        },
        { label: "Information", variable: "--color-info" },
        { label: "Focus", variable: "--color-focus" },
      ],
    },
    {
      label: "Status",
      description: "Semantic feedback without ruleset-specific meaning.",
      tokens: [
        { label: "Success", variable: "--color-success" },
        { label: "Warning", variable: "--color-warning" },
        { label: "Danger", variable: "--color-danger" },
        { label: "Danger hover", variable: "--color-danger-hover" },
      ],
    },
  ] as const satisfies readonly TokenGroup[];

  const fontSizes = [
    { label: "Extra small", variable: "--font-size-xs" },
    { label: "Small", variable: "--font-size-sm" },
    { label: "Medium", variable: "--font-size-md" },
    { label: "Large", variable: "--font-size-lg" },
    { label: "Extra large", variable: "--font-size-xl" },
    { label: "Two extra large", variable: "--font-size-2xl" },
  ] as const satisfies readonly Token[];

  const fontWeights = [
    { label: "Regular", value: 400 },
    { label: "Medium", value: 500 },
    { label: "Semibold", value: 600 },
    { label: "Bold", value: 700 },
  ] as const;

  const spacingTokens = [
    { label: "Space 1", variable: "--space-1" },
    { label: "Space 2", variable: "--space-2" },
    { label: "Space 3", variable: "--space-3" },
    { label: "Space 4", variable: "--space-4" },
    { label: "Space 5", variable: "--space-5" },
    { label: "Space 6", variable: "--space-6" },
    { label: "Space 8", variable: "--space-8" },
  ] as const satisfies readonly Token[];

  const radiusTokens = [
    { label: "Small", variable: "--radius-sm" },
    { label: "Medium", variable: "--radius-md" },
    { label: "Large", variable: "--radius-lg" },
    { label: "Pill", variable: "--radius-pill" },
  ] as const satisfies readonly Token[];

  const layoutTokens = [
    { label: "Header height", variable: "--header-height" },
    { label: "Maximum content width", variable: "--content-max-width" },
    { label: "Touch target size", variable: "--touch-target-size" },
    { label: "Fast duration", variable: "--duration-fast" },
    { label: "Normal duration", variable: "--duration-normal" },
  ] as const satisfies readonly Token[];

  const { Story } = defineMeta({
    title: "Foundation/Tokens",
    tags: ["autodocs"],
    parameters: {
      layout: "fullscreen",
      controls: {
        disable: true,
      },
    },
  });
</script>

<Story name="Colours">
  {#snippet template()}
    <main class="foundation-page">
      <header class="foundation-header">
        <p class="eyebrow">Foundation</p>
        <h1>Semantic colours</h1>
        <p>
          Components consume semantic tokens rather than Catppuccin palette
          values directly. Use the Storybook theme selector to compare Mocha and
          Latte.
        </p>
      </header>

      {#each colourGroups as group (group.label)}
        <section class="token-section">
          <header class="section-header">
            <h2>{group.label}</h2>
            <p>{group.description}</p>
          </header>

          <ul class="token-grid" role="list">
            {#each group.tokens as token (token.variable)}
              <li class="token-card">
                <div
                  class="colour-swatch"
                  style={`--token-value: var(${token.variable});`}
                  aria-hidden="true"
                ></div>

                <div class="token-details">
                  <strong>{token.label}</strong>
                  <code>{token.variable}</code>
                </div>
              </li>
            {/each}
          </ul>
        </section>
      {/each}
    </main>
  {/snippet}
</Story>

<Story name="Typography">
  {#snippet template()}
    <main class="foundation-page">
      <header class="foundation-header">
        <p class="eyebrow">Foundation</p>
        <h1>Typography</h1>
        <p>
          The interface uses JetBrains Mono Nerd Font throughout, with a compact
          scale suitable for information-dense character views.
        </p>
      </header>

      <section class="token-section">
        <header class="section-header">
          <h2>Type scale</h2>
          <p>Each sample inherits the global family and line height.</p>
        </header>

        <ul class="type-list" role="list">
          {#each fontSizes as token (token.variable)}
            <li class="type-row">
              <div class="type-metadata">
                <strong>{token.label}</strong>
                <code>{token.variable}</code>
              </div>

              <p
                class="type-sample"
                style={`font-size: var(${token.variable});`}
              >
                Strength 18 · Modifier +4
              </p>
            </li>
          {/each}
        </ul>
      </section>

      <section class="token-section">
        <header class="section-header">
          <h2>Font weights</h2>
          <p>The locally bundled weights used by the component library.</p>
        </header>

        <ul class="weight-grid" role="list">
          {#each fontWeights as weight (weight.value)}
            <li class="weight-card">
              <span style={`font-weight: ${weight.value};`}> Aa 0123 +− </span>

              <div class="token-details">
                <strong>{weight.label}</strong>
                <code>{weight.value}</code>
              </div>
            </li>
          {/each}
        </ul>
      </section>
    </main>
  {/snippet}
</Story>

<Story name="Spacing and shape">
  {#snippet template()}
    <main class="foundation-page">
      <header class="foundation-header">
        <p class="eyebrow">Foundation</p>
        <h1>Spacing and shape</h1>
        <p>
          A small spacing scale keeps layouts predictable while radius tokens
          distinguish controls, cards and pill-shaped indicators.
        </p>
      </header>

      <section class="token-section">
        <header class="section-header">
          <h2>Spacing scale</h2>
          <p>The rendered bar represents the token’s actual size.</p>
        </header>

        <ul class="spacing-list" role="list">
          {#each spacingTokens as token (token.variable)}
            <li class="spacing-row">
              <div class="spacing-preview" aria-hidden="true">
                <span style={`width: var(${token.variable});`}></span>
              </div>

              <div class="token-details">
                <strong>{token.label}</strong>
                <code>{token.variable}</code>
              </div>
            </li>
          {/each}
        </ul>
      </section>

      <section class="token-section">
        <header class="section-header">
          <h2>Corner radii</h2>
          <p>Shape should communicate structure without becoming ornamental.</p>
        </header>

        <ul class="radius-grid" role="list">
          {#each radiusTokens as token (token.variable)}
            <li class="radius-card">
              <div
                class="radius-preview"
                style={`border-radius: var(${token.variable});`}
                aria-hidden="true"
              ></div>

              <div class="token-details">
                <strong>{token.label}</strong>
                <code>{token.variable}</code>
              </div>
            </li>
          {/each}
        </ul>
      </section>

      <section class="token-section">
        <header class="section-header">
          <h2>Layout and motion</h2>
          <p>Shared constraints consumed by shells and interactive controls.</p>
        </header>

        <ul class="reference-grid" role="list">
          {#each layoutTokens as token (token.variable)}
            <li class="reference-card">
              <strong>{token.label}</strong>
              <code>{token.variable}</code>
            </li>
          {/each}
        </ul>
      </section>
    </main>
  {/snippet}
</Story>

<Story name="Surfaces">
  {#snippet template()}
    <main class="foundation-page">
      <header class="foundation-header">
        <p class="eyebrow">Foundation</p>
        <h1>Surface hierarchy</h1>
        <p>
          The application uses colour, borders and spacing to establish depth
          without relying on prominent shadows.
        </p>
      </header>

      <section class="surface-demo" aria-label="Nested application surfaces">
        <div class="surface-label">
          <strong>Canvas</strong>
          <code>--color-canvas</code>
        </div>

        <div class="surface-layer surface-layer--shell">
          <div class="surface-label">
            <strong>Shell</strong>
            <code>--color-shell</code>
          </div>

          <div class="surface-layer surface-layer--surface">
            <div class="surface-label">
              <strong>Surface</strong>
              <code>--color-surface</code>
            </div>

            <p>
              Cards and important information regions sit above the application
              shell using a semantic surface and structural border.
            </p>

            <div class="accent-preview">
              Primary accent
              <code>--color-primary</code>
            </div>
          </div>
        </div>
      </section>
    </main>
  {/snippet}
</Story>

<style>
  .foundation-page {
    display: grid;
    gap: var(--space-8);
    width: min(100%, var(--content-max-width));
    margin-inline: auto;
    padding: clamp(var(--space-4), 4vw, var(--space-8));
    color: var(--color-text);
  }

  .foundation-header {
    display: grid;
    gap: var(--space-2);
    max-width: 54rem;
  }

  .foundation-header h1,
  .foundation-header p,
  .section-header h2,
  .section-header p,
  .type-sample,
  .surface-layer p {
    margin: 0;
  }

  .foundation-header h1 {
    font-size: var(--font-size-2xl);
    line-height: 1.2;
  }

  .foundation-header > p:last-child,
  .section-header p,
  .surface-layer p {
    color: var(--color-text-muted);
  }

  .eyebrow {
    color: var(--color-primary-text);
    font-size: var(--font-size-xs);
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  .token-section {
    display: grid;
    gap: var(--space-4);
  }

  .section-header {
    display: grid;
    gap: var(--space-1);
  }

  .section-header h2 {
    font-size: var(--font-size-lg);
  }

  .token-grid,
  .weight-grid,
  .radius-grid,
  .reference-grid,
  .type-list,
  .spacing-list {
    margin: 0;
    padding: 0;
    list-style: none;
  }

  .token-grid,
  .weight-grid,
  .radius-grid,
  .reference-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(14rem, 1fr));
    gap: var(--space-4);
  }

  .token-card,
  .weight-card,
  .radius-card,
  .reference-card,
  .type-row,
  .spacing-row {
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background: var(--color-surface);
  }

  .colour-swatch {
    height: 4rem;
    border-bottom: 1px solid var(--color-border);
    border-radius: var(--radius-md) var(--radius-md) 0 0;
    background: var(--token-value);
  }

  .token-details,
  .reference-card {
    display: grid;
    gap: var(--space-1);
  }

  .token-details {
    padding: var(--space-3);
  }

  code {
    color: var(--color-text-muted);
    font-size: var(--font-size-xs);
    overflow-wrap: anywhere;
  }

  .type-list,
  .spacing-list {
    display: grid;
    gap: var(--space-3);
  }

  .type-row {
    display: grid;
    grid-template-columns: minmax(10rem, 0.35fr) 1fr;
    gap: var(--space-4);
    align-items: center;
    padding: var(--space-4);
  }

  .type-metadata {
    display: grid;
    gap: var(--space-1);
  }

  .type-sample {
    min-width: 0;
    overflow-wrap: anywhere;
  }

  .weight-card {
    display: grid;
    gap: var(--space-4);
    padding: var(--space-4);
  }

  .weight-card > span {
    font-size: var(--font-size-xl);
  }

  .weight-card .token-details {
    padding: 0;
  }

  .spacing-row {
    display: grid;
    grid-template-columns: minmax(10rem, 0.35fr) 1fr;
    gap: var(--space-4);
    align-items: center;
    padding: var(--space-3);
  }

  .spacing-preview {
    display: flex;
    align-items: center;
    height: 1.5rem;
  }

  .spacing-preview span {
    display: block;
    min-width: 1px;
    height: 1rem;
    background: var(--color-primary);
  }

  .radius-card {
    display: grid;
    gap: var(--space-3);
    padding: var(--space-3);
  }

  .radius-preview {
    height: 5rem;
    border: 2px solid var(--color-primary);
    background: var(--color-shell);
  }

  .radius-card .token-details {
    padding: 0;
  }

  .reference-card {
    padding: var(--space-4);
  }

  .surface-demo {
    display: grid;
    gap: var(--space-4);
    padding: var(--space-5);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    background: var(--color-canvas);
  }

  .surface-layer {
    display: grid;
    gap: var(--space-4);
    padding: var(--space-5);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
  }

  .surface-layer--shell {
    background: var(--color-shell);
  }

  .surface-layer--surface {
    background: var(--color-surface);
  }

  .surface-label {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2) var(--space-4);
    align-items: baseline;
    justify-content: space-between;
  }

  .accent-preview {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2);
    align-items: center;
    justify-content: space-between;
    padding: var(--space-3) var(--space-4);
    border-radius: var(--radius-md);
    background: var(--color-primary);
    color: var(--color-text-on-accent);
    font-weight: 700;
  }

  .accent-preview code {
    color: inherit;
  }

  @media (max-width: 36rem) {
    .type-row,
    .spacing-row {
      grid-template-columns: 1fr;
    }

    .token-grid,
    .weight-grid,
    .radius-grid,
    .reference-grid {
      grid-template-columns: 1fr;
    }

    .surface-demo,
    .surface-layer {
      padding: var(--space-4);
    }
  }
</style>
