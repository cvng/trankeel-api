import { MutationHookOptions } from "@apollo/client";
import moment from "moment";
import { RecentActivityListQuery } from ".";
import {
  ContractRequirementData as ContractRequirementDataQuery,
  InvoiceList as InvoiceListQuery,
  LeaseList as LeaseListQuery,
  Lender as LenderQuery,
  PropertyList as PropertyListQuery,
  RentList as RentListQuery,
  RentReceivedStatus as RentReceivedStatusQuery,
  RentReceivedSummary as RentReceivedSummaryQuery,
  TenantInput,
  TenantList as TenantListQuery,
  TenantStatus,
} from "../types";
import { DATE_ISO_FORMAT } from "../validators";

// # Queries

export {
  AccountActivatePlan as AccountActivatePlanMutation,
  AccountUpdatePaymentMethod as AccountUpdatePaymentMethodMutation,
  ContractDelete as ContractDeleteMutation,
  ContractRequirementData as ContractRequirementDataQuery,
  ContractUpdate as ContractUpdateMutation,
  File as FileListQuery,
  FileUpload as FileUploadMutation,
  ImportUpload as ImportUploadMutation,
  InvoiceList as InvoiceListQuery,
  LeaseCreate as LeaseCreateMutation,
  LeaseDelete as LeaseDeleteMutation,
  Lease as LeaseQuery,
  LeaseList as LeaseListQuery,
  Lender as LenderQuery,
  LenderIndividualUpdate as LenderIndividualUpdateMutation,
  PricingPlans as PricingPlansQuery,
  PropertyCreate as PropertyCreateMutation,
  PropertyDelete as PropertyDeleteMutation,
  PropertyList as PropertyListQuery,
  PropertyUpdate as PropertyUpdateMutation,
  RecentActivityList as RecentActivityListQuery,
  RentalReceiptData as RentalReceiptDataQuery,
  RentList as RentListQuery,
  RentReceiptCreate as RentReceiptCreateMutation,
  RentReceivedStatus as RentReceivedStatusQuery,
  RentReceivedSummary as RentReceivedSummaryQuery,
  SendPaymentNotice as SendPaymentNoticeMutation,
  TenantCreate as TenantCreateMutation,
  TenantDelete as TenantDeleteMutation,
  TenantList as TenantListQuery,
  TenantUpdate as TenantUpdateMutation,
  TenantWithRentalReceipts as TenantWithRentalReceiptsQuery,
  TransactionCreate as TransactionCreateMutation,
  TransactionDelete as TransactionDeleteMutation,
  Transaction as TransactionQuery,
  UserCreateWithAccount as AccountCreateMutation,
  User as UserQuery,
} from "../types";

// Default since date is the first day of the current month
const SINCE_DEFAULT = moment().startOf("month").format(DATE_ISO_FORMAT);
// Default until date is the last day of the current month
const UNTIL_DEFAULT = moment().add(1, "month").startOf("month").format(
  DATE_ISO_FORMAT,
);

const RentListDefaultQuery = {
  query: RentListQuery,
  variables: {
    until: UNTIL_DEFAULT,
    since: SINCE_DEFAULT,
  },
};

const RentReceivedSummaryDefaultQuery = {
  query: RentReceivedSummaryQuery,
  variables: {
    until: UNTIL_DEFAULT,
    since: SINCE_DEFAULT,
  },
};

// # Mutations Options

export const AccountActivatePlanMutationOptions = (): MutationHookOptions => ({
  refetchQueries: [{ query: InvoiceListQuery }],
});

export const TenantCreateMutationOptions = (): MutationHookOptions => ({
  refetchQueries: [
    { query: TenantListQuery },
    { query: ContractRequirementDataQuery },
  ],
});

export const TenantUpdateMutationOptions = (): MutationHookOptions => ({
  refetchQueries: [
    { query: TenantListQuery },
    { query: RentReceivedStatusQuery },
  ],
});

export const TenantDeleteMutationOptions = (): MutationHookOptions => ({
  refetchQueries: [{ query: TenantListQuery }],
});

export const PropertyCreateMutationOptions = (): MutationHookOptions => ({
  refetchQueries: [
    { query: PropertyListQuery },
    { query: ContractRequirementDataQuery },
  ],
});

export const PropertyUpdateMutationOptions = (): MutationHookOptions => ({
  refetchQueries: [
    { query: TenantListQuery },
    { query: PropertyListQuery },
    { query: RentReceivedStatusQuery },
  ],
});

export const LeaseCreateMutationOptions = (): MutationHookOptions => ({
  refetchQueries: [
    { query: PropertyListQuery },
    { query: LeaseListQuery },
    RentReceivedSummaryDefaultQuery,
  ],
});

export const LeaseDeleteMutationOptions = (): MutationHookOptions => ({
  refetchQueries: [
    { query: PropertyListQuery },
    { query: LeaseListQuery },
    { query: ContractRequirementDataQuery },
    RentReceivedSummaryDefaultQuery,
  ],
});

export const ContractDeleteMutationOptions = (): MutationHookOptions => ({
  refetchQueries: [
    { query: LeaseListQuery },
    { query: ContractRequirementDataQuery },
  ],
});

export const ContractUpdateMutationOptions = (): MutationHookOptions => ({
  refetchQueries: [
    { query: LeaseListQuery },
    { query: ContractRequirementDataQuery },
  ],
});

export const TransactionCreateMutationOptions = (): MutationHookOptions => ({
  refetchQueries: [
    { query: RentReceivedStatusQuery },
    { query: TenantListQuery },
    RentReceivedSummaryDefaultQuery,
    {
      query: RecentActivityListQuery,
    },
  ],
});

export const TransactionDeleteMutationOptions = (): MutationHookOptions => ({
  refetchQueries: [
    { query: RentReceivedStatusQuery },
    RentReceivedSummaryDefaultQuery,
  ],
});

export const ImportUploadMutationOptions = (): MutationHookOptions => ({
  refetchQueries: [
    { query: PropertyListQuery },
    { query: TenantListQuery },
    { query: LeaseListQuery },
  ],
});

export const LenderIndividualUpdateMutationOptions =
  (): MutationHookOptions => ({
    refetchQueries: [
      { query: LenderQuery },
      { query: RentReceivedStatusQuery },
    ],
  });

export const PropertyDeleteMutationOptions = (): MutationHookOptions => ({
  refetchQueries: [{ query: PropertyListQuery }],
});

export const RentReceiptCreateMutationOptions = (): MutationHookOptions => ({
  refetchQueries: [RentReceivedSummaryDefaultQuery, RentListDefaultQuery, {
    query: TenantListQuery,
  }, {
    query: RecentActivityListQuery,
  }],
});

export const AllRentReceiptsCreateMutationOptions =
  (): MutationHookOptions => ({
    refetchQueries: [RentReceivedSummaryDefaultQuery, {
      query: TenantListQuery,
    }],
  });

export const SendPaymentNoticeMutationOptions = (): MutationHookOptions => ({
  refetchQueries: [{
    query: RecentActivityListQuery,
  }],
});

// # Optimistic Responses

export const TenantCreateMutationOptimisticResponse = (
  values: TenantInput,
) => ({
  tenantCreate: {
    id: "-1", // Placeholder ID
    fullName: [values.firstName, values.lastName?.toUpperCase()].join(" "),
    createdAt: new Date().getTime(),
    status: TenantStatus.New,
    lastTransaction: null,
    rentPayedThisYear: null,
    unpaidRentAmount: null,
    propertyName: null,
    arrivalDate: null,
    departureDate: null,
    ...values,
    __typename: "Tenant",
  },
});
