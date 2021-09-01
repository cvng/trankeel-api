import { ThemeProvider } from "evergreen-ui";
import React from "react";
import { AppTheme } from "../../theme/app-theme";
import { PropertyEnergyClass } from "./property-energy-class";

export default {
  title: "Property/PropertyEnergyClass",
  component: PropertyEnergyClass,
};

export const standard = () => (
  <ThemeProvider value={AppTheme}>
    <PropertyEnergyClass />
  </ThemeProvider>
);
