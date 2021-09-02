import {
  ArrowDownIcon,
  ArrowUpIcon,
  defaultTheme,
  Icon,
  Pane,
  Text,
} from "evergreen-ui";
import React from "react";
import { NumberHelper } from "../../helpers";

export type VariationComponentProps = {
  value?: number;
};

export const VariationComponent: React.FunctionComponent<
  VariationComponentProps
> = ({
  value,
}) => {
  return (
    <Pane>
      {!!value && (
        <Pane>
          <Icon
            icon={value > 0 ? ArrowUpIcon : ArrowDownIcon}
            size={10}
            color={value > 0
              ? defaultTheme.palette.green.dark
              : defaultTheme.palette.red.dark}
            marginX={2}
          />
          <Text
            size={300}
            color={value > 0
              ? defaultTheme.palette.green.dark
              : defaultTheme.palette.red.dark}
          >
            {NumberHelper.formatToPercentage(value)}
          </Text>
        </Pane>
      )}
    </Pane>
  );
};
