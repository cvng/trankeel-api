import { translate } from "piteo-kit";
import React, { useContext, useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import {
  PropertySynthesisDefault,
  PropertySynthesisHeader,
  PropertySynthesisHeaderContainer,
  PropertySynthesisLease,
  PropertySynthesisList,
  PropertySynthesisListContainer,
  PropertySynthesisRoutes,
} from ".";
import { useRouter } from "../../hooks/use-router";
import { NavBar, NavBarItem } from "../nav-bar/nav-bar";
import { SynthesisPageProps } from "../synthesis/synthesis-page";
import {
  PropertySynthesisContext,
  PropertySynthesisContextProps,
} from "./property-synthesis-context";

const _ = translate();

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const router = useRouter();
    const { route } = useParams();

    const [navigationItems, setNavigationItems] = useState([]);

    const { selectedProperty: property, properties, loading } = useContext(
      PropertySynthesisContext,
    ) as PropertySynthesisContextProps;

    useEffect(() => {
      setNavigationItems([
        {
          title: _("synthesis"),
          route: PropertySynthesisRoutes.Synthesis,
          selected: !route || route === PropertySynthesisRoutes.Synthesis,
          available: true,
        },
        {
          title: property?.leases?.length > 1 ? _("leases") : _("lease"),
          route: PropertySynthesisRoutes.Leases,
          selected: route === PropertySynthesisRoutes.Leases,
          available: true,
        },
        {
          title: _("documents"),
          route: PropertySynthesisRoutes.Documents,
          selected: route === PropertySynthesisRoutes.Documents,
          available: false,
        },
        {
          title: _("works"),
          route: PropertySynthesisRoutes.Works,
          selected: route === PropertySynthesisRoutes.Works,
          available: false,
        },
        {
          title: _("sinisters"),
          route: PropertySynthesisRoutes.Sinisters,
          selected: route === PropertySynthesisRoutes.Sinisters,
          available: false,
        },
      ]);
    }, [route, property?.leases?.length]);

    const onSelectNavigationMenu = (
      item: NavBarItem,
    ): void => {
      if (!item.available) {
        router.showComingSoon();
        return;
      }
      router.showPropertySynthesisRoute(
        property?.id || properties?.[0]?.id,
        item.route,
      );
    };

    const synthesisComponent = PropertySynthesisDefault({
      loading,
      property,
      onLenderSelect: router.showLender,
    });
    const leasesComponent = PropertySynthesisLease({
      loading,
      property,
      onTenantSelect: router.showTenantSynthesisRoute,
      onLeaseAdd: router.showLeaseAddWithProperty,
      onDeleteLease: router.showLeaseDelete,
    });

    const contentComponent = (): React.ReactNode => {
      switch (route) {
        case PropertySynthesisRoutes.Leases:
          return leasesComponent;
        default:
          return synthesisComponent;
      }
    };

    const componentProps: SynthesisPageProps = {
      title: _("properties"),
      listComponent: PropertySynthesisListContainer(PropertySynthesisList)(
        {},
      ),
      items: [
        PropertySynthesisHeaderContainer(
          PropertySynthesisHeader,
        )({}),
        NavBar({
          items: navigationItems,
          onSelectItem: onSelectNavigationMenu,
          hidden: properties?.length === 0,
        }),
        contentComponent(),
      ],
    };

    return WrappedComponent(componentProps);
  };
