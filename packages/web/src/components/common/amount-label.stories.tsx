import React from "react";
import { AmountLabel } from "./amount-label";

export default {
  title: "Design System/AmountLabel",
  component: AmountLabel,
};

export const positive = () => <AmountLabel value={100} />;

export const positiveWithSign = () => <AmountLabel value={100} addSign />;

export const negative = () => <AmountLabel value={-100} />;

export const negativeWithSign = () => <AmountLabel value={-100} addSign />;

export const zero = () => <AmountLabel value={0} />;
