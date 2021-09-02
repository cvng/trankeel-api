import { majorScale, Pane, ThemeProvider } from "evergreen-ui";
import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { AppTheme } from "../../theme/app-theme";
import { CardItem } from "./card-item";

export default {
  title: "Cards/CardItem",
  component: CardItem,
};

const property = FactoryHelper.propertyList()[0];

const tenant = FactoryHelper.tenantList()[0];

const lender = FactoryHelper.lenderList()[0];

export const standard = () => {
  return (
    <ThemeProvider value={AppTheme}>
      <CardItem
        item={property}
        title={property?.name}
        subtitle={`${property?.address?.line1}, ${property?.address?.city}`}
      />
    </ThemeProvider>
  );
};

export const standardTenant = () => {
  return (
    <ThemeProvider value={AppTheme}>
      <CardItem
        item={tenant}
        title={tenant?.fullName}
        subtitle={tenant?.email}
      />
    </ThemeProvider>
  );
};

export const selected = () =>
  <ThemeProvider value={AppTheme}>
    <CardItem
      title={property.name}
      subtitle={`${property?.address?.line1}, ${property?.address?.city}`}
      selected
    />
  </ThemeProvider>;

export const properties = () => (
  <ThemeProvider value={AppTheme}>
    <Pane>
      <CardItem title={property.name} />
      <CardItem
        title={property.name}
        subtitle={`${property?.address?.line1}, ${property?.address?.city}`}
        marginBottom={majorScale(1)}
      />
      <CardItem
        title={property.name}
        subtitle={`${property?.address?.line1}, ${property?.address?.city}`}
        marginBottom={majorScale(1)}
        selected
      />
    </Pane>
  </ThemeProvider>
);

export const lenders = () => (
  <ThemeProvider value={AppTheme}>
    <Pane>
      <CardItem
        title={lender.displayName}
      />
      <CardItem
        title={lender.displayName}
        selected
      />
    </Pane>
  </ThemeProvider>
);
