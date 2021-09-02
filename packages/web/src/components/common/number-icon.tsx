import {
  defaultTheme,
  Icon,
  LockIcon,
  Pane,
  Text,
  TickCircleIcon,
} from "evergreen-ui";
import * as React from "react";

export type NumberIconProps = {
  children: React.ReactNode;
  valid?: boolean;
  disabled?: boolean;
  size?: number;
};

export const NumberIcon: React.FunctionComponent<NumberIconProps> = ({
  children,
  valid,
  disabled,
  size,
  ...props
}) => (
  <Pane>
    <Pane display="flex" flexDirection="row" position="relative">
      {valid && (
        <Icon
          icon={TickCircleIcon}
          color="success"
          position="absolute"
          right={-9}
          top={-4}
        />
      )}
      {disabled && (
        <Icon
          icon={LockIcon}
          color={"gray"}
          position="absolute"
          right={-9}
          top={-4}
        />
      )}
      <Text
        borderRadius="50%"
        width={size || 30}
        height={size || 30}
        background={disabled
          ? defaultTheme.palette.neutral.light
          : defaultTheme.palette.blue.base}
        display="flex"
        justifyContent="center"
        alignItems="center"
        color={"white"}
      >
        {children}
      </Text>
    </Pane>
  </Pane>
);
