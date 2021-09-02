import { defaultTheme, Heading, HeadingProps } from "evergreen-ui";
import * as React from "react";
import { NumberHelper } from "../../helpers";

export enum AmountLabelType {
  SMALL,
  MEDIUM,
  BIG,
}

export type AmountLabelProps = {
  type?: AmountLabelType;
  value?: number;
  addSign?: boolean;
} & HeadingProps;

const getColor = (value: number): string => {
  var color = defaultTheme.palette.neutral.dark;
  if (value < 0) {
    color = defaultTheme.palette.red.dark;
  }
  return color;
};

const getSize = (type: AmountLabelType): 300 | 400 | 700 => {
  if (type === AmountLabelType.MEDIUM) {
    return 400;
  } else if (type === AmountLabelType.BIG) {
    return 700;
  }
  return 300; // SMALL
};

export const AmountLabel: React.FunctionComponent<AmountLabelProps> = ({
  type = AmountLabelType.SMALL,
  value,
  addSign = false,
  ...props
}) => {
  return (
    <Heading {...props} size={getSize(type)} color={getColor(value)}>
      {NumberHelper.formatToString(value, addSign)}
    </Heading>
  );
};
