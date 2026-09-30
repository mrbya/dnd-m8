import {
  Backpack,
  BookOpen,
  LayoutDashboard,
  Sparkles,
  Swords,
} from "@lucide/svelte";

export const appNavigationItems = [
  {
    id: "overview",
    label: "Overview",
    icon: LayoutDashboard,
  },
  {
    id: "combat",
    label: "Combat",
    icon: Swords,
  },
  {
    id: "spells",
    label: "Spells",
    icon: BookOpen,
  },
  {
    id: "inventory",
    label: "Inventory",
    icon: Backpack,
  },
  {
    id: "features",
    label: "Features",
    icon: Sparkles,
  },
] as const;

export type AppViewId = (typeof appNavigationItems)[number]["id"];
