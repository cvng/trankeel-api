import React from "react";
import { Themable } from "../common/themable";
import { LenderForm } from "./lender-form";

export default {
  title: "Lender/LenderForm",
  component: LenderForm,
};

export const physicalPerson = () => (
  <Themable>
    <LenderForm hasFooter={false} form={null} isPhysicalPerson />
  </Themable>
);

export const moralPerson = () => (
  <Themable>
    <LenderForm hasFooter={false} form={null} isPhysicalPerson={false} />
  </Themable>
);
