import {
  FileType,
  ImportSource,
  LeaseFurnishedDuration,
  LeaseRentPeriodicity,
  LeaseRentReferenceIrl,
  LeaseType,
  PropertyBuildingLegalStatus,
  PropertyBuildPeriodType,
  PropertyEnergyClass,
  PropertyGasEmission,
  PropertyHabitationUsageType,
  PropertyRoomType,
  PropertyUsageType,
  RentChargesRecuperationMode,
  RentPaymentMethod,
  TransactionType,
} from "../types";
import {
  array,
  boolean,
  date as dateBase,
  number,
  object,
  setLocale,
  string,
  ValidationError,
} from "./deps";
import { translate } from "./utils";

export { ValidationError };

// # Localization

const _ = translate();

setLocale({
  mixed: {
    required: _("error_required_field"),
  },
  string: {
    email: _("error_email_required"),
    min: _("error_field_min"),
    max: _("error_field_max"),
    length: _("error_length"),
    matches: _("error_match_incorrect"),
  },
  number: {
    min: _("error_field_min"),
    max: _("error_field_max"),
    positive: _("error_positive_value"),
  },
});

// # Constants

export const LEGAL_ENTITY_IDENTIFIER_MAX = 14; // French SIRET: 9 digits (SIREN) + 5 digits (NIC).

export const PASSWORD_MIN_LENGTH = 6; // Password min length.

export const PHONE_NUMBER_REGEX = /^((\+)33|0)[1-9](\d{2}){4}$/; // French phone number regex.

export const DATE_ISO_FORMAT = "YYYY-MM-DD"; // Date ISO format.

// # Scalar validators

function id() {
  return string(); // UUID.
}

function date() {
  return dateBase().typeError(_("error_invalid_date_format"));
}

function decimal() {
  return number();
}

function email() {
  return string().email(_("error_email_invalid"));
}

function money() {
  return number();
}

function password() {
  return string().min(PASSWORD_MIN_LENGTH);
}

function phone() {
  return string().matches(PHONE_NUMBER_REGEX);
}

function url() {
  return string();
}

function oneOf<T>(t: T) {
  if (t instanceof Array) {
    return string().oneOf(t);
  }
  return string().oneOf(Object.values(t));
}

// # Address validators

const AddressValidator = object({
  city: string().required(),
  country: string().nullable(),
  line1: string().required(),
  line2: string().nullable(),
  postalCode: string().length(5).required(),
});

// # Auth validators

export const LoginValidationSchema = object({
  email: email().required(),
  password: password().required(),
  rememberMe: boolean().nullable(),
});

export const RegisterValidationSchema = object({
  email: email().required(),
  password: password().required(),
  firstName: string().required(),
  lastName: string().required(),
});

export const EmailValidationSchema = object({
  email: email().required(),
});

export const UserWithAccountValidator = object({
  authId: string().required(),
  email: email().required(),
  firstName: string().required(),
  lastName: string().required(),
});

// # User validators

export const UserValidator = object({
  email: email().required(),
  firstName: string().required(),
  lastName: string().required(),
});

export const UserUpdateValidator = object({
  address: AddressValidator,
});

// # Property validators

export const PropertyValidator = object({
  address: AddressValidator.required(),
  buildPeriod: oneOf(PropertyBuildPeriodType).required(),
  buildingLegalStatus: oneOf(PropertyBuildingLegalStatus).required(),
  commonSpaces: string().nullable(),
  energyClass: oneOf(PropertyEnergyClass),
  equipments: string().nullable(),
  gasEmission: oneOf(PropertyGasEmission),
  heatingMethod: oneOf(PropertyUsageType),
  housingType: oneOf(PropertyUsageType).required(),
  lenderId: id().required(),
  name: string().required(),
  note: string().nullable(),
  nticEquipments: string().nullable(),
  otherSpaces: string().nullable(),
  roomCount: oneOf(PropertyRoomType).required(),
  tax: number().nullable(),
  tenantPrivateSpaces: string().nullable(),
  surface: number().required(),
  usageType: oneOf(PropertyHabitationUsageType).required(),
  waterHeatingMethod: oneOf(PropertyUsageType).required(),
});

export const PropertyUpdateValidator = object({
  ...PropertyValidator.fields,
  lenderId: id(),
  id: string().required(),
});

// # Tenant validators

export const TenantValidator = object({
  firstName: string().required(),
  lastName: string().required(),
  phoneNumber: phone().required(),
  email: email().required(),
  note: string().nullable(),
  visaleId: string(),
  apl: boolean(),
  birthplace: string(),
  birthdate: date(),
});

// # Company validators

export const CompanyValidator = object({
  email: string().required(),
  legalEntity: string().required(),
});

export const CompanyUpdateValidator = object({
  address: AddressValidator,
});

// # Lender validators

export const LenderIndividualValidator = object({
  individual: UserValidator,
});

export const LenderCompanyValidator = object({
  company: CompanyValidator,
});

export const LenderIndividualUpdateValidator = object({
  id: id().required(),
  individual: UserUpdateValidator,
});

export const LenderCompanyUpdateValidator = object({
  id: id().required(),
  company: CompanyUpdateValidator,
});

// # File validators

export const FileValidator = object({
  downloadUrl: url().required(),
});

// # Lease validators

export const ContractFurnishedDataValidator = object({
  chargesRecuperationMode: oneOf(RentChargesRecuperationMode),
  colocationInsuranceLender: boolean(),
  colocationInsuranceMonthlyAmount: money(),
  colocationInsuranceTotalAmount: money(),
  duration: oneOf(LeaseFurnishedDuration),
  lenderFeeCap: money(),
  lenderFeeCapOther: string(),
  lenderFeeCapPrestations: string(),
  otherConditions: string(),
  rentComplement: money(),
  rentFirstAmount: string(),
  rentIrl: oneOf(LeaseRentReferenceIrl),
  rentMajDecreeIncreasedAmount: money(),
  rentMajDecreeReferenceAmount: money(),
  rentMajorationDecree: boolean(),
  rentMaxEvolutionRelocation: boolean(),
  rentPaymentPlace: string(),
  rentPaymentMethod: oneOf(RentPaymentMethod),
  rentPeriodicity: oneOf(LeaseRentPeriodicity),
  rentUnderestimatedMethod: string(),
  rentUnderestimatedMonthlyVariation: string(),
  resolutaryClause: string(),
  solidarityClause: string(),
  tenantFeeCapNewRental: string(),
  tenantFeeCapPrestations: string(),
  tenantFeeCapReportByMeter: string(),
  tenantFeeCapReportPrestations: string(),
  tenantLastRentAmount: money(),
  worksDecenceSinceLastRental: string(),
  worksRentDecreaseTenant: string(),
  worksRentIncreaseLender: string(),
});

export const LeaseFurnishedValidator = object({
  data: ContractFurnishedDataValidator,
  depositAmount: money(),
  effectDate: date().required(),
  renewDate: date(),
  propertyId: string().required(),
  rentAmount: money().positive(),
  rentChargesAmount: money()
    // Le montant des charges est obligatoire si on choisi le mode réel (avec régularisation annuelle)
    // ou si on choisit le forfait de charges. D'ailleurs ce champ n'est visible que lorsque l'on sélectionne
    // un des deux cas cité précédemment
    .when("chargesRecuperationMode", {
      is: RentChargesRecuperationMode.Reel,
      then: money().required(),
    }),
  signatureDate: date(),
  tenantIds: array().of(id()).required(),
  type: oneOf([LeaseType.Furnished]),
});

export const ContractFurnishedUpdateValidator = object({
  data: ContractFurnishedDataValidator,
  id: string().required(),
  propertyId: string(),
  tenantIds: array().of(string()),
  type: oneOf([LeaseType.Furnished]),
});

// # Document validators

export const DocumentGenerateValidator = object({
  id: id().required(),
  type: oneOf(FileType).required(),
});

// # Mail validators

export const MailSendValidator = object({
  id: id().required(),
  type: oneOf(FileType).required(),
});

// # Rent validators

export const RentValidator = object({
  amount: decimal(),
  leaseId: id().required(),
  periodEnd: date(),
  periodStart: date(),
});

export const PaymentNoticeValidator = object({
  leaseId: id().required(),
  date: date(),
});

// # Rent Receipt validators

export const RentReceiptValidator = object({
  leaseId: id().required(),
  sendMail: boolean(),
});

// # Transaction validators

export const TransactionValidator = object({
  leaseId: string().required(),
  type: oneOf(TransactionType).required(),
  date: date().required(),
  amount: money().required(),
  label: string().nullable(),
});

// # Subscription validators

export const SubscriptionValidator = object({
  name: string().required(),
});

// # Rental receipt validators

export const RentalReceiptValidator = object({
  tenantId: string().required(),
  periodStart: date().required(),
  periodEnd: date().required(),
  date: date().required(),
  amount: money().required(),
  chargesAmount: money().required(),
});

export const TransactionRentValidator = object({
  ...TransactionValidator.fields,
  ...RentalReceiptValidator.fields,
});

// # Import validators

export const ImportValidator = object({
  source: oneOf(ImportSource).required(),
  files: array().of(FileValidator).required(),
});

// # Task validators

export const TaskValidator = object({
  type: string().required(),
  // status: oneOf(TaskStatus).required(),
  progress: number().required(),
});
