import React from "react";
import { FactoryHelper } from "../../helpers/factory-helper";
import { Themable } from "../common/themable";
import { SettingsLenders } from "./settings-lenders";

export default {
  title: "Settings/SettingsLender",
  component: SettingsLenders,
};

const lenders = FactoryHelper.lenderList();
const selectedLenderMoralPerson = lenders[0];
const selectedLenderPhysicalPerson = lenders[1];

export const loading = () => (
  <Themable>
    <SettingsLenders
      loading={true}
      selectedLender={null}
      lenderHeaderProps={{}}
      lenderListProps={{ loading: true, filterEnabled: false }}
    />
  </Themable>
);

export const noData = () => (
  <Themable>
    <SettingsLenders
      loading={false}
      selectedLender={null}
      lenderHeaderProps={{}}
      lenderListProps={{ loading: false, filterEnabled: false, lenders: [] }}
    />
  </Themable>
);

export const standardWithMoralPerson = () => (
  <Themable>
    <SettingsLenders
      loading={false}
      selectedLender={selectedLenderMoralPerson}
      lenderHeaderProps={{}}
      lenderListProps={{
        loading: false,
        filterEnabled: false,
        lenders: lenders,
        selectedLender: selectedLenderMoralPerson,
      }}
    />
  </Themable>
);

export const standardWithPhysicalPerson = () => (
  <Themable>
    <SettingsLenders
      loading={false}
      selectedLender={selectedLenderPhysicalPerson}
      lenderHeaderProps={{}}
      lenderListProps={{
        loading: false,
        filterEnabled: false,
        lenders: lenders,
        selectedLender: selectedLenderPhysicalPerson,
      }}
    />
  </Themable>
);
