import {
  ChevronRightIcon,
  Heading,
  Icon,
  Pane,
  Pill,
  Text,
  TickCircleIcon,
} from "evergreen-ui";
import React from "react";
import { useAppTheme } from "../../hooks/use-app-theme";

const DISABLED_OPACITY = 0.5;
const ENABLED_OPACITY = 1;

export type CheckListItemProps = {
  title: string;
  checked: boolean;
  disabled: boolean;
  selected: boolean;
  subtitle?: string;
  index?: number;
  onClick?: (index: number) => void;
};

export const CheckListItem: React.FunctionComponent<CheckListItemProps> = ({
  index = 1,
  title = "",
  subtitle = "",
  checked = false,
  disabled = false,
  selected = false,
  onClick,
}) => {
  const theme = useAppTheme();
  const borderColor = selected ? theme.accentColor : "white";
  return (
    <Pane
      display="flex"
      alignItems="center"
      height={80}
      paddingX={theme.margin.large}
      opacity={disabled ? DISABLED_OPACITY : ENABLED_OPACITY}
      borderLeft={"3px solid " + borderColor}
      borderBottom="muted"
      onClick={() => {
        !disabled && onClick(index);
      }}
      style={{ cursor: "pointer" }}
      background={selected ? theme.lightBackgroundColor : "none"}
    >
      <Pill color={selected || checked ? "blue" : "neutral"} isSolid={selected}>
        {index}
      </Pill>
      <Pane
        display="flex"
        flexDirection="column"
        marginLeft={theme.margin.large}
      >
        <Heading size={300} color={selected ? theme.accentColor : theme.color}>
          {title}
        </Heading>
        {subtitle && <Text size={300}>{subtitle}</Text>}
      </Pane>
      <Pane display="flex" flex={1} justifyContent="flex-end">
        {checked
          ? (<Icon icon={TickCircleIcon} color={theme.secondaryAccentColor} />)
          : (<Icon icon={ChevronRightIcon} color={theme.accentColor} />)}
      </Pane>
    </Pane>
  );
};
