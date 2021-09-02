import {
  defaultTheme,
  Heading,
  minorScale,
  Pane,
  PaneProps,
} from "evergreen-ui";
import React from "react";

export type ProgressBarProps = {
  value: number;
  label?: string;
  rightLabel?: string;
  topLeftLabel?: string;
  topRightLabel?: string;
  tintColor?: string;
  rightTintColor?: string;
} & PaneProps;

export const ProgressBar: React.FunctionComponent<ProgressBarProps> = ({
  value = 0,
  label = "",
  rightLabel = "",
  topLeftLabel = "",
  topRightLabel = "",
  tintColor = defaultTheme.palette.green.base,
  rightTintColor = defaultTheme.palette.neutral.base,
  ...props
}) => {
  return (
    <Pane flex={1} display="flex" flexDirection="column" {...props}>
      {(topLeftLabel || topRightLabel) && (
        <Pane display="flex" flexDirection="row" justifyContent="space-between">
          <Heading size={100}>{topLeftLabel}</Heading>
          <Heading size={100}>{topRightLabel}</Heading>
        </Pane>
      )}
      <Pane
        height={15}
        display="flex"
        flexDirection="row"
        background={defaultTheme.palette.blue.light}
        borderRadius={20}
      >
        {value > 0 && (
          <Pane
            display="flex"
            alignItems="center"
            justifyContent="center"
            width={`${value}%`}
            background={tintColor}
            borderRadius={20}
          >
            <Heading size={300} color="white">
              {label}
            </Heading>
          </Pane>
        )}
        {rightLabel.length > 0 && (
          <Pane
            flex={1}
            display="flex"
            alignItems="center"
            justifyContent="flex-end"
            marginX={minorScale(2)}
          >
            <Heading size={300} color={rightTintColor}>
              {rightLabel}
            </Heading>
          </Pane>
        )}
      </Pane>
    </Pane>
  );
};
