import { CogIcon, DashboardIcon, HomeIcon, PeopleIcon } from "evergreen-ui";
import React, { useContext, useEffect, useState } from "react";
import { useHistory, useLocation } from "react-router-dom";
import { Routes } from "../../constants";
import { AuthContext } from "../../context/auth-context";
import { translate } from "piteo-kit";
import { isSelectedRoute } from "../../utils";
import { MainMenuItem, MainMenuProps } from "./main-menu";

const _ = translate();

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<MainMenuProps> =>
  () => {
    const history = useHistory();
    const url = useLocation().pathname;

    const { isTrialExpired } = useContext(AuthContext);

    const [items, setItems] = useState([]);

    useEffect(() => {
      setItems([
        {
          title: _("dashboard"),
          link: Routes.DASHBOARD,
          selected: isSelectedRoute(url, Routes.DASHBOARD),
          icon: DashboardIcon,
        },
        {
          title: _("properties"),
          link: Routes.PROPERTIES,
          selected: isSelectedRoute(url, Routes.PROPERTIES) ||
            isSelectedRoute(url, Routes.LEASE_ADD),
          icon: HomeIcon,
        },
        {
          title: _("tenants"),
          link: Routes.TENANTS,
          selected: isSelectedRoute(url, Routes.TENANTS),
          icon: PeopleIcon,
        },
        {
          title: _("settings"),
          link: Routes.SETTINGS_ACCOUNT,
          selected: isSelectedRoute(url, Routes.SETTINGS),
          icon: CogIcon,
        },
      ]);
    }, [url]);

    const handleSelectMenuItem = (item: MainMenuItem): void => {
      history.push(item.link);
    };

    const handleSelectLogo = (): void => {
      history.replace(Routes.DASHBOARD);
    };

    const componentProps: MainMenuProps = {
      items,
      onSelectMenuItem: handleSelectMenuItem,
      onSelectLogo: handleSelectLogo,
      disabled: isTrialExpired,
    };

    return WrappedComponent(componentProps);
  };
