import { useQuery } from "@apollo/client";
import { toaster } from "evergreen-ui";
import React from "react";
import { useHistory } from "react-router-dom";
import { Routes } from "../../constants";
import {
  LeaseListQuery,
  PropertyListQuery,
  TenantListQuery,
} from "../../helpers";
import { OnboardingProps } from "./onboarding";
import { translate } from "piteo-kit";

const _ = translate();

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const history = useHistory();

    const propertyListResult = useQuery(PropertyListQuery);
    const tenantListResult = useQuery(TenantListQuery);
    const leaseListResult = useQuery(LeaseListQuery);

    const shouldRedirectToDashboard = leaseListResult?.data?.leases?.length > 0;

    const handleInviteFriendClick = () => {
      window.open(_("invite_friend_email"));
    };

    const handleImportFromExistingSolutionClick = () => {
      // history.push(Routes.DASHBOARD_IMPORT); TODO: Route is no existing
      toaster.notify(_("feature_available_soon"));
    };

    const handlePropertyAddClick = (): void => {
      history.push(Routes.DASHBOARD_PROPERTY_ADD);
    };

    const handleTenantAddClick = (): void => {
      history.push(Routes.DASHBOARD_TENANT_ADD);
    };

    const onRentalAddClick = () => {
      history.push(Routes.LEASE_ADD);
    };

    const componentProps: OnboardingProps = {
      propertyListResult,
      tenantListResult,
      leaseListResult,
      shouldRedirectToDashboard,
      onAddPropertyButtonClick: handlePropertyAddClick,
      onTenantAddButtonClick: handleTenantAddClick,
      onRentalAddButtonClick: onRentalAddClick,
      onInviteFriendClick: handleInviteFriendClick,
      onImportFromExistingSolution: handleImportFromExistingSolutionClick,
    };

    return WrappedComponent(componentProps);
  };
