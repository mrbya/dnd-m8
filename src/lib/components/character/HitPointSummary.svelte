<script lang="ts">
  import { Badge, Card, ProgressBar } from "$lib/components/ui";

  interface Props {
    currentHitPoints: number;
    maxHitPoints: number;
  }

  type HitPointTone = "success" | "warning" | "danger";

  interface HitPointPresentation {
    label: string;
    tone: HitPointTone;
  }

  let { currentHitPoints, maxHitPoints }: Props = $props();

  const normalizedMaximum = $derived(
    Number.isFinite(maxHitPoints) && maxHitPoints > 0 ? maxHitPoints : 1,
  );

  const normalizedCurrent = $derived(
    Number.isFinite(currentHitPoints)
      ? Math.min(Math.max(currentHitPoints, 0), normalizedMaximum)
      : 0,
  );

  const remaningRatio = $derived(normalizedCurrent / normalizedMaximum);

  const presentation = $derived.by<HitPointPresentation>(() => {
    if (remaningRatio <= 0.25) {
      return {
        label: "Critical",
        tone: "danger",
      };
    }

    if (remaningRatio <= 0.5) {
      return {
        label: "Low",
        tone: "warning",
      };
    }

    return {
      label: "High",
      tone: "success",
    };
  });
</script>

<Card as="section" aria-labelledby="hit-points-title" padding="medium">
  <div class="hit-points" data-tone={presentation.tone}>
    <div class="hit-points__header">
      <div class="hit-points__heading">
        <p id="hit-points-title">Hit points</p>

        <Badge dot label={presentation.label} tone={presentation.tone} />
      </div>

      <p class="hit-points__value" aria-live="polite">
        <span class="hit-points__current" aria-hidden="true">
          {normalizedCurrent}
        </span>

        <span class="hit-points__maximum" aria-hidden="true">
          / {normalizedMaximum}
        </span>
      </p>
    </div>

    <ProgressBar
      label="Current hit points"
      max={normalizedMaximum}
      tone={presentation.tone}
      value={normalizedCurrent}
    />
  </div>
</Card>

<style>
  .hit-points {
    --hit-points-color: var(--color-success-text);

    display: grid;
    gap: var(--space-3);
  }

  .hit-points[data-tone="warning"] {
    --hit-points-color: var(--color-warning-text);
  }

  .hit-points[data-tone="danger"] {
    --hit-points-color: var(--color-danger-text);
  }

  .hit-points__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-4);
  }

  .hit-points__heading {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-2);
    min-width: 0;
  }

  #hit-points-title {
    margin: 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    font-weight: 600;
    text-transform: uppercase;
  }

  .hit-points__value {
    display: flex;
    flex: 0 0 auto;
    align-items: baseline;
    margin: 0;
  }

  .hit-points__current {
    color: var(--hit-points-color);
    font-size: var(--font-size-2xl);
    font-weight: 700;
    line-height: 1;
  }

  .hit-points__maximum {
    margin-left: var(--space-1);
    color: var(--color-text-muted);
    font-size: var(--font-size-md);
    font-weight: 500;
  }

  @media (max-width: 24rem) {
    .hit-points__header {
      align-items: flex-start;
    }

    .hit-points__heading {
      display: grid;
      justify-items: start;
    }
  }
</style>
