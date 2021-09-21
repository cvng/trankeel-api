import { useMutation, useQuery } from "@apollo/client";
import { toaster } from "evergreen-ui";
import moment from "moment";
import { translate } from "piteo-kit";
import React, { useContext, useState } from "react";
import { useParams } from "react-router-dom";
import { AsyncContext, AsyncContextAction } from "../../context/async-context";
import {
  LeaseListQuery,
  RentListQuery,
  SendPaymentNoticeMutation,
  SendPaymentNoticeMutationOptions,
} from "../../helpers";
import { useRouter } from "../../hooks/use-router";
import { Lease, Rent, SendPaymentNoticeInput } from "../../types";
import { DATE_FORMAT } from "../../utils";
import { DATE_ISO_FORMAT } from "../../validators";
import { RentReceiptPreviewProps } from "../rent-receipt/rent-receipt-preview";

const _ = translate();

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const router = useRouter();
    const { dispatch } = useContext(AsyncContext);

    const [isShown, setIsShown] = useState(true);

    const { rentId } = useParams();

    const { loading: isLoadingLease, data: { leases } = { leases: [] } } =
      useQuery(
        LeaseListQuery,
      );

    const { loading: isLoadingRent, data: { rents } = { rents: [] } } =
      useQuery(
        RentListQuery,
        {
          variables: {
            since: moment().startOf("month").format(DATE_ISO_FORMAT),
            until: moment().add(1, "month").startOf("month").format(
              DATE_ISO_FORMAT,
            ),
          },
        },
      );
    const rent = rents?.find((rent: Rent) => rent.id === rentId);

    const lease = leases?.find((item: Lease) => item.id === rent?.leaseId);

    const [sendPaymentNotice] = useMutation(
      SendPaymentNoticeMutation,
      SendPaymentNoticeMutationOptions(),
    );

    const onCloseComplete = (): void => {
      router.goBack();
    };

    const onConfirmSendPaymentNotice = async (): Promise<void> => {
      const rentInput = {
        periodStart: rent.periodStart,
        periodEnd: rent.periodEnd,
        amount: rent.amount,
        chargesAmount: rent.chargesAmount,
        fullAmount: rent.fullAmount,
        leaseId: rent.leaseId,
      };
      // Prepare the payment notice to be send to the tenant
      const input: SendPaymentNoticeInput = { rentList: [rentInput] };
      try {
        // Hide the confirmation modal
        setIsShown(false);
        showAsyncLoader();
        await sendPaymentNotice({ variables: { input } });
        // Hide the loader with simulate all intermediate steps as success
        dispatch({
          type: AsyncContextAction.SimulateAllActionsAsValid,
          payload: toaster.success(
            _("payment_notice_send_success"),
            {
              duration: 10,
              id: "send-payment-notice-success",
            },
          ),
        });
      } catch (error) {
        // Usefull for Logrocket track issue
        console.error({ error });
        toaster.danger(_("error_smi"));
      }
    };

    const showAsyncLoader = (): void => {
      dispatch({
        type: AsyncContextAction.SetActions,
        payload: [
          {
            id: "generate-payment-notice",
            checked: false,
            title: _("generate_payment_notice"),
          },
          {
            id: "mail-payment-notice",
            checked: false,
            title: _("mail_payment_notice"),
          },
        ],
      });
    };

    const onLenderAddMissingAddress = (lenderId: string) => {
      router?.showLenderEdit(lenderId);
    };

    const componentProps: RentReceiptPreviewProps = {
      loading: isLoadingLease || isLoadingRent,
      isShown,
      lease,
      periodStart: moment().startOf("month").format(DATE_FORMAT),
      periodEnd: moment().endOf("month").format(DATE_FORMAT),
      rentAmount: lease?.rentAmount,
      rentChargesAmount: lease?.rentChargesAmount,
      rentFullAmount: lease?.rentFullAmount,
      onCloseComplete,
      onConfirmSendReceipt: onConfirmSendPaymentNotice,
      isNotice: true,
      lenderMissingAddress: !lease?.property?.lender?.identity?.address,
      onLenderAddMissingAddress,
    };

    return WrappedComponent(componentProps);
  };
