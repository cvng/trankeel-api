import {
  FormField,
  FormFieldProps,
  SegmentedControl,
  SegmentedControlProps,
} from "evergreen-ui";
import React from "react";
import { splitFieldProps } from "./utils";

export type SegmentedControlFieldProps = FormFieldProps & SegmentedControlProps;

export const SegmentedControlField = ({
  // Rest props are spread on the FormField
  ...props
}: SegmentedControlFieldProps) => {
  const id = `SegmentedControlField-${props.name}`;

  const { fieldProps, remainingProps } = splitFieldProps(props);

  const { isInvalid, ...segmentedFieldProps } = remainingProps;

  return (
    <FormField labelFor={id} {...fieldProps}>
      <SegmentedControl id={id} {...segmentedFieldProps} />
    </FormField>
  );
};
