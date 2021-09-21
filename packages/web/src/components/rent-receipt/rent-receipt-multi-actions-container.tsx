// @ts-nocheck
import { useMutation } from "@apollo/client";
import { toaster } from "evergreen-ui";
import { translate } from "piteo-kit";
import React, { useContext, useState } from "react";
import { AsyncContext, AsyncContextAction } from "../../context/async-context";
import {
  RentReceiptCreateMutation,
  RentReceiptCreateMutationOptions,
} from "../../helpers";
import { useRouter } from "../../hooks/use-router";
import { Rent, RentInput, RentReceiptInput } from "../../types";
import { ConfirmDialogProps } from "../common/confirm-dialog";
import {
  RentManagerContext,
  RentManagerContextAction,
} from "../rent-manager/rent-manager-context";

const _ = translate("Onboarding");

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const router = useRouter();
    const { dispatch } = useContext(AsyncContext);
    const { dispatch: dispatchRentManager, selectedRentList } = useContext(
      RentManagerContext,
    );

    const [isShown, setIsShown] = useState(true);

    const [allRentReceiptsCreate, { loading: isConfirmLoading }] = useMutation(
      RentReceiptCreateMutation,
      RentReceiptCreateMutationOptions(),
    );

    const onCloseComplete = (): void => {
      router.goBack();
    };

    const showAsyncLoader = (): void => {
      dispatch({
        type: AsyncContextAction.SetActions,
        payload: [
          {
            id: "generate-rent-receipts",
            checked: false,
            title: _("generate_rent_receipts"),
          },
          {
            id: "send-receipts-mail",
            checked: false,
            title: _("send_receipts_mail"),
          },
        ],
      });
    };

    const rentToInput = (rentList: Rent[]): RentInput[] => {
      const rentInput: RentInput[] = [];
      for (const rent of rentList) {
        rentInput.push({
          amount: rent.amount,
          chargesAmount: rent.chargesAmount,
          fullAmount: rent.fullAmount,
          periodEnd: rent.periodEnd,
          periodStart: rent.periodStart,
          leaseId: rent.leaseId,
        });
      }
      return rentInput;
    };

    const onConfirm = async (): Promise<void> => {
      // Prepare the rent receipt to be send to the tenant
      const rentList = rentToInput(selectedRentList);
      const input: RentReceiptInput = { sendMail: true, rentList };
      try {
        // Hide the modal
        setIsShown(false);
        // Display the global async loader
        showAsyncLoader();
        // Run the mutation
        await allRentReceiptsCreate({ variables: { input } });
        // Hide the loader with simulate all intermediate steps as success
        dispatch({
          type: AsyncContextAction.SimulateAllActionsAsValid,
          payload: toaster.success(
            _("all_rent_mark_paid_success"),
            {
              duration: 10,
              id: "rent-receipt-success",
            },
          ),
        });
        dispatchRentManager({
          type: RentManagerContextAction.AllRentsSelected,
          payload: false,
        });
      } catch (error) {
        // Usefull for Logrocket track issue
        console.error({ error });
        toaster.danger(_("error_smi"));
      }
    };

    const componentProps: ConfirmDialogProps = {
      title: _("mark_all_as_paid"),
      message: _("rent_receipt_edit_all_confirmation_msg"),
      intent: "none",
      isShown,
      isConfirmLoading,
      onConfirm,
      onCloseComplete,
      confirmLabel: _("confirm"),
    };

    return WrappedComponent(componentProps);
  };
