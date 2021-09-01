import { majorScale, minorScale, Pane, TextInputField } from "evergreen-ui";
import React, { ReactNode } from "react";
import { TitleItem } from "./title-item";

export const FORM_INPUT_WIDTH = majorScale(43);
export const FORM_SMALL_INPUT_WIDTH = majorScale(10);

export const FormItem = (
  props: {
    children: ReactNode;
    title: string;
    tooltip?: string;
    error?: string;
  },
) => {
  const { children, title, tooltip, error } = props;
  return (
    <Pane
      marginTop={majorScale(2)}
      marginBottom={majorScale(4)}
      width={FORM_INPUT_WIDTH}
      {...props}
    >
      <TitleItem title={title} tooltip={tooltip} />
      <Pane marginTop={minorScale(3)}>{children}</Pane>
      {error && (
        <TextInputField label={""} isInvalid hidden validationMessage={error} />
      )}
    </Pane>
  );
};
