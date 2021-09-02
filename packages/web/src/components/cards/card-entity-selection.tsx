import {
  Alert,
  Button,
  Card,
  Combobox,
  EndorsedIcon,
  Heading,
  Icon,
  IntentTypes,
  Pane,
  PaneProps,
  PlusIcon,
} from "evergreen-ui";
import React from "react";
import Skeleton from "react-loading-skeleton";
import { useAppTheme } from "../../hooks/use-app-theme";
import { Closure } from "../../utils";
import { NumberIcon } from "../common/number-icon";

export type CardEntitySelectionProps<T> = {
  title: string;
  index?: number;
  loading?: boolean;
  selectedItem?: T;
  items?: T[];
  addItemTitle?: string;
  placeholder?: string;
  alert?: { intent: IntentTypes; content: string };
  itemLabel?: (item: T) => string;
  onSelectItem?: (item: T) => void;
  onAddItem?: Closure;
} & PaneProps;

export const CardEntitySelection: React.FunctionComponent<
  CardEntitySelectionProps<unknown>
> = ({
  title,
  index,
  loading = true,
  selectedItem,
  addItemTitle,
  placeholder,
  alert,
  items,
  itemLabel,
  onSelectItem,
  onAddItem,
}) => {
  const theme = useAppTheme();
  const style = (selected: boolean) => {
    return selected
      ? { border: "1px solid " + theme.borderSelectedColor }
      : { border: "1px solid transparent" };
  };
  return (
    <Card
      display="flex"
      flexDirection="column"
      marginTop={theme.margin.large}
      background="white"
      elevation={1}
      padding={theme.margin.large}
      style={{
        ...style(!!selectedItem),
      }}
    >
      <Pane display="flex" alignItems="center" justifyContent="space-between">
        <Pane display="flex" alignItems="center">
          <NumberIcon>{index}</NumberIcon>
          <Heading marginLeft={theme.margin.medium}>
            {title}
          </Heading>
        </Pane>
        <Pane>
          {selectedItem && <Icon
            icon={EndorsedIcon}
            color={"success"}
          />}
        </Pane>
      </Pane>

      {loading
        ? (
          <Pane
            flex={1}
            display="flex"
            justifyContent="space-between"
            marginTop={theme.margin.large}
          >
            <Skeleton count={1} height={30} width={150} />
            <Skeleton count={1} height={30} width={150} />
          </Pane>
        )
        : (
          <Pane>
            <Pane marginTop={theme.margin.large}>
              {alert &&
                <Alert
                  intent={alert.intent}
                  marginBottom={theme.margin.large}
                >
                  {alert.content}
                </Alert>}
            </Pane>
            <Pane
              display="flex"
              justifyContent="space-between"
              marginTop={theme.margin.large}
            >
              {/* Display an alert if necesary */}
              <Combobox
                openOnFocus
                initialSelectedItem={selectedItem}
                items={items}
                itemToString={(item) => itemLabel?.(item) || ""}
                onChange={(item) => onSelectItem?.(item)}
                placeholder={placeholder}
                disabled={items?.length === 0}
              />
              {addItemTitle && onAddItem &&
                <Button
                  marginLeft={theme.margin.large}
                  iconBefore={PlusIcon}
                  intent="none"
                  onClick={() => onAddItem()}
                  width={300}
                >
                  {addItemTitle}
                </Button>}
            </Pane>
          </Pane>
        )}
    </Card>
  );
};
