import {
  Avatar,
  Card,
  Heading,
  minorScale,
  Pane,
  PaneProps,
  Text,
} from "evergreen-ui";
import React from "react";
import Skeleton from "react-loading-skeleton";
import { useAppTheme } from "../../hooks/use-app-theme";

export type CardItemProps<T> = {
  title: string;
  subtitle?: string;
  hasAvatar?: boolean;
  loading?: boolean;
  selected?: boolean;
  item?: T;
  onClickItem?: (item: T) => void;
} & PaneProps;

export const CardItem: React.FunctionComponent<
  CardItemProps<unknown>
> = ({
  title,
  subtitle,
  hasAvatar = true,
  loading,
  selected,
  item,
  onClickItem,
  ...props
}) => {
  const theme = useAppTheme();
  const style = (selected: boolean) => {
    return selected
      ? { border: "1px solid " + theme.borderSelectedColor }
      : { border: "1px solid transparent" };
  };
  return (
    <Card
      maxWidth={400}
      minWidth={240}
      elevation={1}
      padding={minorScale(1)}
      display="flex"
      flexDirection="row"
      alignItems="center"
      style={{
        cursor: "pointer",
        ...style(selected),
      }}
      backgroundColor="white"
      onClick={() => onClickItem?.(item)}
      height={80}
      {...props}
    >
      <Pane
        height={80}
        display="flex"
        width={40}
        alignItems="center"
        marginX={theme.margin.medium}
      >
        {loading
          ? (
            <Skeleton circle={true} width={40} height={40} />
          )
          : (
            <Pane>
              {hasAvatar && <Avatar size={40} name={title} />}
            </Pane>
          )}
      </Pane>
      <Pane
        flexDirection="column"
        marginLeft={theme.margin.small}
        display="flex"
      >
        <Heading size={400}>{title}</Heading>
        {subtitle && <Text size={400}>
          {subtitle}
        </Text>}
      </Pane>
    </Card>
  );
};
