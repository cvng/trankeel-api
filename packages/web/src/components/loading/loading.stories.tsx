import React from "react";
import { Loading } from "./loading";

export default {
  title: "Design System/Loading",
  component: Loading,
  parameters: {
    chromatic: { disableSnapshot: true },
  },
};

export const standard = () => <Loading />;

export const withTitle = () => <Loading title="Veuillez patienter..." />;

export const withTitleAndMessage = () => (
  <Loading
    title="Veuillez patienter..."
    message="Votre paiement est en cours de validation"
  />
);
