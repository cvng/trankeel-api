import {
  FormField,
  FormFieldProps,
  Textarea,
  TextareaProps,
} from "evergreen-ui";
import React from "react";
import { splitFieldProps } from "./utils";

export type TextareaFieldProps = FormFieldProps & TextareaProps;

export const TextareaField = ({
  // Textarea props
  style = { resize: "none" },
  // Rest props are spread on the FormField
  ...props
}: TextareaFieldProps) => {
  const id = `TextareaField-${props.name}`;

  const { fieldProps, remainingProps } = splitFieldProps(props);

  return (
    <FormField labelFor={id} {...fieldProps}>
      <Textarea id={id} style={style} {...remainingProps} />
    </FormField>
  );
};
