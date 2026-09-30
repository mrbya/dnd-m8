<script lang="ts">
  import {
    DesktopNavigation,
    MobileNavigation,
    type AppViewId,
  } from "$lib/components/navigation";

  let activeView = $state<AppViewId>("overview");

  function selectView(view: AppViewId): void {
    activeView = view;
  }
</script>

<div class="navigation-harness">
  <section class="desktop-preview" aria-label="Desktop navigation preview">
    <DesktopNavigation
      {activeView}
      navigationLabel="Desktop character ssection"
      onSelect={selectView}
    />
  </section>

  <section class="mobile-preview" aria-label="Mobile navigation preview">
    <MobileNavigation
      {activeView}
      navigationLabel="Mobile character section"
      onSelect={selectView}
    />
  </section>

  <output class="active-view">
    Current section: {activeView}
  </output>
</div>

<style>
  .navigation-harness {
    display: grid;
    justify-items: start;
    gap: var(--space-6);
  }

  .desktop-preview {
    width: 15rem;
    height: 24rem;
  }

  .mobile-preview {
    width: 20rem;
  }

  .active-view {
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
  }
</style>
