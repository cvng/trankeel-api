import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { Themable } from "../common/themable";
import { PropertySynthesisLease } from "./";

export default {
  title: "Property/Synthesis/PropertySynthesisLease",
  component: PropertySynthesisLease,
};

const properties = FactoryHelper.propertyList();

export const standard = () =>
  <Themable>
    <PropertySynthesisLease
      property={properties[0]}
      loading={false}
    />
  </Themable>;
