import React, { useContext, useState } from "react";
import { TenantSynthesisRoutes } from ".";
import { useRouter } from "../../hooks/use-router";
import { useSearch } from "../../hooks/use-search";
import { Tenant } from "../../types";
import {
  TenantSynthesisContext,
  TenantSynthesisContextProps,
} from "./tenant-synthesis-context";
import { TenantSynthesisListProps } from "./tenant-synthesis-list";

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const router = useRouter();
    const [filteredText, setFilteredText] = useState(null);

    const context = useContext(
      TenantSynthesisContext,
    ) as TenantSynthesisContextProps;
    let { loading, selectedTenant, tenants } = context;

    // Search
    const filterEnabled = !context?.loading && filteredText?.length > 0;
    tenants = useSearch(
      (item: Tenant) => {
        return item.displayName;
      },
      tenants,
      filteredText,
    );

    const onTenantSelect = (tenantId: string): void => {
      router.showTenantSynthesisRoute(
        tenantId,
        TenantSynthesisRoutes.Synthesis,
      );
    };

    const onTenantAdd = (): void => {
      router.showTenantAdd();
    };

    const onSearchFieldChange = (value: string): void => {
      setFilteredText(value);
    };

    const componentProps: TenantSynthesisListProps = {
      tenants,
      selectedTenant,
      loading,
      filterEnabled,
      onTenantSelect,
      onTenantAdd,
      onSearchFieldChange,
    };

    return WrappedComponent(componentProps);
  };
