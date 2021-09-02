import { useMutation, useQuery } from "@apollo/client";
import { toaster } from "evergreen-ui";
import { translate } from "piteo-kit";
import React, { useState } from "react";
import { Routes } from "../../constants";
import {
  PropertyDeleteMutation,
  PropertyDeleteMutationOptions,
  PropertyListQuery,
} from "../../helpers";
import { useParamId } from "../../hooks/use-param-id";
import { useRouter } from "../../hooks/use-router";
import { Property } from "../../types";
import { ConfirmDialogProps } from "../common/confirm-dialog";

const _ = translate();

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const router = useRouter();
    const propertyId = useParamId(Routes.PROPERTY_DELETE);

    const [{ isShown }, setState] = useState({ isShown: true });

    const { data: { properties } = { properties: null } } = useQuery(
      PropertyListQuery,
      {
        variables: { id: propertyId },
      },
    );

    const property = properties?.find((item: Property) =>
      item.id === propertyId
    );

    const [propertyDelete, { loading: isConfirmLoading }] = useMutation(
      PropertyDeleteMutation,
      PropertyDeleteMutationOptions(),
    );

    const handleConfirm = async (): Promise<void> => {
      // Check if there is an active contract ?
      const contractCount = property?.leases?.length;
      const hasAnActiveContract = contractCount > 0;
      if (hasAnActiveContract) {
        toaster.warning(
          contractCount > 1 ? _("delete_property_existing_contract_multiple")
          : _("delete_property_existing_contract"),
        );
        setState({ isShown: false });
        return;
      }
      await propertyDelete({ variables: { id: propertyId } });

      toaster.notify(
        _("delete_property_completed", { propertyName: property.name }),
      );
      setState({ isShown: false });
      router.showPropertySynthesis();
    };

    const handleCloseComplete = (): void => {
      router.goBack();
    };

    const componentProps: ConfirmDialogProps = {
      title: _("delete_property_confirmation_title"),
      message: _("delete_property_confirmation_msg", {
        propertyName: property?.name,
      }),
      isShown,
      isConfirmLoading,
      onConfirm: handleConfirm,
      onCloseComplete: handleCloseComplete,
    };

    return WrappedComponent(componentProps);
  };
