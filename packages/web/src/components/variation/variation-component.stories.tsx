import React from "react";
import { VariationComponent } from "./variation-component";

export default {
  title: "Design System/VariationComponent",
  component: VariationComponent,
};

export const positive = () => <VariationComponent value={34.9} />;

export const negative = () => <VariationComponent value={-34.9} />;
