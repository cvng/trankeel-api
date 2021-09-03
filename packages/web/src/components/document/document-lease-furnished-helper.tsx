// @ts-nocheck
import { FormikProps } from "formik";
import React from "react";
import { ObjectSchema } from "yup";
import {
  ContractHelper,
  FieldHelper,
  FieldSize,
  PropertyHelper,
} from "../../helpers";
import { Lender, Property, Tenant } from "../../types";

export function createLeaseFurnishedContract(
  form: FormikProps<unknown>,
  validationSchema: ObjectSchema,
  property: Property,
  tenant: Tenant,
  lender: Lender,
): Record<string, React.ReactNode> {
  return {
    "lender_name": FieldHelper.textItem(lender.displayName),
    "tenant_fullname": FieldHelper.textItem(tenant.fullName),
    "property_address": FieldHelper.textItem(
      `${property?.address?.line1}, ${property?.address
        ?.postalCode} ${property?.address?.city}`,
    ),
    "property_surface": FieldHelper.textItem(`${property?.surface} m2`),
    "property_room_count": FieldHelper.textItem(
      PropertyHelper.roomPropertyCount().get(
        property?.roomCount,
      ),
    ),
    "property_housing_type": FieldHelper.segmentedField(
      form,
      validationSchema,
      "propertyHousingType",
      PropertyHelper.individualOrCollective(),
    ),
    "property_building_legal_status": FieldHelper.segmentedField(
      form,
      validationSchema,
      "buildingLegalStatus",
      PropertyHelper.buildingLegalStatuses(),
    ),
    "property_construction_period": FieldHelper.segmentedField(
      form,
      validationSchema,
      "propertyBuildPeriod",
      PropertyHelper.buildPeriods(),
      FieldSize.XL,
    ),
    "property_other_spaces_inline": FieldHelper.inputField(
      form,
      validationSchema,
      "propertyOtherSpaces",
      "grenier, comble aménagé, terrasse, balcon, jardin",
    ),
    "property_equipments_inline": FieldHelper.inputField(
      form,
      validationSchema,
      "propertyEquipments",
      "Cuisine équipée, détail des installations sanitaires, etc",
    ),
    "property_heating_method": FieldHelper.segmentedField(
      form,
      validationSchema,
      "propertyHeatingMethod",
      PropertyHelper.individualOrCollective(),
    ),
    "property_water_heating_method": FieldHelper.segmentedField(
      form,
      validationSchema,
      "propertyWaterHeatingMethod",
      PropertyHelper.individualOrCollective(),
    ),
    "property_usage": FieldHelper.segmentedField(
      form,
      validationSchema,
      "propertyUsage",
      PropertyHelper.usageTypes(),
      FieldSize.M,
    ),
    "property_tenant_private_spaces": FieldHelper.inputField(
      form,
      validationSchema,
      "propertyTenantPrivateSpaces",
      "cave, parking, garage etc.",
    ),
    "property_common_spaces": FieldHelper.inputField(
      form,
      validationSchema,
      "propertyCommonSpaces",
      "Garage à vélo, ascenseur, espaces verts",
    ),
    "property_ntic_equipments_inline": FieldHelper.inputField(
      form,
      validationSchema,
      "propertyNticEquipments",
      "Modalités de réception de la télévision dans l'immeuble, modalités de raccordement internet etc",
    ),
    "contract_effect_date": FieldHelper.inputField(
      form,
      validationSchema,
      "contractEffectDate",
      "01/03/2021",
    ),
    "contract_duration": FieldHelper.segmentedField(
      form,
      validationSchema,
      "contractDuration",
      ContractHelper.furnishedDuration(),
    ),
    "contract_rent_excluding_charges": FieldHelper.inputField(
      form,
      validationSchema,
      "contractRentAmount",
      "315",
      null,
      FieldSize.S,
    ),
    "contract_max_evolution_relocation": FieldHelper.segmentedField(
      form,
      validationSchema,
      "contractRentMaxEvolutionRelocation",
      ContractHelper.yesNo(),
    ),
    "contract_rent_majoration_decree": FieldHelper.segmentedField(
      form,
      validationSchema,
      "contractRentMajorationDecree",
      ContractHelper.yesNo(),
    ),
    "contract_rent_maj_decree_reference_amount": FieldHelper.inputField(
      form,
      validationSchema,
      "contractRentMajDecreeReferenceAmount",
      "315",
      "€/m2",
      FieldSize.S,
    ),
    "contract_rent_maj_decree_increased_amount": FieldHelper.inputField(
      form,
      validationSchema,
      "contractRentMajDecreeIncreasedAmount",
      "325",
      "€/m2",
      FieldSize.S,
    ),
    "contract_rent_complement": FieldHelper.inputField(
      form,
      validationSchema,
      "contractRentComplement",
      null,
      null,
      FieldSize.S,
    ),
    "contract_tenant_last_rent_amount": FieldHelper.inputField(
      form,
      validationSchema,
      "contractTenantLastRentAmount",
      "450",
      null,
      FieldSize.S,
    ),
    "contract_rent_irl": FieldHelper.selectField(
      form,
      validationSchema,
      "contractRentIrl",
      ContractHelper.irlReference(),
      null,
      FieldSize.M,
    ),
    "contract_charges_payment_method": FieldHelper.segmentedField(
      form,
      validationSchema,
      "contractRentChargesRecuperationMode",
      ContractHelper.chargesRecuperationMode(),
      500,
    ),
    "contract_charges_amount": FieldHelper.inputField(
      form,
      validationSchema,
      "contractRentChargesAmount",
      null,
      null,
      FieldSize.S,
    ),
    "contract_charges_revision_method": FieldHelper.inputField(
      form,
      validationSchema,
      "contractRentChargesRevisionMethod",
      null,
      null,
      FieldSize.L,
    ),
    "contract_colocation_insurance_lender": FieldHelper.segmentedField(
      form,
      validationSchema,
      "contractColocationInsuranceLender",
      ContractHelper.yesNo(),
    ),
    "contract_colocation_insurance_total_amount": FieldHelper.inputField(
      form,
      validationSchema,
      "contractColocationInsuranceTotal_amount",
      null,
      null,
      FieldSize.S,
    ),
    "contract_colocation_insurance_monthly_amount": FieldHelper.inputField(
      form,
      validationSchema,
      "contractColocationInsuranceMonthlyAmount",
      null,
      null,
      FieldSize.S,
    ),
    "contract_rent_periodicity": FieldHelper.segmentedField(
      form,
      validationSchema,
      "contractRentPeriodicity",
      ContractHelper.rentPeriodicity(),
    ),
    "contract_rent_payment_type": FieldHelper.segmentedField(
      form,
      validationSchema,
      "contractRentPaymentMethod",
      ContractHelper.rentPaymentMethod(),
      FieldSize.M,
    ),
    "contract_rent_payment_date": FieldHelper.inputField(
      form,
      validationSchema,
      "contractRentPaymentDate",
      null,
      null,
      FieldSize.M,
    ),
    "contract_rent_payment_place": FieldHelper.inputField(
      form,
      validationSchema,
      "contractRentPaymentPlace",
    ),
    "contract_rent_first_amount": FieldHelper.inputField(
      form,
      validationSchema,
      "contractRentFirstAmount",
      null,
      null,
      FieldSize.S,
    ),
    "contract_rent_underestimated_monthly_variation": FieldHelper.inputField(
      form,
      validationSchema,
      "contractRentUnderestimatedMonthlyVariation",
      null,
      null,
      FieldSize.S,
    ),
    "contract_rent_underestimated_method": FieldHelper.inputField(
      form,
      validationSchema,
      "contractRentUnderestimatedMethod",
      null,
      null,
      FieldSize.L,
    ),
    "contract_works_decence_since_last_rental": FieldHelper.inputField(
      form,
      validationSchema,
      "contractWorksDecenceSinceLastRental",
      null,
      null,
      FieldSize.L,
    ),
    "contract_works_rent_increase_lender": FieldHelper.inputField(
      form,
      validationSchema,
      "contractWorksRentIncreaseLender",
      null,
      null,
      FieldSize.L,
    ),
    "contractWorksRentDecreaseTenant": FieldHelper.inputField(
      form,
      validationSchema,
      "contractWorksRentDecreaseTenant",
    ),
    "contract_deposit_amount": FieldHelper.inputField(
      form,
      validationSchema,
      "contractDepositAmount",
      null,
      null,
      FieldSize.S,
    ),
    "contract_solidarity_clause": FieldHelper.inputField(
      form,
      validationSchema,
      "contractSolidarityClause",
      null,
      null,
      FieldSize.XL,
    ),
    "contract_resolutary_clause": FieldHelper.inputField(
      form,
      validationSchema,
      "contractResolutaryClause",
      null,
      null,
      FieldSize.XL,
    ),
    "contract_tenant_fee_cap_new_rental": FieldHelper.inputField(
      form,
      validationSchema,
      "contractTenantFeeCapNewRental",
      null,
      null,
      FieldSize.S,
    ),
    "contract_tenant_fee_cap_report_by_meter": FieldHelper.inputField(
      form,
      validationSchema,
      "contractTenantFeeCapReportByMeter",
      null,
      null,
      FieldSize.S,
    ),
    "contract_tenant_fee_cap_report_prestations": FieldHelper.inputField(
      form,
      validationSchema,
      "contractTenantFeeCapReportPrestations",
    ),
    "contract_tenant_fee_cap_prestations": FieldHelper.inputField(
      form,
      validationSchema,
      "contractTenantFeeCapPrestations",
    ),
    "contract_lender_fee_cap_prestations": FieldHelper.inputField(
      form,
      validationSchema,
      "contractLenderFeeCapPrestations",
    ),
    "contract_lender_fee_cap": FieldHelper.inputField(
      form,
      validationSchema,
      "contractLenderFeeCap",
    ),
    "contract_lender_fee_cap_other": FieldHelper.inputField(
      form,
      validationSchema,
      "contractLenderFeeCapOther",
    ),
    "contract_other_conditions": FieldHelper.inputField(
      form,
      validationSchema,
      "contractOtherConditions",
      null,
      null,
      FieldSize.XL,
    ),
  };
}