import { useMutation, useQuery } from "@apollo/client";
import { toaster } from "evergreen-ui";
import { translate } from "piteo-kit";
import React, { useState } from "react";
import { Routes } from "../../constants";
import {
  TenantDeleteMutation,
  TenantDeleteMutationOptions,
  TenantListQuery,
} from "../../helpers";
import { useParamId } from "../../hooks/use-param-id";
import { useRouter } from "../../hooks/use-router";
import { Tenant } from "../../types";
import { ConfirmDialogProps } from "../common/confirm-dialog";

const _ = translate();

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const router = useRouter();
    const tenantId = useParamId(Routes.TENANT_DELETE);
    const [{ isShown }, setState] = useState({ isShown: true });

    const { data: { tenants } = { tenants: null } } = useQuery(
      TenantListQuery,
    );

    const tenant = tenants?.find((item: Tenant) => item.id === tenantId);

    const [tenantDelete, { loading: isConfirmLoading }] = useMutation(
      TenantDeleteMutation,
      TenantDeleteMutationOptions(),
    );

    const handleConfirm = async (): Promise<void> => {
      const hasAnActiveLease = !!tenant?.lease;
      if (hasAnActiveLease) {
        toaster.warning(
          _("delete_tenant_existing_contract"),
        );
        setState({ isShown: false });
        return;
      }
      await tenantDelete({ variables: { id: tenantId } });
      toaster.notify(_("tenant_delete_success"));
      setState({ isShown: false });
    };

    const handleCloseComplete = (): void => {
      router.goBack();
    };

    const componentProps: ConfirmDialogProps = {
      title: _("delete_tenant"),
      message: _("delete_tenant_confirmation_msg", {
        tenantName: tenant?.fullName,
      }),
      isShown,
      isConfirmLoading,
      onConfirm: handleConfirm,
      onCloseComplete: handleCloseComplete,
    };

    return WrappedComponent(componentProps);
  };
