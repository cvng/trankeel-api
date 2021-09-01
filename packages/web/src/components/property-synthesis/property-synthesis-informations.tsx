import { InboxSearchIcon } from "evergreen-ui";
import { PropertyHelper } from "piteo-kit";
import * as React from "react";
import { NumberHelper } from "../../helpers";
import { Property } from "../../types";
import { CardSynthesisItem } from "../cards";
import { EmptyDataset } from "../common";
import { translate } from "piteo-kit";

const _ = translate();

export type PropertySynthesisInformationsProps = {
  /** Loading */
  loading?: boolean;
  /** Property */
  property?: Property;
  /** Fired on select a lender */
  onLenderSelect?: (lenderId: string) => void;
};

export const PropertySynthesisInformations: React.FunctionComponent<
  PropertySynthesisInformationsProps
> = ({
  loading,
  property,
  onLenderSelect,
}) => {
  return (
    <CardSynthesisItem
      emptyDataset={!property && <EmptyDataset
        title={_("no_data")}
        subtitle={_("no_data_property_informations")}
        icon={InboxSearchIcon}
      />}
      title={_("informations")}
      items={[
        {
          title: _("type"),
          badges: [{
            value: PropertyHelper.roomPropertyTypes().get(
              property?.roomCount,
            ) || "-",
            color: property?.roomCount ? "purple" : null,
          }],
        },
        {
          title: _("property_surface"),
          text: property?.surface
            ? _("surface_value", {
              surface: property?.surface,
            })
            : "-",
        },
        {
          title: _("housing_type"),
          text: property?.housingType
            ? PropertyHelper.individualOrCollective().get(
              property?.housingType,
            )
            : "-",
        },
        {
          title: _("building_period"),
          text: property?.buildPeriod
            ? PropertyHelper.buildPeriods().get(
              property?.buildPeriod,
            )
            : "-",
        },
        {
          title: _("building_legal_status"),
          text: property?.buildingLegalStatus
            ? PropertyHelper.buildingLegalStatuses().get(
              property?.buildingLegalStatus,
            )
            : "-",
        },
        {
          title: _("property_tax"),
          text: NumberHelper.formatToString(
            property?.tax,
            false,
          ) || "-",
        },
        {
          title: _("lender"),
          avatars: [
            {
              name: property?.lender?.displayName || "-",
              handler: () => onLenderSelect?.(property?.lender?.id),
            },
          ],
        },
      ]}
      loading={loading}
    />
  );
};
