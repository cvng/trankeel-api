import { Pane } from "evergreen-ui";
import * as React from "react";
import {
  PropertySynthesisEnergy,
  PropertySynthesisEquipments,
  PropertySynthesisInformations,
} from ".";
import { Property } from "../../types";
import { UniquableClosure } from "../../utils";

export type PropertySynthesisDefaultProps = {
  /** Loading */
  loading?: boolean;
  /** Property */
  property?: Property;
  /** Fired on select a lender */
  onLenderSelect?: UniquableClosure;
};

export const PropertySynthesisDefault: React.FunctionComponent<
  PropertySynthesisDefaultProps
> = ({
  loading,
  property,
  onLenderSelect,
}) => {
  return (
    <Pane key={property?.id}>
      {/* Informations */}
      <PropertySynthesisInformations
        loading={loading}
        property={property}
        onLenderSelect={onLenderSelect}
      />

      {/* Equipments */}
      <PropertySynthesisEquipments
        loading={loading}
        property={property}
      />

      {/* Energy */}
      <PropertySynthesisEnergy
        loading={loading}
        property={property}
      />
    </Pane>
  );
};
