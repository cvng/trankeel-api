import { Heading, Icon, Pane, Text } from "evergreen-ui";
import React from "react";
import { Routes } from "../../constants";
import { useAppTheme } from "../../hooks/use-app-theme";

export type SettingsMenuItem = {
  icon: React.ElementType;
  title: string;
  subtitle: string;
  selected?: boolean;
  route: Routes;
};

export type SettingsMenuProps = {
  items: SettingsMenuItem[];
  onSelectMenuItem?: (route: Routes) => void;
};

const MAX_WIDTH = 350;
const MIN_WIDTH = 280;

export const SettingsMenu: React.FunctionComponent<
  SettingsMenuProps
> = ({
  items = [],
  onSelectMenuItem = (route) => {},
}) => {
  const theme = useAppTheme();
  return (
    <Pane maxWidth={MAX_WIDTH} minWidth={MIN_WIDTH} borderRight="muted">
      {items?.map((item: SettingsMenuItem, index) =>
        <Pane
          key={index}
          display="flex"
          padding={theme.margin.large}
          background={item?.selected ? theme.palette.blue.lightest : null}
          borderBottom="muted"
          style={{ cursor: "pointer" }}
          onClick={() => onSelectMenuItem?.(item.route)}
        >
          <Pane marginRight={theme.margin.medium} padding={2}>
            <Icon icon={item.icon} color={theme.colors.text.default} />
          </Pane>
          <Pane
            alignItems="center"
            justifyContent="flex-start"
            flexDirection="row"
          >
            <Heading>{item.title}</Heading>
            <Pane marginTop={theme.margin.small}>
              <Text size={300}>{item.subtitle}</Text>
            </Pane>
          </Pane>
        </Pane>
      )}
    </Pane>
  );
};
