import React from "react";
import { RentReceiptForm } from "./rent-receipt-form";

export default {
  title: "RentReceipt/RentReceiptForm",
  component: RentReceiptForm,
};

export const noData = () => <RentReceiptForm />;

export const noAddress = () => <RentReceiptForm isMissingAddress />;

export const partialRentData = () => (
  <RentReceiptForm
    isPartialRent
    partialRentData={{
      amount: 370,
      chargesAmount: 80,
      rentFullAmount: 470,
      days: 5,
    }}
  />
);
