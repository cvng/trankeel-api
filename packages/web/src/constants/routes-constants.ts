export enum Routes {
  DEFAULT = "/dashboard",
  DASHBOARD_ONBOARDING = "/dashboard/onboarding",
  DASHBOARD_TENANT_ADD = "/dashboard/onboarding/tenant/add",
  DASHBOARD_PROPERTY_ADD = "/dashboard/onboarding/property/add",
  DASHBOARD_IMPORT = "/dashboard/onboarding/import",
  DASHBOARD = "/dashboard",
  DASHBOARD_MARK_ALL_RENT_PAY_CONFIRMATION =
    "/dashboard/mark-all-receipts/confirmation",
  DASHBOARD_PREVIEW_RENT_RECEIPT = "/dashboard/preview-receipt/:leaseId",
  DASHBOARD_TRANSACTION_DELETE = "/dashboard/:transactionId/delete",
  DASHBOARD_SHOW_CONTRACT = "/dashboard/contract/:contractId",
  DASHBOARD_LENDER_EDIT = "/dashboard/lender/:id/edit",
  TRANSACTION_RENT_ADD = "/transactions/add/rent",
  PROPERTIES = "/properties",
  PROPERTY_VIEW = "/properties/:id/:route",
  PROPERTY_VIEW_LEASE_DELETE = "/properties/:id/leases/:leaseId/delete",
  PROPERTY_EDIT = "/properties/:id/edit",
  PROPERTY_DELETE = "/properties/:id/delete",
  PROPERTY_ADD = "/properties/add",
  PROPERTY_TRANSACTION_DELETE =
    "/properties/list/details/:id/:transactionId/delete",
  TENANTS = "/tenants",
  TENANT_VIEW = "/tenants/:id/:route",
  TENANT_ADD = "/tenants/add",
  TENANT_EDIT = "/tenants/:id/edit",
  TENANT_DELETE = "/tenants/:id/delete",
  LEASE_ADD = "/lease/add",
  LEASE_ADD_FROM_PROPERTY = "/lease/add/:propertyId",
  LEASE_ADD_FROM_TENANT = "/lease/add/tenant/:tenantId",
  LEASE_ADD_NEW_TENANT = "/lease/add/newTenant",
  LEASE_ADD_NEW_PROPERTY = "/lease/add/newProperty",
  SETTINGS = "/settings",
  SETTINGS_ACCOUNT = "/settings/account",
  SETTINGS_IMPORT = "/settings/import",
  SETTINGS_SUBSCRIPTION = "/settings/subscription",
  SETTINGS_LENDERS = "/settings/lenders",
  SETTINGS_LENDERS_ADD = "/settings/lenders/add",
  SETTINGS_LENDER_VIEW = "/settings/lenders/:id",
  SETTINGS_LENDER_EDIT = "/settings/lenders/:id/edit",
  SETTINGS_NOTIFICATIONS = "/settings/notifications",
  LOGIN = "/login",
  LOGIN_ACCOUNT_TYPE_SELECTION = "/account-selection",
  REGISTER = "/register",
  FORGOT_PASSWORD = "/forgot-password",
  MAINTENANCE = "/maintenance",
  COMING_SOON = "/coming-soon",
  SUBSCRIBE = "/subscribe",
}

// Replace the occurences in a route and return a formatted route
// For example /tenants/:id become /tenants/f39d74f7-b6a6-4927-b16a-1a70834e6b75
export function RouteById(
  route: Routes,
  values: string[],
  replacements: string[] = [":id"],
): string {
  let value = route.toString();
  replacements.forEach((rep, index) => {
    value = value.replace(rep, values[index]);
  });
  return value;
}
