// @ts-nocheck
import {
  Button,
  EyeOpenIcon,
  InboxSearchIcon,
  PlusIcon,
  ShieldIcon,
} from "evergreen-ui";
import moment from "moment";
import { translate } from "piteo-kit";
import * as React from "react";
import { NumberHelper } from "../../helpers";
import { RentHelper } from "../../helpers/rent-helper";
import { useAppTheme } from "../../hooks/use-app-theme";
import { Tenant } from "../../types";
import { UniquableClosure } from "../../utils";
import { CardSynthesisItem } from "../cards";
import { EmptyDataset } from "../common";

const _ = translate();

const PERIOD_DATE_FORMAT = "DD MMMM YYYY";

export type TenantSynthesisSummaryProps = {
  loading?: boolean;
  tenant?: Tenant;
  onSelectLease?: UniquableClosure;
  onPropertySelect?: UniquableClosure;
  onLeaseAddFromTenant?: UniquableClosure;
};

export const TenantSynthesisSummary: React.FunctionComponent<
  TenantSynthesisSummaryProps
> = ({
  loading,
  tenant,
  onSelectLease,
  onLeaseAddFromTenant,
  onPropertySelect,
}) => {
  const theme = useAppTheme();
  const title = _("synthesis");
  const propertyAvatars = tenant?.lease?.property
    ? [
      {
        name: tenant?.lease?.property?.name || "-",
        handler: () => onPropertySelect?.(tenant?.lease?.property?.id),
      },
    ]
    : [];
  return (
    <CardSynthesisItem
      emptyDataset={(!tenant || !tenant?.lease) && <EmptyDataset
        title={_("no_data")}
        subtitle={_("no_data_lease")}
        icon={InboxSearchIcon}
        button={<Button
          iconBefore={PlusIcon}
          appearance="primary"
          onClick={() => onLeaseAddFromTenant?.(tenant?.id)}
        >
          {_("lease_new")}
        </Button>}
      />}
      title={title}
      items={[
        {
          title: _("rent_payment"),
          tooltip: _("date_period", {
            periodStart: moment().startOf("month").format(PERIOD_DATE_FORMAT),
            periodEnd: moment().endOf("month").format(PERIOD_DATE_FORMAT),
          }),
          badges: [
            {
              color: RentHelper.rentStatusMapColor(
                tenant?.lease?.rents?.[0].status,
              ),
              value: RentHelper.rentStatusMap().get(
                tenant?.lease?.rents?.[0].status,
              ),
            },
          ],
        },
        {
          title: _("tenant_unpaid_amount"),
          badges: [
            {
              color: tenant?.unpaidRentAmount > 0 ? "red" : "green",
              value: NumberHelper.formatToString(tenant?.unpaidRentAmount || 0),
            },
          ],
        },
        {
          title: _("rent_full_amount"),
          text: NumberHelper.formatToString(
            tenant?.lease?.rentFullAmount,
            false,
          ) || "-",
          tooltip: tenant?.lease?.rentFullAmount > 0
            ? _("rent_charges_hint", {
              amount: NumberHelper.formatToString(
                tenant?.lease?.rentChargesAmount,
                false,
              ),
            })
            : null,
        },
        {
          title: _("property"),
          text: tenant?.lease?.property ? "" : "-",
          avatars: propertyAvatars,
        },
      ]}
      buttons={[
        <Button
          disabled={!tenant?.lease?.id}
          iconBefore={EyeOpenIcon}
          width={260}
          marginY={theme.margin.medium}
          onClick={() => onSelectLease?.(tenant?.lease?.property?.id)}
        >
          {_("preview_lease")}
        </Button>,
        <Button
          iconBefore={ShieldIcon}
          width={260}
          marginY={theme.margin.medium}
          intent="danger"
          disabled={!tenant?.unpaidRentAmount}
        >
          {_("start_unpaid_flow")}
        </Button>,
      ]}
      loading={loading}
    />
  );
};
