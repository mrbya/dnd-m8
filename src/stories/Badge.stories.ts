import type { Meta, StoryObj } from "@storybook/sveltekit";

import { Badge } from "$lib/components/ui";

const meta = {
  title: "ui/Badge",
  component: Badge,
  tags: ["autodocs"],
  args: {
    label: "Unarmored",
    size: "small",
    tone: "neutral",
  },
  argTypes: {
    size: {
      control: "select",
      options: ["small", "medium"],
    },
    tone: {
      control: "select",
      options: ["neutral", "primary", "success", "warning", "danger"],
    },
  },
} satisfies Meta<typeof Badge>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Neutral = {} satisfies Story;

export const Primary = {
  args: {
    label: "Concentrating",
    tone: "primary",
  },
} satisfies Story;

export const Success = {
  args: {
    label: "Blessed",
    tone: "success",
  },
} satisfies Story;

export const Warning = {
  args: {
    label: "Low resource",
    tone: "warning",
  },
} satisfies Story;

export const Danger = {
  args: {
    label: "Poisoned",
    tone: "danger",
  },
} satisfies Story;

export const WithDot = {
  args: {
    dot: true,
    label: "Concentrating",
    tone: "primary",
  },
} satisfies Story;

export const Medium = {
  args: {
    label: "Active effect",
    size: "medium",
    tone: "success",
  },
} satisfies Story;

export const LongLabel = {
  args: {
    label: "An exceptionally long active condition name",
    style: "max-width: 12rem",
    tone: "danger",
  },
} satisfies Story;
