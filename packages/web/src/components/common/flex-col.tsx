import { Pane, PaneProps } from "evergreen-ui";
import * as React from "react";

export type FlexColProps = {
  /** Children component */
  children?: React.ReactNode;
} & PaneProps;

export const FlexCol: React.FunctionComponent<FlexColProps> = ({
  children,
  ...props
}) => (
  <Pane display="flex" flexDirection="row" {...props}>
    {children}
  </Pane>
);
