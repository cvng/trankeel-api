import { InboxSearchIcon } from "evergreen-ui";
import { translate } from "piteo-kit";
import * as React from "react";
import { NumberHelper } from "../../helpers";
import { Tenant } from "../../types";
import { CardSynthesisItem } from "../cards";
import { EmptyDataset } from "../common";

const _ = translate();

export type TenantSynthesisGuaranteesProps = {
  loading?: boolean;
  tenant?: Tenant;
};

export const TenantSynthesisGuarantees: React.FunctionComponent<
  TenantSynthesisGuaranteesProps
> = ({
  loading,
  tenant,
}) => {
  return (
    <CardSynthesisItem
      emptyDataset={!tenant && <EmptyDataset
        title={_("no_data")}
        subtitle={_("no_data_guarantees")}
        icon={InboxSearchIcon}
      />}
      title={_("guarantees")}
      items={[
        {
          title: _("caution"),
          tooltip: tenant?.visaleId
            ? _("number", { number: tenant?.visaleId })
            : undefined,
          text: tenant?.visaleId ? _("caution_type_visale") : "-",
        },
        {
          title: _("apl"),
          text: tenant?.apl ? _("yes") : _("no"),
        },
        {
          title: _("deposit_amount"),
          text: NumberHelper.formatToString(
            tenant?.lease?.depositAmount,
            false,
          ) || "-",
        },
      ]}
      loading={loading}
    />
  );
};
