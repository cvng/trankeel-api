import { useQuery } from "@apollo/client";
import { toaster } from "evergreen-ui";
import { FunctionComponent, useContext } from "react";
import { useHistory, useParams } from "react-router-dom";
import { Routes } from "../../constants";
import { PropertyListQuery, TenantListQuery } from "../../helpers";
import { useFilter } from "../../hooks/use-filter";
import { Property, Tenant } from "../../types";
import {
  LeaseAddContext,
  LeaseAddContextAction,
  LeaseAddContextProps,
} from "../lease-add/lease-add-context";
import { LeaseAddFlowDelegate } from "../lease-add/lease-add-flow-page";
import { LeaseSelectPartiesProps } from "./lease-select-parties";
import { translate } from "piteo-kit";

const _ = translate();

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
  delegate: LeaseAddFlowDelegate,
): FunctionComponent =>
  () => {
    const history = useHistory();

    const { propertyId, tenantId } = useParams();

    let {
      dispatch,
      createFromExistingLease,
      selectedProperty,
      selectedTenants,
    } = useContext(
      LeaseAddContext,
    ) as LeaseAddContextProps;

    const onSelectTenant = (tenant: Tenant) => {
      dispatch({ type: LeaseAddContextAction.SetTenants, payload: [tenant] });
    };

    const onAddProperty = () => {
      history.push(Routes.LEASE_ADD_NEW_PROPERTY);
    };

    const onAddTenant = () => {
      history.push(Routes.LEASE_ADD_NEW_TENANT);
    };

    const onSelectProperty = (property: Property) => {
      dispatch({ type: LeaseAddContextAction.SetProperty, payload: property });
    };

    const onConfirmButtonClick = () => {
      // Update the state before change page
      // this usecase is specific as we can launch the flow with a property id or a tenant id
      dispatch({
        type: LeaseAddContextAction.SetTenants,
        payload: selectedTenants,
      });
      dispatch({
        type: LeaseAddContextAction.SetProperty,
        payload: selectedProperty,
      });
      // Call the delegate as we are done
      delegate?.onPageDidFinish();
    };

    const {
      loading: propertyLoading,
      data: { properties } = { properties: [] },
    } = useQuery(
      PropertyListQuery,
    );

    // If there is a property id in url get the property and select it by default
    // otherwise select the first property if there is only one
    selectedProperty = properties?.find((item: Property) =>
      item.id === propertyId
    );
    if (!selectedProperty) {
      selectedProperty = properties?.[0];
    }

    let { loading: tenantLoading, data: { tenants } = { tenants: [] } } =
      useQuery(
        TenantListQuery,
      );

    // Filter tenants to display those without contract
    tenants = useFilter(tenants, (tenant: Tenant) => {
      return tenant?.lease === null;
    });

    // If there is a tenant id in url get the tenant and select it by default
    // otherwise select the first tenant if there is only one
    if (tenantId) {
      selectedTenants = [tenants?.find((item: Tenant) => item.id === tenantId)];
    } else if (!selectedTenants && tenants?.length === 1) {
      selectedTenants = [tenants?.[0]];
    }

    const componentProps: LeaseSelectPartiesProps = {
      loading: propertyLoading || tenantLoading,
      createFromExistingLease,
      properties,
      selectedProperty,
      onSelectProperty,
      onAddProperty,
      tenants,
      selectedTenants,
      onAddTenant,
      onSelectTenant,
      onChangeType: (createFromExistingLease: boolean) => {
        if (createFromExistingLease === false) {
          toaster.notify(_("feature_available_soon"));
        }
      },
      onConfirmButtonClick,
    };
    return WrappedComponent(componentProps);
  };
