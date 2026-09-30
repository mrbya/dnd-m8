<script lang="ts">
  import { Card, Skeleton } from "$lib/components/ui";

  const summaryStats = [0, 1, 2];
  const resourceItems = [0, 1];
</script>

<section class="character-overview-skeleton" aria-label="Loading character">
  <header class="character-header">
    <Skeleton height="2.5rem" shape="rectangle" width="min(16rem, 70%)" />

    <Skeleton height="1rem" width="10rem" />
  </header>

  <Card padding="medium">
    <div class="section-skeleton">
      <div class="section-heading-skeleton">
        <Skeleton height="0.75rem" width="4rem" />
        <Skeleton height="1.125rem" width="9rem" />
      </div>

      <div class="badge-skeletons">
        <Skeleton height="2rem" width="7rem" />
        <Skeleton height="2rem" width="6rem" />
        <Skeleton height="2rem" width="8rem" />
      </div>
    </div>
  </Card>

  <Card padding="medium">
    <div class="section-skeleton">
      <div class="section-heading-skeleton">
        <Skeleton height="0.75rem" width="4rem" />
        <Skeleton height="1.125rem" width="7rem" />
      </div>

      <div class="resource-grid-skeleton">
        {#each resourceItems as resource (resource)}
          <div class="resource-item-skeleton">
            <div class="resource-item-header-skeleton">
              <Skeleton height="0.875rem" width="8rem" />
              <Skeleton height="1.125rem" width="3rem" />
            </div>

            <div class="pip-skeletons">
              <Skeleton height="0.875rem" shape="circle" width="0.875rem" />
              <Skeleton height="0.875rem" shape="circle" width="0.875rem" />
              <Skeleton height="0.875rem" shape="circle" width="0.875rem" />
            </div>
          </div>
        {/each}
      </div>
    </div>
  </Card>

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

  .section-skeleton {
    display: grid;
    gap: var(--space-4);
  }

  .section-heading-skeleton {
    display: grid;
    justify-items: start;
    gap: var(--space-1);
  }

  .badge-skeletons,
  .pip-skeletons {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2);
  }

  .resource-grid-skeleton {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(min(100%, 14rem), 1fr));
    gap: var(--space-3);
  }

  .resource-item-skeleton {
    display: grid;
    gap: var(--space-3);
    padding: var(--space-3);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background: var(--color-shell);
  }

  .resource-item-header-skeleton {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-3);
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
