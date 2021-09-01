import { useMutation, useQuery } from "@apollo/client";
import { toaster } from "evergreen-ui";
import { translate } from "piteo-kit";
import React, { useState } from "react";
import { useParams } from "react-router-dom";
import {
  NumberHelper,
  TransactionDeleteMutation,
  TransactionDeleteMutationOptions,
  TransactionQuery,
} from "../../helpers";
import { useRouter } from "../../hooks/use-router";
import { ConfirmDialogProps } from "../common/confirm-dialog";

const _ = translate("Transaction");

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const router = useRouter();
    const { transactionId } = useParams();
    const [{ isShown }, setState] = useState({ isShown: true });

    const { data: { transactions: [transaction] } = { transactions: [] } } =
      useQuery(
        TransactionQuery,
        {
          variables: { id: transactionId },
        },
      );

    const [transactionDelete, { loading: isConfirmLoading }] = useMutation(
      TransactionDeleteMutation,
      TransactionDeleteMutationOptions(),
    );

    const handleConfirm = async (): Promise<void> => {
      await transactionDelete({ variables: { id: transactionId } });
      toaster.notify(_("transaction_delete_success"));
      setState({ isShown: false });
    };

    const handleCloseComplete = (): void => {
      router.goBack();
    };

    const componentProps: ConfirmDialogProps = {
      title: _("delete_transaction_title"),
      message: _("delete_transaction_message", {
        amount: NumberHelper.formatToString(transaction?.amount, false),
      }),
      isShown,
      isConfirmLoading,
      onConfirm: handleConfirm,
      onCloseComplete: handleCloseComplete,
    };

    return WrappedComponent(componentProps);
  };
