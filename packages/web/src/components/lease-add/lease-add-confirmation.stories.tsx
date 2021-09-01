import { ThemeProvider } from "evergreen-ui";
import React from "react";
import { AppTheme } from "../../theme/app-theme";
import { LeaseAddConfirmation } from "./lease-add-confirmation";
import { LeaseAddProvider } from "./lease-add-context";

export default {
  title: "Lease/LeaseAddConfirmation",
  component: LeaseAddConfirmation,
};

export const loading = () => (
  <ThemeProvider value={AppTheme}>
    <LeaseAddProvider>
      <LeaseAddConfirmation loading={true} error={true} />
    </LeaseAddProvider>
  </ThemeProvider>
);

export const success = () => (
  <ThemeProvider value={AppTheme}>
    <LeaseAddProvider>
      <LeaseAddConfirmation loading={false} error={false} />
    </LeaseAddProvider>
  </ThemeProvider>
);

export const error = () => (
  <ThemeProvider value={AppTheme}>
    <LeaseAddProvider>
      <LeaseAddConfirmation loading={false} error={true} />
    </LeaseAddProvider>
  </ThemeProvider>
);
