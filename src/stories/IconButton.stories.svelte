<script module lang="ts">
  import { Settings } from "@lucide/svelte";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect } from "storybook/test";

  import IconButton from "$lib/components/ui/IconButton.svelte";
  import type { ComponentProps } from "svelte";

  type IconButtonProps = ComponentProps<typeof IconButton>;
  type IconButtonArgs = Omit<IconButtonProps, "icon">;

  const { Story } = defineMeta({
    title: "ui/IconButton",
    component: IconButton,
    tags: ["autodocs"],
    args: {
      label: "Settings",
      size: "medium",
      type: "button",
      variant: "primary",
    },
    argTypes: {
      size: {
        control: "select",
        options: ["small", "medium", "large"],
      },
      variant: {
        control: "select",
        options: ["primary", "secondary", "ghost", "danger"],
      },
    },
    render: template,
  });
</script>

{#snippet template(args: IconButtonArgs)}
  <IconButton {...args} icon={Settings} />
{/snippet}

<Story
  name="Primary"
  play={async ({ canvas }) => {
    const button = canvas.getByRole("button", {
      name: "Settings",
    });

    await expect(button).toHaveAccessibleName("Settings");
  }}
/>

<Story name="Secondary" args={{ variant: "secondary" }} />

<Story name="Ghost" args={{ variant: "ghost" }} />

<Story name="Danger" args={{ variant: "danger" }} />

<Story name="Small" args={{ size: "small" }} />

<Story name="Large" args={{ size: "large" }} />

<Story name="Disabled" args={{ disabled: true }} />

<Story name="Loading" args={{ loading: true }} />
