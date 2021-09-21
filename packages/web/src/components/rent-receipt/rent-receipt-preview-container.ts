// @ts-nocheck
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
  RentReceiptCreateMutation,
  RentReceiptCreateMutationOptions,
} from "../../helpers";
import { useRouter } from "../../hooks/use-router";
import { Lease, Rent, RentReceiptInput } from "../../types";
import { DATE_FORMAT } from "../../utils";
import { DATE_ISO_FORMAT } from "../../validators";
import { RentReceiptPreviewProps } from "./rent-receipt-preview";

const _ = translate("Onboarding");

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const router = useRouter();

    const [isShown, setIsShown] = useState(true);
    const [sendMail, setSendMail] = useState(true); // By default the checkbox is set to true
    const { dispatch } = useContext(AsyncContext);

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

    const [rentReceiptCreate] = useMutation(
      RentReceiptCreateMutation,
      RentReceiptCreateMutationOptions(),
    );

    const onCloseComplete = (): void => {
      router.goBack();
    };

    const onSendMailChange = (checked: boolean) => {
      setSendMail(checked);
    };

    const onConfirmSendReceipt = async (): Promise<void> => {
      const rentInput = {
        periodStart: rent.periodStart,
        periodEnd: rent.periodEnd,
        amount: rent.amount,
        chargesAmount: rent.chargesAmount,
        fullAmount: rent.fullAmount,
        leaseId: rent.leaseId,
      };
      // Prepare the rent receipt to be send to the tenant
      const input: RentReceiptInput = {
        rentList: [rentInput],
        sendMail,
      };
      try {
        // Hide the modal
        setIsShown(false);
        // Display the global async loader
        showAsyncLoader(sendMail);
        await rentReceiptCreate({ variables: { input } });
        // Hide the loader with simulate all intermediate steps as success
        dispatch({
          type: AsyncContextAction.SimulateAllActionsAsValid,
          payload: toaster.success(
            _(
              sendMail
                ? "transaction_rent_add_success"
                : "rent_receipt_no_mail_success",
            ),
            {
              duration: 10,
              id: "rent-receipt-success",
            },
          ),
        });
      } catch (error) {
        // Usefull for Logrocket track issue
        console.error({ error });
        toaster.danger(_("error_smi"));
      }
    };

    const showAsyncLoader = (sendMail: boolean): void => {
      const actions = [
        {
          id: "generate-rent-receipt",
          checked: false,
          title: _("generate_rent_receipt"),
        },
        {
          id: "send-receipt-mail",
          checked: false,
          title: _("send_receipt_mail"),
        },
      ];
      if (sendMail === false) {
        actions.splice(1);
      }
      dispatch({
        type: AsyncContextAction.SetActions,
        payload: actions,
      });
    };

    // Return the rent receipt amount data
    const rentReceiptAmountData = (rent: Rent): {
      periodStart?: string;
      periodEnd?: string;
      rentAmount?: number;
      rentChargesAmount?: number;
      rentFullAmount?: number;
    } => {
      if (!rent) {
        return {};
      }
      // If the rent is calculated with prorata
      // we need to subtract one day as the period return by the server as the period end
      // start with the first day of the next month
      const daysDiff = moment(rent?.periodEnd).diff(rent?.periodStart, "days") +
        1; // Number of days in period
      const daysInMonth = moment(rent?.periodStart).daysInMonth(); // Number of days in month
      const isProrata = (daysDiff < daysInMonth); // The rent is prorata when there is less day in the period comparatively to the month
      return {
        periodStart: moment(rent?.periodStart).format(DATE_FORMAT),
        // While using the rent we explicitely remove one day to match the display criteria
        // as for the default period we use the first and the last day of the month
        periodEnd: isProrata
          ? moment(rent?.periodEnd).subtract(1, "day").format(DATE_FORMAT)
          : moment(rent?.periodEnd).format(DATE_FORMAT),
        rentAmount: rent?.amount,
        rentChargesAmount: rent?.chargesAmount,
        rentFullAmount: rent?.fullAmount,
      };
    };

    const onLenderAddMissingAddress = (lenderId: string) => {
      router?.showLenderEdit(lenderId);
    };

    const amountData = rentReceiptAmountData(rent as Rent);

    const componentProps: RentReceiptPreviewProps = {
      loading: isLoadingLease || isLoadingRent,
      isShown,
      lease,
      periodStart: amountData?.periodStart,
      periodEnd: amountData?.periodEnd,
      rentAmount: amountData?.rentAmount,
      rentChargesAmount: amountData?.rentChargesAmount,
      rentFullAmount: amountData?.rentFullAmount,
      sendMail,
      onCloseComplete,
      onConfirmSendReceipt,
      lenderMissingAddress: !lease?.property?.lender?.identity?.address,
      onLenderAddMissingAddress,
      onSendMailChange,
    };

    return WrappedComponent(componentProps);
  };
