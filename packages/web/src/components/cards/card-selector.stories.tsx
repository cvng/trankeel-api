import React from "react";
import physicalPersonAnimation from "../../assets/lotties/28654-happy-freelancer.json";
import moralPersonAnimation from "../../assets/lotties/40815-building.json";
import { Themable } from "../common/themable";
import { CardSelector } from "./card-selector";

export default {
  title: "Cards/CardSelector",
  component: CardSelector,
};

export const personSelection = () => {
  return (
    <Themable>
      <CardSelector
        items={[
          {
            id: "0",
            title: "Personne physique",
            animation: physicalPersonAnimation,
            selected: true,
          },
          {
            id: "1",
            title: "Personne morale",
            animation: moralPersonAnimation,
          },
        ]}
      />
    </Themable>
  );
};
