import React, { useContext } from "react";
import { useRouter } from "../../hooks/use-router";
import {
  TenantSynthesisContext,
  TenantSynthesisContextProps,
} from "./tenant-synthesis-context";
import { TenantSynthesisHeaderProps } from "./tenant-synthesis-header";

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const router = useRouter();

    const { loading, selectedTenant: tenant } = useContext(
      TenantSynthesisContext,
    ) as TenantSynthesisContextProps;

    const onTenantEdit = (tenantId: string): void => {
      router.showTenantEdit(tenantId);
    };
    const onTenantDelete = (tenantId: string): void => {
      router.showTenantDelete(tenantId);
    };

    const componentProps: TenantSynthesisHeaderProps = {
      loading,
      tenant,
      onTenantEdit,
      onTenantDelete,
    };

    return WrappedComponent(componentProps);
  };
