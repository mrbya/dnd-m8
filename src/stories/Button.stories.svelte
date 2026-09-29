<script module lang="ts">
  import { Button } from "$lib/components/ui";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import type { ComponentProps } from "svelte";

  type ButtonProps = ComponentProps<typeof Button>;
  type ButtonArgs = Omit<ButtonProps, "children">;

  const { Story } = defineMeta({
    title: "ui/Button",
    component: Button,
    tags: ["autodocs"],
    args: {
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

{#snippet template(args: ButtonArgs)}
  <Button {...args}>Continue</Button>
{/snippet}

<Story name="Primary" />

<Story name="Secondary" args={{ variant: "secondary" }} />

<Story name="Ghost" args={{ variant: "ghost" }} />

<Story name="Danger" args={{ variant: "danger" }} />

<Story name="Small" args={{ size: "small" }} />

<Story name="Large" args={{ size: "large" }} />

<Story name="Disabled" args={{ disabled: true }} />

<Story name="Loading" args={{ loading: true }} />
