import { QueryResult } from "@apollo/client";
import { ArrowsVerticalIcon, Icon, majorScale, Pane, Text } from "evergreen-ui";
import React, { useContext } from "react";
import { Redirect } from "react-router-dom";
import { Routes } from "../../constants";
import { AuthContext } from "../../context/auth-context";
import { translate } from "piteo-kit";
import {
  LeaseListQuery,
  PropertyListQuery,
  TenantListQuery,
} from "../../types";
import { ScrollableContent } from "../common";
import { PageContent } from "../common/page-content";
import { ProfileItem } from "../common/profile-item";
import { AddDataWidget } from "../widgets/add-data-widget";
import { ImportWidget } from "../widgets/import-widget";
import { WelcomeWidget } from "../widgets/welcome-widget";

const _ = translate();

export type OnboardingProps = {
  propertyListResult: QueryResult<PropertyListQuery>;
  tenantListResult: QueryResult<TenantListQuery>;
  leaseListResult: QueryResult<LeaseListQuery>;
  shouldRedirectToDashboard: boolean;
  onAddPropertyButtonClick: () => void;
  onTenantAddButtonClick: () => void;
  onRentalAddButtonClick: () => void;
  onInviteFriendClick: () => void;
  onImportFromExistingSolution: () => void;
};

export const Onboarding: React.FunctionComponent<OnboardingProps> = ({
  propertyListResult,
  tenantListResult,
  leaseListResult,
  shouldRedirectToDashboard,
  onAddPropertyButtonClick,
  onTenantAddButtonClick,
  onRentalAddButtonClick,
  onInviteFriendClick,
  onImportFromExistingSolution,
}) => {
  const context = useContext(AuthContext);
  if (shouldRedirectToDashboard) {
    return <Redirect to={Routes.DASHBOARD} />;
  }
  return (
    <ScrollableContent>
      <PageContent
        title={_("title")}
        titleRightView={<Pane display="flex" flexDirection="row">
          <ProfileItem profile={context?.user} />
        </Pane>}
      >
        <Pane
          display="flex"
          flexDirection="row"
          justifyContent="center"
        >
          <Pane display="flex" flex={1}></Pane>
          <Pane
            display="flex"
            flexDirection="column"
            minWidth={700}
            flex={3}
            justifyContent="center"
          >
            {/* Composant de bienvenue */}
            <WelcomeWidget
              fullName={context?.user?.firstName}
              onInviteFriendClick={onInviteFriendClick}
            />

            <Pane>
              <Text>{_("onboarding_proposal")}</Text>
              <AddDataWidget
                hasProperty={propertyListResult.data?.properties
                  ?.length > 0}
                isLoadingProperty={propertyListResult.loading}
                hasTenant={tenantListResult.data?.tenants?.length > 0}
                isLoadingTenant={tenantListResult.loading}
                hasContract={leaseListResult.data?.leases.length >
                  0}
                isLoadingContract={leaseListResult.loading}
                onPropertyAddClick={onAddPropertyButtonClick}
                onTenantAddClick={onTenantAddButtonClick}
                onRentalAddClick={onRentalAddButtonClick}
              />
              <Pane
                display="flex"
                justifyContent="center"
                alignItems="center"
                marginY={majorScale(2)}
              >
                <Icon icon={ArrowsVerticalIcon} color="muted" />
              </Pane>
              <ImportWidget
                onImportFromExistingSolutionClick={onImportFromExistingSolution}
              />
            </Pane>
          </Pane>
          <Pane display="flex" flex={1}></Pane>
        </Pane>
      </PageContent>
    </ScrollableContent>
  );
};
