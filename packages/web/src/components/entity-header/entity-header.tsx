import { Pane, PaneProps } from "evergreen-ui";
import * as React from "react";
import { useAppTheme } from "../../hooks/use-app-theme";
import {
  PopinMenuButton,
  PopinMenuButtonItemProps,
} from "../buttons/popin-menu-button";
import { CardItem, CardItemProps } from "../cards/card-item";

export type EntityHeaderProps<T> = {
  loading: boolean;
  cardItemProps: CardItemProps<T>;
  popinItemsProps: PopinMenuButtonItemProps[];
} & PaneProps;

export const EntityHeader: React.FunctionComponent<
  EntityHeaderProps<unknown>
> = ({
  loading,
  cardItemProps,
  popinItemsProps,
  ...props
}) => {
  const theme = useAppTheme();
  return (
    <Pane
      display="flex"
      justifyContent="space-between"
      flex={1}
      marginLeft={theme.margin.large}
      {...props}
    >
      <CardItem
        {...cardItemProps}
      />
      <Pane display="flex" alignItems="center">
        {popinItemsProps && <PopinMenuButton
          items={popinItemsProps || []}
        />}
      </Pane>
    </Pane>
  );
};
