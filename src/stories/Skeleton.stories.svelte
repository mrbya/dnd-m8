<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import type { ComponentProps } from "svelte";

  import { Card, Skeleton } from "$lib/components/ui";

  type SkeletonProps = ComponentProps<typeof Skeleton>;

  const { Story } = defineMeta({
    title: "ui/Skeleton",
    component: Skeleton,
    tags: ["autodocs"],
    args: {
      animated: true,
      shape: "text",
      width: "100%",
    },
    argTypes: {
      shape: {
        control: "select",
        options: ["circle", "rectangle", "text"],
      },
    },
    render: template,
  });
</script>

{#snippet template(args: SkeletonProps)}
  <div class="story-frame">
    <Card padding="medium">
      <Skeleton {...args} />
    </Card>
  </div>
{/snippet}

<Story name="Text" />

<Story
  name="Short Text"
  args={{
    width: "60%",
  }}
/>

<Story
  name="Circle"
  args={{
    height: "3rem",
    shape: "circle",
    width: "3rem",
  }}
/>

<Story
  name="Rectangle"
  args={{
    height: "8rem",
    shape: "rectangle",
  }}
/>

<Story
  name="Static"
  args={{
    animated: false,
    width: "75%",
  }}
/>

<Story
  name="Custom Dimensions"
  args={{
    height: "1.5rem",
    shape: "rectangle",
    width: "12rem",
  }}
/>

<style>
  .story-frame {
    width: min(30rem, calc(100vw - 2rem));
  }
</style>
