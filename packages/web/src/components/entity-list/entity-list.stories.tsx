import React from "react";
import { translate } from "piteo-kit";
import { FactoryHelper } from "../../helpers/factory-helper";
import { Property } from "../../types";
import { Themable } from "../common/themable";
import { EntityList } from "./entity-list";

export default {
  title: "Design System/Entity/EntityList",
  component: EntityList,
};

const properties = FactoryHelper.propertyList();

const selectedProperty = properties[1];

const _ = translate();

export const loading = () => (
  <Themable>
    <EntityList
      entities={[]}
      loading={true}
      onSelectEntity={() => {}}
      onSearchFieldChange={() => {}}
      entity={_("property")}
      cardData={(property: Property) => {
        return {
          key: property.id,
          title: property.name,
          selected: property.id === selectedProperty.id,
          subtitle: property?.address
            ? `${property?.address?.line1} ${property?.address
              ?.city}`
            : "",
        };
      }}
    />
  </Themable>
);

export const emptyDataset = () => (
  <Themable>
    <EntityList
      entities={[]}
      loading={false}
      onSelectEntity={() => {}}
      onSearchFieldChange={() => {}}
      entity={_("property")}
      cardData={(property: Property) => {
        return {
          key: property.id,
          title: property.name,
          selected: property.id === selectedProperty.id,
          subtitle: property?.address
            ? `${property?.address?.line1} ${property?.address
              ?.city}`
            : "",
        };
      }}
    />
  </Themable>
);

export const withNoSelectedEntity = () => (
  <Themable>
    <EntityList
      entities={properties}
      loading={false}
      onSelectEntity={() => {}}
      onSearchFieldChange={() => {}}
      entity={_("property")}
      cardData={(property: Property) => {
        return {
          key: property.id,
          title: property.name,
          selected: false,
          subtitle: property?.address
            ? `${property?.address?.line1} ${property?.address
              ?.city}`
            : "",
        };
      }}
    />
  </Themable>
);

export const withSelectedEntity = () => (
  <Themable>
    <EntityList
      entities={properties}
      loading={false}
      onSelectEntity={() => {}}
      onSearchFieldChange={() => {}}
      entity={_("property")}
      cardData={(property: Property) => {
        return {
          key: property.id,
          title: property.name,
          selected: property.id === selectedProperty.id,
          subtitle: property?.address
            ? `${property?.address?.line1} ${property?.address
              ?.city}`
            : "",
        };
      }}
    />
  </Themable>
);

export const withNoResult = () => (
  <Themable>
    <EntityList
      entities={[]}
      loading={false}
      filterEnabled
      onSelectEntity={() => {}}
      onSearchFieldChange={() => {}}
      entity={_("property")}
      cardData={(property: Property) => {
        return {
          key: property.id,
          title: property.name,
          selected: property.id === selectedProperty.id,
          subtitle: property?.address
            ? `${property?.address?.line1} ${property?.address
              ?.city}`
            : "",
        };
      }}
    />
  </Themable>
);
