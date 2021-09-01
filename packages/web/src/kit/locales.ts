import {
  FileType,
  LeaseFurnishedDuration,
  LeaseRentPeriodicity,
  LeaseRentReferenceIrl,
  LeaseStatus,
  LeaseType,
  LegalEntityType,
  PropertyBuildingLegalStatus,
  PropertyBuildPeriodType,
  PropertyEnergyClass,
  PropertyGasEmission,
  PropertyHabitationUsageType,
  PropertyRoomType,
  PropertyStatus,
  PropertyUsageType,
  RentChargesRecuperationMode,
  RentPaymentMethod,
  TenantStatus,
} from "../types";
import { unreachable } from "./deps";
import { capitalize, translate } from "./utils";

const LOCALE = "fr-FR";

const TIMEZONE = "Europe/Paris";

const CURRENCY = "EUR";

export interface Address {
  city: string;
  country?: string;
  line1: string;
  line2?: string;
  postalCode: string;
}

export interface Person {
  firstName: string;
  lastName: string;
}

// # Basic locales
const _ = translate();

export function toLocaleDateString(
  value: Date,
  fallbackValue = "",
  options?: Intl.DateTimeFormatOptions,
): string {
  return value
    ? new Date(value).toLocaleDateString(LOCALE, {
      ...options,
      timeZone: TIMEZONE,
    })
    : fallbackValue;
}

export function amount(value: number): string {
  const formatter = Intl.NumberFormat(LOCALE, {
    style: "currency",
    currency: CURRENCY,
  });
  return formatter.format(value);
}

export function months(value: number): string {
  return _("n_months", { n: value });
}

export function yesNo(value: boolean): string {
  // deno-fmt-ignore
  switch (value) {
    case true: return _("yes");
    case false: return _("no");
    default: unreachable();
  }
}

// # Lender locales

export class LenderHelper {
  static legalEntityTypes(): Map<LegalEntityType, string> {
    return new Map<LegalEntityType, string>([
      [LegalEntityType.Sci, _("legal_entity_type_sci")],
      [LegalEntityType.Eurl, _("legal_entity_type_eurl")],
      [LegalEntityType.Sa, _("legal_entity_type_sa")],
      [LegalEntityType.Sarl, _("legal_entity_type_sarl")],
      [LegalEntityType.Sas, _("legal_entity_type_sas")],
      [LegalEntityType.Sasu, _("legal_entity_type_sasu")],
      [LegalEntityType.Scp, _("legal_entity_type_scp")],
      [LegalEntityType.Snc, _("legal_entity_type_snc")],
      [LegalEntityType.Other, _("other")],
    ]);
  }

  static legalIdentityTypes(): Map<boolean, string> {
    return new Map<boolean, string>([
      [false, _("physical_person")],
      [true, _("moral_person")],
    ]);
  }
}

export function legalIdentityTypes(value: boolean): string {
  // deno-fmt-ignore
  switch (value) {
    case false: return _("physical_person");
    case true: return _("moral_person");
    default: unreachable();
  }
}

// # Property locales

export class PropertyHelper {
  static roomPropertyTypes(): Map<PropertyRoomType, string> {
    return new Map<PropertyRoomType, string>([
      [PropertyRoomType.T1, _("property_room_type_one")],
      [PropertyRoomType.T2, _("property_room_type_two")],
      [PropertyRoomType.T3, _("property_room_type_three")],
      [PropertyRoomType.T4, _("property_room_type_four")],
      [PropertyRoomType.T5, _("property_room_type_five")],
      [PropertyRoomType.T6, _("property_room_type_six")],
      [PropertyRoomType.Other, _("other")],
    ]);
  }

  static roomPropertyCount(): Map<PropertyRoomType, string> {
    return new Map<PropertyRoomType, string>([
      [PropertyRoomType.T1, _("property_room_type_one_count")],
      [PropertyRoomType.T2, _("property_room_type_two_count")],
      [PropertyRoomType.T3, _("property_room_type_three_count")],
      [PropertyRoomType.T4, _("property_room_type_four_count")],
      [PropertyRoomType.T5, _("property_room_type_five_count")],
      [PropertyRoomType.T6, _("property_room_type_six_count")],
      [PropertyRoomType.Other, _("other")],
    ]);
  }

  static statusMap(): Map<PropertyStatus, string> {
    return new Map<PropertyStatus, string>([
      [PropertyStatus.Inactive, _("status_inactive")],
      [PropertyStatus.ForSale, _("status_for_sale")],
      [PropertyStatus.Rented, _("status_rented")],
      [PropertyStatus.UnderConstruction, _("status_under_construction")],
      [PropertyStatus.Unrented, _("status_unrented")],
    ]);
  }

  static energyClassMap(): Map<PropertyEnergyClass, string> {
    return new Map<PropertyEnergyClass, string>([
      [PropertyEnergyClass.A, _("energy_class_a")],
      [PropertyEnergyClass.B, _("energy_class_b")],
      [PropertyEnergyClass.C, _("energy_class_c")],
      [PropertyEnergyClass.D, _("energy_class_d")],
      [PropertyEnergyClass.E, _("energy_class_e")],
      [PropertyEnergyClass.F, _("energy_class_f")],
      [PropertyEnergyClass.G, _("energy_class_g")],
    ]);
  }

  static gasEmissionMap(): Map<PropertyGasEmission, string> {
    return new Map<PropertyGasEmission, string>([
      [PropertyGasEmission.A, _("gas_emission_a")],
      [PropertyGasEmission.B, _("gas_emission_b")],
      [PropertyGasEmission.C, _("gas_emission_c")],
      [PropertyGasEmission.D, _("gas_emission_d")],
      [PropertyGasEmission.E, _("gas_emission_e")],
      [PropertyGasEmission.F, _("gas_emission_f")],
      [PropertyGasEmission.G, _("gas_emission_g")],
    ]);
  }

  static buildPeriods(): Map<PropertyBuildPeriodType, string> {
    return new Map<PropertyBuildPeriodType, string>([
      [PropertyBuildPeriodType.BeforeY1949, _("property_build_period_1")],
      [PropertyBuildPeriodType.FromY1949Y1974, _("property_build_period_2")],
      [PropertyBuildPeriodType.FromY1975Y1989, _("property_build_period_3")],
      [PropertyBuildPeriodType.FromY1990Y2005, _("property_build_period_4")],
      [PropertyBuildPeriodType.FromY2005, _("property_build_period_5")],
    ]);
  }

  static buildingLegalStatuses(): Map<PropertyBuildingLegalStatus, string> {
    return new Map<PropertyBuildingLegalStatus, string>([
      [PropertyBuildingLegalStatus.Mono, _("property_ownership_single")],
      [PropertyBuildingLegalStatus.Copro, _("property_ownership_copro")],
    ]);
  }

  static usageTypes(): Map<PropertyHabitationUsageType, string> {
    return new Map<PropertyHabitationUsageType, string>([
      [PropertyHabitationUsageType.Habitation, _("property_usage_habitation")],
      [PropertyHabitationUsageType.Mixte, _("property_usage_mixte")],
    ]);
  }

  static individualOrCollective(): Map<PropertyUsageType, string> {
    return new Map<PropertyUsageType, string>([
      [PropertyUsageType.Collective, _("collective")],
      [PropertyUsageType.Individual, _("individual")],
    ]);
  }
}

export function roomPropertyCount(value: PropertyRoomType): string {
  // deno-fmt-ignore
  switch (value) {
    case PropertyRoomType.T1: return _("property_room_type_one_count");
    case PropertyRoomType.T2: return _("property_room_type_two_count");
    case PropertyRoomType.T3: return _("property_room_type_three_count");
    case PropertyRoomType.T4: return _("property_room_type_four_count");
    case PropertyRoomType.T5: return _("property_room_type_five_count");
    case PropertyRoomType.T6: return _("property_room_type_six_count");
    case PropertyRoomType.Other: return _("other");
    default: unreachable();
  }
}

export function buildPeriods(value: PropertyBuildPeriodType): string {
  // deno-fmt-ignore
  switch (value) {
    case PropertyBuildPeriodType.BeforeY1949: return _("property_build_period_1");
    case PropertyBuildPeriodType.FromY1949Y1974: return _("property_build_period_2");
    case PropertyBuildPeriodType.FromY1975Y1989: return _("property_build_period_3");
    case PropertyBuildPeriodType.FromY1990Y2005: return _("property_build_period_4");
    case PropertyBuildPeriodType.FromY2005: return _("property_build_period_5");
    default: unreachable();
  }
}

export function buildingLegalStatuses(
  value: PropertyBuildingLegalStatus,
): string {
  // deno-fmt-ignore
  switch (value) {
    case PropertyBuildingLegalStatus.Mono: return _("property_ownership_single");
    case PropertyBuildingLegalStatus.Copro: return _("property_ownership_copro");
    default: unreachable();
  }
}

export function usageTypes(value: PropertyHabitationUsageType): string {
  // deno-fmt-ignore
  switch (value) {
    case PropertyHabitationUsageType.Habitation: return _("property_usage_habitation");
    case PropertyHabitationUsageType.Mixte: return _("property_usage_mixte");
    default: unreachable();
  }
}

export function individualOrCollective(value: PropertyUsageType): string {
  // deno-fmt-ignore
  switch (value) {
    case PropertyUsageType.Collective: return _("collective");
    case PropertyUsageType.Individual: return _("individual");
    default: unreachable();
  }
}

// # Tenant locales

export class TenantHelper {
  static statusMap(): Map<TenantStatus, string> {
    return new Map<TenantStatus, string>([
      [TenantStatus.New, _("status_new")],
      [TenantStatus.Uptodate, _("status_active")],
      [TenantStatus.Uptodate, _("status_up_to_date")],
      [TenantStatus.Late, _("status_late")],
      [TenantStatus.Gone, _("status_gone")],
    ]);
  }
}

// # Lease locales

export class ContractHelper {
  static statusMap(): Map<LeaseStatus, string | undefined> {
    return new Map<LeaseStatus, string>([
      [LeaseStatus.Active, _("status_active")],
      [LeaseStatus.Ended, _("status_ended")],
    ]);
  }

  static typeMap(): Map<LeaseType, string | undefined> {
    return new Map<LeaseType, string>([
      [LeaseType.Furnished, _("lease_furnished")],
      [LeaseType.Naked, _("lease_naked")],
    ]);
  }

  static statusMapColor(status: LeaseStatus): string | undefined {
    const map = new Map<LeaseStatus, string>([
      [LeaseStatus.Active, "yellow"],
      [LeaseStatus.Ended, "red"],
    ]);
    return map.get(status);
  }

  static yesNo(): Map<boolean, string> {
    return new Map<boolean, string>([
      [true, _("yes")],
      [false, _("no")],
    ]);
  }
  static irlReference(): Map<LeaseRentReferenceIrl, string> {
    return new Map<LeaseRentReferenceIrl, string>([
      [
        LeaseRentReferenceIrl.AprilFirstSemesterY2021,
        _("irl_reference_april_2021"),
      ],
    ]);
  }
  static chargesRecuperationMode(): Map<RentChargesRecuperationMode, string> {
    return new Map<RentChargesRecuperationMode, string>([
      [RentChargesRecuperationMode.Reel, _("rent_charges_reel")],
      [RentChargesRecuperationMode.Package, _("rent_charges_package")],
      [RentChargesRecuperationMode.Periodic, _("rent_charges_periodic")],
    ]);
  }
  static rentPeriodicity(): Map<LeaseRentPeriodicity, string> {
    return new Map<LeaseRentPeriodicity, string>([
      [LeaseRentPeriodicity.Monthly, _("rent_periodicity_monthly")],
      [LeaseRentPeriodicity.Annualy, _("rent_periodicity_annualy")],
    ]);
  }
  static rentPaymentMethod(): Map<RentPaymentMethod, string> {
    return new Map<RentPaymentMethod, string>([
      [RentPaymentMethod.Before, _("rent_payment_method_before")],
      [RentPaymentMethod.After, _("rent_payment_method_after")],
    ]);
  }
  static furnishedDuration(): Map<LeaseFurnishedDuration, string> {
    return new Map<LeaseFurnishedDuration, string>([
      [LeaseFurnishedDuration.NineMonths, _("nine_months")],
      [LeaseFurnishedDuration.OneYear,  _("one_year")],
    ]);
  }

}

export function fileTypeMap(): Map<FileType, string> {
  return new Map<FileType, string>([
    [FileType.PaymentNotice, _("rent_notice")],
    [FileType.LeaseDocument, _("lease")],
    [FileType.RentReceipt, _("rent_receipt")],
  ]);
}

export function furnishedDuration(value: LeaseFurnishedDuration): string {
  // deno-fmt-ignore
  switch (value) {
    case LeaseFurnishedDuration.NineMonths: return _("nine_months");
    case LeaseFurnishedDuration.OneYear: return _("one_year");
  }
}

/*
export function nakedDuration(value: LeaseNakedDuration): string {
  // deno-fmt-ignore
  switch (value) {
    case LeaseNakedDuration.ThreeYears: return _("n_years", { n: 3 });
    case LeaseNakedDuration.SixYears: return _("n_years", { n: 6 });
  }
}
*/

export function irlReference(value: LeaseRentReferenceIrl): string {
  // deno-fmt-ignore
  switch (value) {
    case LeaseRentReferenceIrl.AprilFirstSemesterY2021: return _("irl_reference_april_2021");
    default: unreachable();
  }
}

export function chargesRecuperationMode(
  value: RentChargesRecuperationMode,
): string {
  // deno-fmt-ignore
  switch (value) {
    case RentChargesRecuperationMode.Reel: return _("rent_charges_reel");
    case RentChargesRecuperationMode.Package: return _("rent_charges_package");
    case RentChargesRecuperationMode.Periodic: return _("rent_charges_periodic");
    default: unreachable();
  }
}

export function rentPeriodicity(value: LeaseRentPeriodicity): string {
  // deno-fmt-ignore
  switch (value) {
    case LeaseRentPeriodicity.Monthly: return _("rent_periodicity_monthly");
    case LeaseRentPeriodicity.Annualy: return _("rent_periodicity_annualy");
    default: unreachable();
  }
}

export function rentPaymentMethod(value: RentPaymentMethod): string {
  // deno-fmt-ignore
  switch (value) {
    case RentPaymentMethod.Before: return _("rent_payment_method_before");
    case RentPaymentMethod.After: return _("rent_payment_method_after");
    default: unreachable();
  }
}

// # Utils

export function addressInline(address: Address): string {
  return [
    address.line1,
    address.line2,
    [address.postalCode, capitalize(address.city)].join(" ").trim(),
  ]
    .filter(Boolean)
    .map(capitalize)
    .join(", ");
}

export function fullName(person: Person): string {
  return [
    capitalize(person.firstName),
    person.lastName.toUpperCase(),
  ].filter(Boolean).join(" ").trim();
}

export function inline(delimitedString: string, char = ","): string {
  return delimitedString && delimitedString.split && delimitedString.split(char)
    .filter(Boolean)
    .map(capitalize)
    .join(", ");
}

export function formatPhoneNumber(phoneNumber = ""): string {
  const chunks = [];
  if (phoneNumber.length != 10) {
    return phoneNumber;
  }
  for (let i = 0; i < phoneNumber.length; i += 2) {
    chunks.push(phoneNumber.substring(i, i + 2));
  }
  return chunks.join(" ");
}
