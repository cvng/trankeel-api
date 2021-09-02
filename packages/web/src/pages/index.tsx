import { ThemeProvider } from "evergreen-ui";
import { translate } from "piteo-kit";
import React from "react";
import { BrowserRouter, Redirect, Route, Switch } from "react-router-dom";
import {
  BillingSubscribeContainer,
  BillingSubscribePage
} from "../components/billing";
import { ComingSoon } from "../components/coming-soon/coming-soon";
import { Advertisement } from "../components/common";
import { ConfirmDialog } from "../components/common/confirm-dialog";
import { Dashboard, DashboardContainer } from "../components/dashboard";
import {
  ImportStatusCornerDialog,
  ImportUploadContainer,
  ImportUploadDialog
} from "../components/import-upload-dialog";
import {
  LeaseAddFlowContainer,
  LeaseAddFlowPage
} from "../components/lease-add";
import { LeaseAddProvider } from "../components/lease-add/lease-add-context";
import { LeaseDeleteContainer } from "../components/lease-dialog";
import {
  LenderDialog,
  LenderUpdateContainer
} from "../components/lender-dialog";
import {
  LoginForgotPasswordContainer,
  LoginForgotPasswordForm,
  LoginForm,
  LoginFormContainer,
  LoginRegister,
  RegisterContainer
} from "../components/login-page";
import { MainPage } from "../components/main-menu/main-page";
import { MaintenancePage } from "../components/maintenance-page/maintenance-page";
import { Onboarding, OnboardingContainer } from "../components/onboarding";
import {
  PropertyAddContainer,
  PropertyDeleteContainer,
  PropertyDialog,
  PropertyEditContainer
} from "../components/property-dialog";
import {
  PropertySynthesisPageContainer
} from "../components/property-synthesis";
import {
  PropertySynthesisProvider
} from "../components/property-synthesis/property-synthesis-context";
import {
  RentReceiptAddPage,
  RentReceiptAddPageContainer
} from "../components/rent-receipt";
import {
  SettingsAccount,
  SettingsLendersListContainer,
  SettingsMenu,
  SettingsMenuContainer
} from "../components/settings";
import { SettingsLenders } from "../components/settings/settings-lenders";
import { SynthesisPage } from "../components/synthesis/synthesis-page";
import {
  TenantAddContainer,
  TenantDeleteContainer,
  TenantDialog,
  TenantEditContainer
} from "../components/tenant-dialog";
import { TenantSynthesisPageContainer } from "../components/tenant-synthesis";
import { TenantSynthesisProvider } from "../components/tenant-synthesis/tenant-synthesis-context";
import { TransactionDeleteContainer } from "../components/transaction-delete-dialog";
import { Routes } from "../constants/routes-constants";
import { AuthProvider } from "../context/auth-context";
import { AuthenticatedRoute } from "../routes/authenticated-route";
import { AppTheme } from "../theme/app-theme";

const _ = translate();

const Home: React.FC = () => {
  return (
    <ThemeProvider value={AppTheme}>
      <Advertisement
        title={_("help_us_feedback")}
        buttonTitle={_("contact_us")}
      />
      <BrowserRouter>
        <Switch>
          {/* Default route /login */}
          <Redirect exact from="/" to={Routes.LOGIN} />
          <Route
            exact
            path={Routes.REGISTER}
            component={RegisterContainer(LoginRegister)}
          />
          <Route
            exact
            path={Routes.LOGIN}
            component={LoginFormContainer(LoginForm)}
          />
          <Route
            exact
            path={Routes.FORGOT_PASSWORD}
            component={LoginForgotPasswordContainer(LoginForgotPasswordForm)}
          />
          <Route
            exact
            path={Routes.MAINTENANCE}
            component={MaintenancePage}
          />
          <AuthProvider>
            <MainPage>
              {/* Subscribe */}
              <AuthenticatedRoute
                path={Routes.SUBSCRIBE}
                component={BillingSubscribeContainer(BillingSubscribePage)}
              />
              {/* Dashboard */}
              <AuthenticatedRoute
                exact
                path={Routes.DASHBOARD}
                component={DashboardContainer(Dashboard)}
              />
              <AuthenticatedRoute
                path={Routes.DASHBOARD_ONBOARDING}
                component={OnboardingContainer(Onboarding)}
              />
              <AuthenticatedRoute
                exact
                path={Routes.DASHBOARD_TENANT_ADD}
                component={TenantAddContainer(TenantDialog)}
              />
              <AuthenticatedRoute
                path={Routes.DASHBOARD_IMPORT}
                component={ImportUploadContainer(ImportStatusCornerDialog)}
              />

              <AuthenticatedRoute
                path={Routes.DASHBOARD_TRANSACTION_DELETE}
                component={TransactionDeleteContainer(ConfirmDialog)}
              />

              {/* Transactions */}
              <AuthenticatedRoute
                exact
                path={Routes.TRANSACTION_RENT_ADD}
                component={RentReceiptAddPageContainer(RentReceiptAddPage)}
              />
              <AuthenticatedRoute
                exact
                path={Routes.PROPERTY_TRANSACTION_DELETE}
                component={TransactionDeleteContainer(ConfirmDialog)}
              />

              {/* Properties */}
              <PropertySynthesisProvider>
                <AuthenticatedRoute
                  path={[Routes.PROPERTY_VIEW, Routes.PROPERTIES]}
                  component={PropertySynthesisPageContainer(
                    SynthesisPage,
                  )}
                />
                <AuthenticatedRoute
                  path={[Routes.PROPERTY_VIEW_LEASE_DELETE]}
                  component={LeaseDeleteContainer(
                    ConfirmDialog,
                  )}
                />
              </PropertySynthesisProvider>

              <AuthenticatedRoute
                exact
                path={[
                  Routes.PROPERTY_ADD,
                  Routes.LEASE_ADD_NEW_PROPERTY,
                  Routes.DASHBOARD_PROPERTY_ADD,
                ]}
                component={PropertyAddContainer(PropertyDialog)}
              />
              <AuthenticatedRoute
                path={Routes.PROPERTY_DELETE}
                component={PropertyDeleteContainer(ConfirmDialog)}
              />
              <AuthenticatedRoute
                path={Routes.PROPERTY_EDIT}
                component={PropertyEditContainer(PropertyDialog)}
              />

              {/* Tenants */}
              <TenantSynthesisProvider>
                <AuthenticatedRoute
                  path={[Routes.TENANT_VIEW, Routes.TENANTS]}
                  component={TenantSynthesisPageContainer(SynthesisPage)}
                />
              </TenantSynthesisProvider>

              <AuthenticatedRoute
                exact
                path={Routes.TENANT_ADD}
                component={TenantAddContainer(TenantDialog)}
              />

              <AuthenticatedRoute
                path={Routes.TENANT_DELETE}
                component={TenantDeleteContainer(ConfirmDialog)}
              />
              <AuthenticatedRoute
                path={Routes.TENANT_EDIT}
                component={TenantEditContainer(TenantDialog)}
              />

              {/* Leases */}
              <LeaseAddProvider>
                <AuthenticatedRoute
                  exact
                  path={[
                    Routes.LEASE_ADD,
                    Routes.LEASE_ADD_FROM_PROPERTY,
                    Routes.LEASE_ADD_FROM_TENANT,
                  ]}
                  component={LeaseAddFlowContainer(LeaseAddFlowPage)}
                />
              </LeaseAddProvider>
              <AuthenticatedRoute
                exact
                path={[Routes.LEASE_ADD_NEW_TENANT, Routes.TENANT_ADD]}
                component={TenantAddContainer(TenantDialog)}
              />

              {/* Settings */}
              <AuthenticatedRoute
                path={Routes.SETTINGS}
                component={SettingsMenuContainer(SettingsMenu)}
              />
              <AuthenticatedRoute
                exact
                path={Routes.SETTINGS_ACCOUNT}
                component={SettingsAccount} // Toute la logique est récupérée du contexte du coup pas d'injection
              />
              <AuthenticatedRoute
                exact
                path={Routes.SETTINGS_IMPORT}
                component={ImportUploadContainer(ImportUploadDialog)}
              />
              <AuthenticatedRoute
                path={[Routes.SETTINGS_LENDERS, Routes.SETTINGS_LENDER_VIEW]}
                component={SettingsLendersListContainer(SettingsLenders)}
              />
              <AuthenticatedRoute
                exact
                path={[
                  Routes.SETTINGS_LENDER_EDIT,
                  Routes.DASHBOARD_LENDER_EDIT,
                ]}
                component={LenderUpdateContainer(LenderDialog)}
              />
              <AuthenticatedRoute
                exact
                path={[
                  Routes.SETTINGS_SUBSCRIPTION,
                ]}
                component={ComingSoon}
              />

              <AuthenticatedRoute
                exact
                path={Routes.COMING_SOON}
                component={ComingSoon}
              />
            </MainPage>
          </AuthProvider>
        </Switch>
      </BrowserRouter>
    </ThemeProvider>
  );
};

export default Home;
