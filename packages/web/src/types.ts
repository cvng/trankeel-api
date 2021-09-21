import gql from "graphql-tag";
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
  DateTime: string;
  Decimal: number;
  Email: string;
  PhoneNumber: string;
  UUID: string;
}

export interface Account {
  __typename?: "Account";
  planId?: Maybe<Scalars["ID"]>;
  status?: Maybe<SubscriptionStatus>;
  stripeCustomerId?: Maybe<Scalars["String"]>;
  stripeSubscriptionId?: Maybe<Scalars["String"]>;
  trialEnd?: Maybe<Scalars["DateTime"]>;
  id: Scalars["ID"];
  plan?: Maybe<Plan>;
}

export interface AccountActivatePlanInput {
  id: Scalars["UUID"];
  name: Scalars["String"];
  planCode: PlanCode;
}

export interface AccountUpdateInput {
  id: Scalars["UUID"];
  paymentMethodId: Scalars["String"];
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

export interface Error {
  __typename?: "Error";
  message: Scalars["String"];
}

export interface Event {
  __typename?: "Event";
  id: Scalars["ID"];
  createdAt: Scalars["DateTime"];
  eventableId: Scalars["ID"];
  eventableType: Scalars["String"];
  type: EventType;
  object: Eventable;
}

export enum EventType {
  RentReceiptCreated = "RENT_RECEIPT_CREATED",
  RentReceiptSent = "RENT_RECEIPT_SENT",
  TransactionCreated = "TRANSACTION_CREATED",
}

export type Eventable = Rent | Transaction;

export interface Feature {
  __typename?: "Feature";
  available: Scalars["Boolean"];
  title: Scalars["String"];
  key?: Maybe<Scalars["String"]>;
}

export interface File {
  __typename?: "File";
  createdAt?: Maybe<Scalars["DateTime"]>;
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
  duration: LeaseFurnishedDuration;
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
  rents: Array<Rent>;
  lease?: Maybe<File>;
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
  rentIrlRevisionDate?: Maybe<Scalars["DateTime"]>;
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
  tenantLastRentReceivedDate?: Maybe<Scalars["DateTime"]>;
  tenantLastRentRevisionDate?: Maybe<Scalars["DateTime"]>;
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
  rentIrlRevisionDate?: Maybe<Scalars["DateTime"]>;
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
  tenantLastRentReceivedDate?: Maybe<Scalars["DateTime"]>;
  tenantLastRentRevisionDate?: Maybe<Scalars["DateTime"]>;
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
  effectDate: Scalars["DateTime"];
  renewDate?: Maybe<Scalars["DateTime"]>;
  file?: Maybe<FileInput>;
  propertyId: Scalars["UUID"];
  rentAmount: Scalars["Decimal"];
  rentChargesAmount?: Maybe<Scalars["Decimal"]>;
  signatureDate?: Maybe<Scalars["DateTime"]>;
  tenantIds: Array<Scalars["UUID"]>;
}

export interface LeaseFurnishedUpdateInput {
  data?: Maybe<LeaseFurnishedDataInput>;
  file?: Maybe<FileInput>;
  id: Scalars["UUID"];
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
  accountId: Scalars["ID"];
  individualId?: Maybe<Scalars["ID"]>;
  companyId?: Maybe<Scalars["ID"]>;
  displayName: Scalars["String"];
  identity: Identity;
}

export interface LenderIndividualUpdateInput {
  id: Scalars["UUID"];
  individual: UserUpdateInput;
}

export interface Mutation {
  __typename?: "Mutation";
  userCreateWithAccount: User;
  accountUpdatePaymentMethod: Account;
  accountActivatePlan: Account;
  tenantCreate: Tenant;
  tenantUpdate: Tenant;
  tenantDelete: Scalars["ID"];
  propertyCreate: Property;
  propertyUpdate: Property;
  propertyDelete: Scalars["ID"];
  leaseFurnishedCreate: Lease;
  leaseFurnishedUpdate: Lease;
  leaseDelete: Scalars["ID"];
  lenderIndividualUpdate: Lender;
  transactionCreate: Transaction;
  transactionDelete: Scalars["ID"];
  fileUpload: File;
  importUpload: Task;
  rentReceiptCreate: RentReceiptPayload;
  sendPaymentNotice: SendPaymentNoticePayload;
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

export interface MutationLeaseFurnishedUpdateArgs {
  input: LeaseFurnishedUpdateInput;
}

export interface MutationLeaseDeleteArgs {
  id: Scalars["ID"];
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

export interface MutationSendPaymentNoticeArgs {
  input: SendPaymentNoticeInput;
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
  accountId: Scalars["ID"];
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
  tax?: Maybe<Scalars["Decimal"]>;
  roomCount: PropertyRoomType;
  status?: Maybe<PropertyStatus>;
  surface: Scalars["Float"];
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
  lenderId: Scalars["UUID"];
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
  id: Scalars["UUID"];
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
  rents: Array<Rent>;
  events: Array<Event>;
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

export interface QueryRentsArgs {
  since: Scalars["DateTime"];
  until: Scalars["DateTime"];
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
  noticeId?: Maybe<Scalars["ID"]>;
  lease?: Maybe<Lease>;
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
  rentIds: Array<Scalars["UUID"]>;
  sendMail: Scalars["Boolean"];
}

export interface RentReceiptPayload {
  __typename?: "RentReceiptPayload";
  errors?: Maybe<Array<Error>>;
  receipts?: Maybe<Array<File>>;
}

export enum RentStatus {
  Partial = "PARTIAL",
  Pending = "PENDING",
  Settled = "SETTLED",
}

export interface SendPaymentNoticeInput {
  rentIds: Array<Scalars["UUID"]>;
  date?: Maybe<Scalars["DateTime"]>;
}

export interface SendPaymentNoticePayload {
  __typename?: "SendPaymentNoticePayload";
  errors?: Maybe<Array<Error>>;
  notices?: Maybe<Array<File>>;
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
  birthdate: Scalars["DateTime"];
  birthplace?: Maybe<Scalars["String"]>;
  email: Scalars["Email"];
  firstName: Scalars["String"];
  lastName: Scalars["String"];
  displayName: Scalars["String"];
  note?: Maybe<Scalars["String"]>;
  phoneNumber?: Maybe<Scalars["PhoneNumber"]>;
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
  birthdate: Scalars["DateTime"];
  birthplace?: Maybe<Scalars["String"]>;
  email: Scalars["String"];
  firstName: Scalars["String"];
  lastName: Scalars["String"];
  note?: Maybe<Scalars["String"]>;
  phoneNumber?: Maybe<Scalars["String"]>;
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
  birthdate?: Maybe<Scalars["DateTime"]>;
  birthplace?: Maybe<Scalars["String"]>;
  email?: Maybe<Scalars["String"]>;
  id: Scalars["UUID"];
  firstName?: Maybe<Scalars["String"]>;
  lastName?: Maybe<Scalars["String"]>;
  note?: Maybe<Scalars["String"]>;
  phoneNumber?: Maybe<Scalars["String"]>;
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
  leaseId: Scalars["UUID"];
  date: Scalars["DateTime"];
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
  accountId: Scalars["ID"];
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
  address?: Maybe<AddressInput>;
  firstName?: Maybe<Scalars["String"]>;
  lastName?: Maybe<Scalars["String"]>;
}

export interface UserWithAccountInput {
  address?: Maybe<AddressInput>;
  authId: Scalars["AuthenticationID"];
  email: Scalars["String"];
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

export type TestQuery = {
  __typename?: "Query";
  __schema: {
    __typename?: "__Schema";
    types: Array<{ __typename?: "__Type"; name?: Maybe<string> }>;
  };
};

export type UserQueryVariables = Exact<{ [key: string]: never }>;

export type UserQuery = {
  __typename?: "Query";
  user: {
    __typename?: "User";
    id: string;
    email: string;
    firstName?: Maybe<string>;
    lastName?: Maybe<string>;
    displayName: string;
    photoURL?: Maybe<string>;
    address?: Maybe<{
      __typename?: "Address";
      line1: string;
      line2?: Maybe<string>;
      city: string;
      postalCode: string;
    }>;
    account?: Maybe<{
      __typename?: "Account";
      id: string;
      status?: Maybe<SubscriptionStatus>;
      trialEnd?: Maybe<string>;
      plan?: Maybe<{ __typename?: "Plan"; code: PlanCode }>;
    }>;
  };
};

export type LenderQueryVariables = Exact<{ [key: string]: never }>;

export type LenderQuery = {
  __typename?: "Query";
  lenders: Array<{
    __typename?: "Lender";
    id: string;
    accountId: string;
    displayName: string;
    identity:
      | {
          __typename?: "User";
          id: string;
          email: string;
          displayName: string;
          firstName?: Maybe<string>;
          lastName?: Maybe<string>;
          address?: Maybe<{
            __typename?: "Address";
            inline: string;
            line1: string;
            line2?: Maybe<string>;
            city: string;
            postalCode: string;
          }>;
        }
      | {
          __typename?: "Company";
          id: string;
          email: string;
          displayName: string;
          legalEntity: string;
          legalEntityIdentifier?: Maybe<string>;
          address?: Maybe<{
            __typename?: "Address";
            inline: string;
            line1: string;
            line2?: Maybe<string>;
            city: string;
            postalCode: string;
          }>;
        };
  }>;
};

export type TenantWithRentalReceiptsQueryVariables = Exact<{
  id: Scalars["ID"];
}>;

export type TenantWithRentalReceiptsQuery = {
  __typename?: "Query";
  tenants: Array<{
    __typename?: "Tenant";
    id: string;
    firstName: string;
    lastName: string;
    fullName: string;
    email: string;
    phoneNumber?: Maybe<string>;
    propertyName?: Maybe<string>;
    rentPayedThisYear?: Maybe<string>;
    unpaidRentAmount?: Maybe<number>;
    status?: Maybe<TenantStatus>;
    note?: Maybe<string>;
    lastTransaction?: Maybe<{
      __typename?: "Transaction";
      date: string;
      amount: number;
    }>;
  }>;
};

export type TenantListQueryVariables = Exact<{
  id?: Maybe<Scalars["ID"]>;
  query?: Maybe<Scalars["String"]>;
  status?: Maybe<TenantStatus>;
}>;

export type TenantListQuery = {
  __typename?: "Query";
  tenants: Array<{
    __typename?: "Tenant";
    id: string;
    firstName: string;
    lastName: string;
    fullName: string;
    email: string;
    phoneNumber?: Maybe<string>;
    propertyName?: Maybe<string>;
    rentPayedThisYear?: Maybe<string>;
    unpaidRentAmount?: Maybe<number>;
    status?: Maybe<TenantStatus>;
    note?: Maybe<string>;
    visaleId?: Maybe<string>;
    apl?: Maybe<boolean>;
    birthdate: string;
    birthplace?: Maybe<string>;
    displayName: string;
    accountId: string;
    lastTransaction?: Maybe<{
      __typename?: "Transaction";
      id: string;
      date: string;
      amount: number;
      accountId: string;
      type: TransactionType;
    }>;
    files?: Maybe<
      Array<{
        __typename?: "File";
        id: string;
        filename?: Maybe<string>;
        createdAt?: Maybe<string>;
        type: FileType;
        downloadUrl?: Maybe<string>;
      }>
    >;
    lease?: Maybe<{
      __typename?: "Lease";
      id: string;
      type: LeaseType;
      status: LeaseStatus;
      rentFullAmount: number;
      rentAmount: number;
      rentChargesAmount?: Maybe<number>;
      effectDate: string;
      renewDate?: Maybe<string>;
      signatureDate?: Maybe<string>;
      depositAmount?: Maybe<number>;
      accountId: string;
      propertyId: string;
      rents: Array<{
        __typename?: "Rent";
        id: string;
        periodStart: string;
        periodEnd: string;
        fullAmount: number;
        amount: number;
        status: RentStatus;
        leaseId: string;
      }>;
      data?: Maybe<{
        __typename?: "LeaseFurnishedData";
        duration?: Maybe<LeaseFurnishedDuration>;
      }>;
      account?: Maybe<{ __typename?: "Account"; id: string }>;
      property?: Maybe<{
        __typename?: "Property";
        id: string;
        buildPeriod?: Maybe<PropertyBuildPeriodType>;
        buildingLegalStatus?: Maybe<PropertyBuildingLegalStatus>;
        heatingMethod?: Maybe<PropertyUsageType>;
        housingType?: Maybe<PropertyUsageType>;
        name: string;
        roomCount: PropertyRoomType;
        surface: number;
        usageType?: Maybe<PropertyHabitationUsageType>;
        waterHeatingMethod?: Maybe<PropertyUsageType>;
        address: {
          __typename?: "Address";
          city: string;
          inline: string;
          line1: string;
          postalCode: string;
        };
        lender?: Maybe<{
          __typename?: "Lender";
          id: string;
          displayName: string;
          identity:
            | {
                __typename?: "User";
                address?: Maybe<{ __typename?: "Address"; inline: string }>;
              }
            | {
                __typename?: "Company";
                address?: Maybe<{ __typename?: "Address"; inline: string }>;
              };
        }>;
      }>;
      tenants: Array<{
        __typename?: "Tenant";
        id: string;
        displayName: string;
        email: string;
        firstName: string;
        lastName: string;
        accountId: string;
        birthdate: string;
        fullName: string;
        account?: Maybe<{ __typename?: "Account"; id: string }>;
      }>;
    }>;
    account?: Maybe<{ __typename?: "Account"; id: string }>;
  }>;
};

export type LeaseListQueryVariables = Exact<{
  id?: Maybe<Scalars["ID"]>;
  query?: Maybe<Scalars["String"]>;
}>;

export type LeaseListQuery = {
  __typename?: "Query";
  leases: Array<{
    __typename?: "Lease";
    id: string;
    accountId: string;
    propertyId: string;
    status: LeaseStatus;
    renewDate?: Maybe<string>;
    type: LeaseType;
    rentFullAmount: number;
    effectDate: string;
    rentAmount: number;
    depositAmount?: Maybe<number>;
    rentChargesAmount?: Maybe<number>;
    account?: Maybe<{ __typename?: "Account"; id: string }>;
    property?: Maybe<{
      __typename?: "Property";
      id: string;
      buildPeriod?: Maybe<PropertyBuildPeriodType>;
      buildingLegalStatus?: Maybe<PropertyBuildingLegalStatus>;
      heatingMethod?: Maybe<PropertyUsageType>;
      housingType?: Maybe<PropertyUsageType>;
      name: string;
      roomCount: PropertyRoomType;
      surface: number;
      usageType?: Maybe<PropertyHabitationUsageType>;
      waterHeatingMethod?: Maybe<PropertyUsageType>;
      address: {
        __typename?: "Address";
        city: string;
        inline: string;
        line1: string;
        postalCode: string;
      };
      lender?: Maybe<{
        __typename?: "Lender";
        id: string;
        displayName: string;
        identity:
          | {
              __typename?: "User";
              id: string;
              email: string;
              firstName?: Maybe<string>;
              lastName?: Maybe<string>;
              displayName: string;
              address?: Maybe<{ __typename?: "Address"; inline: string }>;
            }
          | {
              __typename?: "Company";
              id: string;
              displayName: string;
              email: string;
              legalEntity: string;
              legalEntityIdentifier?: Maybe<string>;
              address?: Maybe<{ __typename?: "Address"; inline: string }>;
            };
      }>;
    }>;
    tenants: Array<{
      __typename?: "Tenant";
      id: string;
      email: string;
      displayName: string;
      firstName: string;
      lastName: string;
      fullName: string;
      accountId: string;
      birthdate: string;
      account?: Maybe<{ __typename?: "Account"; id: string }>;
    }>;
    file?: Maybe<{
      __typename?: "File";
      id: string;
      downloadUrl?: Maybe<string>;
      type: FileType;
    }>;
    data?: Maybe<{
      __typename?: "LeaseFurnishedData";
      duration?: Maybe<LeaseFurnishedDuration>;
      rentPaymentMethod?: Maybe<RentPaymentMethod>;
    }>;
  }>;
};

export type LeaseQueryVariables = Exact<{
  id: Scalars["ID"];
}>;

export type LeaseQuery = {
  __typename?: "Query";
  leases: Array<{
    __typename?: "Lease";
    id: string;
    accountId: string;
    status: LeaseStatus;
    rentFullAmount: number;
    effectDate: string;
    rentAmount: number;
    depositAmount?: Maybe<number>;
    rentChargesAmount?: Maybe<number>;
    tenants: Array<{ __typename?: "Tenant"; id: string; fullName: string }>;
    file?: Maybe<{ __typename?: "File"; downloadUrl?: Maybe<string> }>;
    data?: Maybe<{
      __typename?: "LeaseFurnishedData";
      duration?: Maybe<LeaseFurnishedDuration>;
      rentPaymentMethod?: Maybe<RentPaymentMethod>;
    }>;
  }>;
};

export type RentListQueryVariables = Exact<{
  since: Scalars["DateTime"];
  until: Scalars["DateTime"];
}>;

export type RentListQuery = {
  __typename?: "Query";
  rents: Array<{
    __typename?: "Rent";
    id: string;
    periodStart: string;
    periodEnd: string;
    status: RentStatus;
    amount: number;
    chargesAmount?: Maybe<number>;
    fullAmount: number;
    leaseId: string;
    delay?: Maybe<number>;
    lease?: Maybe<{
      __typename?: "Lease";
      id: string;
      rentFullAmount: number;
      tenants: Array<{
        __typename?: "Tenant";
        shortName?: Maybe<string>;
        fullName: string;
      }>;
    }>;
  }>;
};

export type ContractRequirementDataQueryVariables = Exact<{
  query?: Maybe<Scalars["String"]>;
}>;

export type ContractRequirementDataQuery = {
  __typename?: "Query";
  tenants: Array<{
    __typename?: "Tenant";
    id: string;
    fullName: string;
    status?: Maybe<TenantStatus>;
    lease?: Maybe<{
      __typename?: "Lease";
      id: string;
      renewDate?: Maybe<string>;
      type: LeaseType;
    }>;
  }>;
};

export type PropertyListQueryVariables = Exact<{
  id?: Maybe<Scalars["ID"]>;
  query?: Maybe<Scalars["String"]>;
}>;

export type PropertyListQuery = {
  __typename?: "Query";
  properties: Array<{
    __typename?: "Property";
    id: string;
    name: string;
    roomCount: PropertyRoomType;
    note?: Maybe<string>;
    collectedRents?: Maybe<number>;
    surface: number;
    status?: Maybe<PropertyStatus>;
    energyClass?: Maybe<PropertyEnergyClass>;
    gasEmission?: Maybe<PropertyGasEmission>;
    tenantPrivateSpaces?: Maybe<string>;
    equipments?: Maybe<string>;
    nticEquipments?: Maybe<string>;
    otherSpaces?: Maybe<string>;
    commonSpaces?: Maybe<string>;
    waterHeatingMethod?: Maybe<PropertyUsageType>;
    heatingMethod?: Maybe<PropertyUsageType>;
    tax?: Maybe<number>;
    buildPeriod?: Maybe<PropertyBuildPeriodType>;
    housingType?: Maybe<PropertyUsageType>;
    buildingLegalStatus?: Maybe<PropertyBuildingLegalStatus>;
    usageType?: Maybe<PropertyHabitationUsageType>;
    address: {
      __typename?: "Address";
      city: string;
      country?: Maybe<string>;
      inline: string;
      line1: string;
      line2?: Maybe<string>;
      postalCode: string;
    };
    lender?: Maybe<{ __typename?: "Lender"; id: string; displayName: string }>;
    leases?: Maybe<
      Array<{
        __typename?: "Lease";
        id: string;
        status: LeaseStatus;
        type: LeaseType;
        effectDate: string;
        renewDate?: Maybe<string>;
        signatureDate?: Maybe<string>;
        rentAmount: number;
        depositAmount?: Maybe<number>;
        rentChargesAmount?: Maybe<number>;
        rentFullAmount: number;
        accountId: string;
        propertyId: string;
        account?: Maybe<{ __typename?: "Account"; id: string }>;
        tenants: Array<{
          __typename?: "Tenant";
          id: string;
          fullName: string;
          status?: Maybe<TenantStatus>;
          displayName: string;
          email: string;
          firstName: string;
          lastName: string;
          accountId: string;
          birthdate: string;
          account?: Maybe<{ __typename?: "Account"; id: string }>;
        }>;
        property?: Maybe<{
          __typename?: "Property";
          id: string;
          buildPeriod?: Maybe<PropertyBuildPeriodType>;
          buildingLegalStatus?: Maybe<PropertyBuildingLegalStatus>;
          heatingMethod?: Maybe<PropertyUsageType>;
          housingType?: Maybe<PropertyUsageType>;
          name: string;
          roomCount: PropertyRoomType;
          surface: number;
          usageType?: Maybe<PropertyHabitationUsageType>;
          waterHeatingMethod?: Maybe<PropertyUsageType>;
          address: {
            __typename?: "Address";
            city: string;
            inline: string;
            line1: string;
            postalCode: string;
          };
          lender?: Maybe<{
            __typename?: "Lender";
            id: string;
            displayName: string;
          }>;
        }>;
      }>
    >;
  }>;
};

export type TransactionQueryVariables = Exact<{
  id: Scalars["ID"];
}>;

export type TransactionQuery = {
  __typename?: "Query";
  transactions: Array<{
    __typename?: "Transaction";
    id: string;
    date: string;
    amount: number;
    type: TransactionType;
    lease?: Maybe<{ __typename?: "Lease"; id: string }>;
  }>;
};

export type InvoiceListQueryVariables = Exact<{ [key: string]: never }>;

export type InvoiceListQuery = {
  __typename?: "Query";
  invoices: Array<{
    __typename?: "Invoice";
    id: string;
    number: number;
    amountPaid: number;
    invoicePdf: string;
    periodEnd: string;
    status: string;
    planCode: PlanCode;
  }>;
};

export type PricingPlansQueryVariables = Exact<{ [key: string]: never }>;

export type PricingPlansQuery = {
  __typename?: "Query";
  plans: Array<{
    __typename?: "Plan";
    title?: Maybe<string>;
    subtitle?: Maybe<string>;
    id: string;
    price?: Maybe<number>;
    code: PlanCode;
    features: Array<{
      __typename?: "Feature";
      available: boolean;
      title: string;
      key?: Maybe<string>;
    }>;
  }>;
};

export type RentalReceiptDataQueryVariables = Exact<{ [key: string]: never }>;

export type RentalReceiptDataQuery = {
  __typename?: "Query";
  tenants: Array<{
    __typename?: "Tenant";
    id: string;
    firstName: string;
    lastName: string;
    fullName: string;
    propertyName?: Maybe<string>;
    lease?: Maybe<{
      __typename?: "Lease";
      id: string;
      rentAmount: number;
      rentChargesAmount?: Maybe<number>;
      rentFullAmount: number;
      property?: Maybe<{
        __typename?: "Property";
        id: string;
        address: {
          __typename?: "Address";
          city: string;
          country?: Maybe<string>;
          inline: string;
          line1: string;
          line2?: Maybe<string>;
          postalCode: string;
        };
      }>;
    }>;
  }>;
};

export type RentReceivedSummaryQueryVariables = Exact<{
  since: Scalars["DateTime"];
  until: Scalars["DateTime"];
}>;

export type RentReceivedSummaryQuery = {
  __typename?: "Query";
  rentReceivedSummary: {
    __typename?: "Summary";
    since: string;
    until: string;
    amountExpected: number;
    amountReceived: number;
    amountSettled: number;
    amountPartial: number;
    amountPending: number;
    nExpected: number;
    nReceived: number;
    nSettled: number;
    nPartial: number;
    nPending: number;
    ratioExpected: number;
    ratioReceived: number;
    ratioSettled: number;
    ratioPartial: number;
    ratioPending: number;
    variationExpected: number;
    variationReceived: number;
    variationSettled: number;
    variationPartial: number;
    variationPending: number;
    paymentRate: number;
    occupationRate: number;
  };
};

export type RecentActivityListQueryVariables = Exact<{ [key: string]: never }>;

export type RecentActivityListQuery = {
  __typename?: "Query";
  events: Array<{
    __typename?: "Event";
    id: string;
    eventableId: string;
    eventableType: string;
    createdAt: string;
    type: EventType;
    object:
      | {
          __typename: "Rent";
          id: string;
          fullAmount: number;
          amount: number;
          leaseId: string;
          periodEnd: string;
          periodStart: string;
          lease?: Maybe<{
            __typename?: "Lease";
            effectDate: string;
            renewDate?: Maybe<string>;
            id: string;
            propertyId: string;
            rentAmount: number;
            status: LeaseStatus;
            type: LeaseType;
            property?: Maybe<{
              __typename?: "Property";
              id: string;
              buildPeriod?: Maybe<PropertyBuildPeriodType>;
              buildingLegalStatus?: Maybe<PropertyBuildingLegalStatus>;
              heatingMethod?: Maybe<PropertyUsageType>;
              housingType?: Maybe<PropertyUsageType>;
              name: string;
              roomCount: PropertyRoomType;
              surface: number;
              usageType?: Maybe<PropertyHabitationUsageType>;
              waterHeatingMethod?: Maybe<PropertyUsageType>;
              address: {
                __typename?: "Address";
                inline: string;
                line1: string;
                line2?: Maybe<string>;
                postalCode: string;
                city: string;
              };
            }>;
            tenants: Array<{
              __typename?: "Tenant";
              id: string;
              displayName: string;
              birthdate: string;
              email: string;
              firstName: string;
              fullName: string;
              lastName: string;
            }>;
          }>;
        }
      | {
          __typename: "Transaction";
          id: string;
          amount: number;
          type: TransactionType;
          date: string;
          lease?: Maybe<{
            __typename?: "Lease";
            effectDate: string;
            renewDate?: Maybe<string>;
            id: string;
            propertyId: string;
            rentAmount: number;
            status: LeaseStatus;
            type: LeaseType;
            property?: Maybe<{
              __typename?: "Property";
              id: string;
              buildPeriod?: Maybe<PropertyBuildPeriodType>;
              buildingLegalStatus?: Maybe<PropertyBuildingLegalStatus>;
              heatingMethod?: Maybe<PropertyUsageType>;
              housingType?: Maybe<PropertyUsageType>;
              name: string;
              roomCount: PropertyRoomType;
              surface: number;
              usageType?: Maybe<PropertyHabitationUsageType>;
              waterHeatingMethod?: Maybe<PropertyUsageType>;
              address: {
                __typename?: "Address";
                inline: string;
                line1: string;
                line2?: Maybe<string>;
                postalCode: string;
                city: string;
              };
            }>;
            tenants: Array<{
              __typename?: "Tenant";
              id: string;
              displayName: string;
              birthdate: string;
              email: string;
              firstName: string;
              fullName: string;
              lastName: string;
            }>;
          }>;
        };
  }>;
};

export type RentReceivedStatusQueryVariables = Exact<{ [key: string]: never }>;

export type RentReceivedStatusQuery = {
  __typename?: "Query";
  properties: Array<{
    __typename?: "Property";
    id: string;
    name: string;
    collectedRents?: Maybe<number>;
    expectedRents?: Maybe<number>;
    leases?: Maybe<
      Array<{
        __typename?: "Lease";
        id: string;
        rentFullAmount: number;
        property?: Maybe<{
          __typename?: "Property";
          lender?: Maybe<{
            __typename?: "Lender";
            id: string;
            identity:
              | {
                  __typename?: "User";
                  address?: Maybe<{ __typename?: "Address"; inline: string }>;
                }
              | {
                  __typename?: "Company";
                  address?: Maybe<{ __typename?: "Address"; inline: string }>;
                };
          }>;
        }>;
        tenants: Array<{
          __typename?: "Tenant";
          id: string;
          fullName: string;
          shortName?: Maybe<string>;
        }>;
        rents: Array<{
          __typename?: "Rent";
          id: string;
          periodStart: string;
          periodEnd: string;
          fullAmount: number;
          amount: number;
          status: RentStatus;
          leaseId: string;
          transactions?: Maybe<
            Array<{ __typename?: "Transaction"; id: string }>
          >;
        }>;
      }>
    >;
  }>;
};

export type FileQueryVariables = Exact<{ [key: string]: never }>;

export type FileQuery = {
  __typename?: "Query";
  files: Array<{
    __typename?: "File";
    id: string;
    filename?: Maybe<string>;
    type: FileType;
    createdAt?: Maybe<string>;
    downloadUrl?: Maybe<string>;
  }>;
};

export type UserCreateWithAccountMutationVariables = Exact<{
  input: UserWithAccountInput;
}>;

export type UserCreateWithAccountMutation = {
  __typename?: "Mutation";
  accountCreate: { __typename?: "User"; id: string };
};

export type AccountUpdatePaymentMethodMutationVariables = Exact<{
  input: AccountUpdateInput;
}>;

export type AccountUpdatePaymentMethodMutation = {
  __typename?: "Mutation";
  accountUpdatePaymentMethod: { __typename?: "Account"; id: string };
};

export type AccountActivatePlanMutationVariables = Exact<{
  input: AccountActivatePlanInput;
}>;

export type AccountActivatePlanMutation = {
  __typename?: "Mutation";
  accountActivatePlan: {
    __typename?: "Account";
    id: string;
    status?: Maybe<SubscriptionStatus>;
    trialEnd?: Maybe<string>;
  };
};

export type TenantCreateMutationVariables = Exact<{
  input: TenantInput;
}>;

export type TenantCreateMutation = {
  __typename?: "Mutation";
  tenant: {
    __typename?: "Tenant";
    id: string;
    firstName: string;
    lastName: string;
    fullName: string;
    email: string;
    phoneNumber?: Maybe<string>;
    propertyName?: Maybe<string>;
    rentPayedThisYear?: Maybe<string>;
    unpaidRentAmount?: Maybe<number>;
    status?: Maybe<TenantStatus>;
    note?: Maybe<string>;
    lastTransaction?: Maybe<{
      __typename?: "Transaction";
      date: string;
      amount: number;
    }>;
  };
};

export type TenantUpdateMutationVariables = Exact<{
  input: TenantUpdateInput;
}>;

export type TenantUpdateMutation = {
  __typename?: "Mutation";
  tenantUpdate: { __typename?: "Tenant"; id: string };
};

export type TenantDeleteMutationVariables = Exact<{
  id: Scalars["ID"];
}>;

export type TenantDeleteMutation = {
  __typename?: "Mutation";
  tenantId: string;
};

export type PropertyCreateMutationVariables = Exact<{
  input: PropertyInput;
}>;

export type PropertyCreateMutation = {
  __typename?: "Mutation";
  propertyCreate: { __typename?: "Property"; id: string };
};

export type PropertyUpdateMutationVariables = Exact<{
  input: PropertyUpdateInput;
}>;

export type PropertyUpdateMutation = {
  __typename?: "Mutation";
  propertyUpdate: { __typename?: "Property"; id: string };
};

export type PropertyDeleteMutationVariables = Exact<{
  id: Scalars["ID"];
}>;

export type PropertyDeleteMutation = {
  __typename?: "Mutation";
  propertyDelete: string;
};

export type LeaseCreateMutationVariables = Exact<{
  input: LeaseFurnishedInput;
}>;

export type LeaseCreateMutation = {
  __typename?: "Mutation";
  leaseCreate: { __typename?: "Lease"; id: string };
};

export type LeaseDeleteMutationVariables = Exact<{
  id: Scalars["ID"];
}>;

export type LeaseDeleteMutation = {
  __typename?: "Mutation";
  leaseDelete: string;
};

export type ContractDeleteMutationVariables = Exact<{
  id: Scalars["ID"];
}>;

export type ContractDeleteMutation = {
  __typename?: "Mutation";
  leaseId: string;
};

export type ContractUpdateMutationVariables = Exact<{
  input: LeaseFurnishedUpdateInput;
}>;

export type ContractUpdateMutation = {
  __typename?: "Mutation";
  leaseUpdate: { __typename?: "Lease"; id: string };
};

export type LenderIndividualUpdateMutationVariables = Exact<{
  input: LenderIndividualUpdateInput;
}>;

export type LenderIndividualUpdateMutation = {
  __typename?: "Mutation";
  lenderIndividualUpdate: { __typename?: "Lender"; id: string };
};

export type TransactionCreateMutationVariables = Exact<{
  input: TransactionInput;
}>;

export type TransactionCreateMutation = {
  __typename?: "Mutation";
  transactionCreate: { __typename?: "Transaction"; id: string };
};

export type TransactionDeleteMutationVariables = Exact<{
  id: Scalars["ID"];
}>;

export type TransactionDeleteMutation = {
  __typename?: "Mutation";
  transactionId: string;
};

export type FileUploadMutationVariables = Exact<{
  input: FileInput;
}>;

export type FileUploadMutation = {
  __typename?: "Mutation";
  fileUpload: { __typename?: "File"; id: string };
};

export type ImportUploadMutationVariables = Exact<{
  input: ImportInput;
}>;

export type ImportUploadMutation = {
  __typename?: "Mutation";
  importUpload: {
    __typename?: "Task";
    id: string;
    status: string;
    progress: number;
  };
};

export type RentReceiptCreateMutationVariables = Exact<{
  input: RentReceiptInput;
}>;

export type RentReceiptCreateMutation = {
  __typename?: "Mutation";
  rentReceiptCreate: {
    __typename?: "RentReceiptPayload";
    receipts?: Maybe<
      Array<{ __typename?: "File"; id: string; downloadUrl?: Maybe<string> }>
    >;
  };
};

export type SendPaymentNoticeMutationVariables = Exact<{
  input: SendPaymentNoticeInput;
}>;

export type SendPaymentNoticeMutation = {
  __typename?: "Mutation";
  sendPaymentNotice: {
    __typename?: "SendPaymentNoticePayload";
    notices?: Maybe<
      Array<{ __typename?: "File"; id: string; downloadUrl?: Maybe<string> }>
    >;
  };
};

export const Test = gql`
  query Test {
    __schema {
      types {
        name
      }
    }
  }
`;
export const User = gql`
  query User {
    user: viewer {
      id
      email
      firstName
      lastName
      displayName
      photoURL
      address {
        line1
        line2
        city
        postalCode
      }
      account {
        id
        status
        plan {
          code
        }
        trialEnd
      }
    }
  }
`;
export const Lender = gql`
  query Lender {
    lenders {
      id
      accountId
      displayName
      identity {
        ... on Company {
          id
          email
          displayName
          legalEntity
          legalEntityIdentifier
          address {
            inline
            line1
            line2
            city
            postalCode
          }
        }
        ... on User {
          id
          email
          displayName
          firstName
          lastName
          address {
            inline
            line1
            line2
            city
            postalCode
          }
        }
      }
    }
  }
`;
export const TenantWithRentalReceipts = gql`
  query TenantWithRentalReceipts($id: ID!) {
    tenants(id: $id) {
      id
      firstName
      lastName
      fullName
      email
      phoneNumber
      lastTransaction {
        date
        amount
      }
      propertyName
      rentPayedThisYear
      unpaidRentAmount
      status
      note
    }
  }
`;
export const TenantList = gql`
  query TenantList($id: ID, $query: String, $status: TenantStatus) {
    tenants(id: $id, query: $query, status: $status) {
      id
      firstName
      lastName
      fullName
      email
      phoneNumber
      lastTransaction {
        id
        date
        amount
        accountId
        type
      }
      propertyName
      rentPayedThisYear
      unpaidRentAmount
      status
      note
      visaleId
      apl
      files {
        id
        filename
        createdAt
        type
        downloadUrl
      }
      lease {
        id
        type
        status
        rents {
          id
          periodStart
          periodEnd
          fullAmount
          amount
          status
          leaseId
        }
        rentFullAmount
        rentAmount
        rentChargesAmount
        effectDate
        renewDate
        signatureDate
        rentAmount
        depositAmount
        renewDate
        data {
          ... on LeaseFurnishedData {
            duration
          }
        }
        accountId
        account {
          id
        }
        propertyId
        property {
          id
          address {
            city
            inline
            line1
            postalCode
          }
          buildPeriod
          buildingLegalStatus
          heatingMethod
          housingType
          name
          roomCount
          surface
          usageType
          waterHeatingMethod
          lender {
            id
            displayName
            identity {
              ... on User {
                address {
                  inline
                }
              }
              ... on Company {
                address {
                  inline
                }
              }
            }
          }
        }
        tenants {
          id
          displayName
          email
          firstName
          lastName
          accountId
          account {
            id
          }
          birthdate
          fullName
        }
      }
      birthdate
      birthplace
      displayName
      accountId
      account {
        id
      }
    }
  }
`;
export const LeaseList = gql`
  query LeaseList($id: ID, $query: String) {
    leases(id: $id, query: $query) {
      id
      accountId
      account {
        id
      }
      propertyId
      property {
        id
        address {
          city
          inline
          line1
          postalCode
        }
        buildPeriod
        buildingLegalStatus
        heatingMethod
        housingType
        name
        roomCount
        surface
        usageType
        waterHeatingMethod
        lender {
          id
          displayName
          identity {
            ... on User {
              id
              email
              firstName
              lastName
              displayName
              address {
                inline
              }
            }
            ... on Company {
              id
              displayName
              email
              legalEntity
              legalEntityIdentifier
              address {
                inline
              }
            }
          }
        }
      }
      status
      renewDate
      type
      tenants {
        id
        email
        displayName
        firstName
        lastName
        fullName
        accountId
        account {
          id
        }
        birthdate
      }
      file: lease {
        id
        downloadUrl
        type
      }
      rentFullAmount
      effectDate
      rentAmount
      depositAmount
      rentChargesAmount
      data {
        ... on LeaseFurnishedData {
          duration
          rentPaymentMethod
        }
      }
    }
  }
`;
export const Lease = gql`
  query Lease($id: ID!) {
    leases(id: $id) {
      id
      accountId
      status
      tenants {
        id
        fullName
      }
      file: lease {
        downloadUrl
      }
      rentFullAmount
      effectDate
      rentAmount
      depositAmount
      rentChargesAmount
      data {
        ... on LeaseFurnishedData {
          duration
          rentPaymentMethod
        }
      }
    }
  }
`;
export const RentList = gql`
  query RentList($since: DateTime!, $until: DateTime!) {
    rents(since: $since, until: $until) {
      id
      periodStart
      periodEnd
      status
      amount
      chargesAmount
      fullAmount
      leaseId
      delay
      lease {
        id
        rentFullAmount
        tenants {
          shortName
          fullName
        }
      }
    }
  }
`;
export const ContractRequirementData = gql`
  query ContractRequirementData($query: String) {
    tenants(query: $query) {
      id
      fullName
      status
      lease {
        id
        renewDate
        type
      }
    }
  }
`;
export const PropertyList = gql`
  query PropertyList($id: ID, $query: String) {
    properties(id: $id, query: $query) {
      id
      name
      address {
        city
        country
        inline
        line1
        line2
        postalCode
      }
      roomCount
      note
      lender {
        id
        displayName
      }
      collectedRents
      surface
      status
      energyClass
      gasEmission
      tenantPrivateSpaces
      equipments
      nticEquipments
      otherSpaces
      commonSpaces
      waterHeatingMethod
      heatingMethod
      tax
      buildPeriod
      housingType
      buildingLegalStatus
      usageType
      leases {
        id
        status
        type
        effectDate
        renewDate
        signatureDate
        rentAmount
        depositAmount
        renewDate
        rentChargesAmount
        rentFullAmount
        accountId
        account {
          id
        }
        tenants {
          id
          fullName
          status
          displayName
          email
          firstName
          lastName
          accountId
          account {
            id
          }
          birthdate
        }
        propertyId
        property {
          id
          address {
            city
            inline
            line1
            postalCode
          }
          buildPeriod
          buildingLegalStatus
          heatingMethod
          housingType
          lender {
            id
            displayName
          }
          name
          roomCount
          surface
          usageType
          waterHeatingMethod
        }
      }
    }
  }
`;
export const Transaction = gql`
  query Transaction($id: ID!) {
    transactions(id: $id) {
      id
      date
      amount
      type
      lease {
        id
      }
    }
  }
`;
export const InvoiceList = gql`
  query InvoiceList {
    invoices {
      id
      number
      amountPaid
      invoicePdf
      periodEnd
      status
      planCode
    }
  }
`;
export const PricingPlans = gql`
  query PricingPlans {
    plans {
      title
      subtitle
      id
      price
      code
      features {
        available
        title
        key
      }
    }
  }
`;
export const RentalReceiptData = gql`
  query RentalReceiptData {
    tenants {
      id
      firstName
      lastName
      fullName
      propertyName
      lease {
        id
        property {
          id
          address {
            city
            country
            inline
            line1
            line2
            postalCode
          }
        }
        rentAmount
        rentChargesAmount
        rentFullAmount
      }
    }
  }
`;
export const RentReceivedSummary = gql`
  query RentReceivedSummary($since: DateTime!, $until: DateTime!) {
    rentReceivedSummary: summary(since: $since, until: $until) {
      since
      until
      amountExpected
      amountReceived
      amountSettled
      amountPartial
      amountPending
      nExpected
      nReceived
      nSettled
      nPartial
      nPending
      ratioExpected
      ratioReceived
      ratioSettled
      ratioPartial
      ratioPending
      variationExpected
      variationReceived
      variationSettled
      variationPartial
      variationPending
      paymentRate
      occupationRate
    }
  }
`;
export const RecentActivityList = gql`
  query RecentActivityList {
    events {
      id
      eventableId
      eventableType
      createdAt
      type
      object {
        __typename
        ... on Rent {
          id
          fullAmount
          amount
          leaseId
          periodEnd
          periodStart
          lease {
            effectDate
            renewDate
            id
            propertyId
            property {
              id
              address {
                inline
                line1
                line2
                postalCode
                city
              }
              buildPeriod
              buildingLegalStatus
              heatingMethod
              housingType
              name
              roomCount
              surface
              usageType
              waterHeatingMethod
            }
            rentAmount
            status
            type
            tenants {
              id
              displayName
              birthdate
              email
              firstName
              fullName
              lastName
            }
          }
        }
        ... on Transaction {
          id
          amount
          type
          date
          lease {
            effectDate
            renewDate
            id
            propertyId
            property {
              id
              address {
                inline
                line1
                line2
                postalCode
                city
              }
              buildPeriod
              buildingLegalStatus
              heatingMethod
              housingType
              name
              roomCount
              surface
              usageType
              waterHeatingMethod
            }
            rentAmount
            status
            type
            tenants {
              id
              displayName
              birthdate
              email
              firstName
              fullName
              lastName
            }
          }
        }
      }
    }
  }
`;
export const RentReceivedStatus = gql`
  query RentReceivedStatus {
    properties {
      id
      name
      collectedRents
      expectedRents
      leases {
        id
        property {
          lender {
            id
            identity {
              ... on User {
                address {
                  inline
                }
              }
              ... on Company {
                address {
                  inline
                }
              }
            }
          }
        }
        tenants {
          id
          fullName
          shortName
        }
        rentFullAmount
        rents {
          id
          periodStart
          periodEnd
          fullAmount
          amount
          status
          leaseId
          transactions {
            id
          }
        }
      }
    }
  }
`;
export const File = gql`
  query File {
    files {
      id
      filename
      type
      createdAt
      downloadUrl
    }
  }
`;
export const UserCreateWithAccount = gql`
  mutation UserCreateWithAccount($input: UserWithAccountInput!) {
    accountCreate: userCreateWithAccount(input: $input) {
      id
    }
  }
`;
export const AccountUpdatePaymentMethod = gql`
  mutation AccountUpdatePaymentMethod($input: AccountUpdateInput!) {
    accountUpdatePaymentMethod(input: $input) {
      id
    }
  }
`;
export const AccountActivatePlan = gql`
  mutation AccountActivatePlan($input: AccountActivatePlanInput!) {
    accountActivatePlan(input: $input) {
      id
      status
      trialEnd
    }
  }
`;
export const TenantCreate = gql`
  mutation TenantCreate($input: TenantInput!) {
    tenant: tenantCreate(input: $input) {
      id
      firstName
      lastName
      fullName
      email
      phoneNumber
      lastTransaction {
        date
        amount
      }
      propertyName
      rentPayedThisYear
      unpaidRentAmount
      status
      note
    }
  }
`;
export const TenantUpdate = gql`
  mutation TenantUpdate($input: TenantUpdateInput!) {
    tenantUpdate(input: $input) {
      id
    }
  }
`;
export const TenantDelete = gql`
  mutation TenantDelete($id: ID!) {
    tenantId: tenantDelete(id: $id)
  }
`;
export const PropertyCreate = gql`
  mutation PropertyCreate($input: PropertyInput!) {
    propertyCreate(input: $input) {
      id
    }
  }
`;
export const PropertyUpdate = gql`
  mutation PropertyUpdate($input: PropertyUpdateInput!) {
    propertyUpdate(input: $input) {
      id
    }
  }
`;
export const PropertyDelete = gql`
  mutation PropertyDelete($id: ID!) {
    propertyDelete(id: $id)
  }
`;
export const LeaseCreate = gql`
  mutation LeaseCreate($input: LeaseFurnishedInput!) {
    leaseCreate: leaseFurnishedCreate(input: $input) {
      id
    }
  }
`;
export const LeaseDelete = gql`
  mutation LeaseDelete($id: ID!) {
    leaseDelete(id: $id)
  }
`;
export const ContractDelete = gql`
  mutation ContractDelete($id: ID!) {
    leaseId: leaseDelete(id: $id)
  }
`;
export const ContractUpdate = gql`
  mutation ContractUpdate($input: LeaseFurnishedUpdateInput!) {
    leaseUpdate: leaseFurnishedUpdate(input: $input) {
      id
    }
  }
`;
export const LenderIndividualUpdate = gql`
  mutation LenderIndividualUpdate($input: LenderIndividualUpdateInput!) {
    lenderIndividualUpdate(input: $input) {
      id
    }
  }
`;
export const TransactionCreate = gql`
  mutation TransactionCreate($input: TransactionInput!) {
    transactionCreate(input: $input) {
      id
    }
  }
`;
export const TransactionDelete = gql`
  mutation TransactionDelete($id: ID!) {
    transactionId: transactionDelete(id: $id)
  }
`;
export const FileUpload = gql`
  mutation FileUpload($input: FileInput!) {
    fileUpload(input: $input) {
      id
    }
  }
`;
export const ImportUpload = gql`
  mutation ImportUpload($input: ImportInput!) {
    importUpload(input: $input) {
      id
      status
      progress
    }
  }
`;
export const RentReceiptCreate = gql`
  mutation RentReceiptCreate($input: RentReceiptInput!) {
    rentReceiptCreate(input: $input) {
      receipts {
        id
        downloadUrl
      }
    }
  }
`;
export const SendPaymentNotice = gql`
  mutation SendPaymentNotice($input: SendPaymentNoticeInput!) {
    sendPaymentNotice(input: $input) {
      notices {
        id
        downloadUrl
      }
    }
  }
`;
