import { ThemeProvider } from "evergreen-ui";
import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { AppTheme } from "../../theme/app-theme";
import { RentStatus } from "../../types";
import { RentManager } from "./rent-manager";

const rentList = FactoryHelper.rentList();

export default {
  title: "Rent/RentManager",
  component: RentManager,
  argTypes: {
    loading: false,
    allRentsSelected: true,
    tenants: rentList,
    selectedTenantList: [rentList[0]],
    selectedRentStatus: RentStatus.Pending,
  },
};

export const loading = () => (
  <ThemeProvider value={AppTheme}>
    <RentManager loading allRentsSelected={true} />
  </ThemeProvider>
);

export const noData = () => (
  <ThemeProvider value={AppTheme}>
    <RentManager
      loading={false}
      allRentsSelected={false}
      rentList={[]}
      selectedRentList={[]}
      selectedRentStatus={RentStatus.Pending}
    />
  </ThemeProvider>
);

export const allRentsSelected = () => (
  <ThemeProvider value={AppTheme}>
    <RentManager
      loading={false}
      allRentsSelected={true}
      rentList={rentList}
      selectedRentList={[rentList[0]]}
      selectedRentStatus={RentStatus.Pending}
      summaryData={{
        amountReceived: 2923,
        variationReceived: 3.4,
        paymentRate: 67,
        occupationRate: 89,
      }}
      collectedBarData={{
        amountReceived: 2923,
        amountPartial: 177,
        amountPending: 1180,
        ratioReceived: 0,
        ratioPending: 0,
        ratioPartial: 0,
      }}
    />
  </ThemeProvider>
);

export const allRentsNotSelected = () => (
  <ThemeProvider value={AppTheme}>
    <RentManager
      loading={false}
      allRentsSelected={false}
      rentList={rentList}
      selectedRentStatus={RentStatus.Pending}
      summaryData={{
        amountReceived: 2923,
        variationReceived: 3.4,
        paymentRate: 67,
        occupationRate: 89,
      }}
      collectedBarData={{
        amountReceived: 2923,
        amountPartial: 177,
        amountPending: 1180,
        ratioReceived: 0,
        ratioPending: 0,
        ratioPartial: 0,
      }}
    />
  </ThemeProvider>
);

export const filterEnabled = () => (
  <ThemeProvider value={AppTheme}>
    <RentManager
      loading={false}
      allRentsSelected={false}
      rentList={rentList}
      selectedRentStatus={RentStatus.Pending}
      summaryData={{
        amountReceived: 2923,
        variationReceived: 3.4,
        paymentRate: 45,
        occupationRate: 89,
      }}
      collectedBarData={{
        amountReceived: 2923,
        amountPartial: 177,
        amountPending: 1180,
        ratioReceived: 0,
        ratioPending: 0,
        ratioPartial: 0,
      }}
    />
  </ThemeProvider>
);
