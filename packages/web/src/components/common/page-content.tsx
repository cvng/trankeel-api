import { defaultTheme, Heading, majorScale, Pane } from "evergreen-ui";
import * as React from "react";
import { useAppTheme } from "../../hooks/use-app-theme";
import { ScrollableContent } from "./scrollable-content";

export const NAV_BAR_HEIGHT = majorScale(8);

export type PageContentProps = {
  children: React.ReactNode;
  title: string;
  titleRightView?: React.ReactNode;
  adverstisement?: React.ReactNode; // cf. AdvertisementComponent
  disablePadding?: boolean;
};

export const PageContent: React.FunctionComponent<PageContentProps> = ({
  children,
  title,
  titleRightView,
  adverstisement,
  disablePadding = false,
}) => {
  const theme = useAppTheme();
  return (
    <ScrollableContent>
      <Pane display="flex" flex={1} flexDirection="column">
        <Pane display="flex" flex={1} flexDirection="column">
          <Pane
            borderBottom="muted"
            padding={theme.margin.large}
            display="flex"
            height={NAV_BAR_HEIGHT}
            flexDirection="row"
            alignItems="center"
            justifyContent="space-between"
            background="white"
            style={{ position: "sticky", top: 0, zIndex: 1 }}
          >
            {/* Titre */}
            <Heading size={600} marginRight={theme.margin.large} is="h1">
              {title}
            </Heading>
            {/* Zone de droite */}
            <Pane>{titleRightView}</Pane>
          </Pane>
          {adverstisement != null && adverstisement}
          <Pane
            display="flex"
            flex={1}
            flexDirection="column"
            padding={disablePadding ? 0 : theme.margin.large}
            background={defaultTheme.palette.blue.lightest}
          >
            {children}
          </Pane>
        </Pane>
      </Pane>
    </ScrollableContent>
  );
};
