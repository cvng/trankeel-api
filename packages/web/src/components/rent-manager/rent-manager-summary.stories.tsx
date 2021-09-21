import { ThemeProvider } from "evergreen-ui";
import React from "react";
import { AppTheme } from "../../theme/app-theme";
import { RentManagerSummary } from "./rent-manager-summary";

export default {
  title: "Rent/RentManagerSummary",
  component: RentManagerSummary,
};

export const loading = () => (
  <ThemeProvider value={AppTheme}>
    <RentManagerSummary loading />
  </ThemeProvider>
);

export const noData = () => (
  <ThemeProvider value={AppTheme}>
    <RentManagerSummary loading={false} />
  </ThemeProvider>
);
