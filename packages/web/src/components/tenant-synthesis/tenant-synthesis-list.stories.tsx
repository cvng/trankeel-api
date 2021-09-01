import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { Themable } from "../common/themable";
import { TenantSynthesisList } from ".";

export default {
  title: "Tenant/Synthesis/TenantSynthesisList",
  component: TenantSynthesisList,
};

const tenants = FactoryHelper.tenantList();

export const standard = () =>
  <Themable>
    <TenantSynthesisList
      tenants={tenants}
      selectedTenant={tenants?.[0]}
      loading={false}
      filterEnabled={false}
    />
  </Themable>;

export const noProperty = () =>
  <Themable>
    <TenantSynthesisList
      tenants={[]}
      loading={false}
      filterEnabled={false}
    />
  </Themable>;

export const noFilteredProperty = () =>
  <Themable>
    <TenantSynthesisList tenants={[]} loading={false} filterEnabled />
  </Themable>;

export const loading = () =>
  <Themable>
    <TenantSynthesisList tenants={[]} loading filterEnabled={false} />
  </Themable>;
