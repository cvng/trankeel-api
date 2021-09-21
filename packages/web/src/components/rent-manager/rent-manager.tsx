// @ts-nocheck
import {
  Avatar,
  Badge,
  Card,
  Checkbox,
  Heading,
  InboxSearchIcon,
  minorScale,
  Pane,
  Table,
} from "evergreen-ui";
import moment from "moment";
import { rentStatus, rentStatusBadgeMapColor, translate } from "piteo-kit";
import React from "react";
import Skeleton from "react-loading-skeleton";
import { RentListCountByStatusType } from ".";
import { useAppTheme } from "../../hooks/use-app-theme";
import { Rent, RentStatus } from "../../types";
import { EmptyDataset } from "../common";
import { AmountLabel } from "../common/amount-label";
import { PeriodSelector } from "../common/period-selector";
import { NavGroupedItems } from "../nav-grouped-items/nav-grouped-items";
import {
  RentCollectedBar,
  RentCollectedBarProps,
} from "../rent-collected-bar/rent-collected-bar";
import {
  RentManagerMultiActionsMenu,
  RentManagerMultiActionsMenuType,
} from "./rent-manager-multi-actions-menu";
import {
  RentManagerSingleActionsMenu,
  RentManagerSingleActionsMenuType,
} from "./rent-manager-single-actions-menu";
import {
  RentManagerSummary,
  RentManagerSummaryProps,
} from "./rent-manager-summary";

const PERIOD_START_DATE_FORMAT = "DD/MM";
const PERIOD_END_DATE_FORMAT = "DD/MM/YYYY";

const _ = translate("");

export type RentManagerProps = {
  loading: boolean;
  allRentsSelected?: boolean;
  selectedRentList?: Rent[];
  selectedRentStatus?: RentStatus;
  groupedRentList?: RentListCountByStatusType;
  rentList?: Rent[];
  summaryData?: RentManagerSummaryProps;
  collectedBarData?: RentCollectedBarProps;
  onChangeRentStatus?: (status: RentStatus) => void;
  onSelectRentClick?: (rent: Rent, selected: boolean) => void;
  onSelectAllTenantsClick?: (selected: boolean) => void;
  onSelectRentSingleAction?: (
    type: RentManagerSingleActionsMenuType,
    rent: Rent,
  ) => void;
  onSelectRentMultiAction?: (
    type: RentManagerMultiActionsMenuType,
  ) => void;
};

export const RentManager: React.FunctionComponent<
  RentManagerProps
> = ({
  loading,
  allRentsSelected = false,
  selectedRentList = [],
  selectedRentStatus,
  rentList = [],
  collectedBarData,
  summaryData,
  groupedRentList,
  onChangeRentStatus,
  onSelectRentClick,
  onSelectAllTenantsClick,
  onSelectRentSingleAction,
  onSelectRentMultiAction,
}) => {
  const theme = useAppTheme();
  const hasNoData = rentList?.length === 0;
  if (loading) {
    return <Pane padding={minorScale(2)}>
      <Skeleton count={3} height={40} />
    </Pane>;
  }
  return (
    <Card
      display="flex"
      flexDirection="column"
      justifyContent="flex-start"
      background="white"
      flex={1}
      elevation={1}
    >
      <Pane
        display="flex"
        justifyContent="space-between"
      >
        <Pane
          display="flex"
          flex={1}
          justifyContent="space-between"
          justifyItems="center"
          alignItems="center"
          borderBottom="muted"
          paddingX={theme.margin.large}
          paddingY={theme.margin.medium}
        >
          <Heading size={500}>
            {_("synthesis")}
          </Heading>

          <PeriodSelector
            startedDate={moment()
              .startOf("month")
              .toDate()}
            endedDate={moment()
              .endOf("month")
              .toDate()}
          />
        </Pane>
      </Pane>

      {/* Summary */}
      <RentManagerSummary {...summaryData} />

      {/* Rent collected */}
      {collectedBarData && <Pane
        flex={1}
        display="flex"
        justifyContent="center"
        padding={theme.margin.large}
      >
        <RentCollectedBar {...collectedBarData} />
      </Pane>}

      <Pane
        display="flex"
        justifyContent="space-between"
        paddingY={theme.margin.small}
        paddingX={theme.margin.medium}
      >
        {/* Nav grouped items */}
        <NavGroupedItems
          items={[
            {
              id: "0",
              title: _("all"),
              selected: selectedRentStatus === null,
              onNavItemClick: () => onChangeRentStatus?.(null),
              badge: groupedRentList?.all,
            },
            {
              id: "1",
              title: _("rent_status_pending"),
              selected: selectedRentStatus === RentStatus.Pending,
              onNavItemClick: () => onChangeRentStatus?.(RentStatus.Pending),
              badge: groupedRentList?.pending,
              badgeColor: "neutral",
            },
            {
              id: "2",
              title: _("rent_status_partial"),
              selected: selectedRentStatus === RentStatus.Partial,
              onNavItemClick: () => onChangeRentStatus?.(RentStatus.Partial),
              badge: groupedRentList?.partial,
              badgeColor: "neutral",
            },
            {
              id: "3",
              title: _("rent_status_completed"),
              selected: selectedRentStatus === RentStatus.Settled,
              onNavItemClick: () => onChangeRentStatus?.(RentStatus.Settled),
              badge: groupedRentList?.settled,
              badgeColor: "neutral",
            },
          ]}
        />
      </Pane>

      {/* Table */}
      <Table flex={1} paddingX={theme.margin.medium}>
        <Table.Head height={40}>
          <Pane
            marginLeft={10}
            marginRight={12}
            display="flex"
            justifyContent="space-between"
            alignItems="center"
            flex={selectedRentList?.length > 0 ? 1 : null}
          >
            <Checkbox
              data-test-id="all-tenants-selected-id"
              disabled={hasNoData}
              checked={allRentsSelected}
              label={selectedRentList?.length > 0
                ? (_(
                  selectedRentList?.length > 1
                    ? "selected_tenants_count"
                    : "selected_tenant_single_count",
                  { count: selectedRentList?.length },
                ))
                : null}
              onChange={(e) => onSelectAllTenantsClick?.(e.target.checked)}
            />
            {/* Action pour tous les loyers / locataires sélectionnés */}
            {selectedRentList?.length > 0 &&
              <RentManagerMultiActionsMenu
                onSelectItem={onSelectRentMultiAction}
              />}
          </Pane>
          {selectedRentList?.length === 0 && <>
            <Table.TextHeaderCell>
              {_("tenant")}
            </Table.TextHeaderCell>
            <Table.TextHeaderCell>{_("amount")}</Table.TextHeaderCell>
            <Table.TextHeaderCell>{_("period")}</Table.TextHeaderCell>
            <Table.TextHeaderCell>{_("status")}</Table.TextHeaderCell>
            <Table.TextHeaderCell></Table.TextHeaderCell>
          </>}
        </Table.Head>
        <Table.Body borderTop="muted">
          {hasNoData && <EmptyDataset
            title={_("no_data")}
            subtitle={selectedRentStatus === RentStatus.Partial
              ? _("no_data_rent_partial")
              : selectedRentStatus === RentStatus.Settled
              ? _("no_data_rent_completed")
              : selectedRentStatus === RentStatus.Pending
              ? _("no_data_rent_pending")
              : ""}
            icon={InboxSearchIcon}
            height={220}
          />}
          {rentList?.map((rent, index) => {
            const tenantName = rent.lease?.tenants?.map((tenant) =>
              tenant.fullName
            ).join(
              ", ",
            );
            const tenantShortName = rent.lease?.tenants?.map((tenant) =>
              tenant.shortName
            ).join(
              ", ",
            );
            return (
              <Table.Row
                key={rent.id}
                data-test-id="rent-item"
                isSelected={selectedRentList?.includes(rent)}
                display="flex"
                alignItems="center"
                borderBottom={index === rentList?.length - 1 ? "none" : "muted"} // Hide the bottom line for the last item
              >
                <Pane marginLeft={10}>
                  <Checkbox
                    data-test-id="rent-checkbox-item"
                    checked={selectedRentList?.includes(rent)}
                    onChange={(e) =>
                      onSelectRentClick?.(rent, e.target.checked)}
                  />
                </Pane>
                <Table.Cell>
                  <Avatar
                    name={tenantName}
                    size={30}
                    marginRight={theme.margin.medium}
                  />
                  <Heading size={400}>{tenantShortName}</Heading>
                </Table.Cell>
                <Table.Cell>
                  <AmountLabel
                    value={rent.fullAmount}
                  />
                </Table.Cell>
                <Table.TextCell>
                  {_("date_period", {
                    periodStart: moment(rent.periodStart).format(
                      PERIOD_START_DATE_FORMAT,
                    ),
                    periodEnd: moment(rent.periodEnd).subtract(1, "d").format(
                      PERIOD_END_DATE_FORMAT,
                    ),
                  })}
                </Table.TextCell>
                <Table.TextCell>
                  <Badge
                    color={rentStatusBadgeMapColor(rent.status)}
                  >
                    {rentStatus(rent.status)}
                  </Badge>
                </Table.TextCell>

                <Table.Cell display="flex" justifyContent="flex-end">
                  {!allRentsSelected &&
                    <RentManagerSingleActionsMenu
                      status={rent.status}
                      onSelectItem={(type) =>
                        onSelectRentSingleAction?.(type, rent)}
                    />}
                </Table.Cell>
              </Table.Row>
            );
          })}
        </Table.Body>
      </Table>
    </Card>
  );
};
