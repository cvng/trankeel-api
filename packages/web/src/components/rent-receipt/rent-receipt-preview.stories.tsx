import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { RentReceiptPreview } from "./rent-receipt-preview";

export default {
  title: "RentReceipt/RentReceiptPreview",
  component: RentReceiptPreview,
};

export const noData = () => <RentReceiptPreview />;

const lender = FactoryHelper.lenderList()[0];

const tenant = FactoryHelper.tenantList()[0];

const property = FactoryHelper.propertyList()[0];

const periodStart = "01/05/2020";
const periodEnd = "31/05/2020";
const rentAmount = 315;
const rentChargesAmount = 50;
const rentFullAmount = 365;

export const withData = () => (
  <RentReceiptPreview
    lender={lender}
    tenant={tenant}
    property={property}
    periodStart={periodStart}
    periodEnd={periodEnd}
    rentAmount={rentAmount}
    rentChargesAmount={rentChargesAmount}
    rentFullAmount={rentFullAmount}
  />
);

export const withDataAsNotice = () => (
  <RentReceiptPreview
    lender={lender}
    tenant={tenant}
    property={property}
    periodStart={periodStart}
    periodEnd={periodEnd}
    rentAmount={rentAmount}
    rentChargesAmount={rentChargesAmount}
    rentFullAmount={rentFullAmount}
    isNotice={true}
  />
);
