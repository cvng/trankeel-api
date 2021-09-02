import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { Themable } from "../common/themable";
import { TenantSynthesisInformations } from "./tenant-synthesis-informations";

export default {
  title: "Tenant/Synthesis/TenantSynthesisInformations",
  component: TenantSynthesisInformations,
};

export const standard = () =>
  <Themable>
    <TenantSynthesisInformations tenant={FactoryHelper.tenantList()[0]} />
  </Themable>;
