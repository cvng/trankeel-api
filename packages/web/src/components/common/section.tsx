import {
  defaultTheme,
  Heading,
  Icon,
  majorScale,
  Pane,
  PaneProps,
} from "evergreen-ui";
import React, { Children } from "react";

export type SectionProps = {
  /** Section title */
  title?: string;
  /** Children component */
  children?: React.ReactNode;
  /** Icon */
  icon?: React.ElementType;
  /** Number */
  leftAccesory?: React.ReactNode;
  /** Show border line */
  showBorderLine?: boolean;
} & PaneProps;

export const Section: React.FunctionComponent<SectionProps> = ({
  children,
  title = null,
  icon,
  leftAccesory,
  showBorderLine = true,
  ...props
}) => (
  <Pane borderBottom={showBorderLine} marginBottom={majorScale(3)} {...props}>
    <Pane display="flex" alignItems="center" marginBottom={majorScale(3)}>
      {icon &&
        <Icon
          icon={icon}
          color={defaultTheme.palette.blue.base}
          marginRight={10}
        />}
      {leftAccesory}
      {/* Section heading */}
      <Heading>{title}</Heading>
    </Pane>
    {/* Section content */}
    {children}
  </Pane>
);

export type RowProps = {
  /** Grid columns */
  columns?: number;
} & PaneProps;

export const Row: React.FunctionComponent<RowProps> = ({
  children,
  columns = null,
  ...props
}) => (
  <Pane
    display="grid"
    gridTemplateColumns={`repeat(${columns || Children.count(children)}, 1fr)`}
    gridGap={majorScale(2)}
    {...props}
  >
    {children}
  </Pane>
);
