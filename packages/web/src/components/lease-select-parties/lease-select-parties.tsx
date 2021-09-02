import { Button, DoubleChevronDownIcon, Icon, Pane } from "evergreen-ui";
import { translate } from "piteo-kit";
import React from "react";
import existingLeaseAnimation from "../../assets/lotties/19602-contract.json";
import leaseSigningAnimation from "../../assets/lotties/51084-contract-signing.json";
import { useAppTheme } from "../../hooks/use-app-theme";
import { Property, Tenant } from "../../types";
import { Closure } from "../../utils";
import { CardEntitySelection } from "../cards/card-entity-selection";
import { CardSelector } from "../cards/card-selector";

const _ = translate();

export interface LeaseSelectPartiesProps {
  loading: boolean;
  createFromExistingLease: boolean;
  // Property
  properties: Property[];
  selectedProperty?: Property;
  onAddProperty?: Closure;
  onSelectProperty?: (property: Property) => void;
  // Tenant
  tenants: Tenant[];
  selectedTenants?: Tenant[];
  onAddTenant?: Closure;
  onSelectTenant?: (tenant: Tenant) => void;
  // Called on select the card selector items
  onChangeType?: (existingLease: boolean) => void;
  onConfirmButtonClick?: Closure;
}

// Ecran de sélection du bien et du locataire dans le processus d'ajout d'un bail
export const LeaseSelectParties: React.FunctionComponent<
  LeaseSelectPartiesProps
> = ({
  loading,
  createFromExistingLease,
  properties,
  selectedProperty,
  onAddProperty,
  onSelectProperty,
  tenants,
  selectedTenants,
  onSelectTenant,
  onAddTenant,
  onChangeType,
  onConfirmButtonClick,
}) => {
  const theme = useAppTheme();
  return (
    <Pane background="tint1" padding={theme.margin.large} maxWidth={550}>
      <Pane display="flex" alignItems="center" justifyContent="center">
        <CardSelector
          onClickItem={(item) => onChangeType?.(item?.id === "0")}
          items={[
            {
              id: "0",
              title: _("add_existing_rental"),
              animation: existingLeaseAnimation,
              selected: createFromExistingLease,
            },
            {
              id: "1",
              title: _("generate_new_lease"),
              animation: leaseSigningAnimation,
              selected: !createFromExistingLease,
            },
          ]}
        />
      </Pane>

      <Pane
        display="flex"
        justifyContent="center"
        marginTop={theme.margin.large}
      >
        <Icon icon={DoubleChevronDownIcon} color={theme.palette.blue.base} />
      </Pane>

      {/* Sélection du bien */}
      <CardEntitySelection
        title={_("select_property")}
        index={1}
        loading={loading}
        items={properties}
        itemLabel={(item: Property) => item?.name}
        selectedItem={selectedProperty}
        onSelectItem={(property: Property) => onSelectProperty?.(property)}
        onAddItem={onAddProperty}
        addItemTitle={_("add_property")}
      />

      <Pane
        display="flex"
        justifyContent="center"
        marginTop={theme.margin.large}
      >
        <Icon icon={DoubleChevronDownIcon} color={theme.palette.blue.base} />
      </Pane>

      {/* Sélection du locataire */}
      <CardEntitySelection
        title={_("select_tenant")}
        index={2}
        loading={loading}
        items={tenants}
        itemLabel={(item: Tenant) => item?.displayName}
        selectedItem={selectedTenants?.[0]}
        onSelectItem={(tenant: Tenant) => onSelectTenant?.(tenant)}
        onAddItem={onAddTenant}
        addItemTitle={_("add_tenant")}
        alert={{
          intent: "none",
          content: tenants?.length === 0
            ? _("tenant_filter_no_contract_alert")
            : _("tenant_filter_no_contract"),
        }}
      />

      <Pane
        display="flex"
        justifyContent="center"
        marginTop={theme.margin.large}
      >
        <Icon icon={DoubleChevronDownIcon} color={theme.palette.blue.base} />
      </Pane>

      <Pane
        display="flex"
        justifyContent="center"
        marginTop={theme.margin.large}
      >
        <Button
          appearance="primary"
          disabled={!selectedProperty || selectedTenants?.length === 0}
          onClick={onConfirmButtonClick}
        >
          {_("continue")}
        </Button>
      </Pane>
    </Pane>
  );
};
