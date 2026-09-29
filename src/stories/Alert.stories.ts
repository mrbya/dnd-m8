import type { Meta, StoryObj } from "@storybook/sveltekit";

import { Alert } from "$lib/components/ui";

const meta = {
  title: "ui/Alert",
  component: Alert,
  tags: ["autodocs"],
  args: {
    description:
      "Additional information that may help the player understand the current state.",
    title: "Character information",
    tone: "info",
  },
  argTypes: {
    actions: {
      table: { disable: true },
    },
    tone: {
      control: "select",
      options: ["info", "success", "warning", "danger"],
    },
  },
} satisfies Meta<typeof Alert>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Info = {} satisfies Story;

export const Success = {
  args: {
    description: "All character changes were saved successfully.",
    title: "Character saved",
    tone: "success",
  },
} satisfies Story;

export const Warning = {
  args: {
    description:
      "Casting another concentration spell will end the current effect.",
    title: "Already concentrating",
    tone: "warning",
  },
} satisfies Story;

export const Danger = {
  args: {
    description:
      "The character could not be loaded from the application backend.",
    title: "Unable to load character",
    tone: "danger",
  },
} satisfies Story;

export const TitleOnly = {
  args: {
    description: undefined,
    title: "No active effects",
  },
} satisfies Story;

export const LiveStatus = {
  args: {
    description: "The character has been synchronized.",
    role: "status",
    title: "Synchronization complete",
    tone: "success",
  },
} satisfies Story;

export const LiveError = {
  args: {
    description: "The requested character is unavailable.",
    role: "alert",
    title: "Character loading failed",
    tone: "danger",
  },
} satisfies Story;

export const LongDescription = {
  args: {
    description:
      "This is a deliberately longer message used to verify that alert descriptions wrap naturally at narrow viewport widths without overflowing their containing surface.",
    title: "Additional ruleset information",
  },
} satisfies Story;
