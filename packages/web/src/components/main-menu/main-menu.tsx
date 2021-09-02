import { Heading, Icon, LockIcon, Pane, Pill, SidebarTab } from "evergreen-ui";
import React from "react";
import { useMediaQuery } from "react-responsive";
import { Dimens } from "../../constants";
import { TourType } from "../../constants/tour-constants";
import { VersionNumberHelper } from "../../helpers";
import { useAppTheme } from "../../hooks/use-app-theme";
import { AppTheme } from "../../theme/app-theme";
import { translate } from "piteo-kit";

const _ = translate();

export const MAIN_MENU_WIDTH = 140;
export const MAIN_MENU_LARGE_WIDTH = 200;
const MAIN_MENU_DISABLED_OPACITY = 0.3;

export type MainMenuItem = {
  title: string;
  link?: string;
  selected?: boolean;
  icon?: React.ElementType;
  badge?: number;
  collapsed?: boolean;
};

export type MainMenuProps = {
  items?: MainMenuItem[];
  onSelectMenuItem?: (item: MainMenuItem) => void;
  onSelectLogo?: () => void;
  disabled?: boolean;
};

const renderItem = (
  items: MainMenuItem[],
  callback: (item: MainMenuItem) => void,
  disabled: boolean,
  theme: typeof AppTheme,
): React.ReactNode => {
  return (
    <Pane>
      {items?.map((menuItem, index) => (
        <Pane key={index} height={50} marginY={theme.margin.medium}>
          <SidebarTab
            key={index}
            onSelect={() => callback?.(menuItem)}
            aria-controls={`panel-${menuItem.title}`}
            color={menuItem.selected
              ? theme.menuSelectedColor
              : theme.menuUnselectedColor}
            margin={0}
            padding={0}
          >
            <Pane
              display="flex"
              flexDirection="row"
              justifyContent="space-between"
              alignItems="center"
              background={menuItem.selected ? theme.menuSelectedColor
              : theme.menuBackgroundColor}
              minHeight={50}
              borderRadius={4}
              marginX={theme.margin.small}
            >
              <Pane
                display="flex"
                flexDirection="row"
                alignItems="center"
                padding={theme.margin.medium}
              >
                {/* Menu icon */}
                <Icon
                  icon={disabled ? LockIcon : menuItem.icon}
                  color={menuItem.selected ? theme.accentColor
                  : theme.menuUnselectedColor}
                  marginRight={theme.margin.medium}
                />
                <Heading
                  size={400}
                  color={menuItem.selected ? theme.accentColor
                  : theme.menuUnselectedColor}
                >
                  {menuItem.title}
                </Heading>
              </Pane>
              {/* Badge */}
              {menuItem.badge > 0 &&
                <Pill color="red" isSolid marginX={theme.margin.small}>
                  {menuItem.badge}
                </Pill>}
            </Pane>
          </SidebarTab>
        </Pane>
      ))}
    </Pane>
  );
};

export const MainMenu: React.FunctionComponent<MainMenuProps> = ({
  items,
  onSelectMenuItem,
  onSelectLogo,
  disabled,
}) => {
  const theme = useAppTheme();
  const isDesktopOrLaptop = useMediaQuery({
    minDeviceWidth: Dimens.LARGE_TABLET_WIDTH,
  });
  const width = isDesktopOrLaptop ? MAIN_MENU_LARGE_WIDTH : MAIN_MENU_WIDTH;
  return (
    <Pane
      display="flex"
      flexDirection="column"
      backgroundColor={theme.menuBackgroundColor}
      borderRight="muted"
      width={width}
      paddingX={theme.margin.small}
    >
      {/* Logo */}
      <Pane
        minHeight={64}
        display="flex"
        flexDirection="row"
        justifyContent="center"
        alignItems="center"
        onClick={onSelectLogo}
        style={{ cursor: "pointer" }}
        paddingX={theme.margin.medium}
      >
        <Heading
          size={600}
          color={theme.appNameColor}
          style={{ letterSpacing: 3 }}
        >
          {_("app_name")}
        </Heading>
      </Pane>

      {/* Menu */}
      <Pane
        className={TourType.MAIN_MENU}
        opacity={disabled ? MAIN_MENU_DISABLED_OPACITY : 1}
        marginTop={theme.margin.largest}
      >
        {renderItem(items, onSelectMenuItem, disabled, theme)}
      </Pane>

      <Pane
        display="flex"
        flex={1}
        justifyContent="center"
        alignItems="flex-end"
        paddingBottom={theme.margin.medium}
      >
        <Heading size={100} color="white">
          {VersionNumberHelper.versionNumber()}
        </Heading>
      </Pane>
    </Pane>
  );
};
