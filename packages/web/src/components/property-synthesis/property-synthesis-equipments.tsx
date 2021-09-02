import { InboxSearchIcon } from "evergreen-ui";
import { inline, translate } from "piteo-kit";
import * as React from "react";
import { Property } from "../../types";
import { CardSynthesisItem } from "../cards";
import { EmptyDataset } from "../common";

const _ = translate();

export type PropertySynthesisEquipmentsProps = {
  /** Loading */
  loading?: boolean;
  /** Property */
  property?: Property;
};

export const PropertySynthesisEquipments: React.FunctionComponent<
  PropertySynthesisEquipmentsProps
> = ({
  loading,
  property,
}) => {
  return (
    <CardSynthesisItem
      emptyDataset={!property && <EmptyDataset
        title={_("no_data")}
        subtitle={_("no_data_property_equipments")}
        icon={InboxSearchIcon}
      />}
      title={_("equipments")}
      items={[
        {
          title: _("tenant_private_spaces"),
          text: inline(property?.tenantPrivateSpaces) || "-",
        },
        {
          title: _("common_equipments"),
          text: inline(property?.equipments) || "-",
        },
        {
          title: _("ntic_equipments"),
          text: inline(
            property?.nticEquipments,
          ) || "-",
        },
        {
          title: _("house_other_part"),
          text: inline(property?.otherSpaces) ||
            "-",
        },
        {
          title: _("common_spaces"),
          text: inline(property?.commonSpaces) ||
            "-",
        },
      ]}
      loading={loading}
    />
  );
};
