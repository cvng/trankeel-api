import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { Property } from "../../types";
import { Themable } from "../common/themable";
import { ListItemSelection } from "./list-item-selection";

export default {
  title: "Selection/ListItemSelection",
  component: ListItemSelection,
};

const properties = FactoryHelper.propertyList();

export const standard = () => (
  <Themable>
    <ListItemSelection
      title={"Sélectionnez le bien"}
      subtitle="Sélectionner le bien concerné par la création du contrat"
      items={properties}
      titleForItem={(item: Property) => item.name}
    />
  </Themable>
);
