import React from "react";
import { SynthesisWidget } from "./synthesis-widget";

export default {
  title: "Dashboard/SynthesisWidget",
  component: SynthesisWidget,
};

export const noData = () => <SynthesisWidget />;

export const cashIn = () => <SynthesisWidget cashin={43450} />;

export const cashInVariationPositive = () => (
  <SynthesisWidget cashin={43450} cashinVariation={23} />
);

export const cashInVariationNegative = () => (
  <SynthesisWidget cashin={-43450} cashinVariation={-23} />
);

export const cashOut = () => <SynthesisWidget cashout={-21980} />;

export const cashOutNegativeVariation = () => (
  <SynthesisWidget cashout={-21980} cashoutVariation={-12} />
);

export const cashOutPositiveVariation = () => (
  <SynthesisWidget cashout={-21980} cashoutVariation={30} />
);

export const cashflow = () => <SynthesisWidget cashflow={1200} />;

export const cashflowPositiveVariation = () => (
  <SynthesisWidget cashflow={1200} cashflowVariation={12} />
);

export const cashflowNegativeVariation = () => (
  <SynthesisWidget cashflow={-1200} cashflowVariation={-12} />
);

export const rentNb = () => (
  <SynthesisWidget receivedRentNb={1} totalRentNb={5} />
);
