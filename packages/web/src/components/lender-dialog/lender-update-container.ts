// @ts-nocheck
import { useMutation, useQuery } from "@apollo/client";
import { toaster } from "evergreen-ui";
import { useFormik } from "formik";
import { translate } from "piteo-kit";
import React, { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import {
  LenderIndividualUpdateMutation,
  LenderIndividualUpdateMutationOptions,
  LenderQuery,
} from "../../helpers";
import { useRouter } from "../../hooks/use-router";
import { Lender, LenderIndividualUpdateInput } from "../../types";
import { LenderIndividualUpdateValidator } from "../../validators";
import { LenderDialogProps } from "./lender-dialog";

const _ = translate();

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const router = useRouter();
    const { id } = useParams();

    const [isShown, setIsShown] = useState(true);
    const [isPhysicalPerson] = useState(true);
    const [lender, setLender] = useState(null);

    const { data: { lenders } = { lenders: [] } } = useQuery(
      LenderQuery,
    );

    useEffect(() => {
      const lender = lenders?.find((lender: Lender) => lender.id === id);
      setLender(lender);
    }, [lenders, id]);

    const initialValues: LenderIndividualUpdateInput = {
      id,
      individual: {
        address: {
          line1: lender?.identity.__typename === "User"
            ? lender?.identity?.address?.line1
            : "",
          line2: lender?.identity.__typename === "User"
            ? lender?.identity?.address?.line2
            : "",
          postalCode: lender?.identity.__typename === "User"
            ? lender?.identity?.address?.postalCode
            : "",
          city: lender?.identity.__typename === "User"
            ? lender?.identity?.address?.city
            : "",
        },
        firstName: lender?.identity.__typename === "User"
          ? lender?.identity?.firstName
          : "",
        lastName: lender?.identity.__typename === "User"
          ? lender?.identity?.lastName
          : "",
      },
    };

    const [lenderUpdate] = useMutation(
      LenderIndividualUpdateMutation,
      LenderIndividualUpdateMutationOptions(),
    );

    const handleSubmit = async (
      values: LenderIndividualUpdateInput,
    ): Promise<void> => {
      const input: LenderIndividualUpdateInput =
        await LenderIndividualUpdateValidator.validate(
          values,
        );
      const variables = { input };

      const promise = lenderUpdate({ variables });
      const toast = { id: "lender-update-action" };

      // Attach an error handler if ever things goes wrong
      promise.catch(() => toaster.danger(_("error_smi"), toast));

      // Continue execution flow asynchronously
      toaster.success(_("lender_edit_success"), toast);
      setIsShown(false);
    };

    const handleCloseComplete = () => {
      router.goBack();
    };

    const validationSchema = LenderIndividualUpdateValidator;

    const form = useFormik({
      enableReinitialize: true,
      initialValues,
      validationSchema,
      onSubmit: handleSubmit,
    });

    const componentProps: LenderDialogProps<LenderIndividualUpdateInput> = {
      form,
      validationSchema,
      isShown,
      onCloseComplete: handleCloseComplete,
      isPhysicalPerson,
      title: _("lender_edit_title"),
      // changeLegalEntityType: () => setIsPhysicalPerson(!isPhysicalPerson), // TODO: Implement change entity legal type
      changeLegalEntityType: () => toaster.notify(_("feature_available_soon")),
    };

    return WrappedComponent(componentProps);
  };
