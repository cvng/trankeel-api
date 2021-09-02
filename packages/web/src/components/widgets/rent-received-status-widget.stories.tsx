import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { RentReceivedStatusWidget } from "./rent-received-status-widget";

export default {
  title: "Dashboard/RentReceivedStatusWidget",
  component: RentReceivedStatusWidget,
};

export const loading = () => <RentReceivedStatusWidget loading={true} />;

export const noValue = () => <RentReceivedStatusWidget />;

const propertyWithOneTenant = FactoryHelper.propertyList()[2];

const propertyWithManyTenant = FactoryHelper.propertyList()[1];

const propertyWithNoActiveTenant = FactoryHelper.propertyList()[3];

export const standard = () => (
  <RentReceivedStatusWidget property={propertyWithOneTenant} />
);

export const multipleTenants = () => (
  <RentReceivedStatusWidget property={propertyWithManyTenant} />
);

export const noActiveContrat = () => (
  <RentReceivedStatusWidget property={propertyWithNoActiveTenant} />
);
