import React from "react";
import { ProgressBar } from "./progress-bar";

export default {
  title: "Design System/ProgressBar",
  component: ProgressBar,
};

export const noValue = () => <ProgressBar value={0} maxWidth={300} />;

export const withValue = () => <ProgressBar value={43} maxWidth={300} />;

export const withRightLabel = () => (
  <ProgressBar
    value={28}
    label="480,00 €"
    rightLabel="1690,00 €"
    maxWidth={300}
  />
);
