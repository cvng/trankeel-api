import { CogIcon, CreditCardIcon, PersonIcon } from "evergreen-ui";
import React from "react";
import { Routes } from "../../constants";
import { translate } from "piteo-kit";
import { Themable } from "../common/themable";
import { SettingsMenu } from "./settings-menu";

const _ = translate();

export default {
  title: "Settings/SettingsMenu",
  component: SettingsMenu,
};

const items = [
  {
    title: _("account"),
    subtitle: _("account_subtitle"),
    icon: CogIcon,
    selected: true,
    route: Routes.SETTINGS,
  },
  {
    title: _("profiles"),
    subtitle: _("profiles_subtitle"),
    icon: PersonIcon,
    route: Routes.SETTINGS,
  },
  {
    title: _("billing"),
    subtitle: _("billing_subtitle"),
    icon: CreditCardIcon,
    route: Routes.SETTINGS,
  },
];

export const standard = () => (
  <Themable>
    <SettingsMenu items={items} />
  </Themable>
);
