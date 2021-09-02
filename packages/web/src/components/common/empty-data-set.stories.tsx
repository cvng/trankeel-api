import { Button, PeopleIcon, PlusIcon } from "evergreen-ui";
import React from "react";
import animationData from "../../assets/lotties/people-morph-flow.json";
import { EmptyDataset } from "./empty-data-set";
import { LottieAnimation } from "./lottie-animation";

export default {
  title: "Design System/EmptyDataSet",
  component: EmptyDataset,
};

export const withIcon = () => (
  <EmptyDataset
    title={"Titre"}
    subtitle={"Sous-titre"}
    icon={PeopleIcon}
    removeBorder={false}
    button={<Button iconBefore={PlusIcon}>Ajouter un locataire</Button>}
  />
);

const animation = <LottieAnimation height={300} data={animationData} />;

export const withAnimation = () => (
  <EmptyDataset
    title={"Pas de locataire disponible "}
    subtitle={"Vous n'avez aucun locataire disponible pour créer votre nouvelle location !"}
    animation={animation}
    removeBorder={false}
    button={<Button iconBefore={PlusIcon}>Ajouter un locataire</Button>}
  />
);

withAnimation.parameters = {
  // disables Chromatic's snapshotting
  chromatic: { disableSnapshot: true },
};
