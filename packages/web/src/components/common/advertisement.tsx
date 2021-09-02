import {
  Button,
  ChatIcon,
  majorScale,
  minorScale,
  Pane,
  Text,
} from "evergreen-ui";
import * as React from "react";
import { useAppTheme } from "../../hooks/use-app-theme";
import { translate } from "piteo-kit";
import { useMediaQuery } from "react-responsive";
import { Dimens } from "../../constants";

const _ = translate();

export type AdvertisementProps = {
  title: string;
  buttonTitle?: string;
};

export const Advertisement: React.FunctionComponent<AdvertisementProps> = ({
  title,
  buttonTitle,
}) => {
  const theme = useAppTheme();
  const isMobile = useMediaQuery({ maxWidth: Dimens.MOBILE_MAX_WIDTH });
  if (isMobile) {
    return null;
  }
  return (
    <React.Fragment>
      <Pane
        height={majorScale(5)}
        padding={minorScale(2)}
        display="flex"
        flexDirection="row"
        alignItems="center"
        justifyContent="center"
        borderBottom="muted"
        background={theme.palette.blue.light}
      >
        <Text size={400}>{title}</Text>
        {buttonTitle && (
          <Button
            iconAfter={ChatIcon}
            marginLeft={majorScale(2)}
            appearance="primary"
            intent="success"
            padding={theme.margin.medium}
            height={30}
            width={150}
            minWidth={150}
            // onClick={() => $crisp.push(["do", "chat:toggle"])}
            onClick={() => {
              window.open(
                _("send_feedback_email", {
                  email: _("app_email_contact"),
                }),
              );
            }}
          >
            {buttonTitle}
          </Button>
        )}
      </Pane>
    </React.Fragment>
  );
};
