import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { Lease, RentStatus } from "../../types";
import { uncast } from "../../utils";
import { RentReceivedMenuPopin } from "./rent-received-menu-popin";

export default {
  title: "Dashboard/Popin/RentReceivedMenuPopin",
  component: RentReceivedMenuPopin,
};

const contract: Lease = uncast({
  specialTerms: {
    rentFullAmount: {
      amount: 450,
    },
  },
});

const tenant = FactoryHelper.tenantList()[0];

export const completeState = () => (
  <RentReceivedMenuPopin
    contract={contract}
    tenant={tenant}
    rentStatus={RentStatus.Settled}
  />
);

export const pendingState = () => (
  <RentReceivedMenuPopin
    contract={contract}
    tenant={tenant}
    rentStatus={RentStatus.Pending}
  />
);
