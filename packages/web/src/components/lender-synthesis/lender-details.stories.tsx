import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { Themable } from "../common/themable";
import { LenderDetails } from "./lender-details";

export default {
  title: "Lender/LenderDetails",
  component: LenderDetails,
};

const lenders = FactoryHelper.lenderList();

export const loading = () =>
  <Themable>
    <LenderDetails
      lender={null}
      loading
    />
  </Themable>;

export const noLender = () =>
  <Themable>
    <LenderDetails
      lender={null}
      loading={false}
    />
  </Themable>;

export const withMoralPersonLender = () =>
  <Themable>
    <LenderDetails
      lender={lenders[0]}
      loading={false}
    />
  </Themable>;

export const withPhysicalPersonLender = () =>
  <Themable>
    <LenderDetails
      lender={lenders[1]}
      loading={false}
    />
  </Themable>;
