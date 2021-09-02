import { Pane, ThemeProvider } from "evergreen-ui";
import React from "react";
import { AppTheme } from "../../theme/app-theme";
import { CheckListItem } from "./check-list-item";

export default {
  title: "Design System/CheckListItem",
  component: CheckListItem,
};

export const checked = () => (
  <ThemeProvider value={AppTheme}>
    <CheckListItem
      index={1}
      checked
      disabled={false}
      selected
      title={"Sélectionner le bien"}
    />
  </ThemeProvider>
);

export const uncheck = () => (
  <ThemeProvider value={AppTheme}>
    <CheckListItem
      index={1}
      checked={false}
      disabled={false}
      selected={false}
      title={"Sélectionner le bien"}
    />
  </ThemeProvider>
);

export const disabled = () => (
  <ThemeProvider value={AppTheme}>
    <CheckListItem
      index={1}
      checked={false}
      disabled={true}
      selected={false}
      title={"Sélectionner le bien"}
    />
  </ThemeProvider>
);

export const withSubtitle = () => (
  <ThemeProvider value={AppTheme}>
    <CheckListItem
      index={1}
      checked={false}
      disabled={true}
      selected={false}
      title={"Locataire"}
      subtitle={"Sélectionner votre nouveau locataire"}
    />
  </ThemeProvider>
);

export const multiple = () => (
  <ThemeProvider value={AppTheme}>
    <Pane>
      <CheckListItem
        index={1}
        checked={true}
        disabled={false}
        selected={false}
        title={"Sélectionner le bien"}
      />
      <CheckListItem
        index={2}
        checked={false}
        disabled={false}
        selected={true}
        title={"Sélectionner le(s) locataire(s)"}
      />
      <CheckListItem
        index={3}
        checked={false}
        disabled={true}
        selected={false}
        title={"Remplir les détails du contrat"}
      />
    </Pane>
  </ThemeProvider>
);
