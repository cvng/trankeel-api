import { useMutation } from "@apollo/client";
import { toaster } from "evergreen-ui";
import { useFormik } from "formik";
import { translate } from "piteo-kit";
import React, { useState } from "react";
import {
  TenantCreateMutation,
  TenantCreateMutationOptimisticResponse,
  TenantCreateMutationOptions,
} from "../../helpers";
import { useRouter } from "../../hooks/use-router";
import { TenantInput } from "../../types";
import { TenantValidator } from "../../validators";
import { TenantDialogProps } from "./tenant-dialog";

const _ = translate();

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const initialValues: TenantInput = {
      firstName: "",
      lastName: "",
      phoneNumber: "",
      email: "",
      note: "",
      apl: false,
      visaleId: "",
      birthplace: "",
      birthdate: "05-29-1989",
    };
    const router = useRouter();

    const [isShown, setIsShown] = useState(true);

    const [tenantCreate] = useMutation(
      TenantCreateMutation,
      TenantCreateMutationOptions(),
    );

    const handleSubmit = (values: TenantInput): void => {
      const input: TenantInput = values;
      const variables = { input };
      const optimisticResponse = TenantCreateMutationOptimisticResponse(
        input,
      );

      // Run optimistic mutation without awaiting for result
      const promise = tenantCreate({ variables, optimisticResponse });
      const toast = { id: "tenant-create-action" };

      // Attach an error handler if ever things goes wrong
      promise.catch(() => toaster.danger(_("error_smi"), toast));

      // Continue execution flow asynchronously
      toaster.success(_("tenant_add_success"), toast);
      setIsShown(false);
    };

    const handleCloseComplete = () => {
      router.goBack();
    };

    const validationSchema = TenantValidator;

    const form = useFormik({
      initialValues,
      validationSchema,
      onSubmit: handleSubmit,
    });

    const componentProps: TenantDialogProps = {
      form,
      validationSchema,
      isShown,
      onCloseComplete: handleCloseComplete,
      title: _("action_add_tenant"),
    };

    return WrappedComponent(componentProps);
  };
