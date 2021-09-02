import { translate } from "piteo-kit";
import * as React from "react";
import { Lease, Property, Tenant } from "../../types";
import { EntityList } from "../entity-list/entity-list";
import animationData from "../../assets/lotties/22489-city-office-building-center-mocca-animation.json";

const _ = translate();

export type PropertySynthesisListProps = {
  /** Properties list */
  properties?: Property[];
  /** The selected property */
  selectedProperty?: Property;
  /** Loading status */
  loading: boolean;
  /** True if the search field is not empty */
  filterEnabled: boolean;
  /** Fired when a table table row is selected  */
  onPropertySelect?: (propertyId: string) => void;
  /** Fired when clicking "add" button */
  onPropertyAdd?: () => void;
  /** Fired when search field change */
  onSearchFieldChange?: (value: string) => void;
};

export const PropertySynthesisList: React.FunctionComponent<
  PropertySynthesisListProps
> = ({
  properties,
  selectedProperty,
  loading,
  filterEnabled,
  onPropertySelect,
  onPropertyAdd,
  onSearchFieldChange,
}) => {
  const tenantNameList = (lease: Lease): string[] => {
    return lease?.tenants?.map((tenant: Tenant) => tenant.displayName);
  };
  const subtitle = (property: Property): string => {
    let value = "";
    if (property.leases?.length > 1) {
      value = property?.leases?.[0].tenants?.[0]?.displayName + " +" +
        String(property.leases?.length - 1);
    } else if (property.leases?.length === 1) {
      value = property?.leases?.map(tenantNameList).join("");
    }
    return value;
  };
  return (
    <EntityList
      loading={loading}
      filterEnabled={filterEnabled}
      entities={properties}
      onSearchFieldChange={onSearchFieldChange}
      onSelectEntity={onPropertySelect}
      onAddNewEntity={onPropertyAdd}
      entity={_("property")}
      emptyDatasetAnimation={animationData}
      cardData={(property: Property) => {
        return {
          key: property.id,
          title: property.name?.toLocaleUpperCase(),
          subtitle: subtitle(property),
          selected: property.id === selectedProperty?.id,
        };
      }}
    />
  );
};
