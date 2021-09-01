import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { Themable } from "../common/themable";
import { LenderHeader } from "./lender-header";

export default {
  title: "Lender/Header",
  component: LenderHeader,
};

export const standard = () =>
  <Themable>
    <LenderHeader lender={FactoryHelper.lenderList()[0]} />
  </Themable>;
