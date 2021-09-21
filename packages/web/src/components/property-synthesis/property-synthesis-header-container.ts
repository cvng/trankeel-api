import React, { useContext } from "react";
import { useHistory } from "react-router-dom";
import { RouteById, Routes } from "../../constants";
import { useRouter } from "../../hooks/use-router";
import {
  PropertySynthesisContext,
  PropertySynthesisContextProps,
} from "./property-synthesis-context";
import { PropertySynthesisHeaderProps } from "./property-synthesis-header";

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
      onPropertyDelete,
      onLeaseAdd,
    };

    return WrappedComponent(componentProps);
  };
