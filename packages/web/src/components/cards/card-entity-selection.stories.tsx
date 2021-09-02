import { ThemeProvider } from "evergreen-ui";
import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { AppTheme } from "../../theme/app-theme";
import { Property } from "../../types";
import { CardEntitySelection } from "./card-entity-selection";

export default {
  title: "Cards/CardEntitySelection",
  component: CardEntitySelection,
};

const properties = FactoryHelper.propertyList();

export const loading = () => {
  return (
    <ThemeProvider value={AppTheme}>
      <CardEntitySelection
        title={"Sélectionner le bien"}
        index={1}
        loading
        items={properties}
        selectedItem={null}
        onAddItem={() => {}}
        addItemTitle={"Ajouter un bien"}
      />
    </ThemeProvider>
  );
};

export const standard = () => {
  return (
    <ThemeProvider value={AppTheme}>
      <CardEntitySelection
        title={"Sélectionner le bien"}
        index={1}
        loading={false}
        items={properties}
        itemLabel={(item: Property) => item?.name}
        selectedItem={null}
        onAddItem={() => {}}
        addItemTitle={"Ajouter un bien"}
      />
    </ThemeProvider>
  );
};

export const selectedItem = () => {
  return (
    <ThemeProvider value={AppTheme}>
      <CardEntitySelection
        title={"Sélectionner le bien"}
        index={1}
        loading={false}
        items={properties}
        itemLabel={(item: Property) => item?.name}
        selectedItem={properties[0]}
        onAddItem={() => {}}
        addItemTitle={"Ajouter un bien"}
      />
    </ThemeProvider>
  );
};

export const alert = () => {
  return (
    <ThemeProvider value={AppTheme}>
      <CardEntitySelection
        title={"Sélectionner le bien"}
        index={1}
        loading={false}
        items={properties}
        itemLabel={(item: Property) => item?.name}
        selectedItem={properties[0]}
        alert={{
          intent: "warning",
          content:
            "Vous ne verrez uniquement les locataires qui n'ont pas de location en cours.",
        }}
      />
    </ThemeProvider>
  );
};
