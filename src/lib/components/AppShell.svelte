<script lang="ts">
</script>

<div class="app-shell">
  <header class="app-header">
    <span class="app-name">D&amp;D Mate</span>
    <span class="app-status">Walking skeleton</span>
  </header>

  <main class="app-content">
    <section class="placeholder" aria-labelledby="welcome-title">
      <p class="eyebrow">Character dashboard</p>
      <h1 id="welcome-title">D&amp;D Mate</h1>
      <p>The frontend foundation is ready. Character data will appear next.</p>
    </section>
  </main>
</div>

<style>
  .app-shell {
    display: grid;
    grid-template-rows: var(--header-height) minmax(0, 1fr);
    width: 100%;
    height: 100dvh;
    background: var(--color-canvas);
  }

  .app-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-4);
    padding-inline: var(--space-4);
    border-bottom: 1px solid var(--color-border);
    background: var(--color-shell);
  }

  .app-name {
    color: var(--color-primary);
    font-size: var(--font-size-md);
    font-weight: 700;
  }

  .app-status {
    color: var(--color-text-muted);
    font-size: var(--font-size-xs);
  }

  .app-content {
    min-height: 0;
    overflow: auto;
    padding: clamp(var(--space-4), 4vw, var(--space-8));
  }

  .placeholder {
    width: min(100%, 42rem);
    margin-inline: auto;
    padding: var(--space-6);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    background: var(--color-surface);
  }

  .placeholder h1 {
    margin-block: var(--space-2) var(--space-3);
    font-size: var(--font-size-2xl);
    line-height: 1.2;
  }

  .placeholder p {
    margin: 0;
    color: var(--color-text-muted);
  }

  .eyebrow {
    color: var(--color-primary) !important;
    font-size: var(--font-size-xs);
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }
</style>
