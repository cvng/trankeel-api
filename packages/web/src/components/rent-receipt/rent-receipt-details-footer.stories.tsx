import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { RentReceiptDetailsFooter } from "./rent-receipt-details-footer";

export default {
  title: "RentReceipt/RentReceiptDetailsFooter",
  component: RentReceiptDetailsFooter,
};

const selectedTenants = FactoryHelper.tenantList();

export const previousDisabled = () => (
  <RentReceiptDetailsFooter
    selectedTenants={selectedTenants}
    selectedTenant={selectedTenants[0]}
    validateButtonEnabled={true}
    validateButtonClick={() => {}}
  />
);

export const nextDisabled = () => (
  <RentReceiptDetailsFooter
    selectedTenants={selectedTenants}
    selectedTenant={selectedTenants[1]}
    validateButtonEnabled={true}
    validateButtonClick={() => {}}
  />
);

export const bothDisabled = () => (
  <RentReceiptDetailsFooter
    selectedTenants={[]}
    selectedTenant={null}
    validateButtonEnabled={true}
    validateButtonClick={() => {}}
  />
);

export const validateButtonDisabled = () => (
  <RentReceiptDetailsFooter
    selectedTenants={selectedTenants}
    selectedTenant={selectedTenants[1]}
    validateButtonEnabled={false}
    validateButtonClick={() => {}}
  />
);
