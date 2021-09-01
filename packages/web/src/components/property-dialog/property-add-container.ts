import { useMutation, useQuery } from "@apollo/client";
import { toaster } from "evergreen-ui";
import { useFormik } from "formik";
import { translate } from "piteo-kit";
import React, { useEffect, useState } from "react";
import {
  LenderQuery,
  PropertyCreateMutation,
  PropertyCreateMutationOptions,
} from "../../helpers";
import { useRouter } from "../../hooks/use-router";
import {
  Lender,
  PropertyBuildingLegalStatus,
  PropertyBuildPeriodType,
  PropertyEnergyClass,
  PropertyGasEmission,
  PropertyHabitationUsageType,
  PropertyInput,
  PropertyRoomType,
  PropertyUsageType,
} from "../../types";
import { PropertyValidator } from "../../validators";
import { PropertyDialogProps } from "./property-dialog";

const _ = translate();

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const router = useRouter();

    const [lenderId, setLenderId] = useState("");

    const { data: { lenders } = { lenders: [] } } = useQuery(
      LenderQuery,
    );

    const initialValues: PropertyInput = {
      lenderId: lenderId,
      name: "",
      address: {
        line1: "",
        line2: "",
        postalCode: "",
        city: "",
      },
      surface: undefined,
      buildPeriod: PropertyBuildPeriodType.FromY1975Y1989,
      roomCount: PropertyRoomType.T1,
      buildingLegalStatus: PropertyBuildingLegalStatus.Copro,
      housingType: PropertyUsageType.Individual,
      usageType: PropertyHabitationUsageType.Habitation,
      heatingMethod: PropertyUsageType.Collective,
      waterHeatingMethod: PropertyUsageType.Individual,
      energyClass: PropertyEnergyClass.A,
      gasEmission: PropertyGasEmission.B,
      note: "",
      tax: undefined,
      commonSpaces: "",
      tenantPrivateSpaces: "",
      otherSpaces: "",
      equipments: "",
      nticEquipments: "",
    };

    // Set the default lender by default
    useEffect(() => {
      setLenderId(lenders?.[0]?.id);
    }, [lenders]);

    const [isShown, setIsShown] = useState(true);

    const [propertyCreate] = useMutation(
      PropertyCreateMutation,
      PropertyCreateMutationOptions(),
    );

    // Return a lender formatted list for the form selectField
    const formattedLenderList = (
      lenders: Lender[],
    ): Array<[string, string]> => {
      const arr: Array<[string, string]> = [];
      lenders?.map((lender: Lender) => {
        arr.push([
          lender.id,
          lender.displayName,
        ]);
        return arr;
      });
      return arr;
    };

    const handleSubmit = async (values: PropertyInput): Promise<void> => {
      const input = PropertyValidator.cast(values, { stripUnknown: true });
      try {
        await propertyCreate({ variables: { input } });
        toaster.success(_("property_add_success"));
        router.goBack();
        setIsShown(false);
      } catch {
        toaster.danger(_("error_smi"));
      }
    };

    const handleCloseComplete = () => {
      router.goBack();
    };

    const validationSchema = PropertyValidator;

    const form = useFormik({
      enableReinitialize: true,
      initialValues,
      validationSchema,
      onSubmit: handleSubmit,
    });

    const componentProps: PropertyDialogProps = {
      form,
      validationSchema,
      isShown,
      onCloseComplete: handleCloseComplete,
      title: _("add_property"),
      lenderList: formattedLenderList(lenders),
    };

    return WrappedComponent(componentProps);
  };
