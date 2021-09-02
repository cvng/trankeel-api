import { Pane } from "evergreen-ui";
import React from "react";
import animationData from "../../assets/lotties/51084-contract-signing.json";
import { translate } from "piteo-kit";
import { useAppTheme } from "../../hooks/use-app-theme";
import { PageContent } from "../common";
import { NavMenu } from "../nav-menu/nav-menu";

const _ = translate();

export type LeaseAddFlowDelegate = {
  onPageDidFinish();
};

export type LeaseAddFlowPageProps = {
  selectedIndex: number;
  checkedItems: number[];
  allItemsDisabled: boolean;
  leasePartiesComponent: React.ReactNode;
  leaseFormComponent: React.ReactNode;
  leaseConfirmationComponent: React.ReactNode;
  menuItemClick?: (index: number) => void;
};

export const LeaseAddFlowPage: React.FunctionComponent<
  LeaseAddFlowPageProps
> = ({
  selectedIndex,
  checkedItems,
  allItemsDisabled,
  leasePartiesComponent,
  leaseFormComponent,
  leaseConfirmationComponent,
  menuItemClick,
}) => {
  const theme = useAppTheme();
  const items = [
    {
      title: _("parties"),
      subtitle: _("select_property_tenant"),
      checked: checkedItems.includes(0),
      selected: selectedIndex === 0,
      disabled: allItemsDisabled || false,
      component: leasePartiesComponent,
    },
    {
      title: _("contract_conditions"),
      subtitle: _("contract_conditions_hint"),
      checked: checkedItems.includes(1),
      selected: selectedIndex === 1,
      disabled: allItemsDisabled || !checkedItems.includes(0),
      component: leaseFormComponent,
    },
    {
      title: _("contract_confirmation"),
      subtitle: _("contract_confirmation_hint"),
      checked: checkedItems.includes(2),
      selected: selectedIndex === 2,
      disabled: !checkedItems.includes(1),
      component: leaseConfirmationComponent,
    },
  ];
  return (
    <PageContent title={_("lease_new")}>
      <Pane display="flex">
        <Pane flexBasis={350}>
          <NavMenu
            subtitle={_("new_rental_hint")}
            animation={animationData}
            items={items}
            onNavItemClick={menuItemClick}
            style={{ position: "sticky", top: theme.margin.large }}
          />
        </Pane>

        <Pane
          flex={1}
          display="flex"
          justifyContent="center"
        >
          {items[selectedIndex].component}
        </Pane>
      </Pane>
    </PageContent>
  );
};
