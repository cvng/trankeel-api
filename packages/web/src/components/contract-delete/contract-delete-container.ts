import { useMutation, useQuery } from "@apollo/client";
import { toaster } from "evergreen-ui";
import { translate } from "piteo-kit";
import React, { useState } from "react";
import { useParams } from "react-router-dom";
import {
  ContractDeleteMutation,
  ContractDeleteMutationOptions,
  LeaseQuery,
} from "../../helpers";
import { useRouter } from "../../hooks/use-router";
import { ConfirmDialogProps } from "../common/confirm-dialog";

const _ = translate();

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const router = useRouter();
    const { contractId } = useParams();
    const [{ isShown }, setState] = useState({ isShown: true });

    const { data: { leases: [contract] } = { leases: [] } } = useQuery(
      LeaseQuery,
      {
        variables: { id: contractId },
      },
    );

    const [contractDelete, { loading: isConfirmLoading }] = useMutation(
      ContractDeleteMutation,
      ContractDeleteMutationOptions(),
    );

    const handleConfirm = async (): Promise<void> => {
      await contractDelete({ variables: { id: contractId } });
      toaster.notify(_("contract_delete_success"));
      // history.push(Routes.PROPERTY_DETAIL.replace(":id", propertyId));
      setState({ isShown: false });
    };

    const handleCloseComplete = (): void => {
      router.goBack();
    };

    const componentProps: ConfirmDialogProps = {
      title: _("delete_contract_title"),
      message: _("delete_contract_message", {
        tenantName: contract?.tenants?.[0].fullName,
      }),
      isShown,
      isConfirmLoading,
      onConfirm: handleConfirm,
      onCloseComplete: handleCloseComplete,
    };

    return WrappedComponent(componentProps);
  };
