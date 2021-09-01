import { defaultTheme, majorScale, minorScale, Theme } from "evergreen-ui";

type AppThemeProps = {
  appNameColor: string;
  menuSelectedColor: string;
  menuUnselectedColor: string;
  menuBackgroundColor: string;
  margin: {
    small: number;
    medium: number;
    large: number;
    largest: number;
  };
  color: string;
  accentColor: string;
  secondaryAccentColor: string;
  lightBackgroundColor: string;
  borderSelectedColor: string;
} & Theme;

export const AppTheme: Theme & AppThemeProps = {
  ...defaultTheme,
  appNameColor: "white",
  menuSelectedColor: "white",
  menuUnselectedColor: "#9DA9D5",
  menuBackgroundColor: defaultTheme.palette.blue.dark,
  margin: {
    small: minorScale(1),
    medium: majorScale(1),
    large: majorScale(3),
    largest: majorScale(4),
  },
  color: defaultTheme.palette.neutral.base,
  accentColor: defaultTheme.palette.blue.base,
  secondaryAccentColor: defaultTheme.palette.green.base,
  lightBackgroundColor: defaultTheme.palette.blue.lightest,
  borderSelectedColor: "#1070CA",
};
