import { DocumentIcon, PlusIcon, ThemeProvider } from "evergreen-ui";
import React from "react";
import { AppTheme } from "../../theme/app-theme";
import { PopinMenuButton } from "./popin-menu-button";
import { translate } from "piteo-kit";

const _ = translate();

export default {
  title: "Design System/Buttons/PopinMenuButton",
  component: PopinMenuButton,
};

export const standard = () => (
  <ThemeProvider value={AppTheme}>
    <PopinMenuButton
      items={[
        {
          title: "Type de bail",
          bottomDivider: true,
          subItems: [
            {
              name: `${_("lease_furnished")}...`,
              handler: () => {},
              icon: DocumentIcon,
            },
            {
              name: `${_("lease_nude")}...`,
              handler: () => {},
              icon: DocumentIcon,
            },
          ],
        },
        {
          subItems: [
            {
              name: `${_("lease_furnished")}...`,
              handler: () => {},
              icon: DocumentIcon,
            },
            {
              name: `${_("lease_nude")}...`,
              handler: () => {},
              icon: DocumentIcon,
            },
          ],
        },
      ]}
      buttonIcon={PlusIcon}
    />
  </ThemeProvider>
);
