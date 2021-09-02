import { toaster } from "evergreen-ui";
import { translate } from "piteo-kit";
import { useHistory } from "react-router-dom";
import { PropertySynthesisRoutes } from "../components/property-synthesis";
import { TenantSynthesisRoutes } from "../components/tenant-synthesis";
import { RouteById, Routes } from "../constants";
import { Closure, UniquableClosure } from "../utils";

const _ = translate();

export type Destinations = {
  goBack: Closure;
  showLogin: Closure;
  showRegister: Closure;
  showForgotPassword: Closure;
  showPropertyAdd: Closure;
  showPropertySynthesis: Closure;
  showPropertySynthesisRoute: (
    propertyId: string,
    route?: string,
  ) => void;
  showTenantAdd: Closure;
  showTenantEdit: UniquableClosure;
  showTenantDelete: UniquableClosure;
  showTenantSynthesis: Closure;
  showTenantSynthesisRoute: (
    tenantId: string,
    route?: string,
  ) => void;
  showLeaseAdd: Closure;
  showLeaseAddWithProperty: UniquableClosure;
  showLeaseAddWithTenant: UniquableClosure;
  showLeaseDelete: (propertyId: string, leaseId: string) => void;
  showLender: UniquableClosure;
  showLenderEdit: UniquableClosure;
  showComingSoon: Closure;
  showRentReceiptPreview: UniquableClosure;
  showRentReceiptSendAllConfirmation: UniquableClosure;
};

export const useRouter = (): Destinations => {
  const history = useHistory();
  return {
    goBack: () => {
      history.goBack();
    },
    showLogin: () => {
      history.push(Routes.LOGIN);
    },
    showRegister: () => {
      history.push(Routes.REGISTER);
    },
    showForgotPassword: () => {
      history.push(Routes.FORGOT_PASSWORD);
    },
    showPropertyAdd: () => {
      history.push(Routes.PROPERTY_ADD);
    },
    showPropertySynthesis: () => {
      history.push(Routes.PROPERTIES);
    },
    showPropertySynthesisRoute: (
      propertyId: string,
      route = PropertySynthesisRoutes.Synthesis,
    ) => {
      history.push(
        RouteById(Routes.PROPERTY_VIEW, [
          propertyId,
          route,
        ], [":id", ":route"]),
      );
    },
    showTenantAdd: () => {
      history.push(Routes.TENANT_ADD);
    },
    showTenantEdit: (id: string) => {
      history.push(RouteById(Routes.TENANT_EDIT, [id]));
    },
    showTenantDelete: (id: string) => {
      history.push(RouteById(Routes.TENANT_DELETE, [id]));
    },
    showTenantSynthesis: () => {
      history.push(Routes.TENANTS);
    },
    showTenantSynthesisRoute: (
      tenantId: string,
      route = TenantSynthesisRoutes.Synthesis,
    ) => {
      history.push(
        RouteById(Routes.TENANT_VIEW, [
          tenantId,
          route,
        ], [":id", ":route"]),
      );
    },
    showLeaseAdd: () => {
      history.push(
        Routes.LEASE_ADD,
      );
    },
    showLeaseAddWithProperty: (propertyId: string) => {
      history.push(
        RouteById(
          Routes.LEASE_ADD_FROM_PROPERTY,
          [propertyId],
          [":propertyId"],
        ),
      );
    },
    showLeaseAddWithTenant: (tenantId: string) => {
      history.push(
        RouteById(
          Routes.LEASE_ADD_FROM_TENANT,
          [tenantId],
          [":tenantId"],
        ),
      );
    },
    showLeaseDelete: (propertyId: string, leaseId: string) => {
      history.push(
        RouteById(Routes.PROPERTY_VIEW_LEASE_DELETE, [
          propertyId,
          leaseId,
        ], [":id", ":leaseId"]),
      );
    },
    showLender: (lenderId: string) => {
      history.push(RouteById(Routes.SETTINGS_LENDER_VIEW, [lenderId]));
    },
    showLenderEdit: (lenderId: string) => {
      history.push(RouteById(Routes.SETTINGS_LENDER_EDIT, [lenderId]));
    },
    showComingSoon: () => {
      toaster.notify(_("feature_available_soon"), {
        id: "soon",
      });
    },
    showRentReceiptPreview: (leaseId: string) => {
      history.push(
        RouteById(Routes.DASHBOARD_PREVIEW_RENT_RECEIPT, [
          leaseId,
        ], [
          ":leaseId",
        ]),
      );
    },
    showRentReceiptSendAllConfirmation: () => {
      history.push(Routes.DASHBOARD_MARK_ALL_RENT_PAY_CONFIRMATION);
    },
  };
};
