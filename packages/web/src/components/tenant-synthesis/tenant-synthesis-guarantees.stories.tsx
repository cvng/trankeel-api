import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { Themable } from "../common/themable";
import { TenantSynthesisGuarantees } from "./tenant-synthesis-guarantees";

export default {
  title: "Tenant/Synthesis/TenantSynthesisGuarantees",
  component: TenantSynthesisGuarantees,
};

export const standard = () =>
  <Themable>
    <TenantSynthesisGuarantees tenant={FactoryHelper.tenantList()[0]} />
  </Themable>;
