import { TypedDocumentNode as DocumentNode } from "@graphql-typed-document-node/core";
export type Maybe<T> = T | null;
export type Exact<T extends { [key: string]: unknown }> = {
  [K in keyof T]: T[K];
};
export type MakeOptional<T, K extends keyof T> = Omit<T, K> &
  { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> &
  { [SubKey in K]: Maybe<T[SubKey]> };
/** All built-in and custom scalars, mapped to their actual values */
export interface Scalars {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
  AuthenticationID: string;
  Date: string;
  DateTime: string;
  Decimal: number;
  Email: string;
  PhoneNumber: string;
}

export interface Account {
  __typename?: "Account";
  planId?: Maybe<Scalars["ID"]>;
  status?: Maybe<SubscriptionStatus>;
  stripeCustomerId?: Maybe<Scalars["String"]>;
  stripeSubscriptionId?: Maybe<Scalars["String"]>;
  trialEnd?: Maybe<Scalars["DateTime"]>;
  ownerId?: Maybe<Scalars["String"]>;
  id: Scalars["ID"];
  plan?: Maybe<Plan>;
}

export interface AccountActivatePlanInput {
  id: Scalars["ID"];
  name: Scalars["String"];
  planCode: PlanCode;
}

export interface AccountUpdateInput {
  id: Scalars["ID"];
  paymentMethodId: Scalars["ID"];
}

export interface Address {
  __typename?: "Address";
  city: Scalars["String"];
  country?: Maybe<Scalars["String"]>;
  line1: Scalars["String"];
  line2?: Maybe<Scalars["String"]>;
  postalCode: Scalars["String"];
  inline: Scalars["String"];
}

export interface AddressInput {
  city: Scalars["String"];
  country?: Maybe<Scalars["String"]>;
  line1: Scalars["String"];
  line2?: Maybe<Scalars["String"]>;
  postalCode: Scalars["String"];
}

export interface Company {
  __typename?: "Company";
  address?: Maybe<Address>;
  displayName: Scalars["String"];
  email: Scalars["Email"];
  id: Scalars["ID"];
  legalEntity: Scalars["String"];
  legalEntityIdentifier?: Maybe<Scalars["String"]>;
  legalEntityType?: Maybe<LegalEntityType>;
  legalEntityTypeOther?: Maybe<Scalars["String"]>;
  phoneNumber?: Maybe<Scalars["PhoneNumber"]>;
}

export interface Feature {
  __typename?: "Feature";
  available: Scalars["Boolean"];
  title: Scalars["String"];
  key?: Maybe<Scalars["String"]>;
}

export interface File {
  __typename?: "File";
  createdAt: Scalars["DateTime"];
  downloadUrl?: Maybe<Scalars["String"]>;
  externalId?: Maybe<Scalars["String"]>;
  filename?: Maybe<Scalars["String"]>;
  previewUrl?: Maybe<Scalars["String"]>;
  status?: Maybe<FileStatus>;
  type: FileType;
  updatedAt?: Maybe<Scalars["DateTime"]>;
  id: Scalars["ID"];
}

export interface FileInput {
  downloadUrl: Scalars["String"];
  type: FileType;
}

/** https://www.pdfmonkey.io/fr/doc/api/generer-un-document */
export enum FileStatus {
  Draft = "DRAFT",
  Failure = "FAILURE",
  Generating = "GENERATING",
  Pending = "PENDING",
  Success = "SUCCESS",
}

export enum FileType {
  PaymentNotice = "PAYMENT_NOTICE",
  LeaseDocument = "LEASE_DOCUMENT",
  RentReceipt = "RENT_RECEIPT",
}

export type Identity = User | Company;

export interface ImportInput {
  files: Array<FileInput>;
  source: ImportSource;
}

export enum ImportSource {
  Rentila = "RENTILA",
}

export interface Invoice {
  __typename?: "Invoice";
  id: Scalars["ID"];
  number: Scalars["Int"];
  amountPaid: Scalars["Decimal"];
  invoicePdf: Scalars["String"];
  periodEnd: Scalars["DateTime"];
  status: Scalars["String"];
  planCode: PlanCode;
}

export interface Lease {
  __typename?: "Lease";
  accountId: Scalars["ID"];
  depositAmount?: Maybe<Scalars["Decimal"]>;
  effectDate: Scalars["DateTime"];
  signatureDate?: Maybe<Scalars["DateTime"]>;
  rentAmount: Scalars["Decimal"];
  rentChargesAmount?: Maybe<Scalars["Decimal"]>;
  rentFullAmount: Scalars["Decimal"];
  type: LeaseType;
  leaseId?: Maybe<Scalars["ID"]>;
  propertyId: Scalars["ID"];
  id: Scalars["ID"];
  data?: Maybe<LeaseFurnishedData>;
  expiredAt?: Maybe<Scalars["DateTime"]>;
  renewDate?: Maybe<Scalars["DateTime"]>;
  status: LeaseStatus;
  lease?: Maybe<File>;
  rents?: Maybe<Array<Rent>>;
  tenants: Array<Tenant>;
  account?: Maybe<Account>;
  property?: Maybe<Property>;
}

export interface LeaseFurnishedData {
  __typename?: "LeaseFurnishedData";
  chargesRecuperationMode?: Maybe<RentChargesRecuperationMode>;
  chargesRevisionMethod?: Maybe<Scalars["String"]>;
  colocationInsuranceLender?: Maybe<Scalars["Boolean"]>;
  colocationInsuranceMonthlyAmount?: Maybe<Scalars["Decimal"]>;
  colocationInsuranceTotalAmount?: Maybe<Scalars["Decimal"]>;
  duration?: Maybe<LeaseFurnishedDuration>;
  lenderFeeCap?: Maybe<Scalars["Decimal"]>;
  lenderFeeCapOther?: Maybe<Scalars["String"]>;
  lenderFeeCapPrestations?: Maybe<Scalars["Decimal"]>;
  otherConditions?: Maybe<Scalars["String"]>;
  rentComplement?: Maybe<Scalars["Decimal"]>;
  rentComplementPropertyJustification?: Maybe<Scalars["String"]>;
  rentFirstAmount?: Maybe<Scalars["Decimal"]>;
  rentIrl?: Maybe<LeaseRentReferenceIrl>;
  rentIrlRevisionDate?: Maybe<Scalars["String"]>;
  rentMajDecreeIncreasedAmount?: Maybe<Scalars["Decimal"]>;
  rentMajDecreeReferenceAmount?: Maybe<Scalars["Decimal"]>;
  rentMajorationDecree?: Maybe<Scalars["Boolean"]>;
  rentMaxEvolutionRelocation?: Maybe<Scalars["Boolean"]>;
  rentPaymentDate?: Maybe<Scalars["DateTime"]>;
  rentPaymentMethod?: Maybe<RentPaymentMethod>;
  rentPaymentPlace?: Maybe<Scalars["String"]>;
  rentPeriodicity?: Maybe<LeaseRentPeriodicity>;
  rentUnderestimatedMethod?: Maybe<Scalars["String"]>;
  rentUnderestimatedMonthlyVariation?: Maybe<Scalars["Decimal"]>;
  resolutaryClause?: Maybe<Scalars["String"]>;
  solidarityClause?: Maybe<Scalars["String"]>;
  tenantFeeCapNewRental?: Maybe<Scalars["Decimal"]>;
  tenantFeeCapPrestations?: Maybe<Scalars["Decimal"]>;
  tenantFeeCapReportByMeter?: Maybe<Scalars["Decimal"]>;
  tenantFeeCapReportPrestations?: Maybe<Scalars["Decimal"]>;
  tenantLastRentAmount?: Maybe<Scalars["Decimal"]>;
  tenantLastRentReceivedDate?: Maybe<Scalars["String"]>;
  tenantLastRentRevisionDate?: Maybe<Scalars["String"]>;
  worksDecenceSinceLastRental?: Maybe<Scalars["String"]>;
  worksRentDecreaseTenant?: Maybe<Scalars["String"]>;
  worksRentIncreaseLender?: Maybe<Scalars["String"]>;
}

export interface LeaseFurnishedDataInput {
  chargesRecuperationMode?: Maybe<RentChargesRecuperationMode>;
  chargesRevisionMethod?: Maybe<Scalars["String"]>;
  colocationInsuranceLender?: Maybe<Scalars["Boolean"]>;
  colocationInsuranceMonthlyAmount?: Maybe<Scalars["Decimal"]>;
  colocationInsuranceTotalAmount?: Maybe<Scalars["Decimal"]>;
  duration?: Maybe<LeaseFurnishedDuration>;
  lenderFeeCap?: Maybe<Scalars["Decimal"]>;
  lenderFeeCapOther?: Maybe<Scalars["String"]>;
  lenderFeeCapPrestations?: Maybe<Scalars["Decimal"]>;
  otherConditions?: Maybe<Scalars["String"]>;
  rentComplement?: Maybe<Scalars["Decimal"]>;
  rentComplementPropertyJustification?: Maybe<Scalars["String"]>;
  rentFirstAmount?: Maybe<Scalars["Decimal"]>;
  rentIrl?: Maybe<LeaseRentReferenceIrl>;
  rentIrlRevisionDate?: Maybe<Scalars["Date"]>;
  rentMajDecreeIncreasedAmount?: Maybe<Scalars["Decimal"]>;
  rentMajDecreeReferenceAmount?: Maybe<Scalars["Decimal"]>;
  rentMajorationDecree?: Maybe<Scalars["Boolean"]>;
  rentMaxEvolutionRelocation?: Maybe<Scalars["Boolean"]>;
  rentPaymentDate?: Maybe<Scalars["Date"]>;
  rentPaymentMethod?: Maybe<RentPaymentMethod>;
  rentPaymentPlace?: Maybe<Scalars["String"]>;
  rentPeriodicity?: Maybe<LeaseRentPeriodicity>;
  rentUnderestimatedMethod?: Maybe<Scalars["String"]>;
  rentUnderestimatedMonthlyVariation?: Maybe<Scalars["Decimal"]>;
  resolutaryClause?: Maybe<Scalars["String"]>;
  solidarityClause?: Maybe<Scalars["String"]>;
  tenantFeeCapNewRental?: Maybe<Scalars["Decimal"]>;
  tenantFeeCapPrestations?: Maybe<Scalars["Decimal"]>;
  tenantFeeCapReportByMeter?: Maybe<Scalars["Decimal"]>;
  tenantFeeCapReportPrestations?: Maybe<Scalars["Decimal"]>;
  tenantLastRentAmount?: Maybe<Scalars["Decimal"]>;
  tenantLastRentReceivedDate?: Maybe<Scalars["Date"]>;
  tenantLastRentRevisionDate?: Maybe<Scalars["Date"]>;
  worksDecenceSinceLastRental?: Maybe<Scalars["String"]>;
  worksRentDecreaseTenant?: Maybe<Scalars["String"]>;
  worksRentIncreaseLender?: Maybe<Scalars["String"]>;
}

export enum LeaseFurnishedDuration {
  NineMonths = "NINE_MONTHS",
  OneYear = "ONE_YEAR",
}

export interface LeaseFurnishedInput {
  data?: Maybe<LeaseFurnishedDataInput>;
  depositAmount?: Maybe<Scalars["Decimal"]>;
  effectDate: Scalars["Date"];
  renewDate?: Maybe<Scalars["Date"]>;
  file?: Maybe<FileInput>;
  propertyId: Scalars["ID"];
  rentAmount: Scalars["Decimal"];
  rentChargesAmount?: Maybe<Scalars["Decimal"]>;
  signatureDate?: Maybe<Scalars["Date"]>;
  tenantIds: Array<Scalars["ID"]>;
}

export interface LeaseFurnishedUpdateInput {
  data?: Maybe<LeaseFurnishedDataInput>;
  file?: Maybe<FileInput>;
  id: Scalars["ID"];
}

export enum LeaseRentPeriodicity {
  Annualy = "ANNUALY",
  Monthly = "MONTHLY",
}

/** https://www.service-public.fr/particuliers/vosdroits/F13723 */
export enum LeaseRentReferenceIrl {
  AprilFirstSemesterY2021 = "APRIL_FIRST_SEMESTER_Y2021",
}

export enum LeaseStatus {
  Active = "ACTIVE",
  Ended = "ENDED",
}

export enum LeaseType {
  Furnished = "FURNISHED",
  Naked = "NAKED",
}

export enum LegalEntityType {
  Eurl = "EURL",
  Other = "OTHER",
  Sa = "SA",
  Sarl = "SARL",
  Sas = "SAS",
  Sasu = "SASU",
  Sci = "SCI",
  Scp = "SCP",
  Snc = "SNC",
}

export interface Lender {
  __typename?: "Lender";
  id: Scalars["ID"];
  accountId?: Maybe<Scalars["ID"]>;
  individualId?: Maybe<Scalars["ID"]>;
  companyId?: Maybe<Scalars["ID"]>;
  displayName: Scalars["String"];
  identity?: Maybe<Identity>;
}

export interface LenderIndividualUpdateInput {
  id: Scalars["ID"];
  individual: UserUpdateInput;
}

export interface Mutation {
  __typename?: "Mutation";
  userCreateWithAccount: Account;
  accountUpdatePaymentMethod: Account;
  accountActivatePlan: Account;
  tenantCreate: Tenant;
  tenantUpdate: Tenant;
  tenantDelete: Scalars["ID"];
  propertyCreate: Property;
  propertyUpdate: Property;
  propertyDelete: Scalars["ID"];
  leaseFurnishedCreate: Lease;
  leaseDelete: Scalars["ID"];
  leaseFurnishedUpdate: Lease;
  lenderIndividualUpdate: Lender;
  transactionCreate: Transaction;
  transactionDelete: Scalars["ID"];
  fileUpload: File;
  importUpload: Task;
  rentReceiptCreate: RentReceiptPayload;
}

export interface MutationUserCreateWithAccountArgs {
  input: UserWithAccountInput;
}

export interface MutationAccountUpdatePaymentMethodArgs {
  input: AccountUpdateInput;
}

export interface MutationAccountActivatePlanArgs {
  input: AccountActivatePlanInput;
}

export interface MutationTenantCreateArgs {
  input: TenantInput;
}

export interface MutationTenantUpdateArgs {
  input: TenantUpdateInput;
}

export interface MutationTenantDeleteArgs {
  id: Scalars["ID"];
}

export interface MutationPropertyCreateArgs {
  input: PropertyInput;
}

export interface MutationPropertyUpdateArgs {
  input: PropertyUpdateInput;
}

export interface MutationPropertyDeleteArgs {
  id: Scalars["ID"];
}

export interface MutationLeaseFurnishedCreateArgs {
  input: LeaseFurnishedInput;
}

export interface MutationLeaseDeleteArgs {
  id: Scalars["ID"];
}

export interface MutationLeaseFurnishedUpdateArgs {
  input: LeaseFurnishedUpdateInput;
}

export interface MutationLenderIndividualUpdateArgs {
  input: LenderIndividualUpdateInput;
}

export interface MutationTransactionCreateArgs {
  input: TransactionInput;
}

export interface MutationTransactionDeleteArgs {
  id: Scalars["ID"];
}

export interface MutationFileUploadArgs {
  input: FileInput;
}

export interface MutationImportUploadArgs {
  input: ImportInput;
}

export interface MutationRentReceiptCreateArgs {
  input: RentReceiptInput;
}

export interface Plan {
  __typename?: "Plan";
  code: PlanCode;
  price?: Maybe<Scalars["Decimal"]>;
  subtitle?: Maybe<Scalars["String"]>;
  title?: Maybe<Scalars["String"]>;
  id: Scalars["ID"];
  features: Array<Feature>;
}

export enum PlanCode {
  Solo = "SOLO",
}

export interface Property {
  __typename?: "Property";
  account?: Maybe<Account>;
  accountId?: Maybe<Scalars["ID"]>;
  address: Address;
  buildPeriod?: Maybe<PropertyBuildPeriodType>;
  buildingLegalStatus?: Maybe<PropertyBuildingLegalStatus>;
  commonSpaces?: Maybe<Scalars["String"]>;
  energyClass?: Maybe<PropertyEnergyClass>;
  equipments?: Maybe<Scalars["String"]>;
  gasEmission?: Maybe<PropertyGasEmission>;
  heatingMethod?: Maybe<PropertyUsageType>;
  housingType?: Maybe<PropertyUsageType>;
  name: Scalars["String"];
  note?: Maybe<Scalars["String"]>;
  nticEquipments?: Maybe<Scalars["String"]>;
  otherSpaces?: Maybe<Scalars["String"]>;
  tax?: Maybe<Scalars["Float"]>;
  roomCount: PropertyRoomType;
  status?: Maybe<PropertyStatus>;
  surface: Scalars["Int"];
  tenantPrivateSpaces?: Maybe<Scalars["String"]>;
  usageType?: Maybe<PropertyHabitationUsageType>;
  waterHeatingMethod?: Maybe<PropertyUsageType>;
  id: Scalars["ID"];
  lenderId?: Maybe<Scalars["ID"]>;
  expectedRents?: Maybe<Scalars["Decimal"]>;
  collectedRents?: Maybe<Scalars["Decimal"]>;
  lender?: Maybe<Lender>;
  leases?: Maybe<Array<Lease>>;
}

export enum PropertyBuildPeriodType {
  BeforeY1949 = "BEFORE_Y1949",
  FromY1949Y1974 = "FROM_Y1949Y1974",
  FromY1975Y1989 = "FROM_Y1975Y1989",
  FromY1990Y2005 = "FROM_Y1990Y2005",
  FromY2005 = "FROM_Y2005",
}

export enum PropertyBuildingLegalStatus {
  Copro = "COPRO",
  Mono = "MONO",
}

export enum PropertyEnergyClass {
  A = "A",
  B = "B",
  C = "C",
  D = "D",
  E = "E",
  F = "F",
  G = "G",
}

export enum PropertyGasEmission {
  A = "A",
  B = "B",
  C = "C",
  D = "D",
  E = "E",
  F = "F",
  G = "G",
}

export enum PropertyHabitationUsageType {
  Habitation = "HABITATION",
  Mixte = "MIXTE",
}

export interface PropertyInput {
  address: AddressInput;
  buildPeriod: PropertyBuildPeriodType;
  buildingLegalStatus: PropertyBuildingLegalStatus;
  commonSpaces?: Maybe<Scalars["String"]>;
  energyClass?: Maybe<PropertyEnergyClass>;
  equipments?: Maybe<Scalars["String"]>;
  gasEmission?: Maybe<PropertyGasEmission>;
  heatingMethod: PropertyUsageType;
  housingType: PropertyUsageType;
  lenderId: Scalars["ID"];
  name: Scalars["String"];
  note?: Maybe<Scalars["String"]>;
  nticEquipments?: Maybe<Scalars["String"]>;
  otherSpaces?: Maybe<Scalars["String"]>;
  roomCount: PropertyRoomType;
  status?: Maybe<PropertyStatus>;
  surface: Scalars["Float"];
  tax?: Maybe<Scalars["Decimal"]>;
  tenantPrivateSpaces?: Maybe<Scalars["String"]>;
  usageType: PropertyHabitationUsageType;
  waterHeatingMethod: PropertyUsageType;
}

export enum PropertyRoomType {
  Other = "OTHER",
  T1 = "T1",
  T2 = "T2",
  T3 = "T3",
  T4 = "T4",
  T5 = "T5",
  T6 = "T6",
}

export enum PropertyStatus {
  ForSale = "FOR_SALE",
  Inactive = "INACTIVE",
  Rented = "RENTED",
  UnderConstruction = "UNDER_CONSTRUCTION",
  Unrented = "UNRENTED",
}

export interface PropertyUpdateInput {
  address?: Maybe<AddressInput>;
  buildPeriod?: Maybe<PropertyBuildPeriodType>;
  buildingLegalStatus?: Maybe<PropertyBuildingLegalStatus>;
  commonSpaces?: Maybe<Scalars["String"]>;
  energyClass?: Maybe<PropertyEnergyClass>;
  equipments?: Maybe<Scalars["String"]>;
  gasEmission?: Maybe<PropertyGasEmission>;
  heatingMethod?: Maybe<PropertyUsageType>;
  housingType?: Maybe<PropertyUsageType>;
  id: Scalars["ID"];
  name?: Maybe<Scalars["String"]>;
  note?: Maybe<Scalars["String"]>;
  nticEquipments?: Maybe<Scalars["String"]>;
  otherSpaces?: Maybe<Scalars["String"]>;
  roomCount?: Maybe<PropertyRoomType>;
  status?: Maybe<PropertyStatus>;
  surface?: Maybe<Scalars["Float"]>;
  tax?: Maybe<Scalars["Decimal"]>;
  tenantPrivateSpaces?: Maybe<Scalars["String"]>;
  usageType?: Maybe<PropertyHabitationUsageType>;
  waterHeatingMethod?: Maybe<PropertyUsageType>;
}

export enum PropertyUsageType {
  Collective = "COLLECTIVE",
  Individual = "INDIVIDUAL",
}

export interface Query {
  __typename?: "Query";
  viewer: User;
  properties: Array<Property>;
  summary: Summary;
  tenants: Array<Tenant>;
  leases: Array<Lease>;
  lenders: Array<Lender>;
  transactions: Array<Transaction>;
  invoices: Array<Invoice>;
  plans: Array<Plan>;
  files: Array<File>;
}

export interface QueryPropertiesArgs {
  id?: Maybe<Scalars["ID"]>;
  query?: Maybe<Scalars["String"]>;
}

export interface QuerySummaryArgs {
  since?: Maybe<Scalars["DateTime"]>;
  until?: Maybe<Scalars["DateTime"]>;
}

export interface QueryTenantsArgs {
  id?: Maybe<Scalars["ID"]>;
  query?: Maybe<Scalars["String"]>;
  status?: Maybe<TenantStatus>;
}

export interface QueryLeasesArgs {
  id?: Maybe<Scalars["ID"]>;
  query?: Maybe<Scalars["String"]>;
}

export interface QueryLendersArgs {
  id?: Maybe<Scalars["ID"]>;
  query?: Maybe<Scalars["String"]>;
}

export interface QueryTransactionsArgs {
  id?: Maybe<Scalars["ID"]>;
}

export interface Rent {
  __typename?: "Rent";
  id: Scalars["ID"];
  periodEnd: Scalars["DateTime"];
  periodStart: Scalars["DateTime"];
  amount: Scalars["Decimal"];
  chargesAmount?: Maybe<Scalars["Decimal"]>;
  fullAmount: Scalars["Decimal"];
  status: RentStatus;
  leaseId: Scalars["ID"];
  receiptId?: Maybe<Scalars["ID"]>;
  transactionId?: Maybe<Scalars["ID"]>;
  noticeId?: Maybe<Scalars["ID"]>;
  delay?: Maybe<Scalars["Int"]>;
  transactions?: Maybe<Array<Transaction>>;
}

export enum RentChargesRecuperationMode {
  Package = "PACKAGE",
  Periodic = "PERIODIC",
  Reel = "REEL",
}

export enum RentPaymentMethod {
  After = "AFTER",
  Before = "BEFORE",
}

export interface RentReceiptInput {
  leaseId: Scalars["ID"];
  sendMail?: Maybe<Scalars["Boolean"]>;
}

export interface RentReceiptPayload {
  __typename?: "RentReceiptPayload";
  receipt: File;
}

export enum RentStatus {
  Partial = "PARTIAL",
  Pending = "PENDING",
  Settled = "SETTLED",
}

/** https://stripe.com/docs/billing/subscriptions/overview#subscription-states */
export enum SubscriptionStatus {
  Active = "ACTIVE",
  Canceled = "CANCELED",
  Incomplete = "INCOMPLETE",
  IncompleteExpired = "INCOMPLETE_EXPIRED",
  PastDue = "PAST_DUE",
  Trialing = "TRIALING",
  Unpaid = "UNPAID",
}

export interface Summary {
  __typename?: "Summary";
  since: Scalars["DateTime"];
  until: Scalars["DateTime"];
  amountExpected: Scalars["Decimal"];
  amountReceived: Scalars["Decimal"];
  amountSettled: Scalars["Decimal"];
  amountPartial: Scalars["Decimal"];
  amountPending: Scalars["Decimal"];
  nExpected: Scalars["Int"];
  nReceived: Scalars["Int"];
  nSettled: Scalars["Int"];
  nPartial: Scalars["Int"];
  nPending: Scalars["Int"];
  ratioExpected: Scalars["Float"];
  ratioReceived: Scalars["Float"];
  ratioSettled: Scalars["Float"];
  ratioPartial: Scalars["Float"];
  ratioPending: Scalars["Float"];
  variationExpected: Scalars["Float"];
  variationReceived: Scalars["Float"];
  variationSettled: Scalars["Float"];
  variationPartial: Scalars["Float"];
  variationPending: Scalars["Float"];
  paymentRate: Scalars["Float"];
  occupationRate: Scalars["Float"];
}

export interface Task {
  __typename?: "Task";
  id: Scalars["ID"];
  status: Scalars["String"];
  progress: Scalars["Int"];
}

export interface Tenant {
  __typename?: "Tenant";
  accountId: Scalars["ID"];
  apl?: Maybe<Scalars["Boolean"]>;
  authId?: Maybe<Scalars["AuthenticationID"]>;
  birthdate: Scalars["DateTime"];
  birthplace?: Maybe<Scalars["String"]>;
  email: Scalars["String"];
  firstName: Scalars["String"];
  lastName: Scalars["String"];
  displayName: Scalars["String"];
  note?: Maybe<Scalars["String"]>;
  phoneNumber?: Maybe<Scalars["String"]>;
  role?: Maybe<Scalars["String"]>;
  id: Scalars["ID"];
  leaseId?: Maybe<Scalars["ID"]>;
  visaleId?: Maybe<Scalars["String"]>;
  account?: Maybe<Account>;
  property?: Maybe<Property>;
  status?: Maybe<TenantStatus>;
  fullName: Scalars["String"];
  shortName?: Maybe<Scalars["String"]>;
  lastTransaction?: Maybe<Transaction>;
  propertyName?: Maybe<Scalars["String"]>;
  rentPayedThisYear?: Maybe<Scalars["String"]>;
  unpaidRentAmount?: Maybe<Scalars["Int"]>;
  files?: Maybe<Array<File>>;
  lease?: Maybe<Lease>;
}

export interface TenantInput {
  apl?: Maybe<Scalars["Boolean"]>;
  birthdate: Scalars["Date"];
  birthplace?: Maybe<Scalars["String"]>;
  email: Scalars["Email"];
  firstName: Scalars["String"];
  lastName: Scalars["String"];
  note?: Maybe<Scalars["String"]>;
  phoneNumber?: Maybe<Scalars["PhoneNumber"]>;
  visaleId?: Maybe<Scalars["String"]>;
}

export enum TenantStatus {
  Gone = "GONE",
  Late = "LATE",
  New = "NEW",
  Uptodate = "UPTODATE",
}

export interface TenantUpdateInput {
  apl?: Maybe<Scalars["Boolean"]>;
  birthdate?: Maybe<Scalars["Date"]>;
  birthplace?: Maybe<Scalars["String"]>;
  email?: Maybe<Scalars["Email"]>;
  id: Scalars["ID"];
  firstName?: Maybe<Scalars["String"]>;
  lastName?: Maybe<Scalars["String"]>;
  note?: Maybe<Scalars["String"]>;
  phoneNumber?: Maybe<Scalars["PhoneNumber"]>;
  visaleId?: Maybe<Scalars["String"]>;
}

export interface Transaction {
  __typename?: "Transaction";
  id: Scalars["ID"];
  date: Scalars["DateTime"];
  amount: Scalars["Decimal"];
  accountId: Scalars["ID"];
  type: TransactionType;
  lease?: Maybe<Lease>;
}

export interface TransactionInput {
  amount: Scalars["Decimal"];
  leaseId: Scalars["ID"];
  date: Scalars["Date"];
  type?: Maybe<TransactionType>;
}

export enum TransactionType {
  InsuranceHab = "INSURANCE_HAB",
  InsurancePno = "INSURANCE_PNO",
  Invoice = "INVOICE",
  LoanInterest = "LOAN_INTEREST",
  LoanPayment = "LOAN_PAYMENT",
  Other = "OTHER",
  Rent = "RENT",
}

export interface User {
  __typename?: "User";
  authId?: Maybe<Scalars["AuthenticationID"]>;
  email: Scalars["Email"];
  firstName?: Maybe<Scalars["String"]>;
  lastName?: Maybe<Scalars["String"]>;
  address?: Maybe<Address>;
  photoURL?: Maybe<Scalars["String"]>;
  role?: Maybe<UserRole>;
  id: Scalars["ID"];
  phoneNumber?: Maybe<Scalars["PhoneNumber"]>;
  accountId?: Maybe<Scalars["ID"]>;
  displayName: Scalars["String"];
  accounts?: Maybe<Array<Account>>;
  account?: Maybe<Account>;
}

export enum UserRole {
  Admin = "ADMIN",
  User = "USER",
  Viewer = "VIEWER",
}

export interface UserUpdateInput {
  address: AddressInput;
  firstName: Scalars["String"];
  lastName: Scalars["String"];
}

export interface UserWithAccountInput {
  address?: Maybe<AddressInput>;
  authId: Scalars["AuthenticationID"];
  email: Scalars["Email"];
  firstName: Scalars["String"];
  lastName: Scalars["String"];
  skipCreateCustomer?: Maybe<Scalars["Boolean"]>;
}

/** A GraphQL Schema defines the capabilities of a GraphQL server. It exposes all available types and directives on the server, as well as the entry points for query, mutation, and subscription operations. */
export interface __Schema {
  __typename?: "__Schema";
  description?: Maybe<Scalars["String"]>;
  /** A list of all types supported by this server. */
  types: Array<__Type>;
  /** The type that query operations will be rooted at. */
  queryType: __Type;
  /** If this server supports mutation, the type that mutation operations will be rooted at. */
  mutationType?: Maybe<__Type>;
  /** If this server support subscription, the type that subscription operations will be rooted at. */
  subscriptionType?: Maybe<__Type>;
  /** A list of all directives supported by this server. */
  directives: Array<__Directive>;
}

/**
 * The fundamental unit of any GraphQL Schema is the type. There are many kinds of types in GraphQL as represented by the `__TypeKind` enum.
 *
 * Depending on the kind of a type, certain fields describe information about that type. Scalar types provide no information beyond a name, description and optional `specifiedByUrl`, while Enum types provide their values. Object and Interface types provide the fields they describe. Abstract types, Union and Interface, provide the Object types possible at runtime. List and NonNull types compose other types.
 */
export interface __Type {
  __typename?: "__Type";
  kind: __TypeKind;
  name?: Maybe<Scalars["String"]>;
  description?: Maybe<Scalars["String"]>;
  specifiedByUrl?: Maybe<Scalars["String"]>;
  fields?: Maybe<Array<__Field>>;
  interfaces?: Maybe<Array<__Type>>;
  possibleTypes?: Maybe<Array<__Type>>;
  enumValues?: Maybe<Array<__EnumValue>>;
  inputFields?: Maybe<Array<__InputValue>>;
  ofType?: Maybe<__Type>;
}

/**
 * The fundamental unit of any GraphQL Schema is the type. There are many kinds of types in GraphQL as represented by the `__TypeKind` enum.
 *
 * Depending on the kind of a type, certain fields describe information about that type. Scalar types provide no information beyond a name, description and optional `specifiedByUrl`, while Enum types provide their values. Object and Interface types provide the fields they describe. Abstract types, Union and Interface, provide the Object types possible at runtime. List and NonNull types compose other types.
 */
export interface __TypeFieldsArgs {
  includeDeprecated?: Maybe<Scalars["Boolean"]>;
}

/**
 * The fundamental unit of any GraphQL Schema is the type. There are many kinds of types in GraphQL as represented by the `__TypeKind` enum.
 *
 * Depending on the kind of a type, certain fields describe information about that type. Scalar types provide no information beyond a name, description and optional `specifiedByUrl`, while Enum types provide their values. Object and Interface types provide the fields they describe. Abstract types, Union and Interface, provide the Object types possible at runtime. List and NonNull types compose other types.
 */
export interface __TypeEnumValuesArgs {
  includeDeprecated?: Maybe<Scalars["Boolean"]>;
}

/**
 * The fundamental unit of any GraphQL Schema is the type. There are many kinds of types in GraphQL as represented by the `__TypeKind` enum.
 *
 * Depending on the kind of a type, certain fields describe information about that type. Scalar types provide no information beyond a name, description and optional `specifiedByUrl`, while Enum types provide their values. Object and Interface types provide the fields they describe. Abstract types, Union and Interface, provide the Object types possible at runtime. List and NonNull types compose other types.
 */
export interface __TypeInputFieldsArgs {
  includeDeprecated?: Maybe<Scalars["Boolean"]>;
}

/** An enum describing what kind of type a given `__Type` is. */
export enum __TypeKind {
  /** Indicates this type is a scalar. */
  Scalar = "SCALAR",
  /** Indicates this type is an object. `fields` and `interfaces` are valid fields. */
  Object = "OBJECT",
  /** Indicates this type is an interface. `fields`, `interfaces`, and `possibleTypes` are valid fields. */
  Interface = "INTERFACE",
  /** Indicates this type is a union. `possibleTypes` is a valid field. */
  Union = "UNION",
  /** Indicates this type is an enum. `enumValues` is a valid field. */
  Enum = "ENUM",
  /** Indicates this type is an input object. `inputFields` is a valid field. */
  InputObject = "INPUT_OBJECT",
  /** Indicates this type is a list. `ofType` is a valid field. */
  List = "LIST",
  /** Indicates this type is a non-null. `ofType` is a valid field. */
  NonNull = "NON_NULL",
}

/** Object and Interface types are described by a list of Fields, each of which has a name, potentially a list of arguments, and a return type. */
export interface __Field {
  __typename?: "__Field";
  name: Scalars["String"];
  description?: Maybe<Scalars["String"]>;
  args: Array<__InputValue>;
  type: __Type;
  isDeprecated: Scalars["Boolean"];
  deprecationReason?: Maybe<Scalars["String"]>;
}

/** Object and Interface types are described by a list of Fields, each of which has a name, potentially a list of arguments, and a return type. */
export interface __FieldArgsArgs {
  includeDeprecated?: Maybe<Scalars["Boolean"]>;
}

/** Arguments provided to Fields or Directives and the input fields of an InputObject are represented as Input Values which describe their type and optionally a default value. */
export interface __InputValue {
  __typename?: "__InputValue";
  name: Scalars["String"];
  description?: Maybe<Scalars["String"]>;
  type: __Type;
  /** A GraphQL-formatted string representing the default value for this input value. */
  defaultValue?: Maybe<Scalars["String"]>;
  isDeprecated: Scalars["Boolean"];
  deprecationReason?: Maybe<Scalars["String"]>;
}

/** One possible value for a given Enum. Enum values are unique values, not a placeholder for a string or numeric value. However an Enum value is returned in a JSON response as a string. */
export interface __EnumValue {
  __typename?: "__EnumValue";
  name: Scalars["String"];
  description?: Maybe<Scalars["String"]>;
  isDeprecated: Scalars["Boolean"];
  deprecationReason?: Maybe<Scalars["String"]>;
}

/**
 * A Directive provides a way to describe alternate runtime execution and type validation behavior in a GraphQL document.
 *
 * In some cases, you need to provide options to alter GraphQL's execution behavior in ways field arguments will not suffice, such as conditionally including or skipping a field. Directives provide this by describing additional information to the executor.
 */
export interface __Directive {
  __typename?: "__Directive";
  name: Scalars["String"];
  description?: Maybe<Scalars["String"]>;
  isRepeatable: Scalars["Boolean"];
  locations: Array<__DirectiveLocation>;
  args: Array<__InputValue>;
}

/** A Directive can be adjacent to many parts of the GraphQL language, a __DirectiveLocation describes one such possible adjacencies. */
export enum __DirectiveLocation {
  /** Location adjacent to a query operation. */
  Query = "QUERY",
  /** Location adjacent to a mutation operation. */
  Mutation = "MUTATION",
  /** Location adjacent to a subscription operation. */
  Subscription = "SUBSCRIPTION",
  /** Location adjacent to a field. */
  Field = "FIELD",
  /** Location adjacent to a fragment definition. */
  FragmentDefinition = "FRAGMENT_DEFINITION",
  /** Location adjacent to a fragment spread. */
  FragmentSpread = "FRAGMENT_SPREAD",
  /** Location adjacent to an inline fragment. */
  InlineFragment = "INLINE_FRAGMENT",
  /** Location adjacent to a variable definition. */
  VariableDefinition = "VARIABLE_DEFINITION",
  /** Location adjacent to a schema definition. */
  Schema = "SCHEMA",
  /** Location adjacent to a scalar definition. */
  Scalar = "SCALAR",
  /** Location adjacent to an object type definition. */
  Object = "OBJECT",
  /** Location adjacent to a field definition. */
  FieldDefinition = "FIELD_DEFINITION",
  /** Location adjacent to an argument definition. */
  ArgumentDefinition = "ARGUMENT_DEFINITION",
  /** Location adjacent to an interface definition. */
  Interface = "INTERFACE",
  /** Location adjacent to a union definition. */
  Union = "UNION",
  /** Location adjacent to an enum definition. */
  Enum = "ENUM",
  /** Location adjacent to an enum value definition. */
  EnumValue = "ENUM_VALUE",
  /** Location adjacent to an input object type definition. */
  InputObject = "INPUT_OBJECT",
  /** Location adjacent to an input object field definition. */
  InputFieldDefinition = "INPUT_FIELD_DEFINITION",
}

export type TestQueryVariables = Exact<{ [key: string]: never }>;

export type TestQuery = { __typename?: "Query" } & {
  __schema: { __typename?: "__Schema" } & {
    types: Array<{ __typename?: "__Type" } & Pick<__Type, "name">>;
  };
};

export type UserQueryVariables = Exact<{ [key: string]: never }>;

export type UserQuery = { __typename?: "Query" } & {
  user: { __typename?: "User" } & Pick<
    User,
    "id" | "email" | "firstName" | "lastName" | "displayName" | "photoURL"
  > & {
      address?: Maybe<
        { __typename?: "Address" } & Pick<
          Address,
          "line1" | "line2" | "city" | "postalCode"
        >
      >;
      account?: Maybe<
        { __typename?: "Account" } & Pick<
          Account,
          "id" | "status" | "trialEnd"
        > & { plan?: Maybe<{ __typename?: "Plan" } & Pick<Plan, "code">> }
      >;
    };
};

export type LenderQueryVariables = Exact<{ [key: string]: never }>;

export type LenderQuery = { __typename?: "Query" } & {
  lenders: Array<
    { __typename?: "Lender" } & Pick<
      Lender,
      "id" | "accountId" | "displayName"
    > & {
        identity?: Maybe<
          | ({ __typename?: "User" } & Pick<
              User,
              "id" | "email" | "displayName" | "firstName" | "lastName"
            > & {
                address?: Maybe<
                  { __typename?: "Address" } & Pick<
                    Address,
                    "inline" | "line1" | "line2" | "city" | "postalCode"
                  >
                >;
              })
          | ({ __typename?: "Company" } & Pick<
              Company,
              | "id"
              | "email"
              | "displayName"
              | "legalEntity"
              | "legalEntityIdentifier"
            > & {
                address?: Maybe<
                  { __typename?: "Address" } & Pick<
                    Address,
                    "inline" | "line1" | "line2" | "city" | "postalCode"
                  >
                >;
              })
        >;
      }
  >;
};

export type TenantWithRentalReceiptsQueryVariables = Exact<{
  id: Scalars["ID"];
}>;

export type TenantWithRentalReceiptsQuery = { __typename?: "Query" } & {
  tenants: Array<
    { __typename?: "Tenant" } & Pick<
      Tenant,
      | "id"
      | "firstName"
      | "lastName"
      | "fullName"
      | "email"
      | "phoneNumber"
      | "propertyName"
      | "rentPayedThisYear"
      | "unpaidRentAmount"
      | "status"
      | "note"
    > & {
        lastTransaction?: Maybe<
          { __typename?: "Transaction" } & Pick<Transaction, "date" | "amount">
        >;
      }
  >;
};

export type TenantListQueryVariables = Exact<{
  id?: Maybe<Scalars["ID"]>;
  query?: Maybe<Scalars["String"]>;
  status?: Maybe<TenantStatus>;
}>;

export type TenantListQuery = { __typename?: "Query" } & {
  tenants: Array<
    { __typename?: "Tenant" } & Pick<
      Tenant,
      | "id"
      | "firstName"
      | "lastName"
      | "fullName"
      | "email"
      | "phoneNumber"
      | "propertyName"
      | "rentPayedThisYear"
      | "unpaidRentAmount"
      | "status"
      | "note"
      | "visaleId"
      | "apl"
      | "birthdate"
      | "birthplace"
      | "displayName"
      | "accountId"
    > & {
        lastTransaction?: Maybe<
          { __typename?: "Transaction" } & Pick<
            Transaction,
            "id" | "date" | "amount" | "accountId" | "type"
          >
        >;
        files?: Maybe<
          Array<
            { __typename?: "File" } & Pick<
              File,
              "id" | "filename" | "createdAt" | "type" | "downloadUrl"
            >
          >
        >;
        lease?: Maybe<
          { __typename?: "Lease" } & Pick<
            Lease,
            | "id"
            | "type"
            | "status"
            | "rentFullAmount"
            | "rentAmount"
            | "rentChargesAmount"
            | "effectDate"
            | "renewDate"
            | "signatureDate"
            | "depositAmount"
            | "accountId"
            | "propertyId"
          > & {
              rents?: Maybe<
                Array<
                  { __typename?: "Rent" } & Pick<
                    Rent,
                    | "id"
                    | "periodStart"
                    | "periodEnd"
                    | "fullAmount"
                    | "amount"
                    | "status"
                    | "leaseId"
                  >
                >
              >;
              data?: Maybe<
                { __typename?: "LeaseFurnishedData" } & Pick<
                  LeaseFurnishedData,
                  "duration"
                >
              >;
              account?: Maybe<{ __typename?: "Account" } & Pick<Account, "id">>;
              property?: Maybe<
                { __typename?: "Property" } & Pick<
                  Property,
                  | "id"
                  | "buildPeriod"
                  | "buildingLegalStatus"
                  | "heatingMethod"
                  | "housingType"
                  | "name"
                  | "roomCount"
                  | "surface"
                  | "usageType"
                  | "waterHeatingMethod"
                > & {
                    address: { __typename?: "Address" } & Pick<
                      Address,
                      "city" | "inline" | "line1" | "postalCode"
                    >;
                  }
              >;
              tenants: Array<
                { __typename?: "Tenant" } & Pick<
                  Tenant,
                  | "id"
                  | "displayName"
                  | "email"
                  | "firstName"
                  | "lastName"
                  | "accountId"
                  | "birthdate"
                  | "fullName"
                > & {
                    account?: Maybe<
                      { __typename?: "Account" } & Pick<Account, "id">
                    >;
                  }
              >;
            }
        >;
        account?: Maybe<{ __typename?: "Account" } & Pick<Account, "id">>;
      }
  >;
};

export type LeaseListQueryVariables = Exact<{
  id?: Maybe<Scalars["ID"]>;
  query?: Maybe<Scalars["String"]>;
}>;

export type LeaseListQuery = { __typename?: "Query" } & {
  leases: Array<
    { __typename?: "Lease" } & Pick<
      Lease,
      | "id"
      | "accountId"
      | "propertyId"
      | "status"
      | "rentFullAmount"
      | "effectDate"
      | "rentAmount"
      | "depositAmount"
      | "rentChargesAmount"
    > & {
        account?: Maybe<{ __typename?: "Account" } & Pick<Account, "id">>;
        property?: Maybe<
          { __typename?: "Property" } & Pick<
            Property,
            | "id"
            | "buildPeriod"
            | "buildingLegalStatus"
            | "heatingMethod"
            | "housingType"
            | "name"
            | "roomCount"
            | "surface"
            | "usageType"
            | "waterHeatingMethod"
          > & {
              address: { __typename?: "Address" } & Pick<
                Address,
                "city" | "inline" | "line1" | "postalCode"
              >;
            }
        >;
        tenants: Array<
          { __typename?: "Tenant" } & Pick<
            Tenant,
            | "id"
            | "email"
            | "displayName"
            | "firstName"
            | "lastName"
            | "fullName"
            | "accountId"
            | "birthdate"
          > & {
              account?: Maybe<{ __typename?: "Account" } & Pick<Account, "id">>;
            }
        >;
        file?: Maybe<
          { __typename?: "File" } & Pick<File, "id" | "downloadUrl" | "type">
        >;
        data?: Maybe<
          { __typename?: "LeaseFurnishedData" } & Pick<
            LeaseFurnishedData,
            "duration" | "rentPaymentMethod"
          >
        >;
      }
  >;
};

export type LeaseQueryVariables = Exact<{
  id: Scalars["ID"];
}>;

export type LeaseQuery = { __typename?: "Query" } & {
  leases: Array<
    { __typename?: "Lease" } & Pick<
      Lease,
      | "id"
      | "accountId"
      | "status"
      | "rentFullAmount"
      | "effectDate"
      | "rentAmount"
      | "depositAmount"
      | "rentChargesAmount"
    > & {
        tenants: Array<
          { __typename?: "Tenant" } & Pick<Tenant, "id" | "fullName">
        >;
        file?: Maybe<{ __typename?: "File" } & Pick<File, "downloadUrl">>;
        data?: Maybe<
          { __typename?: "LeaseFurnishedData" } & Pick<
            LeaseFurnishedData,
            "duration" | "rentPaymentMethod"
          >
        >;
      }
  >;
};

export type ContractRequirementDataQueryVariables = Exact<{
  query?: Maybe<Scalars["String"]>;
}>;

export type ContractRequirementDataQuery = { __typename?: "Query" } & {
  tenants: Array<
    { __typename?: "Tenant" } & Pick<Tenant, "id" | "fullName" | "status"> & {
        lease?: Maybe<{ __typename?: "Lease" } & Pick<Lease, "id">>;
      }
  >;
};

export type PropertyListQueryVariables = Exact<{
  id?: Maybe<Scalars["ID"]>;
  query?: Maybe<Scalars["String"]>;
}>;

export type PropertyListQuery = { __typename?: "Query" } & {
  properties: Array<
    { __typename?: "Property" } & Pick<
      Property,
      | "id"
      | "name"
      | "roomCount"
      | "note"
      | "collectedRents"
      | "surface"
      | "status"
      | "energyClass"
      | "gasEmission"
      | "tenantPrivateSpaces"
      | "equipments"
      | "nticEquipments"
      | "otherSpaces"
      | "commonSpaces"
      | "waterHeatingMethod"
      | "heatingMethod"
      | "tax"
      | "buildPeriod"
      | "housingType"
      | "buildingLegalStatus"
      | "usageType"
    > & {
        address: { __typename?: "Address" } & Pick<
          Address,
          "city" | "country" | "inline" | "line1" | "line2" | "postalCode"
        >;
        lender?: Maybe<
          { __typename?: "Lender" } & Pick<Lender, "id" | "displayName">
        >;
        leases?: Maybe<
          Array<
            { __typename?: "Lease" } & Pick<
              Lease,
              | "id"
              | "status"
              | "type"
              | "effectDate"
              | "renewDate"
              | "signatureDate"
              | "rentAmount"
              | "depositAmount"
              | "rentChargesAmount"
              | "rentFullAmount"
              | "accountId"
              | "propertyId"
            > & {
                account?: Maybe<
                  { __typename?: "Account" } & Pick<Account, "id">
                >;
                tenants: Array<
                  { __typename?: "Tenant" } & Pick<
                    Tenant,
                    | "id"
                    | "fullName"
                    | "status"
                    | "displayName"
                    | "email"
                    | "firstName"
                    | "lastName"
                    | "accountId"
                    | "birthdate"
                  > & {
                      account?: Maybe<
                        { __typename?: "Account" } & Pick<Account, "id">
                      >;
                    }
                >;
                property?: Maybe<
                  { __typename?: "Property" } & Pick<
                    Property,
                    | "id"
                    | "buildPeriod"
                    | "buildingLegalStatus"
                    | "heatingMethod"
                    | "housingType"
                    | "name"
                    | "roomCount"
                    | "surface"
                    | "usageType"
                    | "waterHeatingMethod"
                  > & {
                      address: { __typename?: "Address" } & Pick<
                        Address,
                        "city" | "inline" | "line1" | "postalCode"
                      >;
                      lender?: Maybe<
                        { __typename?: "Lender" } & Pick<
                          Lender,
                          "id" | "displayName"
                        >
                      >;
                    }
                >;
              }
          >
        >;
      }
  >;
};

export type TransactionQueryVariables = Exact<{
  id: Scalars["ID"];
}>;

export type TransactionQuery = { __typename?: "Query" } & {
  transactions: Array<
    { __typename?: "Transaction" } & Pick<
      Transaction,
      "id" | "date" | "amount" | "type"
    > & { lease?: Maybe<{ __typename?: "Lease" } & Pick<Lease, "id">> }
  >;
};

export type InvoiceListQueryVariables = Exact<{ [key: string]: never }>;

export type InvoiceListQuery = { __typename?: "Query" } & {
  invoices: Array<
    { __typename?: "Invoice" } & Pick<
      Invoice,
      | "id"
      | "number"
      | "amountPaid"
      | "invoicePdf"
      | "periodEnd"
      | "status"
      | "planCode"
    >
  >;
};

export type PricingPlansQueryVariables = Exact<{ [key: string]: never }>;

export type PricingPlansQuery = { __typename?: "Query" } & {
  plans: Array<
    { __typename?: "Plan" } & Pick<
      Plan,
      "title" | "subtitle" | "id" | "price" | "code"
    > & {
        features: Array<
          { __typename?: "Feature" } & Pick<
            Feature,
            "available" | "title" | "key"
          >
        >;
      }
  >;
};

export type RentalReceiptDataQueryVariables = Exact<{ [key: string]: never }>;

export type RentalReceiptDataQuery = { __typename?: "Query" } & {
  tenants: Array<
    { __typename?: "Tenant" } & Pick<
      Tenant,
      "id" | "firstName" | "lastName" | "fullName" | "propertyName"
    > & {
        lease?: Maybe<
          { __typename?: "Lease" } & Pick<
            Lease,
            "id" | "rentAmount" | "rentChargesAmount" | "rentFullAmount"
          > & {
              property?: Maybe<
                { __typename?: "Property" } & Pick<Property, "id"> & {
                    address: { __typename?: "Address" } & Pick<
                      Address,
                      | "city"
                      | "country"
                      | "inline"
                      | "line1"
                      | "line2"
                      | "postalCode"
                    >;
                  }
              >;
            }
        >;
      }
  >;
};

export type RentReceivedSummaryQueryVariables = Exact<{
  since: Scalars["DateTime"];
  until: Scalars["DateTime"];
}>;

export type RentReceivedSummaryQuery = { __typename?: "Query" } & {
  rentReceivedSummary: { __typename?: "Summary" } & Pick<
    Summary,
    | "since"
    | "until"
    | "amountExpected"
    | "amountReceived"
    | "amountSettled"
    | "amountPartial"
    | "amountPending"
    | "nExpected"
    | "nReceived"
    | "nSettled"
    | "nPartial"
    | "nPending"
    | "ratioExpected"
    | "ratioReceived"
    | "ratioSettled"
    | "ratioPartial"
    | "ratioPending"
    | "variationExpected"
    | "variationReceived"
    | "variationSettled"
    | "variationPartial"
    | "variationPending"
    | "paymentRate"
    | "occupationRate"
  >;
};

export type RentReceivedStatusQueryVariables = Exact<{ [key: string]: never }>;

export type RentReceivedStatusQuery = { __typename?: "Query" } & {
  properties: Array<
    { __typename?: "Property" } & Pick<
      Property,
      "id" | "name" | "collectedRents" | "expectedRents"
    > & {
        leases?: Maybe<
          Array<
            { __typename?: "Lease" } & Pick<Lease, "id" | "rentFullAmount"> & {
                property?: Maybe<
                  { __typename?: "Property" } & {
                    lender?: Maybe<
                      { __typename?: "Lender" } & Pick<Lender, "id"> & {
                          identity?: Maybe<
                            | ({ __typename?: "User" } & {
                                address?: Maybe<
                                  { __typename?: "Address" } & Pick<
                                    Address,
                                    "inline"
                                  >
                                >;
                              })
                            | ({ __typename?: "Company" } & {
                                address?: Maybe<
                                  { __typename?: "Address" } & Pick<
                                    Address,
                                    "inline"
                                  >
                                >;
                              })
                          >;
                        }
                    >;
                  }
                >;
                tenants: Array<
                  { __typename?: "Tenant" } & Pick<
                    Tenant,
                    "id" | "fullName" | "shortName"
                  >
                >;
                rents?: Maybe<
                  Array<
                    { __typename?: "Rent" } & Pick<
                      Rent,
                      | "id"
                      | "periodStart"
                      | "periodEnd"
                      | "fullAmount"
                      | "amount"
                      | "status"
                      | "leaseId"
                    > & {
                        transactions?: Maybe<
                          Array<
                            { __typename?: "Transaction" } & Pick<
                              Transaction,
                              "id"
                            >
                          >
                        >;
                      }
                  >
                >;
              }
          >
        >;
      }
  >;
};

export type FileQueryVariables = Exact<{ [key: string]: never }>;

export type FileQuery = { __typename?: "Query" } & {
  files: Array<
    { __typename?: "File" } & Pick<
      File,
      "id" | "filename" | "type" | "createdAt" | "downloadUrl"
    >
  >;
};

export type UserCreateWithAccountMutationVariables = Exact<{
  input: UserWithAccountInput;
}>;

export type UserCreateWithAccountMutation = { __typename?: "Mutation" } & {
  accountCreate: { __typename?: "Account" } & Pick<Account, "id">;
};

export type AccountUpdatePaymentMethodMutationVariables = Exact<{
  input: AccountUpdateInput;
}>;

export type AccountUpdatePaymentMethodMutation = { __typename?: "Mutation" } & {
  accountUpdatePaymentMethod: { __typename?: "Account" } & Pick<Account, "id">;
};

export type AccountActivatePlanMutationVariables = Exact<{
  input: AccountActivatePlanInput;
}>;

export type AccountActivatePlanMutation = { __typename?: "Mutation" } & {
  accountActivatePlan: { __typename?: "Account" } & Pick<
    Account,
    "id" | "status" | "trialEnd"
  >;
};

export type TenantCreateMutationVariables = Exact<{
  input: TenantInput;
}>;

export type TenantCreateMutation = { __typename?: "Mutation" } & {
  tenant: { __typename?: "Tenant" } & Pick<
    Tenant,
    | "id"
    | "firstName"
    | "lastName"
    | "fullName"
    | "email"
    | "phoneNumber"
    | "propertyName"
    | "rentPayedThisYear"
    | "unpaidRentAmount"
    | "status"
    | "note"
  > & {
      lastTransaction?: Maybe<
        { __typename?: "Transaction" } & Pick<Transaction, "date" | "amount">
      >;
    };
};

export type TenantUpdateMutationVariables = Exact<{
  input: TenantUpdateInput;
}>;

export type TenantUpdateMutation = { __typename?: "Mutation" } & {
  tenantUpdate: { __typename?: "Tenant" } & Pick<Tenant, "id">;
};

export type TenantDeleteMutationVariables = Exact<{
  id: Scalars["ID"];
}>;

export type TenantDeleteMutation = { __typename?: "Mutation" } & {
  tenantId: Mutation["tenantDelete"];
};

export type PropertyCreateMutationVariables = Exact<{
  input: PropertyInput;
}>;

export type PropertyCreateMutation = { __typename?: "Mutation" } & {
  propertyCreate: { __typename?: "Property" } & Pick<Property, "id">;
};

export type PropertyUpdateMutationVariables = Exact<{
  input: PropertyUpdateInput;
}>;

export type PropertyUpdateMutation = { __typename?: "Mutation" } & {
  propertyUpdate: { __typename?: "Property" } & Pick<Property, "id">;
};

export type PropertyDeleteMutationVariables = Exact<{
  id: Scalars["ID"];
}>;

export type PropertyDeleteMutation = { __typename?: "Mutation" } & Pick<
  Mutation,
  "propertyDelete"
>;

export type LeaseCreateMutationVariables = Exact<{
  input: LeaseFurnishedInput;
}>;

export type LeaseCreateMutation = { __typename?: "Mutation" } & {
  leaseCreate: { __typename?: "Lease" } & Pick<Lease, "id">;
};

export type LeaseDeleteMutationVariables = Exact<{
  id: Scalars["ID"];
}>;

export type LeaseDeleteMutation = { __typename?: "Mutation" } & Pick<
  Mutation,
  "leaseDelete"
>;

export type ContractDeleteMutationVariables = Exact<{
  id: Scalars["ID"];
}>;

export type ContractDeleteMutation = { __typename?: "Mutation" } & {
  leaseId: Mutation["leaseDelete"];
};

export type ContractUpdateMutationVariables = Exact<{
  input: LeaseFurnishedUpdateInput;
}>;

export type ContractUpdateMutation = { __typename?: "Mutation" } & {
  leaseUpdate: { __typename?: "Lease" } & Pick<Lease, "id">;
};

export type LenderIndividualUpdateMutationVariables = Exact<{
  input: LenderIndividualUpdateInput;
}>;

export type LenderIndividualUpdateMutation = { __typename?: "Mutation" } & {
  lenderIndividualUpdate: { __typename?: "Lender" } & Pick<Lender, "id">;
};

export type TransactionCreateMutationVariables = Exact<{
  input: TransactionInput;
}>;

export type TransactionCreateMutation = { __typename?: "Mutation" } & {
  transactionCreate: { __typename?: "Transaction" } & Pick<Transaction, "id">;
};

export type TransactionDeleteMutationVariables = Exact<{
  id: Scalars["ID"];
}>;

export type TransactionDeleteMutation = { __typename?: "Mutation" } & {
  transactionId: Mutation["transactionDelete"];
};

export type FileUploadMutationVariables = Exact<{
  input: FileInput;
}>;

export type FileUploadMutation = { __typename?: "Mutation" } & {
  fileUpload: { __typename?: "File" } & Pick<File, "id">;
};

export type ImportUploadMutationVariables = Exact<{
  input: ImportInput;
}>;

export type ImportUploadMutation = { __typename?: "Mutation" } & {
  importUpload: { __typename?: "Task" } & Pick<
    Task,
    "id" | "status" | "progress"
  >;
};

export type RentReceiptCreateMutationVariables = Exact<{
  input: RentReceiptInput;
}>;

export type RentReceiptCreateMutation = { __typename?: "Mutation" } & {
  rentReceiptCreate: { __typename?: "RentReceiptPayload" } & {
    receipt: { __typename?: "File" } & Pick<File, "id" | "downloadUrl">;
  };
};

export const TestDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "Test" },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "__schema" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                {
                  kind: "Field",
                  name: { kind: "Name", value: "types" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "name" } },
                    ],
                  },
                },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<TestQuery, TestQueryVariables>;
export const UserDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "User" },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            alias: { kind: "Name", value: "user" },
            name: { kind: "Name", value: "viewer" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "email" } },
                { kind: "Field", name: { kind: "Name", value: "firstName" } },
                { kind: "Field", name: { kind: "Name", value: "lastName" } },
                { kind: "Field", name: { kind: "Name", value: "displayName" } },
                { kind: "Field", name: { kind: "Name", value: "photoURL" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "address" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "line1" } },
                      { kind: "Field", name: { kind: "Name", value: "line2" } },
                      { kind: "Field", name: { kind: "Name", value: "city" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "postalCode" },
                      },
                    ],
                  },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "account" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "status" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "plan" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "code" },
                            },
                          ],
                        },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "trialEnd" },
                      },
                    ],
                  },
                },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<UserQuery, UserQueryVariables>;
export const LenderDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "Lender" },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "lenders" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "accountId" } },
                { kind: "Field", name: { kind: "Name", value: "displayName" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "identity" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "InlineFragment",
                        typeCondition: {
                          kind: "NamedType",
                          name: { kind: "Name", value: "Company" },
                        },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "id" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "email" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "displayName" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "legalEntity" },
                            },
                            {
                              kind: "Field",
                              name: {
                                kind: "Name",
                                value: "legalEntityIdentifier",
                              },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "address" },
                              selectionSet: {
                                kind: "SelectionSet",
                                selections: [
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "inline" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "line1" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "line2" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "city" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "postalCode" },
                                  },
                                ],
                              },
                            },
                          ],
                        },
                      },
                      {
                        kind: "InlineFragment",
                        typeCondition: {
                          kind: "NamedType",
                          name: { kind: "Name", value: "User" },
                        },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "id" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "email" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "displayName" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "firstName" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "lastName" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "address" },
                              selectionSet: {
                                kind: "SelectionSet",
                                selections: [
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "inline" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "line1" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "line2" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "city" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "postalCode" },
                                  },
                                ],
                              },
                            },
                          ],
                        },
                      },
                    ],
                  },
                },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<LenderQuery, LenderQueryVariables>;
export const TenantWithRentalReceiptsDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "TenantWithRentalReceipts" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "id" } },
          type: {
            kind: "NonNullType",
            type: { kind: "NamedType", name: { kind: "Name", value: "ID" } },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "tenants" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "id" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "id" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "firstName" } },
                { kind: "Field", name: { kind: "Name", value: "lastName" } },
                { kind: "Field", name: { kind: "Name", value: "fullName" } },
                { kind: "Field", name: { kind: "Name", value: "email" } },
                { kind: "Field", name: { kind: "Name", value: "phoneNumber" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "lastTransaction" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "date" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "amount" },
                      },
                    ],
                  },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "propertyName" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "rentPayedThisYear" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "unpaidRentAmount" },
                },
                { kind: "Field", name: { kind: "Name", value: "status" } },
                { kind: "Field", name: { kind: "Name", value: "note" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  TenantWithRentalReceiptsQuery,
  TenantWithRentalReceiptsQueryVariables
>;
export const TenantListDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "TenantList" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "id" } },
          type: { kind: "NamedType", name: { kind: "Name", value: "ID" } },
        },
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "query" },
          },
          type: { kind: "NamedType", name: { kind: "Name", value: "String" } },
        },
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "status" },
          },
          type: {
            kind: "NamedType",
            name: { kind: "Name", value: "TenantStatus" },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "tenants" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "id" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "id" },
                },
              },
              {
                kind: "Argument",
                name: { kind: "Name", value: "query" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "query" },
                },
              },
              {
                kind: "Argument",
                name: { kind: "Name", value: "status" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "status" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "firstName" } },
                { kind: "Field", name: { kind: "Name", value: "lastName" } },
                { kind: "Field", name: { kind: "Name", value: "fullName" } },
                { kind: "Field", name: { kind: "Name", value: "email" } },
                { kind: "Field", name: { kind: "Name", value: "phoneNumber" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "lastTransaction" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      { kind: "Field", name: { kind: "Name", value: "date" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "amount" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "accountId" },
                      },
                      { kind: "Field", name: { kind: "Name", value: "type" } },
                    ],
                  },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "propertyName" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "rentPayedThisYear" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "unpaidRentAmount" },
                },
                { kind: "Field", name: { kind: "Name", value: "status" } },
                { kind: "Field", name: { kind: "Name", value: "note" } },
                { kind: "Field", name: { kind: "Name", value: "visaleId" } },
                { kind: "Field", name: { kind: "Name", value: "apl" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "files" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "filename" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "createdAt" },
                      },
                      { kind: "Field", name: { kind: "Name", value: "type" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "downloadUrl" },
                      },
                    ],
                  },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "lease" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      { kind: "Field", name: { kind: "Name", value: "type" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "status" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "rents" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "id" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "periodStart" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "periodEnd" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "fullAmount" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "amount" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "status" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "leaseId" },
                            },
                          ],
                        },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "rentFullAmount" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "rentAmount" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "rentChargesAmount" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "effectDate" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "renewDate" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "signatureDate" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "rentAmount" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "depositAmount" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "data" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "InlineFragment",
                              typeCondition: {
                                kind: "NamedType",
                                name: {
                                  kind: "Name",
                                  value: "LeaseFurnishedData",
                                },
                              },
                              selectionSet: {
                                kind: "SelectionSet",
                                selections: [
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "duration" },
                                  },
                                ],
                              },
                            },
                          ],
                        },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "accountId" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "account" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "id" },
                            },
                          ],
                        },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "propertyId" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "property" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "id" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "address" },
                              selectionSet: {
                                kind: "SelectionSet",
                                selections: [
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "city" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "inline" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "line1" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "postalCode" },
                                  },
                                ],
                              },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "buildPeriod" },
                            },
                            {
                              kind: "Field",
                              name: {
                                kind: "Name",
                                value: "buildingLegalStatus",
                              },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "heatingMethod" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "housingType" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "name" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "roomCount" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "surface" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "usageType" },
                            },
                            {
                              kind: "Field",
                              name: {
                                kind: "Name",
                                value: "waterHeatingMethod",
                              },
                            },
                          ],
                        },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "tenants" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "id" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "displayName" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "email" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "firstName" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "lastName" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "accountId" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "account" },
                              selectionSet: {
                                kind: "SelectionSet",
                                selections: [
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "id" },
                                  },
                                ],
                              },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "birthdate" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "fullName" },
                            },
                          ],
                        },
                      },
                    ],
                  },
                },
                { kind: "Field", name: { kind: "Name", value: "birthdate" } },
                { kind: "Field", name: { kind: "Name", value: "birthplace" } },
                { kind: "Field", name: { kind: "Name", value: "displayName" } },
                { kind: "Field", name: { kind: "Name", value: "accountId" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "account" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                    ],
                  },
                },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<TenantListQuery, TenantListQueryVariables>;
export const LeaseListDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "LeaseList" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "id" } },
          type: { kind: "NamedType", name: { kind: "Name", value: "ID" } },
        },
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "query" },
          },
          type: { kind: "NamedType", name: { kind: "Name", value: "String" } },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "leases" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "id" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "id" },
                },
              },
              {
                kind: "Argument",
                name: { kind: "Name", value: "query" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "query" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "accountId" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "account" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                    ],
                  },
                },
                { kind: "Field", name: { kind: "Name", value: "propertyId" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "property" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "address" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "city" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "inline" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "line1" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "postalCode" },
                            },
                          ],
                        },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "buildPeriod" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "buildingLegalStatus" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "heatingMethod" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "housingType" },
                      },
                      { kind: "Field", name: { kind: "Name", value: "name" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "roomCount" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "surface" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "usageType" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "waterHeatingMethod" },
                      },
                    ],
                  },
                },
                { kind: "Field", name: { kind: "Name", value: "status" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "tenants" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      { kind: "Field", name: { kind: "Name", value: "email" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "displayName" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "firstName" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "lastName" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "fullName" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "accountId" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "account" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "id" },
                            },
                          ],
                        },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "birthdate" },
                      },
                    ],
                  },
                },
                {
                  kind: "Field",
                  alias: { kind: "Name", value: "file" },
                  name: { kind: "Name", value: "lease" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "downloadUrl" },
                      },
                      { kind: "Field", name: { kind: "Name", value: "type" } },
                    ],
                  },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "rentFullAmount" },
                },
                { kind: "Field", name: { kind: "Name", value: "effectDate" } },
                { kind: "Field", name: { kind: "Name", value: "rentAmount" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "depositAmount" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "rentChargesAmount" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "data" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "InlineFragment",
                        typeCondition: {
                          kind: "NamedType",
                          name: { kind: "Name", value: "LeaseFurnishedData" },
                        },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "duration" },
                            },
                            {
                              kind: "Field",
                              name: {
                                kind: "Name",
                                value: "rentPaymentMethod",
                              },
                            },
                          ],
                        },
                      },
                    ],
                  },
                },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<LeaseListQuery, LeaseListQueryVariables>;
export const LeaseDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "Lease" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "id" } },
          type: {
            kind: "NonNullType",
            type: { kind: "NamedType", name: { kind: "Name", value: "ID" } },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "leases" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "id" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "id" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "accountId" } },
                { kind: "Field", name: { kind: "Name", value: "status" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "tenants" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "fullName" },
                      },
                    ],
                  },
                },
                {
                  kind: "Field",
                  alias: { kind: "Name", value: "file" },
                  name: { kind: "Name", value: "lease" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "downloadUrl" },
                      },
                    ],
                  },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "rentFullAmount" },
                },
                { kind: "Field", name: { kind: "Name", value: "effectDate" } },
                { kind: "Field", name: { kind: "Name", value: "rentAmount" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "depositAmount" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "rentChargesAmount" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "data" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "InlineFragment",
                        typeCondition: {
                          kind: "NamedType",
                          name: { kind: "Name", value: "LeaseFurnishedData" },
                        },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "duration" },
                            },
                            {
                              kind: "Field",
                              name: {
                                kind: "Name",
                                value: "rentPaymentMethod",
                              },
                            },
                          ],
                        },
                      },
                    ],
                  },
                },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<LeaseQuery, LeaseQueryVariables>;
export const ContractRequirementDataDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "ContractRequirementData" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "query" },
          },
          type: { kind: "NamedType", name: { kind: "Name", value: "String" } },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "tenants" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "query" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "query" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "fullName" } },
                { kind: "Field", name: { kind: "Name", value: "status" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "lease" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                    ],
                  },
                },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  ContractRequirementDataQuery,
  ContractRequirementDataQueryVariables
>;
export const PropertyListDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "PropertyList" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "id" } },
          type: { kind: "NamedType", name: { kind: "Name", value: "ID" } },
        },
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "query" },
          },
          type: { kind: "NamedType", name: { kind: "Name", value: "String" } },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "properties" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "id" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "id" },
                },
              },
              {
                kind: "Argument",
                name: { kind: "Name", value: "query" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "query" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "name" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "address" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "city" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "country" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "inline" },
                      },
                      { kind: "Field", name: { kind: "Name", value: "line1" } },
                      { kind: "Field", name: { kind: "Name", value: "line2" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "postalCode" },
                      },
                    ],
                  },
                },
                { kind: "Field", name: { kind: "Name", value: "roomCount" } },
                { kind: "Field", name: { kind: "Name", value: "note" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "lender" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "displayName" },
                      },
                    ],
                  },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "collectedRents" },
                },
                { kind: "Field", name: { kind: "Name", value: "surface" } },
                { kind: "Field", name: { kind: "Name", value: "status" } },
                { kind: "Field", name: { kind: "Name", value: "energyClass" } },
                { kind: "Field", name: { kind: "Name", value: "gasEmission" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "tenantPrivateSpaces" },
                },
                { kind: "Field", name: { kind: "Name", value: "equipments" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "nticEquipments" },
                },
                { kind: "Field", name: { kind: "Name", value: "otherSpaces" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "commonSpaces" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "waterHeatingMethod" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "heatingMethod" },
                },
                { kind: "Field", name: { kind: "Name", value: "tax" } },
                { kind: "Field", name: { kind: "Name", value: "buildPeriod" } },
                { kind: "Field", name: { kind: "Name", value: "housingType" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "buildingLegalStatus" },
                },
                { kind: "Field", name: { kind: "Name", value: "usageType" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "leases" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "status" },
                      },
                      { kind: "Field", name: { kind: "Name", value: "type" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "effectDate" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "renewDate" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "signatureDate" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "rentAmount" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "depositAmount" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "rentChargesAmount" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "rentFullAmount" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "accountId" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "account" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "id" },
                            },
                          ],
                        },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "tenants" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "id" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "fullName" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "status" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "displayName" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "email" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "firstName" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "lastName" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "accountId" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "account" },
                              selectionSet: {
                                kind: "SelectionSet",
                                selections: [
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "id" },
                                  },
                                ],
                              },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "birthdate" },
                            },
                          ],
                        },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "propertyId" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "property" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "id" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "address" },
                              selectionSet: {
                                kind: "SelectionSet",
                                selections: [
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "city" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "inline" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "line1" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "postalCode" },
                                  },
                                ],
                              },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "buildPeriod" },
                            },
                            {
                              kind: "Field",
                              name: {
                                kind: "Name",
                                value: "buildingLegalStatus",
                              },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "heatingMethod" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "housingType" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "lender" },
                              selectionSet: {
                                kind: "SelectionSet",
                                selections: [
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "id" },
                                  },
                                  {
                                    kind: "Field",
                                    name: {
                                      kind: "Name",
                                      value: "displayName",
                                    },
                                  },
                                ],
                              },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "name" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "roomCount" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "surface" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "usageType" },
                            },
                            {
                              kind: "Field",
                              name: {
                                kind: "Name",
                                value: "waterHeatingMethod",
                              },
                            },
                          ],
                        },
                      },
                    ],
                  },
                },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<PropertyListQuery, PropertyListQueryVariables>;
export const TransactionDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "Transaction" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "id" } },
          type: {
            kind: "NonNullType",
            type: { kind: "NamedType", name: { kind: "Name", value: "ID" } },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "transactions" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "id" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "id" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "date" } },
                { kind: "Field", name: { kind: "Name", value: "amount" } },
                { kind: "Field", name: { kind: "Name", value: "type" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "lease" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                    ],
                  },
                },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<TransactionQuery, TransactionQueryVariables>;
export const InvoiceListDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "InvoiceList" },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "invoices" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "number" } },
                { kind: "Field", name: { kind: "Name", value: "amountPaid" } },
                { kind: "Field", name: { kind: "Name", value: "invoicePdf" } },
                { kind: "Field", name: { kind: "Name", value: "periodEnd" } },
                { kind: "Field", name: { kind: "Name", value: "status" } },
                { kind: "Field", name: { kind: "Name", value: "planCode" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<InvoiceListQuery, InvoiceListQueryVariables>;
export const PricingPlansDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "PricingPlans" },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "plans" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "title" } },
                { kind: "Field", name: { kind: "Name", value: "subtitle" } },
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "price" } },
                { kind: "Field", name: { kind: "Name", value: "code" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "features" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "available" },
                      },
                      { kind: "Field", name: { kind: "Name", value: "title" } },
                      { kind: "Field", name: { kind: "Name", value: "key" } },
                    ],
                  },
                },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<PricingPlansQuery, PricingPlansQueryVariables>;
export const RentalReceiptDataDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "RentalReceiptData" },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "tenants" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "firstName" } },
                { kind: "Field", name: { kind: "Name", value: "lastName" } },
                { kind: "Field", name: { kind: "Name", value: "fullName" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "propertyName" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "lease" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "property" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "id" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "address" },
                              selectionSet: {
                                kind: "SelectionSet",
                                selections: [
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "city" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "country" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "inline" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "line1" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "line2" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "postalCode" },
                                  },
                                ],
                              },
                            },
                          ],
                        },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "rentAmount" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "rentChargesAmount" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "rentFullAmount" },
                      },
                    ],
                  },
                },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  RentalReceiptDataQuery,
  RentalReceiptDataQueryVariables
>;
export const RentReceivedSummaryDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "RentReceivedSummary" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "since" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "DateTime" },
            },
          },
        },
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "until" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "DateTime" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            alias: { kind: "Name", value: "rentReceivedSummary" },
            name: { kind: "Name", value: "summary" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "since" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "since" },
                },
              },
              {
                kind: "Argument",
                name: { kind: "Name", value: "until" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "until" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "since" } },
                { kind: "Field", name: { kind: "Name", value: "until" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "amountExpected" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "amountReceived" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "amountSettled" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "amountPartial" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "amountPending" },
                },
                { kind: "Field", name: { kind: "Name", value: "nExpected" } },
                { kind: "Field", name: { kind: "Name", value: "nReceived" } },
                { kind: "Field", name: { kind: "Name", value: "nSettled" } },
                { kind: "Field", name: { kind: "Name", value: "nPartial" } },
                { kind: "Field", name: { kind: "Name", value: "nPending" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "ratioExpected" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "ratioReceived" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "ratioSettled" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "ratioPartial" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "ratioPending" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "variationExpected" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "variationReceived" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "variationSettled" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "variationPartial" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "variationPending" },
                },
                { kind: "Field", name: { kind: "Name", value: "paymentRate" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "occupationRate" },
                },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  RentReceivedSummaryQuery,
  RentReceivedSummaryQueryVariables
>;
export const RentReceivedStatusDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "RentReceivedStatus" },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "properties" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "name" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "collectedRents" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "expectedRents" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "leases" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "property" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "lender" },
                              selectionSet: {
                                kind: "SelectionSet",
                                selections: [
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "id" },
                                  },
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "identity" },
                                    selectionSet: {
                                      kind: "SelectionSet",
                                      selections: [
                                        {
                                          kind: "InlineFragment",
                                          typeCondition: {
                                            kind: "NamedType",
                                            name: {
                                              kind: "Name",
                                              value: "User",
                                            },
                                          },
                                          selectionSet: {
                                            kind: "SelectionSet",
                                            selections: [
                                              {
                                                kind: "Field",
                                                name: {
                                                  kind: "Name",
                                                  value: "address",
                                                },
                                                selectionSet: {
                                                  kind: "SelectionSet",
                                                  selections: [
                                                    {
                                                      kind: "Field",
                                                      name: {
                                                        kind: "Name",
                                                        value: "inline",
                                                      },
                                                    },
                                                  ],
                                                },
                                              },
                                            ],
                                          },
                                        },
                                        {
                                          kind: "InlineFragment",
                                          typeCondition: {
                                            kind: "NamedType",
                                            name: {
                                              kind: "Name",
                                              value: "Company",
                                            },
                                          },
                                          selectionSet: {
                                            kind: "SelectionSet",
                                            selections: [
                                              {
                                                kind: "Field",
                                                name: {
                                                  kind: "Name",
                                                  value: "address",
                                                },
                                                selectionSet: {
                                                  kind: "SelectionSet",
                                                  selections: [
                                                    {
                                                      kind: "Field",
                                                      name: {
                                                        kind: "Name",
                                                        value: "inline",
                                                      },
                                                    },
                                                  ],
                                                },
                                              },
                                            ],
                                          },
                                        },
                                      ],
                                    },
                                  },
                                ],
                              },
                            },
                          ],
                        },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "tenants" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "id" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "fullName" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "shortName" },
                            },
                          ],
                        },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "rentFullAmount" },
                      },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "rents" },
                        selectionSet: {
                          kind: "SelectionSet",
                          selections: [
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "id" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "periodStart" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "periodEnd" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "fullAmount" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "amount" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "status" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "leaseId" },
                            },
                            {
                              kind: "Field",
                              name: { kind: "Name", value: "transactions" },
                              selectionSet: {
                                kind: "SelectionSet",
                                selections: [
                                  {
                                    kind: "Field",
                                    name: { kind: "Name", value: "id" },
                                  },
                                ],
                              },
                            },
                          ],
                        },
                      },
                    ],
                  },
                },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  RentReceivedStatusQuery,
  RentReceivedStatusQueryVariables
>;
export const FileDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "query",
      name: { kind: "Name", value: "File" },
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "files" },
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "filename" } },
                { kind: "Field", name: { kind: "Name", value: "type" } },
                { kind: "Field", name: { kind: "Name", value: "createdAt" } },
                { kind: "Field", name: { kind: "Name", value: "downloadUrl" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<FileQuery, FileQueryVariables>;
export const UserCreateWithAccountDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "UserCreateWithAccount" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "input" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "UserWithAccountInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            alias: { kind: "Name", value: "accountCreate" },
            name: { kind: "Name", value: "userCreateWithAccount" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "input" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "input" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  UserCreateWithAccountMutation,
  UserCreateWithAccountMutationVariables
>;
export const AccountUpdatePaymentMethodDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "AccountUpdatePaymentMethod" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "input" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "AccountUpdateInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "accountUpdatePaymentMethod" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "input" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "input" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  AccountUpdatePaymentMethodMutation,
  AccountUpdatePaymentMethodMutationVariables
>;
export const AccountActivatePlanDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "AccountActivatePlan" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "input" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "AccountActivatePlanInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "accountActivatePlan" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "input" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "input" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "status" } },
                { kind: "Field", name: { kind: "Name", value: "trialEnd" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  AccountActivatePlanMutation,
  AccountActivatePlanMutationVariables
>;
export const TenantCreateDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "TenantCreate" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "input" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "TenantInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            alias: { kind: "Name", value: "tenant" },
            name: { kind: "Name", value: "tenantCreate" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "input" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "input" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "firstName" } },
                { kind: "Field", name: { kind: "Name", value: "lastName" } },
                { kind: "Field", name: { kind: "Name", value: "fullName" } },
                { kind: "Field", name: { kind: "Name", value: "email" } },
                { kind: "Field", name: { kind: "Name", value: "phoneNumber" } },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "lastTransaction" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "date" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "amount" },
                      },
                    ],
                  },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "propertyName" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "rentPayedThisYear" },
                },
                {
                  kind: "Field",
                  name: { kind: "Name", value: "unpaidRentAmount" },
                },
                { kind: "Field", name: { kind: "Name", value: "status" } },
                { kind: "Field", name: { kind: "Name", value: "note" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  TenantCreateMutation,
  TenantCreateMutationVariables
>;
export const TenantUpdateDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "TenantUpdate" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "input" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "TenantUpdateInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "tenantUpdate" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "input" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "input" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  TenantUpdateMutation,
  TenantUpdateMutationVariables
>;
export const TenantDeleteDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "TenantDelete" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "id" } },
          type: {
            kind: "NonNullType",
            type: { kind: "NamedType", name: { kind: "Name", value: "ID" } },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            alias: { kind: "Name", value: "tenantId" },
            name: { kind: "Name", value: "tenantDelete" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "id" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "id" },
                },
              },
            ],
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  TenantDeleteMutation,
  TenantDeleteMutationVariables
>;
export const PropertyCreateDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "PropertyCreate" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "input" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "PropertyInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "propertyCreate" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "input" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "input" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  PropertyCreateMutation,
  PropertyCreateMutationVariables
>;
export const PropertyUpdateDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "PropertyUpdate" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "input" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "PropertyUpdateInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "propertyUpdate" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "input" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "input" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  PropertyUpdateMutation,
  PropertyUpdateMutationVariables
>;
export const PropertyDeleteDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "PropertyDelete" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "id" } },
          type: {
            kind: "NonNullType",
            type: { kind: "NamedType", name: { kind: "Name", value: "ID" } },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "propertyDelete" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "id" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "id" },
                },
              },
            ],
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  PropertyDeleteMutation,
  PropertyDeleteMutationVariables
>;
export const LeaseCreateDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "LeaseCreate" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "input" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "LeaseFurnishedInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            alias: { kind: "Name", value: "leaseCreate" },
            name: { kind: "Name", value: "leaseFurnishedCreate" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "input" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "input" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  LeaseCreateMutation,
  LeaseCreateMutationVariables
>;
export const LeaseDeleteDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "LeaseDelete" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "id" } },
          type: {
            kind: "NonNullType",
            type: { kind: "NamedType", name: { kind: "Name", value: "ID" } },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "leaseDelete" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "id" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "id" },
                },
              },
            ],
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  LeaseDeleteMutation,
  LeaseDeleteMutationVariables
>;
export const ContractDeleteDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "ContractDelete" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "id" } },
          type: {
            kind: "NonNullType",
            type: { kind: "NamedType", name: { kind: "Name", value: "ID" } },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            alias: { kind: "Name", value: "leaseId" },
            name: { kind: "Name", value: "leaseDelete" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "id" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "id" },
                },
              },
            ],
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  ContractDeleteMutation,
  ContractDeleteMutationVariables
>;
export const ContractUpdateDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "ContractUpdate" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "input" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "LeaseFurnishedUpdateInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            alias: { kind: "Name", value: "leaseUpdate" },
            name: { kind: "Name", value: "leaseFurnishedUpdate" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "input" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "input" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  ContractUpdateMutation,
  ContractUpdateMutationVariables
>;
export const LenderIndividualUpdateDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "LenderIndividualUpdate" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "input" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "LenderIndividualUpdateInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "lenderIndividualUpdate" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "input" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "input" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  LenderIndividualUpdateMutation,
  LenderIndividualUpdateMutationVariables
>;
export const TransactionCreateDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "TransactionCreate" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "input" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "TransactionInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "transactionCreate" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "input" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "input" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  TransactionCreateMutation,
  TransactionCreateMutationVariables
>;
export const TransactionDeleteDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "TransactionDelete" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: { kind: "Variable", name: { kind: "Name", value: "id" } },
          type: {
            kind: "NonNullType",
            type: { kind: "NamedType", name: { kind: "Name", value: "ID" } },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            alias: { kind: "Name", value: "transactionId" },
            name: { kind: "Name", value: "transactionDelete" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "id" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "id" },
                },
              },
            ],
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  TransactionDeleteMutation,
  TransactionDeleteMutationVariables
>;
export const FileUploadDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "FileUpload" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "input" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "FileInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "fileUpload" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "input" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "input" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<FileUploadMutation, FileUploadMutationVariables>;
export const ImportUploadDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "ImportUpload" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "input" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "ImportInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "importUpload" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "input" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "input" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                { kind: "Field", name: { kind: "Name", value: "id" } },
                { kind: "Field", name: { kind: "Name", value: "status" } },
                { kind: "Field", name: { kind: "Name", value: "progress" } },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  ImportUploadMutation,
  ImportUploadMutationVariables
>;
export const RentReceiptCreateDocument = ({
  kind: "Document",
  definitions: [
    {
      kind: "OperationDefinition",
      operation: "mutation",
      name: { kind: "Name", value: "RentReceiptCreate" },
      variableDefinitions: [
        {
          kind: "VariableDefinition",
          variable: {
            kind: "Variable",
            name: { kind: "Name", value: "input" },
          },
          type: {
            kind: "NonNullType",
            type: {
              kind: "NamedType",
              name: { kind: "Name", value: "RentReceiptInput" },
            },
          },
        },
      ],
      selectionSet: {
        kind: "SelectionSet",
        selections: [
          {
            kind: "Field",
            name: { kind: "Name", value: "rentReceiptCreate" },
            arguments: [
              {
                kind: "Argument",
                name: { kind: "Name", value: "input" },
                value: {
                  kind: "Variable",
                  name: { kind: "Name", value: "input" },
                },
              },
            ],
            selectionSet: {
              kind: "SelectionSet",
              selections: [
                {
                  kind: "Field",
                  name: { kind: "Name", value: "receipt" },
                  selectionSet: {
                    kind: "SelectionSet",
                    selections: [
                      { kind: "Field", name: { kind: "Name", value: "id" } },
                      {
                        kind: "Field",
                        name: { kind: "Name", value: "downloadUrl" },
                      },
                    ],
                  },
                },
              ],
            },
          },
        ],
      },
    },
  ],
} as unknown) as DocumentNode<
  RentReceiptCreateMutation,
  RentReceiptCreateMutationVariables
>;
