import { Heading, majorScale, Pane, Paragraph } from "evergreen-ui";
import React from "react";
import { useAppTheme } from "../../hooks/use-app-theme";
import { CardItem } from "../cards/card-item";

export type ListItemSelectionProps<T> = {
  items: T[];
  title: string;
  subtitle: string;
  selectedItem?: T;
  hasAvatar?: boolean;
  footer?: React.ReactNode;
  titleForItem(item: T): string;
  subtitleForItem?(item: T): string;
  onSelectItem?(item: T): void;
};

export const ListItemSelection: React.FunctionComponent<
  ListItemSelectionProps<{ id: string }>
> = ({
  items,
  title,
  subtitle,
  selectedItem,
  hasAvatar = true,
  footer,
  titleForItem,
  subtitleForItem,
  onSelectItem,
}) => {
  const theme = useAppTheme();
  return (
    <Pane
      display="flex"
      flexDirection="column"
      alignItems="flex-start"
    >
      <Pane marginBottom={majorScale(2)}>
        {title && <Heading size={600}>{title}</Heading>}
        {subtitle && <Paragraph size={400}>{subtitle}</Paragraph>}
      </Pane>

      <Pane overflowY={"auto"}>
        {items?.map((item) => (
          <CardItem
            key={item.id}
            title={titleForItem(item)}
            subtitle={subtitleForItem && subtitleForItem(item)}
            selected={item?.id === selectedItem?.id}
            hasAvatar={hasAvatar}
            onClickItem={onSelectItem}
            item={item}
            marginY={theme.margin.medium}
          />
        ))}
        {footer}
      </Pane>
    </Pane>
  );
};
