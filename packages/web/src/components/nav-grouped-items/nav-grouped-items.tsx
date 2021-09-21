import { Heading, Pane, Pill } from "evergreen-ui";
import React from "react";
import { useAppTheme } from "../../hooks/use-app-theme";
import { UniquableClosure } from "../../utils";
import { FlexRow } from "../common/flex-row";

export type NavGroupedItemProps = {
  id: string;
  title: string;
  selected: boolean;
  onNavItemClick: UniquableClosure;
  badge?: number;
  badgeColor?: "blue" | "red" | "orange" | "neutral";
};

export type NavGroupedItemsProps = {
  items: NavGroupedItemProps[];
};

export const NavGroupedItems: React.FunctionComponent<
  NavGroupedItemsProps
> = ({
  items,
}) => {
  const theme = useAppTheme();
  return (
    <FlexRow
      justifyContent="center"
      alignItems="flex-start"
      flexDirection="column"
      marginLeft={10}
    >
      <Pane display="flex">
        {items?.map((item: NavGroupedItemProps, index: number) => {
          return (<Pane
            key={item.id}
            data-test-id={"rent-status-nav-item"}
            style={{ cursor: "pointer" }}
            borderBottom={item.selected
              ? "3px solid " + theme.palette.blue.base
              : ""}
            height={30}
            marginRight={theme.margin.large}
            display="flex"
            onClick={() => item?.onNavItemClick(item.id)}
          >
            <Heading size={300}>
              {item.title}
            </Heading>
            <Pill
              isSolid={index === 0}
              color={item.badgeColor || "blue"}
              marginLeft={theme.margin.medium}
            >
              {item.badge}
            </Pill>
          </Pane>);
        })}
      </Pane>
    </FlexRow>
  );
};
