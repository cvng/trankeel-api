import { ThemeProvider } from "evergreen-ui";
import React from "react";
import { AppTheme } from "../../theme/app-theme";
import { PropertyGasEmission } from "./property-gas-emission";

export default {
  title: "Property/PropertyGasEmission",
  component: PropertyGasEmission,
};

export const standard = () => (
  <ThemeProvider value={AppTheme}>
    <PropertyGasEmission />
  </ThemeProvider>
);
