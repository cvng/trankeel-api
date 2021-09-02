// @ts-nocheck
import { Button, EnvelopeIcon, InboxSearchIcon } from "evergreen-ui";
import { formatPhoneNumber, toLocaleDateString, translate } from "piteo-kit";
import * as React from "react";
import { useAppTheme } from "../../hooks/use-app-theme";
import { Tenant } from "../../types";
import { CardSynthesisItem } from "../cards";
import { EmptyDataset } from "../common";

const _ = translate();

export type TenantSynthesisInformationsProps = {
  loading?: boolean;
  tenant?: Tenant;
};

export const TenantSynthesisInformations: React.FunctionComponent<
  TenantSynthesisInformationsProps
> = ({
  loading,
  tenant,
}) => {
  const theme = useAppTheme();

  return (
    <CardSynthesisItem
      emptyDataset={!tenant && <EmptyDataset
        title={_("no_data")}
        subtitle={_("no_data_tenant")}
        icon={InboxSearchIcon}
      />}
      title={_("informations")}
      items={[
        {
          title: _("email"),
          text: tenant?.email || "-",
        },
        {
          title: _("phone_number"),
          text: formatPhoneNumber(tenant?.phoneNumber) || "-",
        },
        {
          title: _("date_of_birth"),
          text: toLocaleDateString(tenant?.birthdate) || "-",
        },
        {
          title: _("place_of_birth"),
          text: tenant?.birthplace || "-",
        },
        {
          title: _("observations"),
          text: tenant?.note || "-",
        },
      ]}
      buttons={[
        <Button
          disabled
          iconBefore={EnvelopeIcon}
          width={260}
          marginY={theme.margin.medium}
          onClick={() => {}}
        >
          {_("send_email")}
        </Button>,
      ]}
      loading={loading}
    />
  );
};
