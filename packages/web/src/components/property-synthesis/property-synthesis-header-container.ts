import { toaster } from "evergreen-ui";
import React, { useContext } from "react";
import { useHistory } from "react-router-dom";
import { RouteById, Routes } from "../../constants";
import { useRouter } from "../../hooks/use-router";
import {
  PropertySynthesisContext,
  PropertySynthesisContextProps,
} from "./property-synthesis-context";
import { PropertySynthesisHeaderProps } from "./property-synthesis-header";
import { translate } from "piteo-kit";

const _ = translate();

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const history = useHistory();
    const router = useRouter();

    const { loading, selectedProperty: property } = useContext(
      PropertySynthesisContext,
    ) as PropertySynthesisContextProps;

    const onPropertyEdit = (propertyId: string): void => {
      history.push(RouteById(Routes.PROPERTY_EDIT, [propertyId]));
    };

    const onPropertyArchive = (propertyId: string): void => {
      toaster.notify(_("feature_available_soon", { propertyId }));
    };

    const onPropertyDelete = (propertyId: string): void => {
      history.push(RouteById(Routes.PROPERTY_DELETE, [propertyId]));
    };

    const onLeaseAdd = (
      propertyId: string,
    ): void => {
      router.showLeaseAddWithProperty(propertyId);
    };

    const componentProps: PropertySynthesisHeaderProps = {
      loading,
      property,
      onPropertyEdit,
      onPropertyArchive,
      onPropertyDelete,
      onLeaseAdd,
    };

    return WrappedComponent(componentProps);
  };
