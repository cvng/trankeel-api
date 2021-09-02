import { Button, EyeOpenIcon } from "evergreen-ui";
import React from "react";
import { Themable } from "../common/themable";
import { CardItem } from "./card-item";
import { CardSynthesisItem } from "./card-synthesis-item";

export default {
  title: "Cards/CardSynthesisItem",
  component: CardItem,
};

export const standard = () => {
  return (
    <Themable>
      <CardSynthesisItem
        title="Informations"
        items={[
          {
            title: "Locataire",
            badges: [{
              value: "Lenie Carpento",
              handler: () => alert("Lenie Carpento"),
              color: "blue",
            }, {
              value: "Jean-Marie Jourdine",
              handler: () => alert("Jean-Marie Jourdine"),
              color: "neutral",
            }],
          },
          {
            title: "Nombre de pièce(s)",
            text: "5",
            tooltip: "4 chambres",
          },
          {
            title: "Période de construction",
            text: "De 1975 à 1989",
          },
          {
            title: "Régime juridique de l'immeuble",
            text: "Mono-propriété",
          },
        ]}
        buttons={[
          <Button
            iconBefore={EyeOpenIcon}
            marginY={8}
            disabled
          >
            Visualiser le contrat
          </Button>,
          <Button
            iconBefore={EyeOpenIcon}
            marginY={8}
            disabled
          >
            Visualiser le contrat
          </Button>,
        ]}
      />
    </Themable>
  );
};

export const standardWithAvatars = () => {
  return (
    <Themable>
      <CardSynthesisItem
        title="Informations"
        items={[
          {
            title: "Locataire",
            avatars: [{
              name: "Lenie Carpento",
              handler: () => alert("Lenie Carpento"),
            }, {
              name: "Jean-Marie Jourdine",
              handler: () => alert("Jean-Marie Jourdine"),
            }],
          },
          {
            title: "Nombre de pièce(s)",
            text: "5",
            tooltip: "4 chambres",
          },
          {
            title: "Période de construction",
            text: "De 1975 à 1989",
          },
          {
            title: "Régime juridique de l'immeuble",
            text: "Mono-propriété",
          },
        ]}
        buttons={[
          <Button
            iconBefore={EyeOpenIcon}
            marginY={8}
            disabled
          >
            Visualiser le contrat
          </Button>,
          <Button
            iconBefore={EyeOpenIcon}
            marginY={8}
            disabled
          >
            Visualiser le contrat
          </Button>,
        ]}
      />
    </Themable>
  );
};
