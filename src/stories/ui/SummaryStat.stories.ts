import type { Meta, StoryObj } from "@storybook/sveltekit";

import { SummaryStat } from "$lib/components/ui";

const meta = {
  title: "ui/SummaryStat",
  component: SummaryStat,
  tags: ["autodocs"],
  args: {
    label: "Armor Class",
    value: 18,
  },
} satisfies Meta<typeof SummaryStat>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default = {} satisfies Story;

export const WithSuffix = {
  args: {
    label: "Speed",
    value: 30,
    suffix: "ft",
  },
} satisfies Story;

export const SignedValue = {
  args: {
    label: "Proficiency",
    value: "+3",
  },
} satisfies Story;

export const LongLabel = {
  args: {
    label: "Extremely long statistic name",
    value: 18,
  },
} satisfies Story;
