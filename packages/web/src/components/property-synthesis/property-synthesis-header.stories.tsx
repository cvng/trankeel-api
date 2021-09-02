import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { Themable } from "../common/themable";
import { PropertySynthesisHeader } from "./property-synthesis-header";

export default {
  title: "Property/Synthesis/PropertySynthesisHeader",
  component: PropertySynthesisHeader,
};

const property = FactoryHelper.propertyList()[0];

export const standard = () =>
  <Themable>
    <PropertySynthesisHeader property={property} />
  </Themable>;
