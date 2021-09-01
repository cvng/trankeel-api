import { Card, Heading, Pane, PaneProps, Text } from "evergreen-ui";
import React from "react";
import { useAppTheme } from "../../hooks/use-app-theme";
import { CheckListItem, CheckListItemProps } from "../common/check-list-item";
import { LottieAnimation } from "../common/lottie-animation";

export type NavMenuComponentProps = {
  component: React.ReactNode;
} & CheckListItemProps;

export type NavMenuProps = {
  items: NavMenuComponentProps[];
  title?: string;
  onNavItemClick?: (index: number) => void;
  subtitle?: string;
  animation?: unknown;
} & PaneProps;

export const NavMenu: React.FunctionComponent<
  NavMenuProps
> = ({
  items,
  title,
  onNavItemClick,
  subtitle,
  animation,
  ...props
}) => {
  const theme = useAppTheme();
  return (
    <Card
      elevation={1}
      minWidth={240}
      flexDirection="column"
      background="white"
      {...props}
    >
      {/* Title bar */}
      <Pane
        display="flex"
        borderBottom="muted"
        paddingX={theme.margin.largest}
      >
        <Pane
          display="flex"
          flexDirection="column"
          flex={1}
          justifyContent="space-around"
          paddingY={theme.margin.large}
        >
          <Pane>
            <Heading size={500}>{title}</Heading>
            <Text>{subtitle}</Text>
          </Pane>
        </Pane>
        {animation && <Pane display="flex">
          {/* Animation */}
          <LottieAnimation data={animation} width={120} />
        </Pane>}
      </Pane>
      <Pane>
        <Pane
          borderRight={"muted"}
          display="flex"
          flexDirection="column"
          justifyContent="space-between"
        >
          <Pane>
            {items.map((item, index) => {
              return (<CheckListItem
                key={index}
                {...item}
                index={index + 1}
                onClick={() => onNavItemClick && onNavItemClick(index)}
              />);
            })}
          </Pane>
        </Pane>
      </Pane>
    </Card>
  );
};
