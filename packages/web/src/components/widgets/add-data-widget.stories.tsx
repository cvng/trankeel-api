import React from "react";
import { AddDataWidget } from "./add-data-widget";

export default {
  title: "Dashboard/AddDataWidget",
  component: AddDataWidget,
};

export const defaultState = () => <AddDataWidget />;

export const withHasPropertySet = () => <AddDataWidget hasProperty={true} />;

export const withHasTenantSet = () => (
  <AddDataWidget hasProperty={true} hasTenant={true} />
);

export const withContractSet = () => (
  <AddDataWidget hasProperty={true} hasTenant={true} hasContract={true} />
);
