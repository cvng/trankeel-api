import { useMutation, useQuery } from "@apollo/client";
import { toaster } from "evergreen-ui";
import React from "react";
import { useHistory } from "react-router-dom";
import { RouteById, Routes } from "../../constants";
import {
  LeaseListQuery,
  RentReceiptCreateMutation,
  RentReceiptCreateMutationOptions,
  RentReceivedStatusQuery,
  RentReceivedSummaryQuery,
  TransactionCreateMutation,
  TransactionCreateMutationOptions,
  TransactionDeleteMutation,
  TransactionDeleteMutationOptions,
} from "../../helpers";
import { Lease, Transaction } from "../../types";
import { DashboardProps } from "./dashboard";
import { translate } from "piteo-kit";

const _ = translate();

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const history = useHistory();

    const rentReceivedSummaryQueryResult = useQuery(RentReceivedSummaryQuery);

    const rentReceivedStatusQueryResult = useQuery(RentReceivedStatusQuery);

    const leaseListResult = useQuery(LeaseListQuery);

    const displayOnboarding = leaseListResult?.data?.leases?.length === 0;

    const [
      transactionCreate,
      { loading: isLoadingTransactionCreate },
    ] = useMutation(
      TransactionCreateMutation,
      TransactionCreateMutationOptions(),
    );
    const [
      transactionDelete,
      { loading: isLoadingTransactionDelete },
    ] = useMutation(
      TransactionDeleteMutation,
      TransactionDeleteMutationOptions(),
    );

    const [
      rentReceiptCreate,
      { loading: isLoadingRentReceiptCreate },
    ] = useMutation(
      RentReceiptCreateMutation,
      RentReceiptCreateMutationOptions(),
    );

    // Le tableau de bord est 100% disponible uniquement lorsque l'on a au moins un contrat
    const handleShowRentalContractClick = (contract: Lease) => {
      history.push(
        Routes.DASHBOARD_SHOW_CONTRACT.replace(":contractId", contract.id),
      );
    };

    const handleMarkRentAsPaid = async (paid: boolean, contract: Lease) => {
      try {
        if (paid) {
          // On supprime toutes les transactions
          // dans la plupart des cas on ne devrait en avoir qu'une
          const transactions: Transaction[] = contract.rents?.[0]?.transactions;
          for (let i = 0; i < transactions.length; i++) {
            await transactionDelete({ variables: { id: transactions[i].id } });
          }
          toaster.success(_("mark_rent_as_unpaid_success"), {
            id: "mark-rent-as-paid-toast",
          });
        } else {
          // Marquer le loyer comme étant payé
          await transactionCreate({
            variables: {
              input: {
                contractId: contract.id,
                date: new Date(),
                amount: contract?.rentFullAmount,
              },
            },
          });
          toaster.success(_("mark_rent_as_paid_success"), {
            id: "mark-rent-as-paid-toast",
          });
        }
      } catch {
        toaster.danger(_("error_smi"));
      }
    };

    const handleEditReceipt = async (lease: Lease) => {
      const hasAddress = !!lease?.property?.lender?.identity?.address;
      if (hasAddress) {
        // Generate rent receipt
        await rentReceiptCreate({
          variables: {
            input: {
              leaseId: lease.id,
              sendMail: true,
            },
          },
        }).catch(() => {
          toaster.danger(_("error_smi"));
        }).then(() => {
          toaster.success(_("edit_receipt_success"), {
            id: "edit-receipt-success",
          });
          // Redirect to the tenants document screen
          history.push(
            RouteById(
              Routes.TENANT_VIEW,
              [lease?.tenants?.[0].id, "documents"],
              [":id", ":route"],
            ),
          );
        });
      } else {
        toaster.notify(_("lender_informations_missing_address"), {
          duration: 10,
          id: "lender-missing-address-toast",
        });
        const lenderId = lease?.property?.lender?.id;
        history.push(RouteById(Routes.DASHBOARD_LENDER_EDIT, [lenderId]));
      }
    };

    const componentProps: DashboardProps = {
      rentReceivedSummaryQueryResult,
      rentReceivedStatusQueryResult,
      editReceipt: handleEditReceipt,
      showRentalContract: handleShowRentalContractClick,
      handleMarkRentAsPaid,
      loading: isLoadingTransactionCreate || isLoadingTransactionDelete ||
        isLoadingRentReceiptCreate,
      displayOnboarding,
    };

    return WrappedComponent(componentProps);
  };
