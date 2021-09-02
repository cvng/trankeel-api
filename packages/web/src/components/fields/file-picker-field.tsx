import {
  FilePicker,
  FilePickerOwnProps,
  FormField,
  FormFieldProps,
  SelectOwnProps,
} from "evergreen-ui";
import React from "react";
import { translate } from "piteo-kit";
import { splitFieldProps } from "./utils";

const _ = translate();

export type FilePickerFieldProps =
  & FormFieldProps
  & SelectOwnProps
  & FilePickerOwnProps;

export const FilePickerField = ({
  // We are not using the isInvalid from the props
  isInvalid: unusedIsInvalid,
  // FilePicker props
  placeholder = _("file_placeholder"),
  // Rest props are spread on the FormField
  ...props
}: FilePickerFieldProps) => {
  const id = `FilePickerField`;

  const { fieldProps, remainingProps } = splitFieldProps(props);

  return (
    <FormField labelFor={id} {...fieldProps}>
      <FilePicker id={id} placeholder={placeholder} {...remainingProps} />
    </FormField>
  );
};
