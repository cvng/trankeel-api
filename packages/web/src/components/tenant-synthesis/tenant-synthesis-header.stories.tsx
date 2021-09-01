import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { Themable } from "../common/themable";
import { TenantSynthesisHeader } from "./tenant-synthesis-header";

export default {
  title: "Tenant/Synthesis/TenantSynthesisHeader",
  component: TenantSynthesisHeader,
};

export const standard = () =>
  <Themable>
    <TenantSynthesisHeader tenant={FactoryHelper.tenantList()[0]} />
  </Themable>;
