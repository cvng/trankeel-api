import { CogIcon, CreditCardIcon, PersonIcon } from "evergreen-ui";
import { translate } from "piteo-kit";
import React from "react";
import { useHistory, useLocation } from "react-router-dom";
import { Routes } from "../../constants";
import { isSelectedRoute } from "../../utils";
import { SettingsMenuProps } from "./settings-menu";

const _ = translate();

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const history = useHistory();
    const url = useLocation().pathname;

    const settingsAccountRoute = Routes.SETTINGS_ACCOUNT;
    const settingsLenderRoute = Routes.SETTINGS_LENDERS;
    const settingsSubscriptionRoute = Routes.SETTINGS_SUBSCRIPTION;

    const items = [
      {
        title: _("account"),
        subtitle: _("account_subtitle"),
        icon: CogIcon,
        selected: isSelectedRoute(url, settingsAccountRoute),
        route: settingsAccountRoute,
      },
      {
        title: _("lenders"),
        subtitle: _("lenders_subtitle"),
        selected: isSelectedRoute(url, settingsLenderRoute),
        icon: PersonIcon,
        route: settingsLenderRoute,
      },
      {
        title: _("billing"),
        subtitle: _("billing_subtitle"),
        selected: isSelectedRoute(url, settingsSubscriptionRoute),
        icon: CreditCardIcon,
        route: settingsSubscriptionRoute,
      },
    ];

    const onSelectMenuItem = (route: Routes): void => {
      history.push(route.toString());
    };

    const componentProps: SettingsMenuProps = {
      items,
      onSelectMenuItem,
    };

    return WrappedComponent(componentProps);
  };
