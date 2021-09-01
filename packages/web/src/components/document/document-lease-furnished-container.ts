import { useFormik } from "formik";
import React, { useState } from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import {
  LeaseRentPeriodicity,
  LeaseRentReferenceIrl,
  PropertyBuildingLegalStatus,
  PropertyBuildPeriodType,
  PropertyHabitationUsageType,
  PropertyUsageType,
  RentChargesRecuperationMode,
  RentPaymentMethod,
} from "../../types";
import { ContractFurnishedDataValidator } from "../../validators";
import {
  LeaseFurnishedTemplatePart1,
} from "../templates/lease-furnished-template-p1";
import {
  LeaseFurnishedTemplatePart2,
} from "../templates/lease-furnished-template-p2";
import {
  LeaseFurnishedTemplatePart3,
} from "../templates/lease-furnished-template-p3";
import { DocumentProps } from "./document";
import { createLeaseFurnishedContract } from "./document-lease-furnished-helper";

export const withContainer = (
  WrappedComponent,
  delegate,
): React.FunctionComponent =>
  () => {
    const property = FactoryHelper.propertyList()[1];

    const tenant = FactoryHelper.tenantList()[0];

    const lender = FactoryHelper.lenderList()[0];

    const initialValues = {
      property,
      tenant,
      propertyHousingType: PropertyUsageType.Collective,
      propertyUsage: PropertyHabitationUsageType.Habitation,
      buildingLegalStatus: PropertyBuildingLegalStatus.Copro,
      propertyEquipments: "",
      propertyBuildPeriod: PropertyBuildPeriodType.FromY1975Y1989,
      propertyOtherSpaces: "",
      propertyUsageType: PropertyUsageType.Individual,
      propertyHeatingMethod: PropertyUsageType.Individual,
      propertyWaterHeatingMethod: PropertyUsageType.Individual,
      propertyCommonSpaces: "",
      propertyNticEquipments: "",
      contractEffectDate: "03/05/2021",
      contractRentAmount: 315,
      contractRentMaxEvolutionRelocation: false,
      contractRentMajorationDecree: false,
      contractTenantLastRentAmount: 295,
      contractRentMajDecreeReferenceAmount: 0,
      contractRentMajDecreeIncreasedAmount: 0,
      contractRentComplement: 0,
      contractRentIrl: LeaseRentReferenceIrl.AprilFirstSemesterY2021,
      contractRentChargesRecuperationMode: RentChargesRecuperationMode.Package,
      contractRentChargesAmount: 0,
      contractColocationInsuranceLender: false,
      contractColocationInsuranceTotalAmount: 0,
      contractColocationInsuranceMonthlyAmount: 0,
      contractRentPeriodicity: LeaseRentPeriodicity.Monthly,
      contractRentPaymentMethod: RentPaymentMethod.Before,
      contractRentPaymentDate: null,
      contractRentPaymentPlace: null,
      contractRentFirstAmount: null,
      contractRentUnderestimatedMonthlyVariation: null,
      contractRentUnderestimatedMethod: null,
      contractWorksDecenceSinceLastRental: null,
      contractWorksRentIncreaseLender: null,
      contractWorksRentDecreaseTenant: null,
      contractDepositAmount: null,
      contractSolidarityClause: null,
      contractResolutaryClause: null,
      contractTenantFeeCapNewRental: null,
      contractTenantFeeCapReportByMeter: null,
      contractTenantFeeCapReportPrestations: null,
      contractTenantFeeCapPrestations: null,
      contractLenderFeeCapPrestations: null,
      contractLenderFeeCap: null,
      contractLenderFeeCapOther: null,
      contractOtherConditions: null,
    };

    const validationSchema = ContractFurnishedDataValidator;

    const [selectedIndex, setSelectedIndex] = useState(0);

    const form = useFormik({
      initialValues,
      validationSchema,
      onSubmit: () => {
        console.log(delegate);
      },
    });

    const fields = createLeaseFurnishedContract(
      form,
      validationSchema,
      property,
      tenant,
      lender,
    );

    const getItem = (key: string) => fields[key];

    const componentProps: DocumentProps = {
      selectedIndex,
      pages: [
        LeaseFurnishedTemplatePart1({ form, getItem }),
        LeaseFurnishedTemplatePart2({ form, getItem }),
        LeaseFurnishedTemplatePart3({ form, getItem }),
      ],
      onClickPreviousPage: () => {
        setSelectedIndex(selectedIndex - 1);
        window.scrollTo(0, 0);
      },
      onClickNextPage: () => {
        setSelectedIndex(selectedIndex + 1);
        window.scrollTo(0, 0);
      },
      maxWidth: 800,
      minHeight: 700,
    };

    return WrappedComponent(componentProps);
  };
