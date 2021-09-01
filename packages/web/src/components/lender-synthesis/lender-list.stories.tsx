import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { Themable } from "../common/themable";
import { LenderList } from ".";

export default {
  title: "Lender/List",
  component: LenderList,
};

const lenders = FactoryHelper.lenderList();

export const standard = () =>
  <Themable>
    <LenderList
      lenders={lenders}
      selectedLender={lenders?.[0]}
      loading={false}
      filterEnabled={false}
    />
  </Themable>;

export const noLender = () =>
  <Themable>
    <LenderList
      lenders={[]}
      loading={false}
      filterEnabled={false}
    />
  </Themable>;

export const noFilteredLender = () =>
  <Themable>
    <LenderList lenders={[]} loading={false} filterEnabled />
  </Themable>;

export const loading = () =>
  <Themable>
    <LenderList lenders={[]} loading filterEnabled={false} />
  </Themable>;
