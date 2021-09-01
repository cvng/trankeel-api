import { InboxSearchIcon } from "evergreen-ui";
import { capitalize, PropertyHelper } from "piteo-kit";
import * as React from "react";
import { Property } from "../../types";
import { CardSynthesisItem } from "../cards";
import { EmptyDataset } from "../common";
import { translate } from "piteo-kit";

const _ = translate();

export type PropertySynthesisEnergyProps = {
  /** Loading */
  loading?: boolean;
  /** Property */
  property?: Property;
};

export const PropertySynthesisEnergy: React.FunctionComponent<
  PropertySynthesisEnergyProps
> = ({
  loading,
  property,
}) => {
  return (
    <CardSynthesisItem
      emptyDataset={!property && <EmptyDataset
        title={_("no_data")}
        subtitle={_("no_data_property_other")}
        icon={InboxSearchIcon}
      />}
      title={_("other")}
      items={[
        {
          title: _("energy_class"),
          text: property?.energyClass
            ? PropertyHelper.energyClassMap().get(
              property?.energyClass,
            )
            : "-",
        },
        {
          title: _("gas_emission"),
          text: property?.gasEmission
            ? PropertyHelper.gasEmissionMap().get(
              property?.gasEmission,
            )
            : "-",
        },
        {
          title: _("heating_method"),
          text: property?.heatingMethod
            ? PropertyHelper.individualOrCollective().get(
              property?.heatingMethod,
            )
            : "-",
        },
        {
          title: _("water_heating_method"),
          text: property?.waterHeatingMethod
            ? PropertyHelper.individualOrCollective().get(
              property?.waterHeatingMethod,
            )
            : "-",
        },
        {
          title: _("observations"),
          text: capitalize(property?.note) || "-",
        },
      ]}
      loading={loading}
    />
  );
};
