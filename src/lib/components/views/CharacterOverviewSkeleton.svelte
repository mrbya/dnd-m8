<script lang="ts">
  import { Card, Skeleton } from "$lib/components/ui";

  const summaryStats = [0, 1, 2];
</script>

<section class="character-overview-skeleton" aria-label="Loading character">
  <header class="character-header">
    <Skeleton height="2.5rem" shape="rectangle" width="min(16rem, 70%)" />

    <Skeleton height="1rem" width="10rem" />
  </header>

  <Card padding="medium">
    <div class="resource-skeleton">
      <div class="resource-header">
        <Skeleton height="0.875rem" width="6rem" />
        <Skeleton height="1.75rem" width="5rem" />
      </div>

      <Skeleton height="0.75rem" shape="rectangle" width="100%" />
    </div>
  </Card>

  <div class="stat-grid">
    {#each summaryStats as stat (stat)}
      <Card padding="small">
        <div class="stat-skeleton">
          <Skeleton height="0.75rem" width="70%" />
          <Skeleton height="1.375rem" width="45%" />
        </div>
      </Card>
    {/each}
  </div>
</section>

<style>
  .character-overview-skeleton {
    display: grid;
    gap: var(--space-5);
    width: min(100%, 48rem);
    margin-inline: auto;
  }

  .character-header {
    display: grid;
    justify-items: start;
    gap: var(--space-2);
    padding-block: var(--space-2);
  }

  .resource-skeleton,
  .stat-skeleton {
    display: grid;
    gap: var(--space-3);
  }

  .resource-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-4);
  }

  .stat-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: var(--space-3);
  }

  .stat-skeleton {
    gap: var(--space-2);
  }

  @media (max-width: 32rem) {
    .stat-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
