import React from "react";
import { Themable } from "../common/themable";
import { SettingsAccount } from "./settings-account";

export default {
  title: "Settings/SettingsAccount",
  component: SettingsAccount,
};

export const standard = () => (
  <Themable>
    <SettingsAccount />
  </Themable>
);
