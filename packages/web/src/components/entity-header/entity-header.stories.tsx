import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { translate } from "piteo-kit";
import { Themable } from "../common/themable";
import { EntityHeader } from "./entity-header";

export default {
  title: "Design System/Entity/EntityHeader",
  component: EntityHeader,
};

const _ = translate();

const lender = FactoryHelper.lenderList()[0];

export const loading = () =>
  <Themable>
    <EntityHeader
      loading
      cardItemProps={{
        title: null,
        hasAvatar: false,
        loading: false,
        item: lender,
        elevation: null,
        backgroundColor: null,
        style: null, // Remove the cursor style
      }}
      popinItemsProps={null}
    />
  </Themable>;

export const noData = () =>
  <Themable>
    <EntityHeader
      loading={false}
      cardItemProps={{
        title: null,
        hasAvatar: false,
        loading: false,
        item: lender,
        elevation: null,
        backgroundColor: null,
        style: null, // Remove the cursor style
      }}
      popinItemsProps={null}
    />
  </Themable>;

export const withPhysicalPersonLender = () =>
  <Themable>
    <EntityHeader
      loading={false}
      cardItemProps={{
        title: lender.displayName,
        subtitle: lender.identity.__typename === "Company"
          ? _("moral_person")
          : _("physical_person"),
        hasAvatar: !!lender,
        loading: false,
        item: lender,
        elevation: null,
        backgroundColor: null,
        style: null, // Remove the cursor style
      }}
      popinItemsProps={null}
    />
  </Themable>;
