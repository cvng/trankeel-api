import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { Themable } from "../common/themable";
import { PropertySynthesisList } from "./";

export default {
  title: "Property/Synthesis/PropertySynthesisList",
  component: PropertySynthesisList,
};

const properties = FactoryHelper.propertyList();

export const standard = () =>
  <Themable>
    <PropertySynthesisList
      properties={properties}
      selectedProperty={properties?.[0]}
      loading={false}
      filterEnabled={false}
    />
  </Themable>;

export const noProperty = () =>
  <Themable>
    <PropertySynthesisList
      properties={[]}
      loading={false}
      filterEnabled={false}
    />
  </Themable>;

export const noFilteredProperty = () =>
  <Themable>
    <PropertySynthesisList properties={[]} loading={false} filterEnabled />
  </Themable>;

export const loading = () =>
  <Themable>
    <PropertySynthesisList properties={[]} loading filterEnabled={false} />
  </Themable>;
