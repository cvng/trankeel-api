import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { Themable } from "../common/themable";
import { LeaseSelectParties } from "./lease-select-parties";

export default {
  title: "Lease/LeaseSelectParties",
  component: LeaseSelectParties,
};

const properties = FactoryHelper.propertyList();
const tenants = FactoryHelper.tenantList();

export const standard = () =>
  <Themable>
    <LeaseSelectParties
      createFromExistingLease
      loading={false}
      properties={properties}
      tenants={tenants}
      selectedProperty={properties[0]}
    />
  </Themable>;
