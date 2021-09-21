import { MutationHookOptions } from "@apollo/client";
import moment from "moment";
import { RecentActivityListQuery } from ".";
import {
  ContractRequirementDataDocument as ContractRequirementDataQuery,
  InvoiceListDocument as InvoiceListQuery,
  LeaseListDocument as LeaseListQuery,
  LenderDocument as LenderQuery,
  PropertyListDocument as PropertyListQuery,
  RentListDocument as RentListQuery,
  RentReceivedStatusDocument as RentReceivedStatusQuery,
  RentReceivedSummaryDocument as RentReceivedSummaryQuery,
  TenantInput,
  TenantListDocument as TenantListQuery,
  TenantStatus,
} from "../types";
import { DATE_ISO_FORMAT } from "../validators";

// # Queries

export {
  AccountActivatePlanDocument as AccountActivatePlanMutation,
  AccountUpdatePaymentMethodDocument as AccountUpdatePaymentMethodMutation,
  ContractDeleteDocument as ContractDeleteMutation,
  ContractRequirementDataDocument as ContractRequirementDataQuery,
  ContractUpdateDocument as ContractUpdateMutation,
  FileDocument as FileListQuery,
  FileUploadDocument as FileUploadMutation,
  ImportUploadDocument as ImportUploadMutation,
  InvoiceListDocument as InvoiceListQuery,
  LeaseCreateDocument as LeaseCreateMutation,
  LeaseDeleteDocument as LeaseDeleteMutation,
  LeaseDocument as LeaseQuery,
  LeaseListDocument as LeaseListQuery,
  LenderDocument as LenderQuery,
  LenderIndividualUpdateDocument as LenderIndividualUpdateMutation,
  PricingPlansDocument as PricingPlansQuery,
  PropertyCreateDocument as PropertyCreateMutation,
  PropertyDeleteDocument as PropertyDeleteMutation,
  PropertyListDocument as PropertyListQuery,
  PropertyUpdateDocument as PropertyUpdateMutation,
  RecentActivityListDocument as RecentActivityListQuery,
  RentalReceiptDataDocument as RentalReceiptDataQuery,
  RentListDocument as RentListQuery,
  RentReceiptCreateDocument as RentReceiptCreateMutation,
  RentReceivedStatusDocument as RentReceivedStatusQuery,
  RentReceivedSummaryDocument as RentReceivedSummaryQuery,
  SendPaymentNoticeDocument as SendPaymentNoticeMutation,
  TenantCreateDocument as TenantCreateMutation,
  TenantDeleteDocument as TenantDeleteMutation,
  TenantListDocument as TenantListQuery,
  TenantUpdateDocument as TenantUpdateMutation,
  TenantWithRentalReceiptsDocument as TenantWithRentalReceiptsQuery,
  TransactionCreateDocument as TransactionCreateMutation,
  TransactionDeleteDocument as TransactionDeleteMutation,
  TransactionDocument as TransactionQuery,
  UserCreateWithAccountDocument as AccountCreateMutation,
  UserDocument as UserQuery,
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
