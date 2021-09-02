import {
  CogIcon,
  DashboardIcon,
  HomeIcon,
  PeopleIcon,
  ThemeProvider,
} from "evergreen-ui";
import React from "react";
import { Routes } from "../../constants";
import { translate } from "piteo-kit";
import { AppTheme } from "../../theme/app-theme";
import { MainMenu } from "./main-menu";

export default {
  title: "Menu/MainMenu",
  component: MainMenu,
};

const _ = translate();

const collapsedItem = {
  title: _("settings"),
  link: "#",
  icon: CogIcon,
  selected: false,
  collapsed: true,
  items: [
    {
      title: "Profil",
      link: "#",
      icon: CogIcon,
      selected: false,
    },
  ],
};

const uncollapsedItem = {
  title: _("settings"),
  link: "#",
  icon: CogIcon,
  selected: false,
  collapsed: false,
  items: [
    {
      title: "Profil",
      link: "#",
      icon: CogIcon,
      selected: false,
    },
  ],
};

const items = [
  {
    title: _("dashboard"),
    link: Routes.DASHBOARD,
    icon: DashboardIcon,
    selected: true,
    badge: 2,
  },
  {
    title: _("properties"),
    link: Routes.PROPERTY_VIEW,
    icon: HomeIcon,
    selected: false,
  },
  {
    title: _("tenants"),
    link: Routes.TENANTS,
    icon: PeopleIcon,
    selected: false,
  },
];

export const normal = () => (
  <ThemeProvider value={AppTheme}>
    <MainMenu items={items} />
  </ThemeProvider>
);

export const disabled = () => (
  <ThemeProvider value={AppTheme}>
    <MainMenu items={items} disabled />
  </ThemeProvider>
);

export const collapsed = () => (
  <ThemeProvider value={AppTheme}>
    <MainMenu items={[...items, { ...collapsedItem }]} />
  </ThemeProvider>
);

export const uncollapsed = () => (
  <ThemeProvider value={AppTheme}>
    <MainMenu items={[...items, { ...uncollapsedItem }]} />
  </ThemeProvider>
);
