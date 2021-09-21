// @ts-nocheck
import {
  Alert,
  Avatar,
  Badge,
  Button,
  Card,
  defaultTheme,
  Heading,
  majorScale,
  minorScale,
  Pane,
  PaneProps,
  Table,
  Text,
} from "evergreen-ui";
import moment from "moment";
import { translate } from "piteo-kit";
import React from "react";
import Skeleton from "react-loading-skeleton";
import { NumberHelper } from "../../helpers";
import { RentHelper } from "../../helpers/rent-helper";
import { Lease, Property, RentStatus } from "../../types";
import { AmountLabel } from "../common/amount-label";

const _ = translate();

export type RentReceivedStatusWidgetProps = {
  loading?: boolean;
  loadingByAction?: boolean;
  property?: Property;
  showRentalContract?: (contract: Lease) => void;
  markRentAsPaid?: (paid: boolean, contract: Lease) => void;
  editReceipt?: (lease: Lease) => void;
} & Omit<PaneProps, "property">;

export const RentReceivedStatusWidget: React.FunctionComponent<
  RentReceivedStatusWidgetProps
> = ({
  loading,
  loadingByAction,
  property,
  editReceipt,
  showRentalContract,
  markRentAsPaid,
}) => {
  const pendingRentAmount = property?.expectedRents - property?.collectedRents;
  return (
    <Card
      minHeight={100}
      elevation={1}
      background="white"
      border="muted"
      padding={majorScale(2)}
      marginY={majorScale(2)}
    >
      <Pane
        display="flex"
        justifyContent="space-between"
        marginBottom={minorScale(3)}
      >
        <Pane display="flex" alignItems="start">
          {loading
            ? (
              <Skeleton circle={true} width={40} height={40} />
            )
            : (
              <Avatar size={40} name={property?.name} />
            )}
          <Pane marginLeft={10} display="flex" flexDirection="column">
            <Heading>{property?.name}</Heading>
            {!loading && property && (
              <Pane display="flex" marginTop={minorScale(1)}>
                {property?.leases?.length > 0
                  ? (
                    <Heading size={100}>
                      {_(
                        property?.leases?.length === 1
                          ? "tenant_single_count"
                          : "tenant_multi_count",
                        { count: property?.leases?.length },
                      )}
                    </Heading>
                  )
                  : (
                    <Alert
                      intent="warning"
                      title={_("no_lease")}
                    />
                  )}
              </Pane>
            )}
          </Pane>
        </Pane>
        <Pane>
          {loading
            ? (
              <Skeleton width={100} />
            )
            : (
              <Card
                flex={1}
                display="flex"
                flexDirection="column"
                justifyContent="space-between"
                background={defaultTheme.palette.blue.lightest}
                padding={minorScale(2)}
              >
                <Pane
                  display="flex"
                  flexDirection="row"
                  alignItems="flex-end"
                  justifyContent="space-between"
                >
                  <Heading size={100}>{_("cashin")}</Heading>
                  <AmountLabel
                    marginLeft={minorScale(1)}
                    value={property?.collectedRents || 0}
                  />
                </Pane>
                <Pane
                  display="flex"
                  flexDirection="row"
                  alignItems="flex-end"
                  justifyContent="space-between"
                >
                  <Heading size={100}>{_("pending")}</Heading>
                  <Heading
                    size={300}
                    color={pendingRentAmount > 0
                      ? defaultTheme.palette.red.dark
                      : defaultTheme.palette.neutral.dark}
                    marginLeft={minorScale(1)}
                  >
                    {property?.expectedRents
                      ? NumberHelper.formatToString(pendingRentAmount, false)
                      : "-"}
                  </Heading>
                </Pane>
              </Card>
            )}
        </Pane>
      </Pane>
      <Pane borderTop="muted">
        <Table>
          <Table.Body>
            {property?.leases.map((lease, index) => (
              <Table.Row
                key={lease.id}
                borderBottom={index + 1 === property?.leases?.length
                  ? "none"
                  : "muted"}
              >
                <Table.Cell textAlign="start">
                  <Avatar name={lease.tenants?.[0]?.fullName} />
                  <Text marginLeft={minorScale(2)} size={400}>
                    {lease.tenants?.[0]?.shortName}
                  </Text>
                </Table.Cell>
                <Table.TextCell>
                  {_("date_period", {
                    periodStart: moment(lease.rents?.[0]?.periodStart).format(
                      "DD/MM",
                    ),
                    periodEnd: moment(lease.rents?.[0]?.periodEnd).format(
                      "DD/MM",
                    ),
                  })}
                </Table.TextCell>
                <Table.Cell>
                  <AmountLabel value={lease.rentFullAmount} />
                </Table.Cell>
                <Table.Cell>
                  <Badge
                    color={RentHelper.rentStatusMapColor(
                      lease.rents?.[0]?.status,
                    )}
                  >
                    {RentHelper.rentStatusMap().get(lease.rents?.[0]?.status)}
                  </Badge>
                </Table.Cell>
                <Pane display="flex" alignItems="center">
                  {/* TODO: Get back to the popin menu */}
                  {
                    /* <RentReceivedMenuPopin
                    tenant={lease.tenants?.[0]}
                    rentStatus={lease.rent?.status}
                    lease={lease}
                    showRentalContract={showRentalContract}
                    markRentAsPaid={markRentAsPaid}
                    loading={loadingByAction}
                  /> */
                  }
                  <Button
                    isLoading={loading || loadingByAction}
                    disabled={lease.rents?.[0]?.status === RentStatus.Settled}
                    appearance="primary"
                    onClick={() => editReceipt?.(lease)}
                  >
                    {_("edit_receipt")}
                  </Button>
                </Pane>
              </Table.Row>
            ))}
          </Table.Body>
        </Table>
      </Pane>
    </Card>
  );
};
