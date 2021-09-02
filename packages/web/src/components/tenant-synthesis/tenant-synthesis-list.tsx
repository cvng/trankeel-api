import { translate } from "piteo-kit";
import * as React from "react";
import animationData from "../../assets/lotties/people-morph-flow.json";
import { Tenant } from "../../types";
import { EntityList } from "../entity-list/entity-list";

const _ = translate();

export type TenantSynthesisListProps = {
  /** Tenants list */
  tenants?: Tenant[];
  /** The selected tenant */
  selectedTenant?: Tenant;
  /** Loading status */
  loading: boolean;
  /** True if the search field is not empty */
  filterEnabled: boolean;
  /** Fired when a tenant is selected  */
  onTenantSelect?: (tenantId: string) => void;
  /** Fired when clicking "add" button */
  onTenantAdd?: () => void;
  /** Fired when search field change */
  onSearchFieldChange?: (value: string) => void;
};
export const TenantSynthesisList: React.FunctionComponent<
  TenantSynthesisListProps
> = ({
  tenants,
  selectedTenant,
  loading,
  filterEnabled,
  onTenantSelect,
  onTenantAdd,
  onSearchFieldChange,
}) => {
  return (
    <EntityList
      loading={loading}
      filterEnabled={filterEnabled}
      entities={tenants}
      onSearchFieldChange={onSearchFieldChange}
      onSelectEntity={onTenantSelect}
      onAddNewEntity={onTenantAdd}
      emptyDatasetAnimation={animationData}
      entity={_("tenant")}
      cardData={(tenant: Tenant) => {
        return {
          key: tenant.id,
          title: tenant.fullName,
          subtitle: tenant.propertyName || "",
          selected: tenant.id === selectedTenant?.id,
        };
      }}
    />
  );
};
