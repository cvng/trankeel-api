import { majorScale } from "evergreen-ui";
import { splitBoxProps } from "ui-box";

export function splitFieldProps({
  hint,
  label,
  description,
  validationMessage,
  ...props
  // deno-lint-ignore no-explicit-any
}: any): { fieldProps: any; remainingProps: any } {
  const { matchedProps, remainingProps } = splitBoxProps(props);

  // https://github.com/segmentio/evergreen/blob/master/src/text-input/src/TextInputField.js
  const fieldProps = {
    marginBottom: majorScale(3),
    hint,
    label,
    description,
    validationMessage,
    isRequired: props.required,
    ...matchedProps,
  };

  return { fieldProps: fieldProps, remainingProps: remainingProps };
}
