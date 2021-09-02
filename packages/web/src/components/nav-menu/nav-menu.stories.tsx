import { Pane, ThemeProvider } from "evergreen-ui";
import React from "react";
import { AppTheme } from "../../theme/app-theme";
import { NavMenu } from "./nav-menu";
import animationData from "../../assets/lotties/51084-contract-signing.json";

export default {
  title: "Menu/NavMenu",
  component: NavMenu,
};

export const standard = () => (
  <ThemeProvider value={AppTheme}>
    <NavMenu
      title="Nouveau bail de location"
      subtitle="Prenons quelques minutes ensemble pour créer votre nouveau bail de
              location !"
      animation={null}
      items={[
        {
          title: "Parties",
          subtitle: "Sélectionner le bien, le locataire",
          checked: true,
          selected: false,
          disabled: false,
          component: <Pane />,
        },
        {
          title: "Détails du bien",
          subtitle: "Vérifier les données de votre bien",
          checked: false,
          selected: true,
          disabled: false,
          component: <Pane />,
        },
        {
          title: "Conditions du contrat",
          subtitle: "Sélectionner la date de prise d'effet du contrat",
          checked: false,
          selected: false,
          disabled: true,
          component: <Pane />,
        },
        {
          title: "Conditions financières",
          subtitle: "Valoriser le loyer et le détails concernant les charges",
          checked: false,
          selected: false,
          disabled: true,
          component: <Pane />,
        },
        {
          title: "Prévisualisation du contrat",
          subtitle:
            "Visualiser votre contrat avant de l'envoyer pour signature",
          checked: false,
          selected: false,
          disabled: true,
          component: <Pane />,
        },
      ]}
    />
  </ThemeProvider>
);

export const withAnimation = () => (
  <ThemeProvider value={AppTheme}>
    <NavMenu
      title="Nouveau bail de location"
      subtitle="Prenons quelques minutes ensemble pour créer votre nouveau bail de
              location !"
      animation={animationData}
      items={[
        {
          title: "Parties",
          subtitle: "Sélectionner le bien, le locataire",
          checked: true,
          selected: false,
          disabled: false,
          component: <Pane />,
        },
        {
          title: "Détails du bien",
          subtitle: "Vérifier les données de votre bien",
          checked: false,
          selected: true,
          disabled: false,
          component: <Pane />,
        },
        {
          title: "Conditions du contrat",
          subtitle: "Sélectionner la date de prise d'effet du contrat",
          checked: false,
          selected: false,
          disabled: true,
          component: <Pane />,
        },
        {
          title: "Conditions financières",
          subtitle: "Valoriser le loyer et le détails concernant les charges",
          checked: false,
          selected: false,
          disabled: true,
          component: <Pane />,
        },
        {
          title: "Prévisualisation du contrat",
          subtitle:
            "Visualiser votre contrat avant de l'envoyer pour signature",
          checked: false,
          selected: false,
          disabled: true,
          component: <Pane />,
        },
      ]}
    />
  </ThemeProvider>
);

withAnimation.parameters = {
  // disables Chromatic's snapshotting
  chromatic: { disableSnapshot: true },
};
