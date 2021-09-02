import { useMutation } from "@apollo/client";
import { useFormik } from "formik";
import React, { useContext } from "react";
import { LeaseCreateMutation, LeaseCreateMutationOptions } from "../../helpers";
import { LeaseFurnishedInput } from "../../types";
import { LeaseFurnishedValidator } from "../../validators";
import {
  LeaseAddContext,
  LeaseAddContextAction,
  LeaseAddContextProps,
} from "../lease-add/lease-add-context";
import { LeaseAddFlowDelegate } from "../lease-add/lease-add-flow-page";
import { LeaseExistingFormProps } from "./lease-existing-form";

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
  delegate: LeaseAddFlowDelegate,
): React.FunctionComponent =>
  () => {
    const { dispatch, selectedProperty, selectedTenants } = useContext(
      LeaseAddContext,
    ) as LeaseAddContextProps;

    const initialValues: LeaseFurnishedInput = {
      propertyId: selectedProperty?.id,
      tenantIds: selectedTenants?.map((t) => t?.id),
      effectDate: undefined,
      renewDate: undefined,
      signatureDate: undefined,
      rentAmount: undefined,
      rentChargesAmount: undefined,
      depositAmount: undefined,
    };

    const [leaseCreate] = useMutation(
      LeaseCreateMutation,
      LeaseCreateMutationOptions(),
    );

    const handleSubmit = (values: LeaseFurnishedInput): void => {
      const input: LeaseFurnishedInput = values;
      const variables = { input };

      const promise = leaseCreate({ variables });
      promise.then(() => {
        dispatch({
          type: LeaseAddContextAction.SetFlowFinishWithError,
          payload: false,
        });
      });
      promise.catch((error) => {
        // Log the error to be catch in a remote debug tool
        console.error(error);
        dispatch({
          type: LeaseAddContextAction.SetFlowFinishWithError,
          payload: true,
        });
      });
      promise.finally(() => {
        delegate?.onPageDidFinish();
      });
    };

    const validationSchema = LeaseFurnishedValidator;

    const form = useFormik({
      enableReinitialize: true,
      initialValues,
      validationSchema,
      onSubmit: handleSubmit,
    });

    const componentProps: LeaseExistingFormProps<LeaseFurnishedInput> = {
      form,
      validationSchema,
    };

    return WrappedComponent(componentProps);
  };
