import { Card, Pane, TextInputField } from "evergreen-ui";
import React, { useContext } from "react";
import { AuthContext } from "../../context/auth-context";
import { useAppTheme } from "../../hooks/use-app-theme";
import { CardItem } from "../cards";
import { translate } from "piteo-kit";

const _ = translate();

const FIELD_WIDTH = 300;

export type SettingsAccountProps = unknown;

export const SettingsAccount: React.FunctionComponent<
  SettingsAccountProps
> = () => {
  const theme = useAppTheme();
  const context = useContext(AuthContext);
  return (
    <Pane
      display="flex"
      flex={1}
      background={theme.colors.background.tint1}
      justifyContent="center"
      padding={theme.margin.largest}
    >
      <Card
        display="flex"
        elevation={1}
        minWidth={400}
        maxHeight={400}
        flexDirection="column"
        alignItems="center"
        justifyContent="flex-start"
        background={"white"}
      >
        <Pane
          display="flex"
          justifyContent="flex-start"
          margin={theme.margin.large}
        >
          <CardItem
            title={context?.user?.displayName}
            elevation={null}
            backgroundColor={null}
            style={{}} // Remove the cursor
          />
        </Pane>

        <TextInputField
          label={_("owner_name")}
          width={FIELD_WIDTH}
          value={context?.user?.displayName}
          disabled
        />

        <TextInputField
          label={_("input_email_label")}
          type="email"
          hint={_("email_informations")}
          width={FIELD_WIDTH}
          value={context?.user?.email}
          disabled
        />
      </Card>
    </Pane>
  );
};
