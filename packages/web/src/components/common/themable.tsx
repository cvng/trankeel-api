import { ThemeProvider } from "evergreen-ui";
import * as React from "react";
import { AppTheme } from "../../theme/app-theme";

export type ThemableProps = {
  /** Children component */
  children?: React.ReactNode;
};

export const Themable: React.FunctionComponent<ThemableProps> = ({
  children,
}) => (
  <ThemeProvider value={AppTheme}>
    {children}
  </ThemeProvider>
);
