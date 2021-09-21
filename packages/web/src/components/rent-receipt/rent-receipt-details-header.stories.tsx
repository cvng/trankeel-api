import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { RentReceiptDetailsHeader } from "./rent-receipt-details-header";

export default {
  title: "RentReceipt/RentReceiptDetailsHeader",
  component: RentReceiptDetailsHeader,
};

const tenant = FactoryHelper.tenantList()[0];

const contract = FactoryHelper.leaseList()[0];

const tenantB = { ...tenant, contract: { ...contract } };

export const noRent = () => <RentReceiptDetailsHeader tenant={tenant} />;

export const withRent = () => <RentReceiptDetailsHeader tenant={tenantB} />;
