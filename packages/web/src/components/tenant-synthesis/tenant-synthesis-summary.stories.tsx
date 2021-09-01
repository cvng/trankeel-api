import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { Themable } from "../common/themable";
import { TenantSynthesisSummary } from ".";

export default {
  title: "Tenant/Synthesis/TenantSynthesisSummary",
  component: TenantSynthesisSummary,
};

const tenant = FactoryHelper.tenantList()[0];

export const standard = () =>
  <Themable>
    <TenantSynthesisSummary
      tenant={tenant}
    />
  </Themable>;
