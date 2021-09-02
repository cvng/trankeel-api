import { Heading, Pane, Paragraph } from "evergreen-ui";
import * as React from "react";
import { translate } from "piteo-kit";
import { useAppTheme } from "../../hooks/use-app-theme";

const _ = translate();

export const AppUnavailable: React.FunctionComponent = () => {
  const theme = useAppTheme();
  return (
    <Pane
      display="flex"
      flexDirection="column"
      alignItems="center"
      justifyItems="center"
      margin={theme.margin.largest}
    >
      <Pane width={window.innerWidth / 2} alignItems={"center"}>
        <Heading size={400}>{_("app_unavailable")}</Heading>
        <Paragraph size={400}>{_("app_unavailable_message")}</Paragraph>
      </Pane>
    </Pane>
  );
};
