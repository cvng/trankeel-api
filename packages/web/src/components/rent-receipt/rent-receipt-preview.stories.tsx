import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { Themable } from "../common/themable";
import { RentReceiptPreview } from "./rent-receipt-preview";

export default {
  title: "RentReceipt/RentReceiptPreview",
  component: RentReceiptPreview,
};

const lease = FactoryHelper.leaseList()[0];

const periodStart = "01/05/2020";
const periodEnd = "31/05/2020";
const rentAmount = 315;
const rentChargesAmount = 50;
const rentFullAmount = 365;

export const noData = () =>
  <Themable>
    <RentReceiptPreview />
  </Themable>;

export const loading = () => (
  <Themable>
    <RentReceiptPreview
      loading
      lease={lease}
      periodStart={periodStart}
      periodEnd={periodEnd}
      rentAmount={rentAmount}
      rentChargesAmount={rentChargesAmount}
      rentFullAmount={rentFullAmount}
    />
  </Themable>
);

export const withData = () => (
  <Themable>
    <RentReceiptPreview
      lease={lease}
      periodStart={periodStart}
      periodEnd={periodEnd}
      rentAmount={rentAmount}
      rentChargesAmount={rentChargesAmount}
      rentFullAmount={rentFullAmount}
    />
  </Themable>
);

export const withDataAsNotice = () => (
  <Themable>
    <RentReceiptPreview
      lease={lease}
      periodStart={periodStart}
      periodEnd={periodEnd}
      rentAmount={rentAmount}
      rentChargesAmount={rentChargesAmount}
      rentFullAmount={rentFullAmount}
      isNotice={true}
    />
  </Themable>
);
