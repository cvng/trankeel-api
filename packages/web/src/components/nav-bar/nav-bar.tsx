import { Pane, PaneProps, Tab, TabNavigation } from "evergreen-ui";
import * as React from "react";
import { useAppTheme } from "../../hooks/use-app-theme";

const DEFAULT_NAV_BAR_HEIGHT = 80;

export type NavBarItem = {
  title: string;
  route: string;
  selected?: boolean;
  available?: boolean;
};

export type NavBarItemHandler = (item: NavBarItem) => void;

export type NavBarProps = {
  items: NavBarItem[];
  onSelectItem?: NavBarItemHandler;
  hidden?: boolean;
} & PaneProps;

export const NavBar: React.FunctionComponent<
  NavBarProps
> = ({
  items,
  onSelectItem,
  hidden,
  ...props
}) => {
  const theme = useAppTheme();
  return (
    <Pane
      key={0}
      display="flex"
      flexDirection="column"
      {...props}
      marginBottom={theme.margin.large}
      height={hidden ? 0 : props.height || DEFAULT_NAV_BAR_HEIGHT}
    >
      <TabNavigation marginTop={theme.margin.large}>
        {!hidden && items.map((tab, index) => (
          <Tab
            key={index}
            id={tab.title?.toLowerCase()}
            isSelected={tab.selected}
            onSelect={() => onSelectItem?.(tab)}
          >
            {tab.title}
          </Tab>
        ))}
      </TabNavigation>
    </Pane>
  );
};
