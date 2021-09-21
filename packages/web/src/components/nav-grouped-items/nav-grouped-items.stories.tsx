import { ThemeProvider } from "evergreen-ui";
import React from "react";
import { AppTheme } from "../../theme/app-theme";
import { NavGroupedItems } from "./nav-grouped-items";

export default {
  title: "Design System/NavGroupedItems",
  component: NavGroupedItems,
};

export const standard = () => (
  <ThemeProvider value={AppTheme}>
    <NavGroupedItems
      items={[
        {
          id: "0",
          title: "En attente",
          selected: true,
          onNavItemClick: () => {},
        },
        {
          id: "1",
          title: "Payé",
          selected: false,
          badge: 0,
          onNavItemClick: () => {},
        },
      ]}
    />
  </ThemeProvider>
);

export const withBadges = () => (
  <ThemeProvider value={AppTheme}>
    <NavGroupedItems
      items={[
        {
          id: "0",
          title: "En attente",
          selected: true,
          badge: 2,
          onNavItemClick: () => {},
        },
        {
          id: "1",
          title: "Payé",
          selected: false,
          badge: 0,
          onNavItemClick: () => {},
        },
      ]}
    />
  </ThemeProvider>
);
