import {
  Avatar,
  Checkbox,
  majorScale,
  minorScale,
  Pane,
  Table,
  Text,
} from "evergreen-ui";
import React from "react";
import Skeleton from "react-loading-skeleton";
import { TourType } from "../../constants/tour-constants";
import { Tenant } from "../../types";
import { EmptyItem } from "../common";
import { AmountLabel } from "../common/amount-label";
import { FlexRow } from "../common/flex-row";
import { translate } from "piteo-kit";

const _ = translate("Transaction");

export type RentReceiptTenantSelectionProps = {
  isAllTenantsSelected: boolean;
  selectedTenants: Tenant[];
  tenants: Tenant[];
  loading: boolean;
  onSelectTenantClick: (tenant: Tenant) => void;
  onSelectAllTenantsClick: () => void;
};

export const RentReceiptTenantSelection: React.FunctionComponent<
  RentReceiptTenantSelectionProps
> = ({
  isAllTenantsSelected,
  selectedTenants,
  tenants,
  loading,
  onSelectTenantClick,
  onSelectAllTenantsClick,
}) => {
  return (
    <Pane
      display="flex"
      flexDirection="column"
      justifyContent="flex-start"
      borderRight={"default"}
      background="white"
      flex={1}
    >
      {loading && (
        <Pane padding={minorScale(2)}>
          <Skeleton count={3} height={40} />
        </Pane>
      )}

      {!loading && (
        <FlexRow
          justifyContent="center"
          alignItems="flex-start"
          flexDirection="column"
          padding={majorScale(2)}
          borderBottom="muted"
          height={80}
        >
          {!loading && <Text>{_("select_all_tenant_rent_receipt")}</Text>}
        </FlexRow>
      )}

      {!loading && (
        <Pane>
          <Checkbox
            checked={isAllTenantsSelected}
            marginLeft={10}
            label={_("select_all_tenants")}
            onChange={onSelectAllTenantsClick}
          />
        </Pane>
      )}
      <Table flex={1}>
        <Table.Body className={TourType.TRANSACTION_RENT_ADD_SEL_TENANT}>
          {tenants.map((tenant) => (
            <Table.Row
              key={tenant.fullName}
              isSelected={!!selectedTenants.find((aTenant: Tenant) =>
                aTenant.id === tenant.id
              )}
            >
              <Pane marginLeft={10}>
                {/* On affiche la checkbox uniquement pour les locataires ayant un contrat de location actif */}
                <Checkbox
                  disabled={!!tenant?.lease?.rentFullAmount === false}
                  checked={!!selectedTenants.find((aTenant) =>
                    aTenant.id === tenant.id
                  )}
                  onChange={() => onSelectTenantClick(tenant)}
                />
              </Pane>
              <Table.Cell>
                <Avatar
                  name={tenant.fullName}
                  size={30}
                  paddingX={10}
                  marginRight={10}
                />
                <Text size={300}>{tenant.fullName}</Text>
              </Table.Cell>
              <Table.Cell display="flex" justifyContent="flex-end">
                {/* Si le locataire possède une location active */}
                {tenant?.lease?.rentFullAmount
                  ? (
                    <AmountLabel
                      value={tenant.lease?.rentFullAmount}
                    />
                  )
                  : (
                    <EmptyItem />
                  )}
              </Table.Cell>
            </Table.Row>
          ))}
        </Table.Body>
      </Table>
    </Pane>
  );
};
