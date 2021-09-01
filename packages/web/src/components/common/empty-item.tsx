import { Pane, PaneProps, Text } from "evergreen-ui";
import React from "react";

export type EmptyItemsProps = PaneProps;

export const EmptyItem: React.FunctionComponent<EmptyItemsProps> = ({
  ...props
}) => (
  <Pane {...props}>
    <Text justifyItems="center">-</Text>
  </Pane>
);
