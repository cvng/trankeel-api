import { translate } from "piteo-kit";
import React, { useContext, useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import {
  TenantDocumentListContainer,
  TenantSynthesisDefault,
  TenantSynthesisHeaderContainer,
  TenantSynthesisList,
  TenantSynthesisListContainer,
  TenantSynthesisRoutes,
} from ".";
import { useRouter } from "../../hooks/use-router";
import { NavBar, NavBarItem } from "../nav-bar/nav-bar";
import { PropertySynthesisRoutes } from "../property-synthesis";
import { SynthesisPageProps } from "../synthesis/synthesis-page";
import { TenantDocumentList } from "./tenant-document-list";
import {
  TenantSynthesisContext,
  TenantSynthesisContextProps,
} from "./tenant-synthesis-context";
import { TenantSynthesisHeader } from "./tenant-synthesis-header";

const _ = translate();

export type WrappedComponentProps = unknown;

export const withContainer = (
  WrappedComponent,
): React.FunctionComponent<WrappedComponentProps> =>
  () => {
    const router = useRouter();
    // Get the tenant id in the url
    const { id: tenantId, route } = useParams();

    const { selectedTenant: tenant, tenants, loading } = useContext(
      TenantSynthesisContext,
    ) as TenantSynthesisContextProps;

    const [navigationItems, setNavigationItems] = useState([]);

    useEffect(() => {
      setNavigationItems([
        {
          title: _("synthesis"),
          route: TenantSynthesisRoutes.Synthesis,
          selected: !route || route === TenantSynthesisRoutes.Synthesis,
        },
        {
          title: _("documents"),
          route: TenantSynthesisRoutes.Documents,
          selected: route === TenantSynthesisRoutes.Documents,
        },
      ]);
    }, [route]);

    const onSelectNavigationMenu = (item: NavBarItem): void => {
      router.showTenantSynthesisRoute(tenantId || tenants?.[0]?.id, item.route);
    };

    const onSelectLease = (propertyId: string): void => {
      router.showPropertySynthesisRoute(
        propertyId,
        PropertySynthesisRoutes.Leases,
      );
    };

    const onPropertySelect = (propertyId: string): void => {
      router.showPropertySynthesisRoute(
        propertyId,
        PropertySynthesisRoutes.Synthesis,
      );
    };

    const onLeaseAddFromTenant = (tenantId: string): void => {
      router.showLeaseAddWithTenant(tenantId);
    };

    const synthesisComponent = TenantSynthesisDefault({
      loading,
      tenant,
      onSelectLease,
      onLeaseAddFromTenant,
      onPropertySelect,
    });
    const documentListComponent = TenantDocumentListContainer(
      TenantDocumentList,
    )({});

    const contentComponent = (): React.ReactNode => {
      switch (route) {
        case TenantSynthesisRoutes.Documents:
          return documentListComponent;
        default:
          return synthesisComponent;
      }
    };

    const componentProps: SynthesisPageProps = {
      title: _("tenants"),
      listComponent: TenantSynthesisListContainer(TenantSynthesisList)(
        {},
      ),
      items: [
        TenantSynthesisHeaderContainer(
          TenantSynthesisHeader,
        )({}),
        NavBar({
          items: navigationItems,
          onSelectItem: onSelectNavigationMenu,
          hidden: tenants?.length === 0,
        }),
        contentComponent(),
      ],
    };

    return WrappedComponent(componentProps);
  };
