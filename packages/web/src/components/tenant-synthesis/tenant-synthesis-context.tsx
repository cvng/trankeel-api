import { useQuery } from "@apollo/client";
import React, { createContext, useEffect, useState } from "react";
import { Routes } from "../../constants";
import { TenantListQuery } from "../../helpers";
import { useParamId } from "../../hooks/use-param-id";
import { Tenant } from "../../types";

export const TenantSynthesisContext = createContext(null);

export interface TenantSynthesisContextProps {
  tenants: Tenant[];
  selectedTenant?: Tenant;
  loading: boolean;
}

export const TenantSynthesisProvider = ({ children }) => {
  // Get the ids in the url
  const tenantId = useParamId(Routes.TENANT_VIEW);

  const [selectedTenant, setSelectedTenant] = useState(null);

  // Fetch the tenants
  const { loading, data: { tenants } = { tenants: [] } } = useQuery(
    TenantListQuery,
  );

  // Select the first tenant by default or get the selected in the list by id
  useEffect(() => {
    setSelectedTenant(
      tenants?.find((tenant: Tenant) => tenant.id === tenantId) ||
        tenants?.[0],
    );
  }, [tenantId, tenants]);

  return (
    <TenantSynthesisContext.Provider
      value={{ tenants, loading, selectedTenant }}
    >
      {children}
    </TenantSynthesisContext.Provider>
  );
};
