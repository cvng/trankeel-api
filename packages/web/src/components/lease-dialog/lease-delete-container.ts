import { useMutation } from "@apollo/client";
import { toaster } from "evergreen-ui";
import { translate } from "piteo-kit";
import React, { useState } from "react";
import { useParams } from "react-router-dom";
import { LeaseDeleteMutation, LeaseDeleteMutationOptions } from "../../helpers";
import { useRouter } from "../../hooks/use-router";
import { ConfirmDialogProps } from "../common/confirm-dialog";

const _ = translate();

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const router = useRouter();
    const { leaseId } = useParams();

    const [{ isShown }, setState] = useState({ isShown: true });

    const [leaseDelete, { loading: isConfirmLoading }] = useMutation(
      LeaseDeleteMutation,
      LeaseDeleteMutationOptions(),
    );

    const handleConfirm = async (): Promise<void> => {
      await leaseDelete({ variables: { id: leaseId } });

      toaster.notify(
        _("delete_lease_confirmation"),
      );
      setState({ isShown: false });
    };

    const handleCloseComplete = (): void => {
      router.goBack();
    };

    const componentProps: ConfirmDialogProps = {
      title: _("delete_lease_title"),
      message: _("delete_lease_message"),
      isShown,
      isConfirmLoading,
      onConfirm: handleConfirm,
      onCloseComplete: handleCloseComplete,
    };

    return WrappedComponent(componentProps);
  };
