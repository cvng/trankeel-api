import { Pane, PaneProps } from "evergreen-ui";
import * as React from "react";

export type FlexRowProps = {
  /** Children component */
  children?: React.ReactNode;
} & PaneProps;

export const FlexRow: React.FunctionComponent<FlexRowProps> = ({
  children,
  ...props
}) => (
  <Pane display="flex" flexDirection="row" {...props}>
    {children}
  </Pane>
);
