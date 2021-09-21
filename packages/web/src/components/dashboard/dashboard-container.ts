import { useQuery } from "@apollo/client";
import moment from "moment";
import React, { useContext } from "react";
import {
  LeaseListQuery,
  RecentActivityListQuery,
  RentReceivedSummaryQuery,
} from "../../helpers";
import { useRouter } from "../../hooks/use-router";
import { Rent, RentStatus } from "../../types";
import { DATE_ISO_FORMAT } from "../../validators";
import {
  RentManagerContext,
  RentManagerContextAction,
} from "../rent-manager/rent-manager-context";
import { RentManagerMultiActionsMenuType } from "../rent-manager/rent-manager-multi-actions-menu";
import { RentManagerSingleActionsMenuType } from "../rent-manager/rent-manager-single-actions-menu";
import { DashboardProps } from "./dashboard";

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const {
      dispatch,
      loading,
      rentList,
      groupedRentList,
      selectedRentList,
      selectedRentStatus,
      allRentsSelected,
    } = useContext(RentManagerContext);

    const router = useRouter();

    // Lease query
    const leaseListResult = useQuery(LeaseListQuery);
    const {
      data: { rentReceivedSummary } = { rentReceivedSummary: null },
      loading: isLoadingSummary,
    } = useQuery(
      RentReceivedSummaryQuery,
      {
        variables: {
          until: moment().add(1, "month").startOf("month").format(
            DATE_ISO_FORMAT,
          ),
          since: moment().startOf("month").format(DATE_ISO_FORMAT),
        },
      },
    );

    // Recent activities query
    const {
      data: { events } = { events: [] },
      loading: isLoadingRecentActivityList,
    } = useQuery(
      RecentActivityListQuery,
    );

    const onSelectAllTenantsClick = (selected: boolean) => {
      dispatch({
        type: RentManagerContextAction.AllRentsSelected,
        payload: selected,
      });
    };

    const onSelectRentClick = (rent: Rent, selected: boolean) => {
      dispatch({
        type: RentManagerContextAction.SelectRent,
        payload: rent,
        selected,
      });
    };

    const onSelectRentSingleAction = (
      type: RentManagerSingleActionsMenuType,
      rent: Rent,
    ) => {
      switch (type) {
        case RentManagerSingleActionsMenuType.MarkAsPaid:
          router.showRentReceiptPreview(rent?.id);
          break;
        case RentManagerSingleActionsMenuType.Remind:
          router.showPaymentNoticePreview(rent?.id);
          break;
        case RentManagerSingleActionsMenuType.ShowTenant:
          router.showTenantSynthesisRoute(rent?.lease?.tenants?.[0]?.id);
          break;
      }
    };

    const onSelectRentMultiAction = (
      type: RentManagerMultiActionsMenuType,
    ) => {
      switch (type) {
        case RentManagerMultiActionsMenuType.MarkAllAsPaid:
          router.showRentReceiptSendAllConfirmation();
          break;
      }
    };

    const onChangeRentStatus = (status: RentStatus) => {
      dispatch({
        type: RentManagerContextAction.SelectedRentStatus,
        payload: status,
      });
    };

    const displayOnboarding = leaseListResult?.data?.leases?.length === 0;
    const isLoading = loading || isLoadingSummary ||
      isLoadingRecentActivityList;

    const componentProps: DashboardProps = {
      loading: isLoading,
      displayOnboarding,
      rentManagerData: {
        loading: isLoading,
        rentList,
        groupedRentList,
        selectedRentList,
        selectedRentStatus,
        summaryData: {
          loading: false,
          amountReceived: rentReceivedSummary?.amountReceived,
          amountPending: rentReceivedSummary?.amountPending,
          variationReceived: rentReceivedSummary?.variationReceived,
          paymentRate: rentReceivedSummary?.paymentRate,
          occupationRate: rentReceivedSummary?.occupationRate,
          nPartial: rentReceivedSummary?.nPartial,
          nReceived: rentReceivedSummary?.nReceived,
          nExpected: rentReceivedSummary?.nExpected,
        },
        collectedBarData: {
          amountReceived: rentReceivedSummary?.amountReceived,
          amountPending: rentReceivedSummary?.amountPending,
          amountPartial: rentReceivedSummary?.amountPartial,
          ratioReceived: rentReceivedSummary?.ratioReceived,
          ratioPending: rentReceivedSummary?.ratioPending,
          ratioPartial: rentReceivedSummary?.ratioPartial,
        },
        onSelectAllTenantsClick,
        onSelectRentClick,
        onChangeRentStatus,
        allRentsSelected,
        onSelectRentSingleAction,
        onSelectRentMultiAction,
      },
      reventActivityList: events,
    };

    return WrappedComponent(componentProps);
  };
