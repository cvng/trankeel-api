import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { RentReceiptTenantSelection } from "./rent-receipt-tenant-selection";

export default {
  title: "RentReceipt/RentReceiptTenantSelection",
  component: RentReceiptTenantSelection,
};

const tenants = FactoryHelper.tenantList();

const selectedTenants = [tenants[0]];

export const loading = () => (
  <RentReceiptTenantSelection
    isAllTenantsSelected={false}
    selectedTenants={[]}
    tenants={[]}
    onSelectTenantClick={() => {}}
    loading={true}
    onSelectAllTenantsClick={() => {}}
  />
);

export const defaultState = () => (
  <RentReceiptTenantSelection
    isAllTenantsSelected={false}
    selectedTenants={selectedTenants}
    tenants={tenants}
    onSelectTenantClick={() => {}}
    loading={false}
    onSelectAllTenantsClick={() => {}}
  />
);
